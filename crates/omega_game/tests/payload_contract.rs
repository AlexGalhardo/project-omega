//! SP-007 payload fixtures: every authored weapon payload attacks through the real schedule,
//! damages a target once per attack ID, and area shapes use their authored centers.

use bevy::math::DVec2;
use bevy::prelude::*;
use omega_game::combat::actor::*;
use omega_game::combat::damage::HitCandidates;
use omega_game::content::{Profile, load_dir};
use omega_game::sim::*;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::Arc;

fn harness() -> (SimHarness, Entity) {
	let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/content/full");
	let content = load_dir(&dir, Profile::Full).expect("full fixture");
	let arena = content.catalogs.arenas["arena_foundry"].bounds;
	let mut app = App::new();
	app.add_plugins(omega_game::GameCorePlugin);
	let mut h = SimHarness::with_app(app, RunSeed::from_hex(&"66".repeat(32)).unwrap());
	h.app_mut().insert_resource(ContentRes(Arc::new(content)));
	h.app_mut().insert_resource(ArenaRes(arena));
	let p = spawn_player(h.app_mut().world_mut(), "char_patch", DVec2::ZERO).unwrap();
	(h, p)
}

fn equip(h: &mut SimHarness, p: Entity, weapon: &str) {
	let mut loadout = h.app_mut().world_mut().get_mut::<Loadout>(p).unwrap();
	loadout.slots[0].as_mut().unwrap().weapon_id = weapon.to_string();
}

fn enemy(h: &mut SimHarness, at: DVec2) -> u64 {
	let e = spawn_enemy(h.app_mut().world_mut(), "enemy_bulwark", at, 1_000_000, 0).unwrap();
	h.app().world().get::<SimId>(e).unwrap().0
}

/// (attack ID, target ID) pairs seen as hit candidates over `ticks`, with duplicates kept.
fn hits(h: &mut SimHarness, input: TickInput, ticks: u32) -> Vec<(u64, u64)> {
	let mut out = Vec::new();
	for _ in 0..ticks {
		h.step(input);
		out.extend(
			h.app()
				.world()
				.resource::<HitCandidates>()
				.0
				.iter()
				.map(|x| (x.attack.0, x.target_id.0)),
		);
	}
	out
}

#[test]
fn every_authored_weapon_hits_a_target_inside_its_reach_once_per_attack() {
	let weapons: Vec<String> = {
		let (h, _) = harness();
		let content = h.app().world().resource::<ContentRes>().0.clone();
		content.catalogs.weapons.keys().cloned().collect()
	};
	assert_eq!(weapons.len(), 12);
	for id in weapons {
		let (mut h, p) = harness();
		equip(&mut h, p, &id);
		let range = h.app().world().resource::<ContentRes>().0.catalogs.weapons[&id].range_lu;
		let target = enemy(&mut h, DVec2::new(range * 0.5, 0.0));
		let seen = hits(&mut h, TickInput::default(), 150);
		assert!(!seen.is_empty(), "{id} never hit a target at half range");
		let mut per_attack: BTreeMap<u64, Vec<u64>> = BTreeMap::new();
		for (attack, hit) in &seen {
			per_attack.entry(*attack).or_default().push(*hit);
		}
		for (attack, targets) in per_attack {
			assert_eq!(
				targets,
				vec![target],
				"{id}: attack {attack} damaged the target more than once"
			);
		}
	}
}

fn manual_x() -> TickInput {
	TickInput {
		movement: DVec2::ZERO,
		aim: AimInput::Manual(DVec2::X),
	}
}

#[test]
fn offset_circle_is_centered_ahead_of_the_aim_and_misses_behind() {
	let (mut h, p) = harness();
	equip(&mut h, p, "wpn_hammer");
	let behind = enemy(&mut h, DVec2::new(-60.0, 0.0));
	let seen = hits(&mut h, manual_x(), 60);
	assert!(
		seen.iter().all(|(_, t)| *t != behind),
		"a circle 50 LU ahead of aim must not reach 60 LU behind"
	);
	let (mut h, p) = harness();
	equip(&mut h, p, "wpn_hammer");
	let ahead = enemy(&mut h, DVec2::new(75.0, 0.0));
	let seen = hits(&mut h, manual_x(), 60);
	assert!(seen.iter().any(|(_, t)| *t == ahead));
}

#[test]
fn centered_pulse_hits_in_every_direction_regardless_of_manual_aim() {
	let (mut h, p) = harness();
	equip(&mut h, p, "wpn_pulse");
	let behind = enemy(&mut h, DVec2::new(-60.0, 0.0));
	let seen = hits(&mut h, manual_x(), 60);
	assert!(
		seen.iter().any(|(_, t)| *t == behind),
		"manual direction never moves a self-centered pulse"
	);
}

#[test]
fn pulse_beyond_its_radius_does_not_hit() {
	let (mut h, p) = harness();
	equip(&mut h, p, "wpn_pulse");
	let far = enemy(&mut h, DVec2::new(200.0, 0.0));
	let seen = hits(&mut h, manual_x(), 60);
	assert!(seen.iter().all(|(_, t)| *t != far));
}
