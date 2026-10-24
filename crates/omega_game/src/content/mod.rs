//! Typed content catalogs and immutable validated snapshots (SP-003, ADR-006).
//!
//! A bundle is `manifest.json` plus one UTF-8 JSON file per family. Loading parses
//! every family, collects record/field errors, cross-validates, and publishes a
//! snapshot only when the whole bundle is valid.

pub mod model;
mod reader;
pub mod validate;

use model::*;
use reader::Rec;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fmt;
use std::path::Path;
use std::sync::Arc;

pub const SCHEMA_VERSION: i64 = 1;

/// Family key in the manifest and its required file name.
pub const FAMILIES: [&str; 9] = [
	"characters",
	"weapons",
	"items",
	"enemies",
	"bosses",
	"arenas",
	"waves",
	"shop",
	"unlocks",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentError {
	pub file: String,
	/// `record_id.field[index].field`; `records` or `file` for file-level problems.
	pub path: String,
	pub message: String,
}

impl ContentError {
	pub fn new(file: &str, path: &str, message: String) -> Self {
		Self {
			file: file.to_string(),
			path: path.to_string(),
			message,
		}
	}
}

impl fmt::Display for ContentError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}: {}: {}", self.file, self.path, self.message)
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentErrors(pub Vec<ContentError>);

impl fmt::Display for ContentErrors {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for e in &self.0 {
			writeln!(f, "{e}")?;
		}
		Ok(())
	}
}

impl std::error::Error for ContentErrors {}

/// Chosen explicitly by the caller; a full release never runs in prototype mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Profile {
	Prototype,
	Full,
}

impl std::str::FromStr for Profile {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"prototype" => Ok(Self::Prototype),
			"full" => Ok(Self::Full),
			other => Err(format!(
				"unknown profile `{other}`, expected prototype or full"
			)),
		}
	}
}

