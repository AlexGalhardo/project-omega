//! SP-008 acceptance for common scheduling: warnings, spawn safety, scaling, chase and
//! timer completion, driven through the real `GameTick` schedule.

use bevy::math::DVec2;
use bevy::prelude::*;
use omega_game::combat::actor::*;
use omega_game::content::{Profile, load_dir};
use omega_game::encounters::*;
use omega_game::sim::*;
use std::path::PathBuf;
use std::sync::Arc;

fn harness(seed: u8) -> SimHarness {
	let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/content/full");
	let content = load_dir(&dir, Profile::Full).expect("full fixture");
	let arena = content.catalogs.arenas["arena_foundry"].bounds;
	let mut app = App::new();
	app.add_plugins(omega_game::GameCorePlugin);
	let mut h = SimHarness::with_app(
		app,
		RunSeed::from_hex(&format!("{seed:02x}").repeat(32)).unwrap(),
	);
	h.app_mut().insert_resource(ContentRes(Arc::new(content)));
	h.app_mut().insert_resource(ArenaRes(arena));
	h
}

fn start(h: &mut SimHarness, wave: u32) -> Entity {
	let player = spawn_player(h.app_mut().world_mut(), "char_patch", DVec2::ZERO).unwrap();
	start_wave(h.app_mut().world_mut(), wave).unwrap();
	player
}

fn disarm(h: &mut SimHarness, player: Entity) {
	h.app_mut()
		.world_mut()
		.get_mut::<Loadout>(player)
		.unwrap()
		.slots[0] = None;
}

fn step_n(h: &mut SimHarness, n: u32) {
	for _ in 0..n {
		h.step(TickInput::default());
	}
}

fn warnings(h: &mut SimHarness) -> Vec<(u64, SpawnWarning)> {
	let world = h.app_mut().world_mut();
	let mut v: Vec<_> = world
		.query::<(&SimId, &SpawnWarning)>()
		.iter(world)
		.map(|(id, w)| (id.0, w.clone()))
		.collect();
	v.sort_by_key(|x| x.0);
	v
}

fn enemy_count(h: &mut SimHarness) -> usize {
	let world = h.app_mut().world_mut();
	world.query::<&Enemy>().iter(world).count()
}

#[test]
fn common_scaling_matches_the_gdd_example() {
	// Wave-4 Chaser: ceil(24 * 1.54) = 37 HP, ceil(8 * 1.24) = 10 contact damage.
	assert_eq!(scaled(24, 18, 4), 37);
	assert_eq!(scaled(8, 8, 4), 10);
	assert_eq!(scaled(24, 18, 1), 24);
}

#[test]
fn ac1_batches_are_requested_at_authored_ticks_and_bosses_only_on_5_and_10() {
	let mut h = harness(1);
	start(&mut h, 1);
	step_n(&mut h, 59);
	assert!(warnings(&mut h).is_empty());
	step_n(&mut h, 1);
	let w = warnings(&mut h);
	assert_eq!(w.len(), 4, "wave_01 batch of four at tick 60");
	assert!(w.iter().all(|(_, w)| w.enemy_id == "enemy_chaser"));
	assert!(w.iter().all(|(_, w)| w.materialize_tick == 105));
	step_n(&mut h, 44);
	assert_eq!(enemy_count(&mut h), 0);
	step_n(&mut h, 1);
	assert_eq!(enemy_count(&mut h), 4, "targetable once the warning ends");
	assert!(warnings(&mut h).is_empty());
	let content = h.app().world().resource::<ContentRes>().0.clone();
	for (n, wave) in &content.catalogs.waves {
		assert_eq!(wave.boss.is_some(), *n == 5 || *n == 10, "wave {n}");
	}
}

#[test]
fn anchors_are_on_the_boundary_and_far_from_the_player() {
	let mut h = harness(2);
	start(&mut h, 1);
	step_n(&mut h, 60);
	let content = h.app().world().resource::<ContentRes>().0.clone();
	let bounds = content.catalogs.arenas["arena_foundry"].bounds;
	let inset = content.catalogs.enemies["enemy_chaser"].radius_lu + 4.0;
	let ws = warnings(&mut h);
	let mut positions: Vec<(i64, i64)> = Vec::new();
	for (_, w) in &ws {
		assert!(w.position.distance(DVec2::ZERO) >= ANCHOR_SAFE_LU);
		let on_x =
			(w.position.x.abs() - (bounds.max().x - inset)).abs() < 1e-9 || w.position.x == 0.0;
		let on_y =
			(w.position.y.abs() - (bounds.max().y - inset)).abs() < 1e-9 || w.position.y == 0.0;
		assert!(on_x && on_y, "{:?}", w.position);
		positions.push((w.position.x as i64, w.position.y as i64));
	}
	positions.sort_unstable();
	positions.dedup();
	assert_eq!(positions.len(), 4, "no anchor reused before all were used");
}

#[test]
fn anchors_near_the_player_are_excluded() {
	let mut h = harness(3);
	let player = start(&mut h, 1);
	// Standing at the top-center anchor excludes it and its neighbours inside 180 LU.
	let content = h.app().world().resource::<ContentRes>().0.clone();
	let top = content.catalogs.arenas["arena_foundry"].bounds.max().y - 17.0;
	h.app_mut().world_mut().get_mut::<Pose2>(player).unwrap().0 = DVec2::new(0.0, top);
	step_n(&mut h, 60);
	for (_, w) in warnings(&mut h) {
		assert!(w.position.distance(DVec2::new(0.0, top)) >= ANCHOR_SAFE_LU);
	}
}

