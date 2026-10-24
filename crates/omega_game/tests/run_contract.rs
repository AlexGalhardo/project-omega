//! SP-011 acceptance for the prototype lifecycle: Wave, Shop, Victory/Defeat, guards and cleanup.

use bevy::math::DVec2;
use bevy::prelude::*;
use omega_game::combat::actor::*;
use omega_game::combat::energy::EnergyAllocation;
use omega_game::combat::weapons::WeaponTimers;
use omega_game::content::{Profile, load_dir};
use omega_game::encounters::WaveRuntime;
use omega_game::shop::ShopVisit;
use omega_game::sim::run::*;
use omega_game::sim::*;
use std::collections::BTreeSet;
use std::path::PathBuf;
use std::sync::Arc;

fn harness(profile: Profile, dir: &str) -> SimHarness {
	let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(dir);
	let content = load_dir(&path, profile).expect("content");
	let arena = content.catalogs.arenas["arena_foundry"].bounds;
	let mut app = App::new();
	app.add_plugins(omega_game::GameCorePlugin);
	let mut h = SimHarness::with_app(app, RunSeed::from_hex(&"33".repeat(32)).unwrap());
	h.app_mut().insert_resource(ContentRes(Arc::new(content)));
	h.app_mut().insert_resource(ArenaRes(arena));
	h
}

fn prototype() -> SimHarness {
	harness(Profile::Prototype, "../../assets/content")
}

fn begin(h: &mut SimHarness) -> Entity {
	begin_run(h.app_mut().world_mut(), "char_patch", BTreeSet::new()).unwrap();
	let world = h.app_mut().world_mut();
	let player = world
		.query_filtered::<Entity, With<Player>>()
		.single(world)
		.unwrap();
	let mut health = world.get_mut::<Health>(player).unwrap();
	health.current = 1_000_000;
	health.max = 1_000_000;
	player
}

fn state(h: &SimHarness) -> RunState {
	h.app().world().resource::<Run>().state
}

fn step_until(h: &mut SimHarness, wanted: RunState, limit: u32) -> u32 {
	for n in 0..limit {
		if state(h) == wanted {
			return n;
		}
		h.step(TickInput::default());
	}
	panic!("never reached {wanted:?}, stuck in {:?}", state(h));
}

fn world(h: &mut SimHarness) -> &mut World {
	h.app_mut().world_mut()
}

#[test]
fn three_wave_run_visits_two_shops_then_victory() {
	let mut h = prototype();
	begin(&mut h);
	let mut shops = 0;
	loop {
		step_until(&mut h, RunState::Shop, 4000);
		shops += 1;
		assert!(h.app().world().contains_resource::<ShopVisit>());
		start_next_wave(world(&mut h)).unwrap();
		assert_eq!(state(&h), RunState::Wave);
		if h.app().world().resource::<Run>().wave == 3 {
			break;
		}
	}
	step_until(&mut h, RunState::Victory, 4000);
	assert_eq!(shops, 2, "final victory bypasses a third shop");
	let run = h.app().world().resource::<Run>().clone();
	assert_eq!(run.result.unwrap().outcome, Outcome::Victory);
	assert_eq!(run.settlements, 1);
	continue_to_summary(world(&mut h)).unwrap();
	assert_eq!(state(&h), RunState::Summary);
}

#[test]
fn da11_timer_completion_removes_survivors_collects_pickups_and_enters_shop() {
	let mut h = prototype();
	begin(&mut h);
	let enemy = spawn_enemy(
		world(&mut h),
		"enemy_chaser",
		DVec2::new(500.0, 300.0),
		24,
		8,
	)
	.unwrap();
	world(&mut h).spawn((
		SimId(9001),
		Pickup { value: 7 },
		Pose2(DVec2::new(-500.0, 300.0)),
	));
	let before = h.app().world().resource::<Scrap>().0;
	step_until(&mut h, RunState::Shop, 4000);
	assert!(h.app().world().get_entity(enemy).is_err());
	let earned_from_kills = h.app().world().resource::<Scrap>().0 - before;
	assert!(earned_from_kills >= 7, "existing pickup collected");
	let w = world(&mut h);
	assert_eq!(w.query::<&Enemy>().iter(w).count(), 0);
	assert_eq!(w.query::<&Pickup>().iter(w).count(), 0);
}

