//! SP-007 acceptance: actor setup, movement, attacks, damage and deaths through the
//! real `GameTick` schedule driven by `SimHarness`.

use bevy::math::DVec2;
use omega_game::combat::actor::*;
use omega_game::content::{Profile, load_dir};
use omega_game::sim::*;
use std::path::PathBuf;
use std::sync::Arc;

fn harness() -> SimHarness {
	let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/content/full");
	let content = load_dir(&dir, Profile::Full).expect("full fixture");
	let arena = content.catalogs.arenas["arena_foundry"].bounds;
	let mut app = bevy::prelude::App::new();
	app.add_plugins(omega_game::GameCorePlugin);
	let mut h = SimHarness::with_app(app, RunSeed::from_hex(&"11".repeat(32)).unwrap());
	h.app_mut().insert_resource(ContentRes(Arc::new(content)));
	h.app_mut().insert_resource(ArenaRes(arena));
	h
}

fn input(x: f64, y: f64) -> TickInput {
	TickInput {
		movement: DVec2::new(x, y),
		aim: AimInput::Auto,
	}
}

fn player_pose(h: &mut SimHarness) -> DVec2 {
	let world = h.app_mut().world_mut();
	world
		.query::<(&Pose2, &Player)>()
		.single(world)
		.unwrap()
		.0
		.0
}

#[test]
fn da01_initial_patch_state() {
	let mut h = harness();
	let e = spawn_player(h.app_mut().world_mut(), "char_patch", DVec2::ZERO).unwrap();
	let world = h.app().world();
	let hp = world.get::<Health>(e).unwrap();
	assert_eq!((hp.current, hp.max), (120, 120));
	let loadout = world.get::<Loadout>(e).unwrap();
	let slot = loadout.slots[0].as_ref().unwrap();
	assert_eq!((slot.weapon_id.as_str(), slot.tier), ("wpn_bolt", 1));
	assert!(loadout.slots[1..].iter().all(Option::is_none));
	assert_eq!(loadout.energy.slots(), [3, 0, 0, 0]);
	assert_eq!(loadout.energy.remainder(), 3);
	assert_eq!(*world.resource::<Scrap>(), Scrap(0));
}

#[test]
fn unknown_character_is_rejected() {
	let mut h = harness();
	assert_eq!(
		spawn_player(h.app_mut().world_mut(), "char_nope", DVec2::ZERO),
		Err(SpawnError::UnknownCharacter)
	);
}

#[test]
fn da07_diagonal_speed_equals_single_axis() {
	let mut h = harness();
	spawn_player(h.app_mut().world_mut(), "char_patch", DVec2::ZERO).unwrap();
	h.step(input(1.0, 0.0));
	let axis = player_pose(&mut h).length();
	assert!(
		(axis - 3.0).abs() < 1e-9,
		"180 LU/s is 3 LU per tick, got {axis}"
	);
	// The input buffer hands the sim an already normalized diagonal.
	let mut d = harness();
	spawn_player(d.app_mut().world_mut(), "char_patch", DVec2::ZERO).unwrap();
	d.step(input(1.0, 1.0).normalize_movement());
	assert!((player_pose(&mut d).length() - axis).abs() < 1e-9);
}

trait NormalizeMovement {
	fn normalize_movement(self) -> Self;
}

impl NormalizeMovement for TickInput {
	fn normalize_movement(mut self) -> Self {
		self.movement = self.movement.normalize();
		self
	}
}

#[test]
fn move_speed_stat_scales_distance() {
	let mut h = harness();
	let e = spawn_player(h.app_mut().world_mut(), "char_patch", DVec2::ZERO).unwrap();
	h.app_mut()
		.world_mut()
		.get_mut::<PlayerStats>(e)
		.unwrap()
		.0
		.move_speed_pct = 20;
	h.step(input(1.0, 0.0));
	assert!((player_pose(&mut h).x - 3.6).abs() < 1e-9);
}

#[test]
fn movement_clamps_to_the_arena_inset_by_the_radius() {
	let mut h = harness();
	spawn_player(
		h.app_mut().world_mut(),
		"char_patch",
		DVec2::new(595.0, 0.0),
	)
	.unwrap();
	for _ in 0..10 {
		h.step(input(1.0, 0.0));
	}
	assert_eq!(player_pose(&mut h).x, 600.0 - PLAYER_RADIUS_LU);
	for _ in 0..400 {
		h.step(input(0.0, -1.0));
	}
	assert_eq!(player_pose(&mut h).y, -337.5 + PLAYER_RADIUS_LU);
}

#[test]
fn previous_pose_follows_the_last_tick() {
	let mut h = harness();
	let e = spawn_player(h.app_mut().world_mut(), "char_patch", DVec2::ZERO).unwrap();
	h.step(input(1.0, 0.0));
	h.step(input(1.0, 0.0));
	let world = h.app().world();
	assert_eq!(world.get::<PreviousPose2>(e).unwrap().0.x, 3.0);
	assert_eq!(world.get::<Pose2>(e).unwrap().0.x, 6.0);
}