#[test]
fn same_seed_gives_the_same_warnings() {
	let run = |seed| {
		let mut h = harness(seed);
		start(&mut h, 2);
		step_n(&mut h, 60);
		warnings(&mut h)
			.into_iter()
			.map(|(id, w)| (id, w.enemy_id, w.position.x as i64, w.position.y as i64))
			.collect::<Vec<_>>()
	};
	assert_eq!(run(9), run(9));
}

#[test]
fn chasers_close_distance_at_their_speed_and_stop_on_equal_centers() {
	let mut h = harness(4);
	let p = start(&mut h, 1);
	disarm(&mut h, p);
	h.app_mut()
		.world_mut()
		.get_mut::<Health>(p)
		.unwrap()
		.current = 1_000_000;
	let e = spawn_enemy(
		h.app_mut().world_mut(),
		"enemy_chaser",
		DVec2::new(300.0, 0.0),
		24,
		8,
	)
	.unwrap();
	h.app_mut().world_mut().entity_mut(e).insert(ChaseAi);
	step_n(&mut h, 1);
	let x = h.app().world().get::<Pose2>(e).unwrap().0.x;
	assert!((x - (300.0 - 70.0 / 60.0)).abs() < 1e-9, "{x}");
	h.app_mut().world_mut().get_mut::<Pose2>(e).unwrap().0 = DVec2::ZERO;
	step_n(&mut h, 1);
	assert_eq!(h.app().world().get::<Pose2>(e).unwrap().0, DVec2::ZERO);
}

fn place_warning(h: &mut SimHarness, position: DVec2) -> Entity {
	let id = h
		.app_mut()
		.world_mut()
		.resource_mut::<IdAllocator>()
		.sim_id();
	h.app_mut()
		.world_mut()
		.spawn((
			id,
			SpawnWarning {
				enemy_id: "enemy_chaser".into(),
				position,
				materialize_tick: 5,
				postponements: 0,
				hp: 24,
				contact_damage: 8,
			},
		))
		.id()
}

#[test]
fn da18_occupied_common_warning_postpones_then_cancels_without_reward() {
	let mut h = harness(5);
	let p = start(&mut h, 1);
	disarm(&mut h, p);
	let w = place_warning(&mut h, DVec2::new(10.0, 0.0));
	// Tick 5 is the first expiry; ten postponements of 30 ticks follow, then cancellation.
	step_n(&mut h, 5);
	assert_eq!(
		h.app()
			.world()
			.get::<SpawnWarning>(w)
			.unwrap()
			.postponements,
		1
	);
	assert_eq!(
		h.app()
			.world()
			.get::<SpawnWarning>(w)
			.unwrap()
			.materialize_tick,
		35
	);
	step_n(&mut h, 30 * 9);
	assert_eq!(
		h.app()
			.world()
			.get::<SpawnWarning>(w)
			.unwrap()
			.postponements,
		10
	);
	step_n(&mut h, 30);
	assert!(
		h.app().world().get_entity(w).is_err(),
		"cancelled after the cap"
	);
	assert_eq!(*h.app().world().resource::<Scrap>(), Scrap(0));
}

#[test]
fn da18_warning_materializes_once_the_player_moves_away() {
	let mut h = harness(6);
	let player = start(&mut h, 1);
	place_warning(&mut h, DVec2::new(10.0, 0.0));
	step_n(&mut h, 5);
	assert_eq!(enemy_count(&mut h), 0);
	h.app_mut().world_mut().get_mut::<Pose2>(player).unwrap().0 = DVec2::new(-300.0, 0.0);
	step_n(&mut h, 30);
	assert_eq!(enemy_count(&mut h), 1);
}

#[test]
fn da11_timer_completion_removes_survivors_without_scrap_and_keeps_pickups() {
	let mut h = harness(7);
	let p = start(&mut h, 1);
	disarm(&mut h, p);
	h.app_mut()
		.world_mut()
		.get_mut::<Health>(p)
		.unwrap()
		.current = 1_000_000;
	let e = spawn_enemy(
		h.app_mut().world_mut(),
		"enemy_chaser",
		DVec2::new(500.0, 300.0),
		24,
		8,
	)
	.unwrap();
	h.app_mut().world_mut().spawn((
		SimId(9000),
		Pickup { value: 3 },
		Pose2(DVec2::new(400.0, 0.0)),
	));
	step_n(&mut h, 3599);
	assert!(!h.app().world().resource::<WaveRuntime>().completed);
	assert!(h.app().world().get_entity(e).is_ok());
	step_n(&mut h, 1);
	assert!(h.app().world().resource::<WaveRuntime>().completed);
	assert_eq!(enemy_count(&mut h), 0);
	let world = h.app_mut().world_mut();
	assert_eq!(world.query::<&Pickup>().iter(world).count(), 1);
	assert_eq!(*world.resource::<Scrap>(), Scrap(0));
}

#[test]
fn a_pending_boss_keeps_the_wave_open_at_timer_expiry() {
	let mut h = harness(8);
	start(&mut h, 5);
	step_n(&mut h, 3600);
	assert!(!h.app().world().resource::<WaveRuntime>().completed);
}

#[test]
fn a_dead_player_never_completes_the_wave() {
	let mut h = harness(9);
	let player = start(&mut h, 1);
	h.app_mut()
		.world_mut()
		.get_mut::<Health>(player)
		.unwrap()
		.current = 0;
	step_n(&mut h, 3600);
	assert!(!h.app().world().resource::<WaveRuntime>().completed);
}
