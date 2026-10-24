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

// ---- GAME-P2-004: attack emission and collision candidates ----

use omega_game::combat::damage::HitCandidates;
use omega_game::combat::weapons::Projectile;

fn equip(h: &mut SimHarness, player: bevy::prelude::Entity, weapon: &str) {
	let mut loadout = h.app_mut().world_mut().get_mut::<Loadout>(player).unwrap();
	loadout.slots[0].as_mut().unwrap().weapon_id = weapon.to_string();
}

fn enemy(h: &mut SimHarness, x: f64, y: f64) -> u64 {
	let e = spawn_enemy(
		h.app_mut().world_mut(),
		"enemy_bulwark",
		DVec2::new(x, y),
		100_000,
		1,
	)
	.unwrap();
	h.app().world().get::<SimId>(e).unwrap().0
}

fn attacks(records: &[PresentationRecord]) -> Vec<(u64, u64, [f64; 2])> {
	records
		.iter()
		.filter_map(|r| match r.kind {
			PresentationKind::AttackEmitted {
				attack, direction, ..
			} => Some((r.tick, attack, direction)),
			_ => None,
		})
		.collect()
}

fn run_ticks(h: &mut SimHarness, n: u32) -> Vec<PresentationRecord> {
	let mut out = Vec::new();
	for _ in 0..n {
		h.step(TickInput::default());
		out.extend(h.drain_presentation());
	}
	out
}

#[test]
fn da04_attack_starts_are_15_ticks_apart() {
	let mut h = harness();
	spawn_player(h.app_mut().world_mut(), "char_patch", DVec2::ZERO).unwrap();
	enemy(&mut h, 200.0, 0.0);
	let ticks: Vec<u64> = attacks(&run_ticks(&mut h, 60))
		.iter()
		.map(|a| a.0)
		.collect();
	assert_eq!(ticks, vec![1, 16, 31, 46]);
}

#[test]
fn no_burst_after_a_period_without_targets() {
	let mut h = harness();
	spawn_player(h.app_mut().world_mut(), "char_patch", DVec2::ZERO).unwrap();
	assert!(attacks(&run_ticks(&mut h, 200)).is_empty());
	enemy(&mut h, 200.0, 0.0);
	let ticks: Vec<u64> = attacks(&run_ticks(&mut h, 31))
		.iter()
		.map(|a| a.0)
		.collect();
	assert_eq!(ticks, vec![201, 216, 231]);
}

#[test]
fn target_tie_breaks_on_the_lower_sim_id() {
	let mut h = harness();
	spawn_player(h.app_mut().world_mut(), "char_patch", DVec2::ZERO).unwrap();
	enemy(&mut h, 0.0, 100.0);
	enemy(&mut h, 0.0, -100.0);
	let first = attacks(&run_ticks(&mut h, 1))[0];
	assert_eq!(first.2, [0.0, 1.0]);
}

#[test]
fn out_of_range_enemies_are_not_targeted_but_manual_aim_fires_anyway() {
	let mut h = harness();
	spawn_player(h.app_mut().world_mut(), "char_patch", DVec2::ZERO).unwrap();
	enemy(&mut h, 500.0, 0.0);
	assert!(attacks(&run_ticks(&mut h, 20)).is_empty());
	h.step(TickInput {
		movement: DVec2::ZERO,
		aim: AimInput::Manual(DVec2::Y),
	});
	let manual = attacks(&h.drain_presentation());
	assert_eq!(manual.len(), 1);
	assert_eq!(manual[0].2, [0.0, 1.0]);
}

fn collect_hits(h: &mut SimHarness, ticks: u32) -> Vec<(u64, u64, u64)> {
	let mut out = Vec::new();
	for _ in 0..ticks {
		h.step(TickInput::default());
		let tick = h.tick();
		for hit in &h.app().world().resource::<HitCandidates>().0 {
			out.push((tick, hit.attack.0, hit.target_id.0));
		}
	}
	out
}

