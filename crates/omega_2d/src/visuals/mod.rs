//! ENG-VIS-001: typed visual catalog, validation, lookup/fallback and pose projection.
//!
//! Gameplay never reads image dimensions: a record's world extent and pivot are the
//! only presentation geometry, and they come from catalog metadata, not textures.

use bevy::math::DVec2;
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

#[cfg(feature = "client")]
mod client;
#[cfg(feature = "client")]
pub use client::{SpriteParts, pose_transform, sprite_parts};

pub mod cues;

pub const CATALOG_SCHEMA_VERSION: u32 = 1;

/// Stable presentation ID, lowercase ASCII `[a-z][a-z0-9_]*`, independent of file paths.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(try_from = "String")]
pub struct VisualId(String);

impl VisualId {
	pub fn new(id: impl Into<String>) -> Result<Self, String> {
		let id = id.into();
		let mut chars = id.chars();
		let valid = chars.next().is_some_and(|c| c.is_ascii_lowercase())
			&& chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_');
		if valid {
			Ok(Self(id))
		} else {
			Err(format!(
				"invalid visual id `{id}`: expected [a-z][a-z0-9_]*"
			))
		}
	}

	pub fn as_str(&self) -> &str {
		&self.0
	}
}

impl TryFrom<String> for VisualId {
	type Error = String;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		Self::new(value)
	}
}

impl fmt::Display for VisualId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(&self.0)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisualCategory {
	Player,
	Enemy,
	Boss,
	Weapon,
	Projectile,
	Pickup,
	Warning,
	Item,
	Arena,
	Effect,
}

/// Semantic draw layers from Art Bible section 6, back to front.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DrawLayer {
	ArenaFloor,
	GroundWarning,
	Pickup,
	Actor,
	Projectile,
	Indicator,
	Hud,
}

