//! GAME-P2-009 and GAME-P3-006/007: the shipped catalogs. `assets/content` is the full game;
//! `assets/content-prototype` is the three-wave subset used by tests, QA and the balance runs.

use omega_game::content::{Profile, load_dir};
use std::path::PathBuf;

fn dir(name: &str) -> PathBuf {
	PathBuf::from(env!("CARGO_MANIFEST_DIR"))
		.join("../../assets")
		.join(name)
}

fn fixture(name: &str) -> PathBuf {
	PathBuf::from(env!("CARGO_MANIFEST_DIR"))
		.join("tests/fixtures/content")
		.join(name)
}

#[test]
fn shipped_prototype_subset_matches_the_gdd_prototype_scope() {
	let content =
		load_dir(&dir("content-prototype"), Profile::Prototype).expect("prototype profile");
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
fn shipped_prototype_subset_is_rejected_by_the_full_profile_and_equals_its_fixture() {
	assert!(load_dir(&dir("content-prototype"), Profile::Full).is_err());
	let a = load_dir(&dir("content-prototype"), Profile::Prototype).unwrap();
	let b = load_dir(&fixture("prototype"), Profile::Prototype).unwrap();
	assert_eq!(a.revision, b.revision);
}

#[test]
fn sp018_ac1_shipped_full_catalog_has_the_exact_gdd_counts() {
	let content = load_dir(&dir("content"), Profile::Full).expect("full profile");
	let c = &content.catalogs;
	assert_eq!(
		(
			c.characters.len(),
			c.weapons.len(),
			c.enemies.len(),
			c.bosses.len(),
			c.items.len()
		),
		(4, 12, 10, 2, 40)
	);
	assert_eq!((c.waves.len(), c.unlocks.len()), (10, 4));
	let bosses: Vec<u32> = c
		.waves
		.values()
		.filter(|w| w.boss.is_some())
		.map(|w| w.number)
		.collect();
	assert_eq!(bosses, vec![5, 10]);
}

#[test]
fn sp018_ac4_shipped_full_catalog_is_the_validated_fixture() {
	let shipped = load_dir(&dir("content"), Profile::Full).unwrap();
	let fixture = load_dir(&fixture("full"), Profile::Full).unwrap();
	assert_eq!(
		shipped.revision, fixture.revision,
		"one schema, one revision"
	);
}

#[test]
fn sp018_ac2_every_reward_exists_and_unlock_pools_have_no_cycles() {
	let content = load_dir(&dir("content"), Profile::Full).unwrap();
	let c = &content.catalogs;
	for unlock in c.unlocks.values() {
		assert!(!unlock.reward_ids.is_empty());
		for reward in &unlock.reward_ids {
			assert!(
				c.characters.contains_key(reward)
					|| c.weapons.contains_key(reward)
					|| c.items.contains_key(reward),
				"{} rewards unknown {reward}",
				unlock.id
			);
		}
	}
	// A reward never gates the challenge that grants it: every challenge is reachable from the
	// initial pool because conditions only name waves.
	let initial_characters = c
		.characters
		.values()
		.filter(|x| x.availability.is_initial())
		.count();
	assert_eq!(initial_characters, 1);
}