#[test]
fn da03_start_needs_a_powered_weapon_and_ignores_unspent_points() {
	let mut h = prototype();
	let p = begin(&mut h);
	step_until(&mut h, RunState::Shop, 4000);
	world(&mut h).get_mut::<Loadout>(p).unwrap().energy = EnergyAllocation::new([0, 0, 0, 0]);
	assert_eq!(
		start_next_wave(world(&mut h)),
		Err(StartError::PowerRequired)
	);
	assert_eq!(state(&h), RunState::Shop);
	world(&mut h).get_mut::<Loadout>(p).unwrap().energy = EnergyAllocation::new([3, 0, 0, 0]);
	assert_eq!(start_next_wave(world(&mut h)), Ok(()));
	assert_eq!(state(&h), RunState::Wave);
}

#[test]
fn start_outside_the_shop_is_rejected() {
	let mut h = prototype();
	begin(&mut h);
	assert_eq!(start_next_wave(world(&mut h)), Err(StartError::NotInShop));
}

#[test]
fn da13_death_in_the_completion_tick_settles_defeat_once() {
	let mut h = prototype();
	let p = begin(&mut h);
	for _ in 0..3599 {
		h.step(TickInput::default());
	}
	world(&mut h).get_mut::<Health>(p).unwrap().current = 0;
	h.step(TickInput::default());
	assert_eq!(state(&h), RunState::Defeat);
	for _ in 0..10 {
		h.step(TickInput::default());
	}
	let run = h.app().world().resource::<Run>().clone();
	assert_eq!(run.settlements, 1);
	assert_eq!(run.result.unwrap().outcome, Outcome::Defeat);
	assert!(
		!h.app().world().contains_resource::<ShopVisit>(),
		"no shop after death"
	);
}

#[test]
fn wave_entry_restores_the_player_and_clears_transients_but_keeps_the_build() {
	let mut h = prototype();
	let p = begin(&mut h);
	step_until(&mut h, RunState::Shop, 4000);
	{
		let w = world(&mut h);
		let mut health = w.get_mut::<Health>(p).unwrap();
		health.current = 10;
		health.max = 120;
		w.get_mut::<Invulnerability>(p).unwrap().next_accept_tick = 999_999;
		w.get_mut::<Pose2>(p).unwrap().0 = DVec2::new(40.0, 40.0);
		w.resource_mut::<Scrap>().0 = 55;
	}
	let loadout_before = h.app().world().get::<Loadout>(p).unwrap().clone();
	start_next_wave(world(&mut h)).unwrap();
	let w = h.app().world();
	assert_eq!(w.get::<Health>(p).unwrap().current, 120);
	assert_eq!(w.get::<Invulnerability>(p).unwrap().next_accept_tick, 0);
	assert_eq!(w.get::<Pose2>(p).unwrap().0, DVec2::ZERO);
	assert_eq!(*w.get::<WeaponTimers>(p).unwrap(), WeaponTimers::ready());
	assert_eq!(w.resource::<Scrap>().0, 55, "currency retained");
	assert_eq!(
		w.get::<Loadout>(p).unwrap(),
		&loadout_before,
		"build and energy retained"
	);
	assert!(!w.contains_resource::<ShopVisit>());
	assert_eq!(w.resource::<WaveRuntime>().number, 2);
}

#[test]
fn shop_freezes_combat_ticks() {
	let mut h = prototype();
	begin(&mut h);
	step_until(&mut h, RunState::Shop, 4000);
	let tick = h.tick();
	let rng = h.trace().rng_positions;
	for _ in 0..120 {
		h.step(TickInput::default());
	}
	assert_eq!(h.tick(), tick);
	assert_eq!(h.trace().rng_positions, rng);
}

#[test]
fn abandon_settles_once_and_cancel_is_simply_not_calling_it() {
	let mut h = prototype();
	begin(&mut h);
	step_until(&mut h, RunState::Shop, 4000);
	abandon(world(&mut h)).unwrap();
	let run = h.app().world().resource::<Run>().clone();
	assert_eq!(run.state, RunState::Summary);
	assert_eq!(run.result.unwrap().outcome, Outcome::Abandoned);
	assert_eq!(abandon(world(&mut h)), Err(RunError::WrongState));
	assert_eq!(h.app().world().resource::<Run>().settlements, 1);
}

#[test]
fn a_locked_character_cannot_start_a_run() {
	let mut h = harness(Profile::Full, "tests/fixtures/content/full");
	assert_eq!(
		begin_run(world(&mut h), "char_coil", BTreeSet::new()),
		Err(BeginError::Locked)
	);
	assert!(!h.app().world().contains_resource::<Run>());
	assert_eq!(
		begin_run(
			world(&mut h),
			"char_coil",
			["unlock_first_shift".to_string()].into()
		),
		Ok(())
	);
}
