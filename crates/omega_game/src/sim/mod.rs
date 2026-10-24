//! Authoritative fixed-tick simulation (SP-002, architecture section 5).
//!
//! One `GameTick` schedule is the only gameplay path. The native client runs it from
//! `FixedUpdate` at 60 Hz; the headless harness runs it directly, N times, with no
//! display loop. No system here reads wall-clock time or renderer state.

mod harness;
mod rng;
pub mod run;

pub use harness::{SimHarness, TraceEntry};
pub use rng::{RngStreams, RunSeed, SeedParseError, SimRng, StreamTag};

use bevy::ecs::schedule::ScheduleLabel;
use bevy::math::DVec2;
use bevy::prelude::*;

/// Ticks per simulated second (GDD-005).
pub const TICKS_PER_SECOND: u32 = 60;

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameTick;

/// Ordered stages of one tick. Deferred commands flush between every stage, so a
/// stage always sees entities spawned or despawned by the previous one.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TickSet {
	Spawn,
	Intent,
	Movement,
	Emit,
	Collide,
	Resolve,
	CollectHeal,
	Transition,
	Publish,
}

impl TickSet {
	pub const ORDER: [TickSet; 9] = [
		TickSet::Spawn,
		TickSet::Intent,
		TickSet::Movement,
		TickSet::Emit,
		TickSet::Collide,
		TickSet::Resolve,
		TickSet::CollectHeal,
		TickSet::Transition,
		TickSet::Publish,
	];
}

#[derive(Resource, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SimTick(pub u64);

/// Pause freezes ticks, timers and RNG. The client also pauses virtual time, so the
/// wall time spent paused never becomes a fixed-step backlog.
#[derive(Resource, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SimPaused(pub bool);

/// Monotonic run-scoped identifiers; never reused within a run.
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub struct IdAllocator {
	next_sim_id: u64,
	next_attack_id: u64,
}

impl Default for IdAllocator {
	fn default() -> Self {
		Self {
			next_sim_id: 1,
			next_attack_id: 1,
		}
	}
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SimId(pub u64);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AttackId(pub u64);

impl IdAllocator {
	pub fn sim_id(&mut self) -> SimId {
		let id = SimId(self.next_sim_id);
		self.next_sim_id += 1;
		id
	}

	pub fn attack_id(&mut self) -> AttackId {
		let id = AttackId(self.next_attack_id);
		self.next_attack_id += 1;
		id
	}

	pub fn peek(&self) -> (u64, u64) {
		(self.next_sim_id, self.next_attack_id)
	}
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum AimInput {
	#[default]
	Auto,
	/// Normalized world direction; game input already rejected letterbox points.
	Manual(DVec2),
}

/// Intent for exactly one tick, sampled before the fixed loop.
#[derive(Resource, Debug, Clone, Copy, Default, PartialEq)]
pub struct TickInput {
	pub movement: DVec2,
	pub aim: AimInput,
}

/// Shape of a hostile warning, copied from the authoritative attack.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CueShapeFact {
	Circle { radius: f64 },
	Capsule { reach: f64, radius: f64 },
}

/// Where a warning is anchored: fixed in the world, or following its attacker's origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CueAnchorFact {
	Locked([f64; 2]),
	Follow { direction: [f64; 2] },
}

