//! GAME-P2-009: the shipped prototype subset validates in prototype mode and fails full mode.

use omega_game::content::{Profile, load_dir};
use std::path::PathBuf;

fn dir() -> PathBuf {
	PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/content")
}

#[test]
fn shipped_subset_matches_the_gdd_prototype_scope() {
	let content = load_dir(&dir(), Profile::Prototype).expect("prototype profile");
	let c = &content.catalogs;
	assert_eq!(c.characters.keys().collect::<Vec<_>>(), ["char_patch"]);
	assert_eq!(
		c.weapons.keys().collect::<Vec<_>>(),
		["wpn_bolt", "wpn_saw"]
	);
	assert_eq!(c.items.len(), 8);
	assert_eq!(
		c.enemies.keys().collect::<Vec<_>>(),
		["enemy_chaser", "enemy_runner"]
	);
	assert_eq!(c.waves.len(), 3);
	assert!(c.bosses.is_empty() && c.unlocks.is_empty());
}

#[test]
fn shipped_subset_is_rejected_by_the_full_profile() {
	assert!(load_dir(&dir(), Profile::Full).is_err());
}

#[test]
fn shipped_subset_equals_the_test_fixture() {
	let fixture =
		PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/content/prototype");
	let a = load_dir(&dir(), Profile::Prototype).unwrap();
	let b = load_dir(&fixture, Profile::Prototype).unwrap();
	assert_eq!(a.revision, b.revision);
}