/// Records keyed by stable ID (waves by number), in deterministic order.
#[derive(Debug, Clone, PartialEq)]
pub struct Catalogs {
	pub characters: BTreeMap<String, Character>,
	pub weapons: BTreeMap<String, Weapon>,
	pub items: BTreeMap<String, Item>,
	pub enemies: BTreeMap<String, Enemy>,
	pub bosses: BTreeMap<String, Boss>,
	pub arenas: BTreeMap<String, Arena>,
	pub waves: BTreeMap<u32, Wave>,
	pub shop: ShopRules,
	pub unlocks: BTreeMap<String, Unlock>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContentSnapshot {
	/// SHA-256 over the manifest and family files in manifest order.
	pub revision: [u8; 32],
	pub profile: Profile,
	pub catalogs: Catalogs,
}

impl ContentSnapshot {
	pub fn revision_hex(&self) -> String {
		self.revision.iter().map(|b| format!("{b:02x}")).collect()
	}
}

fn parse_json(file: &str, bytes: &[u8]) -> Result<Value, ContentError> {
	let text = std::str::from_utf8(bytes)
		.map_err(|_| ContentError::new(file, "file", "not valid UTF-8".into()))?;
	serde_json::from_str(text)
		.map_err(|e| ContentError::new(file, "file", format!("invalid JSON: {e}")))
}

fn header<'v>(file: &'v str, value: &'v Value) -> Result<Rec<'v>, ContentError> {
	let mut r = Rec::new(file, String::new(), value)?;
	r.int("schema_version", SCHEMA_VERSION, SCHEMA_VERSION)?;
	Ok(r)
}

/// Parses one family into records keyed by `key`, collecting every record error.
fn family<T>(
	file: &str,
	value: &Value,
	parse: fn(Rec) -> Result<T, ContentError>,
	key: fn(&T) -> String,
	errors: &mut Vec<ContentError>,
) -> Vec<(String, T)> {
	let mut out = Vec::new();
	let mut r = match header(file, value) {
		Ok(r) => r,
		Err(e) => {
			errors.push(e);
			return out;
		}
	};
	let records = match r.array("records") {
		Ok(v) => v,
		Err(e) => {
			errors.push(e);
			return out;
		}
	};
	if let Err(e) = r.finish() {
		errors.push(e);
	}
	for (i, v) in records.iter().enumerate() {
		let label = v
			.get("id")
			.and_then(Value::as_str)
			.filter(|s| reader::valid_id(s))
			.map_or_else(|| format!("records[{i}]"), str::to_string);
		match Rec::new(file, label, v).and_then(parse) {
			Ok(t) => out.push((key(&t), t)),
			Err(e) => errors.push(e),
		}
	}
	out
}

fn keyed<T>(
	file: &str,
	records: Vec<(String, T)>,
	errors: &mut Vec<ContentError>,
) -> BTreeMap<String, T> {
	let mut map = BTreeMap::new();
	for (id, t) in records {
		if map.insert(id.clone(), t).is_some() {
			errors.push(ContentError::new(
				file,
				&format!("{id}.id"),
				"duplicate id".into(),
			));
		}
	}
	map
}

/// Validates a bundle given as `file name -> bytes`. Nothing is published on error.
pub fn load_bundle(
	files: &BTreeMap<String, Vec<u8>>,
	profile: Profile,
) -> Result<ContentSnapshot, ContentErrors> {
	let mut errors = Vec::new();
	let manifest_bytes = files.get("manifest.json").ok_or_else(|| {
		ContentErrors(vec![ContentError::new(
			"manifest.json",
			"file",
			"missing manifest".into(),
		)])
	})?;
	let manifest =
		parse_json("manifest.json", manifest_bytes).map_err(|e| ContentErrors(vec![e]))?;
	let mut hasher = Sha256::new();
	hasher.update(manifest_bytes);
	let mut names = BTreeMap::new();
	{
		let mut r = header("manifest.json", &manifest).map_err(|e| ContentErrors(vec![e]))?;
		let mut f = r.object("files").map_err(|e| ContentErrors(vec![e]))?;
		for fam in FAMILIES {
			match f.choice(fam, &[&format!("{fam}.json")]) {
				Ok(name) => {
					names.insert(fam, name);
				}
				Err(e) => errors.push(e),
			}
		}
		for e in [f.finish(), r.finish()].into_iter().filter_map(Result::err) {
			errors.push(e);
		}
	}
	if !errors.is_empty() {
		return Err(ContentErrors(errors));
	}
	let mut values = BTreeMap::new();
	for fam in FAMILIES {
		let name = &names[fam];
		let Some(bytes) = files.get(name) else {
			errors.push(ContentError::new(
				name,
				"file",
				"missing family file".into(),
			));
			continue;
		};
		hasher.update(name.as_bytes());
		hasher.update(bytes);
		match parse_json(name, bytes) {
			Ok(v) => {
				values.insert(fam, v);
			}
			Err(e) => errors.push(e),
		}
	}
	if !errors.is_empty() {
		return Err(ContentErrors(errors));
	}
	let e = &mut errors;
	let characters = keyed(
		"characters.json",
		family(
			"characters.json",
			&values["characters"],
			character,
			|r| r.id.clone(),
			e,
		),
		e,
	);
	let weapons = keyed(
		"weapons.json",
		family(
			"weapons.json",
			&values["weapons"],
			weapon,
			|r| r.id.clone(),
			e,
		),
		e,
	);
	let items = keyed(
		"items.json",
		family("items.json", &values["items"], item, |r| r.id.clone(), e),
		e,
	);
	let enemies = keyed(
		"enemies.json",
		family(
			"enemies.json",
			&values["enemies"],
			enemy,
			|r| r.id.clone(),
			e,
		),
		e,
	);
	let bosses = keyed(
		"bosses.json",
		family("bosses.json", &values["bosses"], boss, |r| r.id.clone(), e),
		e,
	);
	let arenas = keyed(
		"arenas.json",
		family("arenas.json", &values["arenas"], arena, |r| r.id.clone(), e),
		e,
	);
	let mut waves = BTreeMap::new();
	for (_, w) in family("waves.json", &values["waves"], wave, |r| r.id.clone(), e) {
		let (number, id) = (w.number, w.id.clone());
		if waves.insert(number, w).is_some() {
			e.push(ContentError::new(
				"waves.json",
				&format!("{id}.number"),
				format!("wave number {number} repeats"),
			));
		}
	}
	let mut shops = family(
		"shop.json",
		&values["shop"],
		shop_rules,
		|r| r.id.clone(),
		e,
	);
	let unlocks = keyed(
		"unlocks.json",
		family(
			"unlocks.json",
			&values["unlocks"],
			unlock,
			|r| r.id.clone(),
			e,
		),
		e,
	);
	if shops.len() != 1 {
		e.push(ContentError::new(
			"shop.json",
			"records",
			"exactly one shop rules record is required".into(),
		));
	}
	if !errors.is_empty() {
		return Err(ContentErrors(errors));
	}
	let catalogs = Catalogs {
		characters,
		weapons,
		items,
		enemies,
		bosses,
		arenas,
		waves,
		shop: shops.remove(0).1,
		unlocks,
	};
	validate::validate(&catalogs, profile, &mut errors);
	if !errors.is_empty() {
		return Err(ContentErrors(errors));
	}
	Ok(ContentSnapshot {
		revision: hasher.finalize().into(),
		profile,
		catalogs,
	})
}

/// Reads `manifest.json` and every `*.json` file in `dir`, then validates.
pub fn load_dir(dir: &Path, profile: Profile) -> Result<ContentSnapshot, ContentErrors> {
	let io = |e: std::io::Error| {
		ContentErrors(vec![ContentError::new(
			&dir.display().to_string(),
			"file",
			e.to_string(),
		)])
	};
	let mut files = BTreeMap::new();
	for entry in std::fs::read_dir(dir).map_err(io)? {
		let path = entry.map_err(io)?.path();
		if path.extension().is_some_and(|e| e == "json")
			&& let Some(name) = path.file_name().and_then(|n| n.to_str())
		{
			files.insert(name.to_string(), std::fs::read(&path).map_err(io)?);
		}
	}
	load_bundle(&files, profile)
}

/// Where a reload is requested from. Only Menu may swap content (GDD-012).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReloadContext {
	Menu,
	ActiveRun,
}

