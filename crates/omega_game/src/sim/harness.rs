use super::{
	GameSimPlugin, GameTick, IdAllocator, PresentationQueue, PresentationRecord, RngStreams,
	RunSeed, SimPaused, SimTick, TickInput,
};
use bevy::prelude::*;
use serde::Serialize;

/// Sorted authoritative state after one tick. Gameplay specs extend it with actor
/// positions, HP, loadout and currency; it never contains presentation data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TraceEntry {
	pub tick: u64,
	pub next_sim_id: u64,
	pub next_attack_id: u64,
	/// Words consumed per stream: spawn types, spawn anchors, shop, crit.
	pub rng_positions: [u128; 4],
	pub presentation_sequence: u64,
}

/// Headless driver for the exact `GameTick` schedule the client runs. It owns no
/// window, renderer, audio or wall clock; each `step` is one tick.
pub struct SimHarness {
	app: App,
}

impl SimHarness {
	pub fn new(seed: RunSeed) -> Self {
		Self::with_app(App::new(), seed)
	}

	/// Uses a caller-prepared app, for example one with extra test systems.
	pub fn with_app(mut app: App, seed: RunSeed) -> Self {
		if !app.is_plugin_added::<GameSimPlugin>() {
			app.add_plugins(GameSimPlugin);
		}
		app.insert_resource(RngStreams::new(&seed))
			.insert_resource(seed);
		Self { app }
	}

	pub fn app(&self) -> &App {
		&self.app
	}

	pub fn app_mut(&mut self) -> &mut App {
		&mut self.app
	}

	pub fn set_paused(&mut self, paused: bool) {
		self.app.world_mut().resource_mut::<SimPaused>().0 = paused;
	}

	/// Runs one tick with `input`. Returns `false` without effects while paused.
	pub fn step(&mut self, input: TickInput) -> bool {
		let world = self.app.world_mut();
		if world.resource::<SimPaused>().0 {
			return false;
		}
		*world.resource_mut::<TickInput>() = input;
		world.run_schedule(GameTick);
		true
	}

	pub fn tick(&self) -> u64 {
		self.app.world().resource::<SimTick>().0
	}

	pub fn drain_presentation(&mut self) -> Vec<PresentationRecord> {
		self.app
			.world_mut()
			.resource_mut::<PresentationQueue>()
			.drain()
	}

	pub fn trace(&self) -> TraceEntry {
		let world = self.app.world();
		let (next_sim_id, next_attack_id) = world.resource::<IdAllocator>().peek();
		TraceEntry {
			tick: world.resource::<SimTick>().0,
			next_sim_id,
			next_attack_id,
			rng_positions: world.resource::<RngStreams>().positions(),
			presentation_sequence: world.resource::<PresentationQueue>().next_sequence(),
		}
	}

	/// Steps `inputs.len()` ticks and returns the trace after each one.
	pub fn run(&mut self, inputs: &[TickInput]) -> Vec<TraceEntry> {
		let mut traces = Vec::with_capacity(inputs.len());
		for input in inputs {
			if self.step(*input) {
				traces.push(self.trace());
			}
		}
		traces
	}
}