#[test]
fn bolt_projectile_hits_the_target_once_and_ends() {
	let mut h = harness();
	spawn_player(h.app_mut().world_mut(), "char_patch", DVec2::ZERO).unwrap();
	let target = enemy(&mut h, 100.0, 0.0);
	let hits = collect_hits(&mut h, 14);
	assert_eq!(hits.len(), 1, "{hits:?}");
	assert_eq!(hits[0].2, target);
	// Bulwark radius 23 + bolt radius 4: contact at 73 LU, 8.33 LU per tick, so tick 9.
	assert_eq!(hits[0].0, 9);
	let world = h.app_mut().world_mut();
	assert_eq!(world.query::<&Projectile>().iter(world).count(), 0);
}

#[test]
fn piercing_projectile_hits_one_plus_pierce_distinct_targets() {
	let mut h = harness();
	let p = spawn_player(h.app_mut().world_mut(), "char_patch", DVec2::ZERO).unwrap();
	equip(&mut h, p, "wpn_slug");
	let ids = [
		enemy(&mut h, 100.0, 0.0),
		enemy(&mut h, 180.0, 0.0),
		enemy(&mut h, 260.0, 0.0),
	];
	let hits = collect_hits(&mut h, 60);
	let first_attack = hits[0].1;
	let targets: Vec<u64> = hits
		.iter()
		.filter(|x| x.1 == first_attack)
		.map(|x| x.2)
		.collect();
	assert_eq!(targets, vec![ids[0], ids[1]]);
}

#[test]
fn spread_volley_damages_a_target_once_per_attack_id() {
	let mut h = harness();
	let p = spawn_player(h.app_mut().world_mut(), "char_patch", DVec2::ZERO).unwrap();
	equip(&mut h, p, "wpn_fan");
	let target = enemy(&mut h, 40.0, 0.0);
	let hits = collect_hits(&mut h, 12);
	assert_eq!(hits.len(), 1, "three pellets, one attack: {hits:?}");
	assert_eq!(hits[0].2, target);
}

#[test]
fn explosion_hits_every_target_in_radius_once_without_a_direct_hit() {
	let mut h = harness();
	let p = spawn_player(h.app_mut().world_mut(), "char_patch", DVec2::ZERO).unwrap();
	equip(&mut h, p, "wpn_rocket");
	let near = enemy(&mut h, 100.0, 0.0);
	let beside = enemy(&mut h, 130.0, 0.0);
	let far = enemy(&mut h, 100.0, 120.0);
	let records = {
		let mut r = Vec::new();
		let mut hits = Vec::new();
		for _ in 0..40 {
			h.step(TickInput::default());
			let tick = h.tick();
			hits.extend(
				h.app()
					.world()
					.resource::<HitCandidates>()
					.0
					.iter()
					.map(|x| (tick, x.target_id.0)),
			);
			r.extend(h.drain_presentation());
		}
		(r, hits)
	};
	let mut targets: Vec<u64> = records.1.iter().map(|x| x.1).collect();
	targets.sort_unstable();
	assert_eq!(targets, vec![near, beside]);
	assert!(!targets.contains(&far));
	assert_eq!(
		records
			.0
			.iter()
			.filter(|r| matches!(r.kind, PresentationKind::Explosion { .. }))
			.count(),
		1
	);
}

#[test]
fn saw_capsule_hits_on_the_windup_completion_tick() {
	let mut h = harness();
	let p = spawn_player(h.app_mut().world_mut(), "char_patch", DVec2::ZERO).unwrap();
	equip(&mut h, p, "wpn_saw");
	let target = enemy(&mut h, 50.0, 0.0);
	let hits = collect_hits(&mut h, 30);
	let first = hits[0];
	// Attack starts on tick 1, 6 windup ticks, emits and hits on tick 7.
	assert_eq!((first.0, first.2), (7, target));
	// The next attack starts 20 ticks after the first (3 points, 1.5/s) and hits 6 later.
	assert_eq!(hits[1].0, 27);
}

#[test]
fn a_dead_attacker_cancels_pending_attacks() {
	let mut h = harness();
	let p = spawn_player(h.app_mut().world_mut(), "char_patch", DVec2::ZERO).unwrap();
	equip(&mut h, p, "wpn_saw");
	enemy(&mut h, 50.0, 0.0);
	run_ticks(&mut h, 3);
	h.app_mut()
		.world_mut()
		.get_mut::<Health>(p)
		.unwrap()
		.current = 0;
	let records = run_ticks(&mut h, 20);
	assert!(attacks(&records).is_empty());
	assert!(h.app().world().resource::<HitCandidates>().0.is_empty());
}