/// Presentation-only fact. Gameplay specs add variants; none is authoritative.
#[derive(Debug, Clone, PartialEq)]
pub enum PresentationKind {
	/// Scenario/test marker with a caller-chosen value.
	Marker(u32),
	/// A player weapon attack was emitted (slot is 0-based).
	AttackEmitted {
		attack: u64,
		slot: u8,
		origin: [f64; 2],
		direction: [f64; 2],
	},
	ProjectileSpawned {
		sim_id: u64,
		attack: u64,
		origin: [f64; 2],
		direction: [f64; 2],
		radius: f64,
	},
	ProjectileEnded {
		sim_id: u64,
		position: [f64; 2],
	},
	Explosion {
		attack: u64,
		center: [f64; 2],
		radius: f64,
	},
	HitAccepted {
		attack: u64,
		target: u64,
		damage: u32,
		crit: bool,
		position: [f64; 2],
	},
	EnemyDied {
		target: u64,
		position: [f64; 2],
		scrap: u32,
	},
	PlayerDamaged {
		damage: u32,
		hp_after: u32,
	},
	SpawnWarned {
		sim_id: u64,
		position: [f64; 2],
		ticks: u32,
	},
	SpawnCancelled {
		sim_id: u64,
	},
	EnemySpawned {
		sim_id: u64,
		warning: u64,
		position: [f64; 2],
	},
	/// A hostile windup began; `sim_id` is the attacker's.
	CueStarted {
		sim_id: u64,
		shape: CueShapeFact,
		anchor: CueAnchorFact,
		ticks: u32,
	},
	/// The windup finished (`cancelled == false`, attack emitted) or was cancelled.
	CueEnded {
		sim_id: u64,
		cancelled: bool,
	},
	HostileProjectileSpawned {
		sim_id: u64,
		attack: u64,
		origin: [f64; 2],
		direction: [f64; 2],
		radius: f64,
	},
	HostileProjectileEnded {
		sim_id: u64,
		position: [f64; 2],
	},
	PickupCollected {
		value: u32,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub struct PresentationRecord {
	pub sequence: u64,
	pub tick: u64,
	pub kind: PresentationKind,
}

/// Run-owned queue that survives several fixed ticks and is drained once per
/// rendered frame (or by the harness), so no record is lost or delivered twice.
#[derive(Resource, Debug, Default)]
pub struct PresentationQueue {
	next_sequence: u64,
	records: Vec<PresentationRecord>,
}

impl PresentationQueue {
	pub fn push(&mut self, tick: SimTick, kind: PresentationKind) {
		self.records.push(PresentationRecord {
			sequence: self.next_sequence,
			tick: tick.0,
			kind,
		});
		self.next_sequence += 1;
	}

	pub fn drain(&mut self) -> Vec<PresentationRecord> {
		std::mem::take(&mut self.records)
	}

	pub fn len(&self) -> usize {
		self.records.len()
	}

	pub fn is_empty(&self) -> bool {
		self.records.is_empty()
	}

	pub fn next_sequence(&self) -> u64 {
		self.next_sequence
	}
}

pub(crate) fn advance_tick(mut tick: ResMut<SimTick>) {
	tick.0 += 1;
}

/// Mirrors [`SimPaused`] into virtual time when the app runs a clock (the client).
fn sync_virtual_pause(paused: Res<SimPaused>, time: Option<ResMut<Time<Virtual>>>) {
	if let Some(mut time) = time
		&& time.is_paused() != paused.0
	{
		if paused.0 {
			time.pause();
		} else {
			time.unpause();
		}
	}
}

fn run_game_tick(world: &mut World) {
	if !world.resource::<SimPaused>().0 {
		world.run_schedule(GameTick);
	}
}

/// Registers simulation resources and the ordered `GameTick` schedule. Seed and RNG
/// streams are inserted by whoever creates a run (client run start or harness).
pub struct GameSimPlugin;

impl Plugin for GameSimPlugin {
	fn build(&self, app: &mut App) {
		let mut schedule = Schedule::new(GameTick);
		for pair in TickSet::ORDER.windows(2) {
			schedule.configure_sets(pair[0].before(pair[1]));
			schedule.add_systems(ApplyDeferred.after(pair[0]).before(pair[1]));
		}
		schedule.add_systems(advance_tick.in_set(TickSet::Spawn));
		app.add_schedule(schedule)
			.init_resource::<SimTick>()
			.init_resource::<SimPaused>()
			.init_resource::<IdAllocator>()
			.init_resource::<TickInput>()
			.init_resource::<PresentationQueue>()
			.add_systems(PreUpdate, sync_virtual_pause)
			.add_systems(FixedUpdate, run_game_tick);
	}
}
