//! SP-002 acceptance: ordered tick schedule, repeatability, pause and presentation
//! queue delivery through the headless harness.

use bevy::prelude::*;
use bevy::time::TimeUpdateStrategy;
use omega_game::sim::*;
use std::time::Duration;

fn seed() -> RunSeed {
	RunSeed::from_hex(&"5e".repeat(32)).unwrap()
}

/// Test-only systems: consume RNG and IDs and emit markers, so traces carry signal.
fn busy_app() -> App {
	fn consume(mut rng: ResMut<RngStreams>, mut ids: ResMut<IdAllocator>, tick: Res<SimTick>) {
		if tick.0.is_multiple_of(7) {
			rng.spawn_types.below(4);
			ids.sim_id();
		}
		if tick.0.is_multiple_of(11) {
			rng.crit.below(10_000);
			ids.attack_id();
		}
	}
	fn emit(mut queue: ResMut<PresentationQueue>, tick: Res<SimTick>) {
		if tick.0.is_multiple_of(3) {
			queue.push(*tick, PresentationKind::Marker(tick.0 as u32));
			queue.push(*tick, PresentationKind::Marker(tick.0 as u32 + 1));
		}
	}
	let mut app = App::new();
	app.add_plugins(GameSimPlugin);
	app.add_systems(
		GameTick,
		(consume.in_set(TickSet::Emit), emit.in_set(TickSet::Publish)),
	);
	app
}

#[test]
fn ac1_identical_seed_and_input_give_identical_traces() {
	let inputs = vec![TickInput::default(); 600];
	let a = SimHarness::with_app(busy_app(), seed()).run(&inputs);
	let b = SimHarness::with_app(busy_app(), seed()).run(&inputs);
	assert_eq!(a.len(), 600);
	assert_eq!(a, b);
	assert!(
		a.last().unwrap().rng_positions.iter().any(|p| *p > 0),
		"fixture consumed RNG"
	);
	let other = SimHarness::with_app(busy_app(), RunSeed([1; 32])).run(&inputs);
	assert_eq!(
		other.len(),
		600,
		"a different seed still runs; positions are counts"
	);
}

#[test]
fn ac2_six_hundred_manual_ticks_without_a_window() {
	let mut h = SimHarness::new(seed());
	let traces = h.run(&vec![TickInput::default(); 600]);
	assert_eq!(h.tick(), 600);
	assert_eq!(
		traces.iter().map(|t| t.tick).collect::<Vec<_>>(),
		(1..=600).collect::<Vec<_>>()
	);
	for (_, info, _) in h.app().world().iter_resources() {
		let name = info.name().to_string();
		assert!(
			!["Window", "Winit", "Render", "Audio", "Gilrs"]
				.iter()
				.any(|f| name.contains(f)),
			"headless harness created {name}"
		);
	}
}

#[test]
fn ac3_harness_pause_advances_nothing() {
	let mut h = SimHarness::with_app(busy_app(), seed());
	h.run(&vec![TickInput::default(); 30]);
	let before = h.trace();
	h.set_paused(true);
	for _ in 0..10 {
		assert!(!h.step(TickInput::default()));
	}
	assert_eq!(h.trace(), before, "no tick, id or RNG change while paused");
	h.set_paused(false);
	assert!(h.step(TickInput::default()));
	assert_eq!(h.tick(), before.tick + 1);
}

/// Pause through the real fixed-step path: ten application frames of 1/60 s each
/// advance no tick, and resuming does not replay the paused wall time.
#[test]
fn ac3_app_frames_while_paused_do_not_build_a_backlog() {
	let mut app = busy_app();
	app.add_plugins(bevy::time::TimePlugin)
		.insert_resource(RngStreams::new(&seed()))
		.insert_resource(seed())
		.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_nanos(
			16_666_667,
		)));
	for _ in 0..20 {
		app.update();
	}
	let running = app.world().resource::<SimTick>().0;
	assert!(running >= 18, "about one tick per frame, got {running}");

	app.world_mut().resource_mut::<SimPaused>().0 = true;
	app.update();
	let paused_at = app.world().resource::<SimTick>().0;
	let rng_at = app.world().resource::<RngStreams>().positions();
	for _ in 0..10 {
		app.update();
	}
	assert_eq!(app.world().resource::<SimTick>().0, paused_at);
	assert_eq!(app.world().resource::<RngStreams>().positions(), rng_at);

	app.world_mut().resource_mut::<SimPaused>().0 = false;
	for _ in 0..3 {
		app.update();
	}
	let resumed = app.world().resource::<SimTick>().0 - paused_at;
	assert!(
		resumed <= 3,
		"resume replayed paused time: {resumed} ticks in 3 frames"
	);
}

#[test]
fn ac4_records_from_many_ticks_arrive_once_in_order() {
	let mut h = SimHarness::with_app(busy_app(), seed());
	// Several fixed ticks before one "rendered frame" drain.
	h.run(&vec![TickInput::default(); 10]);
	let first = h.drain_presentation();
	h.run(&[TickInput::default(); 5]);
	let second = h.drain_presentation();
	assert!(
		h.drain_presentation().is_empty(),
		"drained records never repeat"
	);
	let all: Vec<_> = first.iter().chain(second.iter()).collect();
	assert_eq!(
		all.len(),
		2 * 5,
		"ticks 3, 6, 9, 12, 15 emit two records each"
	);
	for (i, r) in all.iter().enumerate() {
		assert_eq!(r.sequence, i as u64);
	}
	assert!(
		all.windows(2)
			.all(|w| (w[0].tick, w[0].sequence) < (w[1].tick, w[1].sequence))
	);
	assert_eq!(first.last().unwrap().tick, 9);
}

#[test]
fn schedule_sets_run_in_documented_order() {
	#[derive(Resource, Default)]
	struct Order(Vec<TickSet>);
	let mut app = App::new();
	app.add_plugins(GameSimPlugin).init_resource::<Order>();
	for set in TickSet::ORDER {
		app.add_systems(
			GameTick,
			(move |mut o: ResMut<Order>| o.0.push(set)).in_set(set),
		);
	}
	let mut h = SimHarness::with_app(app, seed());
	h.step(TickInput::default());
	assert_eq!(
		h.app().world().resource::<Order>().0,
		TickSet::ORDER.to_vec()
	);
}

#[test]
fn ids_are_monotonic() {
	let mut ids = IdAllocator::default();
	let a = ids.sim_id();
	let b = ids.sim_id();
	assert!(b > a);
	assert!(ids.attack_id() < ids.attack_id());
}
