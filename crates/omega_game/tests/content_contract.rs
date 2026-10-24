//! SP-003 acceptance: typed content validation with record/field paths, profile
//! counts, Menu-only atomic reload and reachability of unlocks/starting weapons.

use omega_game::content::*;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::path::PathBuf;

fn fixture_dir(profile: &str) -> PathBuf {
	PathBuf::from(env!("CARGO_MANIFEST_DIR"))
		.join("tests/fixtures/content")
		.join(profile)
}

fn files(profile: &str) -> BTreeMap<String, Vec<u8>> {
	std::fs::read_dir(fixture_dir(profile))
		.unwrap()
		.map(|e| e.unwrap().path())
		.map(|p| {
			(
				p.file_name().unwrap().to_str().unwrap().to_string(),
				std::fs::read(&p).unwrap(),
			)
		})
		.collect()
}

/// Applies `edit` to one family file's JSON and returns the modified bundle.
fn mutate(profile: &str, file: &str, edit: impl FnOnce(&mut Value)) -> BTreeMap<String, Vec<u8>> {
	let mut f = files(profile);
	let mut v: Value = serde_json::from_slice(&f[file]).unwrap();
	edit(&mut v);
	f.insert(file.to_string(), serde_json::to_vec(&v).unwrap());
	f
}

fn errors(bundle: &BTreeMap<String, Vec<u8>>, profile: Profile) -> Vec<ContentError> {
	load_bundle(bundle, profile)
		.expect_err("bundle must be rejected")
		.0
}

fn has(errs: &[ContentError], file: &str, path: &str) -> bool {
	errs.iter().any(|e| e.file == file && e.path == path)
}

#[test]
fn full_and_prototype_fixtures_validate() {
	let full = load_dir(&fixture_dir("full"), Profile::Full).unwrap_or_else(|e| panic!("{e}"));
	let c = &full.catalogs;
	assert_eq!(
		[
			c.characters.len(),
			c.weapons.len(),
			c.enemies.len(),
			c.bosses.len(),
			c.items.len(),
			c.waves.len(),
			c.unlocks.len()
		],
		[4, 12, 10, 2, 40, 10, 4]
	);
	let spawns: u32 = c
		.waves
		.values()
		.flat_map(|w| &w.batches)
		.map(|b| b.count)
		.sum();
	assert_eq!(spawns, 408, "GDD-011 common spawn requests");
	assert_eq!(
		c.weapons["wpn_rocket"].base_rate_centi, 65,
		"CR-002 exact rate unit"
	);
	assert_eq!(
		c.waves[&5].boss.as_ref().unwrap().request_tick,
		0,
		"CR-001 boss request tick"
	);
	let proto =
		load_dir(&fixture_dir("prototype"), Profile::Prototype).unwrap_or_else(|e| panic!("{e}"));
	assert_eq!(proto.catalogs.waves.len(), 3);
	assert_ne!(full.revision, proto.revision);
}

#[test]
fn ac1_duplicate_id_is_rejected_with_path() {
	let b = mutate("full", "items.json", |v| {
		let dup = v["records"][0].clone();
		v["records"].as_array_mut().unwrap().push(dup);
	});
	assert!(has(
		&errors(&b, Profile::Full),
		"items.json",
		"item_plating.id"
	));
	let cross = mutate("full", "items.json", |v| {
		v["records"][0]["id"] = json!("wpn_bolt")
	});
	assert!(
		has(&errors(&cross, Profile::Full), "items.json", "wpn_bolt.id"),
		"ids are globally unique"
	);
}

#[test]
fn ac1_unknown_field_is_rejected_with_path() {
	let b = mutate("full", "weapons.json", |v| {
		v["records"][0]["payload"]["homing"] = json!(true)
	});
	assert!(has(
		&errors(&b, Profile::Full),
		"weapons.json",
		"wpn_bolt.payload.homing"
	));
	let notes = mutate("full", "weapons.json", |v| {
		v["records"][0]["authoring_notes"] = json!("allowed")
	});
	assert!(
		load_bundle(&notes, Profile::Full).is_ok(),
		"namespaced authoring notes are allowed"
	);
}

#[test]
fn ac1_dangling_reference_is_rejected_with_path() {
	let b = mutate("full", "characters.json", |v| {
		v["records"][0]["starting_weapon_id"] = json!("wpn_missing")
	});
	assert!(has(
		&errors(&b, Profile::Full),
		"characters.json",
		"char_patch.starting_weapon_id"
	));
	let w = mutate("full", "waves.json", |v| {
		v["records"][0]["batches"][0]["enemies"][0]["id"] = json!("enemy_ghost")
	});
	assert!(has(
		&errors(&w, Profile::Full),
		"waves.json",
		"wave_01.batches[0].enemies[0].id"
	));
}

#[test]
fn ac1_nan_like_text_and_wrong_types_are_rejected_with_path() {
	let b = mutate("full", "weapons.json", |v| {
		v["records"][0]["base_damage"] = json!("NaN")
	});
	assert!(has(
		&errors(&b, Profile::Full),
		"weapons.json",
		"wpn_bolt.base_damage"
	));
	let f = mutate("full", "enemies.json", |v| {
		v["records"][0]["radius_lu"] = json!("Infinity")
	});
	assert!(has(
		&errors(&f, Profile::Full),
		"enemies.json",
		"enemy_chaser.radius_lu"
	));
	let d = mutate("full", "weapons.json", |v| {
		v["records"][0]["base_rate_centi"] = json!(2.0)
	});
	assert!(
		has(
			&errors(&d, Profile::Full),
			"weapons.json",
			"wpn_bolt.base_rate_centi"
		),
		"decimals rejected for integer units"
	);
	let mut raw = files("full");
	raw.insert(
		"shop.json".into(),
		b"{\"schema_version\": 1, \"records\": [NaN]}".to_vec(),
	);
	assert!(has(&errors(&raw, Profile::Full), "shop.json", "file"));
}