#[derive(Debug)]
pub enum ReloadError {
	NotInMenu,
	Invalid(ContentErrors),
}

impl fmt::Display for ReloadError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::NotInMenu => f.write_str("content can only be reloaded from the menu"),
			Self::Invalid(e) => write!(f, "candidate content is invalid:\n{e}"),
		}
	}
}

impl std::error::Error for ReloadError {}

/// Holds the published snapshot. A run keeps its own `Arc`, so a later swap never
/// changes the revision an active run uses.
#[derive(bevy::prelude::Resource, Debug, Clone)]
pub struct ContentStore {
	current: Arc<ContentSnapshot>,
}

impl ContentStore {
	pub fn new(snapshot: ContentSnapshot) -> Self {
		Self {
			current: Arc::new(snapshot),
		}
	}

	pub fn current(&self) -> Arc<ContentSnapshot> {
		Arc::clone(&self.current)
	}

	/// Validates a complete candidate and swaps only on success.
	pub fn reload(
		&mut self,
		files: &BTreeMap<String, Vec<u8>>,
		context: ReloadContext,
	) -> Result<Arc<ContentSnapshot>, ReloadError> {
		if context != ReloadContext::Menu {
			return Err(ReloadError::NotInMenu);
		}
		let candidate = load_bundle(files, self.current.profile).map_err(ReloadError::Invalid)?;
		self.current = Arc::new(candidate);
		Ok(self.current())
	}
}