impl DrawLayer {
	/// Z used for world rendering; later layers draw in front.
	pub fn z(self) -> f32 {
		self as u8 as f32 * 10.0
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Sampling {
	Linear,
	Nearest,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum Extent {
	/// Logical world size in LU (width, height).
	WorldLu([f64; 2]),
	/// UI layout decides the size; no world-space use.
	Ui,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VisualRecord {
	pub visual_id: VisualId,
	pub category: VisualCategory,
	pub source_asset_id: String,
	pub provenance_id: String,
	pub extent: Extent,
	/// Normalized from the untrimmed source top-left, +Y down.
	pub pivot_normalized: [f64; 2],
	pub sampling: Sampling,
	pub draw_layer: DrawLayer,
	/// Danger cues and actor bodies: a missing essential visual is a typed error, never hidden.
	pub essential: bool,
	/// Texture paths relative to the asset root, in explicit frame order.
	pub frames: Vec<String>,
	#[serde(default)]
	pub fallback_visual_id: Option<VisualId>,
}

impl VisualRecord {
	pub fn world_extent(&self) -> Option<DVec2> {
		match self.extent {
			Extent::WorldLu([w, h]) => Some(DVec2::new(w, h)),
			Extent::Ui => None,
		}
	}

	/// Bevy-style anchor: pivot relative to the center, +Y up, range -0.5..0.5.
	pub fn anchor(&self) -> DVec2 {
		let [px, py] = self.pivot_normalized;
		DVec2::new(px - 0.5, 0.5 - py)
	}

	/// Art Bible section 4: maps a source point inside view box `(vx, vy, vw, vh)` to
	/// LU relative to the logical pivot. `None` for UI records.
	pub fn source_to_local(&self, point: DVec2, view_box: [f64; 4]) -> Option<DVec2> {
		let extent = self.world_extent()?;
		let [vx, vy, vw, vh] = view_box;
		let [px, py] = self.pivot_normalized;
		Some(DVec2::new(
			((point.x - vx) / vw - px) * extent.x,
			(py - (point.y - vy) / vh) * extent.y,
		))
	}
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CatalogFile {
	schema_version: u32,
	skin_id: String,
	#[serde(default)]
	category_fallbacks: BTreeMap<VisualCategory, VisualId>,
	records: Vec<VisualRecord>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CatalogError {
	/// `visual_id.field`, or `catalog` for file-level problems.
	pub path: String,
	pub message: String,
}

impl fmt::Display for CatalogError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}: {}", self.path, self.message)
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PresentationError {
	/// An essential visual is absent; the game must pause with an actionable error.
	EssentialMissing(VisualId),
	/// A cosmetic visual is absent and no category fallback exists.
	NoFallback(VisualId),
}

impl fmt::Display for PresentationError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::EssentialMissing(id) => write!(f, "essential visual `{id}` is missing"),
			Self::NoFallback(id) => write!(f, "visual `{id}` is missing and has no fallback"),
		}
	}
}

impl std::error::Error for PresentationError {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Resolved<'a> {
	Direct(&'a VisualRecord),
	Fallback(&'a VisualRecord),
}

impl<'a> Resolved<'a> {
	pub fn record(&self) -> &'a VisualRecord {
		match self {
			Self::Direct(r) | Self::Fallback(r) => r,
		}
	}
}

/// Validated, immutable catalog. Swap whole catalogs at a safe state; never mutate one.
#[derive(Debug, Clone)]
pub struct VisualCatalog {
	skin_id: String,
	records: BTreeMap<VisualId, VisualRecord>,
	category_fallbacks: BTreeMap<VisualCategory, VisualId>,
}

impl VisualCatalog {
	/// Parses and validates a catalog. `texture_exists` decides whether a frame path
	/// resolves, so tests and tools can check files without a renderer.
	pub fn from_json(
		bytes: &[u8],
		texture_exists: impl Fn(&str) -> bool,
	) -> Result<Self, Vec<CatalogError>> {
		let file: CatalogFile = serde_json::from_slice(bytes).map_err(|e| {
			vec![CatalogError {
				path: "catalog".into(),
				message: e.to_string(),
			}]
		})?;
		let mut errors = Vec::new();
		let mut err = |path: String, message: String| errors.push(CatalogError { path, message });
		if file.schema_version != CATALOG_SCHEMA_VERSION {
			err(
				"catalog.schema_version".into(),
				format!(
					"unsupported version {}, expected {CATALOG_SCHEMA_VERSION}",
					file.schema_version
				),
			);
		}
		let mut records = BTreeMap::new();
		for r in file.records {
			let id = r.visual_id.clone();
			let p = |field: &str| format!("{id}.{field}");
			if let Extent::WorldLu(e) = r.extent
				&& !e.iter().all(|v| v.is_finite() && *v > 0.0)
			{
				err(
					p("extent"),
					"world extent must be two finite positive LU values".into(),
				);
			}
			if !r
				.pivot_normalized
				.iter()
				.all(|v| v.is_finite() && (0.0..=1.0).contains(v))
			{
				err(
					p("pivot_normalized"),
					"pivot must be two finite values in 0..=1".into(),
				);
			}
			if r.frames.is_empty() {
				err(p("frames"), "at least one frame is required".into());
			}
			for (i, frame) in r.frames.iter().enumerate() {
				if !texture_exists(frame) {
					err(
						format!("{id}.frames[{i}]"),
						format!("texture `{frame}` does not resolve"),
					);
				}
			}
			if r.source_asset_id.is_empty() || r.provenance_id.is_empty() {
				err(
					p("provenance_id"),
					"source and provenance ids are required".into(),
				);
			}
			if records.insert(id.clone(), r).is_some() {
				err(p("visual_id"), "duplicate visual id".into());
			}
		}
		for (id, r) in &records {
			let mut seen = BTreeSet::from([id.clone()]);
			let mut next = r.fallback_visual_id.clone();
			while let Some(f) = next {
				let Some(target) = records.get(&f) else {
					err(
						format!("{id}.fallback_visual_id"),
						format!("unknown fallback `{f}`"),
					);
					break;
				};
				if !seen.insert(f.clone()) {
					err(format!("{id}.fallback_visual_id"), "fallback cycle".into());
					break;
				}
				if target.category != r.category {
					err(
						format!("{id}.fallback_visual_id"),
						format!("fallback `{f}` has another category"),
					);
				}
				next = target.fallback_visual_id.clone();
			}
		}
		for (category, id) in &file.category_fallbacks {
			match records.get(id) {
				Some(r) if r.category == *category => {}
				Some(_) => err(
					format!("catalog.category_fallbacks.{id}"),
					"category mismatch".into(),
				),
				None => err(
					format!("catalog.category_fallbacks.{id}"),
					"unknown visual id".into(),
				),
			}
		}
		if errors.is_empty() {
			Ok(Self {
				skin_id: file.skin_id,
				records,
				category_fallbacks: file.category_fallbacks,
			})
		} else {
			Err(errors)
		}
	}

	pub fn skin_id(&self) -> &str {
		&self.skin_id
	}

	pub fn get(&self, id: &VisualId) -> Option<&VisualRecord> {
		self.records.get(id)
	}

	pub fn records(&self) -> impl Iterator<Item = &VisualRecord> {
		self.records.values()
	}

	/// Looks up `id`. A missing cosmetic visual resolves to its category fallback; a
	/// missing essential visual is an error the game must surface.
	pub fn resolve(
		&self,
		id: &VisualId,
		category: VisualCategory,
		essential: bool,
	) -> Result<Resolved<'_>, PresentationError> {
		if let Some(r) = self.records.get(id) {
			return Ok(Resolved::Direct(r));
		}
		if essential {
			return Err(PresentationError::EssentialMissing(id.clone()));
		}
		self.category_fallbacks
			.get(&category)
			.and_then(|f| self.records.get(f))
			.map(Resolved::Fallback)
			.ok_or_else(|| PresentationError::NoFallback(id.clone()))
	}
}

/// Authoritative presentation facts for one entity. Facing is radians, +X is zero.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VisualPose {
	pub position: DVec2,
	pub facing_radians: f64,
}