#[test]
fn ac1_unsupported_enum_is_rejected_with_path() {
	let b = mutate("full", "weapons.json", |v| {
		v["records"][0]["payload"]["kind"] = json!("laser")
	});
	assert!(has(
		&errors(&b, Profile::Full),
		"weapons.json",
		"wpn_bolt.payload.kind"
	));
	let s = mutate("full", "items.json", |v| {
		v["records"][0]["modifiers"][0]["stat"] = json!("luck")
	});
	assert!(has(
		&errors(&s, Profile::Full),
		"items.json",
		"item_plating.modifiers[0].stat"
	));
	let p = mutate("full", "enemies.json", |v| {
		v["records"][0]["behavior"]["profile"] = json!("teleport")
	});
	assert!(has(
		&errors(&p, Profile::Full),
		"enemies.json",
		"enemy_chaser.behavior.profile"
	));
}

#[test]
fn ac2_full_mode_counts_and_prototype_cannot_pass_full() {
	let b = mutate("full", "items.json", |v| {
		v["records"]
			.as_array_mut()
			.unwrap()
			.retain(|r| r["id"] != "item_optics");
	});
	assert!(has(&errors(&b, Profile::Full), "items.json", "records"));
	assert!(load_dir(&fixture_dir("prototype"), Profile::Full).is_err());
	assert!(load_dir(&fixture_dir("full"), Profile::Prototype).is_err());
}

#[test]
fn ac3_invalid_reload_keeps_previous_snapshot_and_runs_keep_their_revision() {
	let mut store = ContentStore::new(load_bundle(&files("full"), Profile::Full).unwrap());
	let run_snapshot = store.current();
	let before = run_snapshot.revision;

	let bad = mutate("full", "weapons.json", |v| {
		v["records"][0]["base_damage"] = json!(0)
	});
	assert!(matches!(
		store.reload(&bad, ReloadContext::Menu),
		Err(ReloadError::Invalid(_))
	));
	assert_eq!(
		store.current().revision,
		before,
		"invalid candidate is not published"
	);

	let changed = mutate("full", "weapons.json", |v| {
		v["records"][0]["base_damage"] = json!(13)
	});
	assert!(matches!(
		store.reload(&changed, ReloadContext::ActiveRun),
		Err(ReloadError::NotInMenu)
	));
	assert_eq!(store.current().revision, before);

	store.reload(&changed, ReloadContext::Menu).unwrap();
	assert_ne!(store.current().revision, before, "valid Menu reload swaps");
	assert_eq!(
		run_snapshot.revision, before,
		"a captured run snapshot never changes"
	);
	assert_eq!(run_snapshot.catalogs.weapons["wpn_bolt"].base_damage, 12);
}

#[test]
fn ac4_unlocks_and_starting_weapons_are_reachable() {
	let s = load_dir(&fixture_dir("full"), Profile::Full).unwrap();
	let c = &s.catalogs;
	for ch in c.characters.values() {
		let w = &c.weapons[&ch.starting_weapon_id];
		assert!(
			w.availability.is_initial() || w.availability == ch.availability,
			"{}",
			ch.id
		);
	}
	for w in c.waves.values() {
		assert!(
			w.batches
				.iter()
				.all(|b| b.enemies.iter().all(|(_, weight)| *weight > 0))
		);
	}
	// Orphan reward: a gated weapon whose unlock no longer awards it.
	let b = mutate("full", "unlocks.json", |v| {
		v["records"][3]["reward_ids"]
			.as_array_mut()
			.unwrap()
			.retain(|r| r != "wpn_disk");
	});
	assert!(has(
		&errors(&b, Profile::Full),
		"weapons.json",
		"wpn_disk.unlock_id"
	));
	// Starting weapon gated behind another unlock than its character.
	let s2 = mutate("full", "characters.json", |v| {
		v["records"][1]["starting_weapon_id"] = json!("wpn_disk")
	});
	assert!(has(
		&errors(&s2, Profile::Full),
		"characters.json",
		"char_coil.starting_weapon_id"
	));
	// Unreachable condition.
	let u = mutate("full", "unlocks.json", |v| {
		v["records"][0]["condition"]["wave"] = json!(11)
	});
	assert!(has(
		&errors(&u, Profile::Full),
		"unlocks.json",
		"unlock_first_shift.condition.wave"
	));
}

#[test]
fn boss_schedule_rules_from_cr_001() {
	let missing = mutate("full", "waves.json", |v| {
		v["records"][4]
			.as_object_mut()
			.unwrap()
			.remove("boss_request_tick");
	});
	assert!(has(
		&errors(&missing, Profile::Full),
		"waves.json",
		"wave_05.boss_request_tick"
	));
	let late = mutate("full", "waves.json", |v| {
		v["records"][4]["boss_request_tick"] = json!(3500)
	});
	assert!(has(
		&errors(&late, Profile::Full),
		"waves.json",
		"wave_05.boss_request_tick"
	));
	let ordinary = mutate("full", "waves.json", |v| {
		v["records"][0]["boss_id"] = json!("boss_press");
		v["records"][0]["boss_request_tick"] = json!(0);
	});
	assert!(has(
		&errors(&ordinary, Profile::Full),
		"waves.json",
		"wave_01.boss_id"
	));
}
