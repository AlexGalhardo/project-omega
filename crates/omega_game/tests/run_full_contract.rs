//! SP-011 on the full ten-wave catalog: nine shops, victory bypassing a tenth, death
//! precedence with a boss, abandon and profile grants through the real lifecycle.

use bevy::prelude::*;
use omega_game::combat::actor::*;
use omega_game::content::{Profile as ContentProfile, load_dir};
use omega_game::encounters::boss::BossAi;
use omega_game::profile::Profile;
use omega_game::sim::run::*;
use omega_game::sim::*;
use std::collections::BTreeSet;
use std::path::PathBuf;
use std::sync::Arc;

fn harness() -> (SimHarness, Entity) {
	let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/content/full");
	let content = load_dir(&dir, ContentProfile::Full).expect("full fixture");
	let arena = content.catalogs.arenas["arena_foundry"].bounds;
	let mut app = App::new();
	app.add_plugins(omega_game::GameCorePlugin);
	let mut h = SimHarness::with_app(app, RunSeed::from_hex(&"aa".repeat(32)).unwrap());
	h.app_mut().insert_resource(ContentRes(Arc::new(content)));
	h.app_mut().insert_resource(ArenaRes(arena));
	h.app_mut().insert_resource(Profile::initial());
	begin_run(h.app_mut().world_mut(), "char_patch", BTreeSet::new()).unwrap();
	let w = h.app_mut().world_mut();
	let player = w
		.query_filtered::<Entity, With<Player>>()
		.single(w)
		.unwrap();
	(h, player)
}

fn boss_killer(h: &mut SimHarness) {
	let w = h.app_mut().world_mut();
	let bosses: Vec<Entity> = w.query_filtered::<Entity, With<BossAi>>().iter(w).collect();
	for boss in bosses {
		w.despawn(boss);
	}
}

#[test]
fn ac1_ten_waves_give_nine_shops_and_the_final_victory_skips_a_tenth() {
	let (mut h, player) = harness();
	let mut shops = 0;
	let mut guard = 0;
	loop {
		guard += 1;
		assert!(
			guard < 200_000,
			"stuck in {:?}",
			h.app().world().resource::<Run>().state
		);
		match h.app().world().resource::<Run>().state {
			RunState::Wave => {
				let w = h.app_mut().world_mut();
				w.get_mut::<Health>(player).unwrap().current = 1_000_000;
				w.get_mut::<Health>(player).unwrap().max = 1_000_000;
				boss_killer(&mut h);
				h.step(TickInput::default());
				h.drain_presentation();
			}
			RunState::Shop => {
				shops += 1;
				start_next_wave(h.app_mut().world_mut()).unwrap();
			}
			RunState::Victory => break,
			other => panic!("unexpected {other:?}"),
		}
	}
	assert_eq!(shops, 9);
	let world = h.app().world();
	let run = world.resource::<Run>();
	assert_eq!(run.wave, 10);
	assert_eq!(run.result.unwrap().outcome, Outcome::Victory);
	assert_eq!(run.settlements, 1);
	let profile = world.resource::<Profile>();
	assert_eq!(
		profile.granted.len(),
		4,
		"all four challenges earned: {:?}",
		profile.granted
	);
	assert_eq!((profile.completed_runs, profile.victories), (1, 1));
}

#[test]
fn da13_player_and_boss_dying_on_the_same_tick_settles_one_defeat_without_grants() {
	let (mut h, player) = harness();
	// Jump the lifecycle to wave 10 with the boss alive.
	for wave in 2..=10 {
		let w = h.app_mut().world_mut();
		w.get_mut::<Health>(player).unwrap().current = 1_000_000;
		w.get_mut::<Health>(player).unwrap().max = 1_000_000;
		omega_game::encounters::start_wave(h.app_mut().world_mut(), wave).unwrap();
		h.app_mut().world_mut().resource_mut::<Run>().wave = wave;
	}
	for _ in 0..130 {
		h.step(TickInput::default());
	}
	let boss_alive = {
		let w = h.app_mut().world_mut();
		w.query_filtered::<(), With<BossAi>>().iter(w).count()
	};
	assert_eq!(boss_alive, 1, "the Foreman materialized");
	boss_killer(&mut h);
	h.app_mut()
		.world_mut()
		.get_mut::<Health>(player)
		.unwrap()
		.current = 0;
	h.step(TickInput::default());
	for _ in 0..10 {
		h.step(TickInput::default());
	}
	let world = h.app().world();
	let run = world.resource::<Run>();
	assert_eq!(run.state, RunState::Defeat);
	assert_eq!(run.result.unwrap().outcome, Outcome::Defeat);
	assert_eq!(run.settlements, 1);
	assert!(
		!world
			.resource::<Profile>()
			.granted
			.contains("unlock_shutdown"),
		"no wave-10 completion, no shutdown unlock"
	);
}

#[test]
fn abandoning_keeps_already_granted_unlocks() {
	let (mut h, player) = harness();
	let mut guard = 0;
	while !h
		.app()
		.world()
		.resource::<Profile>()
		.granted
		.contains("unlock_first_shift")
	{
		guard += 1;
		assert!(guard < 100_000);
		match h.app().world().resource::<Run>().state {
			RunState::Wave => {
				h.app_mut()
					.world_mut()
					.get_mut::<Health>(player)
					.unwrap()
					.current = 1_000_000;
				h.step(TickInput::default());
			}
			RunState::Shop => start_next_wave(h.app_mut().world_mut()).unwrap(),
			other => panic!("unexpected {other:?}"),
		}
	}
	assert_eq!(h.app().world().resource::<Run>().state, RunState::Shop);
	abandon(h.app_mut().world_mut()).unwrap();
	let world = h.app().world();
	assert_eq!(world.resource::<Run>().state, RunState::Summary);
	assert_eq!(
		world.resource::<Run>().result.unwrap().outcome,
		Outcome::Abandoned
	);
	assert!(
		world
			.resource::<Profile>()
			.granted
			.contains("unlock_first_shift")
	);
	assert_eq!(world.resource::<Profile>().completed_runs, 1);
}
