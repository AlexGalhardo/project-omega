//! Run lifecycle: Wave, Shop, Victory, Defeat and Summary with guarded transitions
//! (SP-011, GDD section 4). Transitions resolve in `TickSet::Transition`, after damage
//! and collection, and death is always decided before wave completion.

use super::{GameTick, TickSet};
use crate::combat::actor::{
	ContentRes, Health, Invulnerability, Loadout, Pickup, Player, Pose2, PreviousPose2, Scrap,
	SpawnError, spawn_player,
};
use crate::combat::damage::{HitCandidates, HostileHits};
use crate::combat::stats::RegenAccumulator;
use crate::combat::weapons::{EmittedShapes, HitLedger, Projectile, WeaponTimers};
use crate::encounters::hostile::HostileProjectile;
use crate::encounters::{WaveError, WaveRuntime, complete_wave, start_wave};
use crate::input::AimOrigin;
use crate::shop::{RunBuild, ShopVisit, open_shop};
use bevy::math::DVec2;
use bevy::prelude::*;
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunState {
	Wave,
	Shop,
	Victory,
	Defeat,
	Summary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
	Victory,
	Defeat,
	Abandoned,
}

/// Immutable once settled; a second settlement attempt is ignored.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunResult {
	pub outcome: Outcome,
	pub waves_completed: u32,
	pub scrap: u32,
}

#[derive(Resource, Debug, Clone, PartialEq, Eq)]
pub struct Run {
	pub state: RunState,
	pub wave: u32,
	pub total_waves: u32,
	pub result: Option<RunResult>,
	/// Times a result was written; stays at most one per run.
	pub settlements: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BeginError {
	Locked,
	Spawn(SpawnError),
	Wave(WaveError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StartError {
	NotInShop,
	/// No equipped weapon has an energy point (DA-03).
	PowerRequired,
	Wave(WaveError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunError {
	WrongState,
}

/// Run condition: combat systems tick only while a wave is live (or no run exists).
pub fn wave_active(run: Option<Res<Run>>) -> bool {
	run.is_none_or(|r| r.state == RunState::Wave)
}

fn settle(world: &mut World, outcome: Outcome) {
	let scrap = world.resource::<Scrap>().0;
	let mut run = world.resource_mut::<Run>();
	if run.result.is_some() {
		return;
	}
	let waves_completed = match outcome {
		Outcome::Victory => run.wave,
		Outcome::Defeat | Outcome::Abandoned => run.wave.saturating_sub(1),
	};
	run.result = Some(RunResult {
		outcome,
		waves_completed,
		scrap,
	});
	run.settlements += 1;
}

fn clear_transients(world: &mut World) {
	let doomed: Vec<Entity> = world
		.query_filtered::<Entity, With<Projectile>>()
		.iter(world)
		.collect();
	for entity in doomed {
		world.despawn(entity);
	}
	world.resource_mut::<EmittedShapes>().0.clear();
	world.resource_mut::<HitLedger>().0.clear();
	world.resource_mut::<HitCandidates>().0.clear();
	let hostile: Vec<Entity> = world
		.query_filtered::<Entity, With<HostileProjectile>>()
		.iter(world)
		.collect();
	for entity in hostile {
		world.despawn(entity);
	}
	let mut hits = world.resource_mut::<HostileHits>();
	hits.pending.clear();
	hits.landed.clear();
}

/// Successful completion collects every remaining earned pickup.
fn collect_remaining_pickups(world: &mut World) {
	let pickups: Vec<(Entity, u32)> = world
		.query::<(Entity, &Pickup)>()
		.iter(world)
		.map(|(e, p)| (e, p.value))
		.collect();
	for (entity, value) in pickups {
		world.resource_mut::<Scrap>().0 += value;
		world.despawn(entity);
	}
}

fn player_alive(world: &mut World) -> Option<bool> {
	world
		.query_filtered::<&Health, With<Player>>()
		.iter(world)
		.next()
		.map(|h| h.current > 0)
}

/// Starts a run: spawns the player, resets currency and enters wave 1.
pub fn begin_run(
	world: &mut World,
	character_id: &str,
	unlocked: BTreeSet<String>,
) -> Result<(), BeginError> {
	let content = world.resource::<ContentRes>().0.clone();
	let character = content
		.catalogs
		.characters
		.get(character_id)
		.ok_or(BeginError::Spawn(SpawnError::UnknownCharacter))?;
	let open = match &character.availability {
		crate::content::model::Availability::Initial => true,
		crate::content::model::Availability::Unlock(id) => unlocked.contains(id),
	};
	if !open {
		return Err(BeginError::Locked);
	}
	let stale: Vec<Entity> = world
		.query_filtered::<Entity, Or<(With<Player>, With<Pickup>, With<Projectile>)>>()
		.iter(world)
		.collect();
	for entity in stale {
		world.despawn(entity);
	}
	spawn_player(world, character_id, DVec2::ZERO).map_err(BeginError::Spawn)?;
	world.insert_resource(Scrap(0));
	world.remove_resource::<ShopVisit>();
	world.insert_resource(RunBuild {
		character_id: character_id.to_string(),
		items: BTreeSet::new(),
		unlocked,
	});
	let total_waves = u32::try_from(content.catalogs.waves.len()).unwrap_or(u32::MAX);
	world.insert_resource(Run {
		state: RunState::Wave,
		wave: 1,
		total_waves,
		result: None,
		settlements: 0,
	});
	start_wave(world, 1).map_err(BeginError::Wave)
}

/// Shop to Wave: needs at least one powered weapon; resets per-wave player state.
pub fn start_next_wave(world: &mut World) -> Result<(), StartError> {
	let run = world.get_resource::<Run>().ok_or(StartError::NotInShop)?;
	if run.state != RunState::Shop {
		return Err(StartError::NotInShop);
	}
	let next = run.wave + 1;
	let powered = world
		.query_filtered::<&Loadout, With<Player>>()
		.iter(world)
		.next()
		.is_some_and(|l| l.energy.any_powered(l.occupied()));
	if !powered {
		return Err(StartError::PowerRequired);
	}
	let player = world
		.query_filtered::<Entity, With<Player>>()
		.iter(world)
		.next()
		.ok_or(StartError::NotInShop)?;
	start_wave(world, next).map_err(StartError::Wave)?;
	if let Some(mut health) = world.get_mut::<Health>(player) {
		health.current = health.max;
	}
	world.entity_mut(player).insert((
		RegenAccumulator::default(),
		Invulnerability::default(),
		WeaponTimers::ready(),
		Pose2(DVec2::ZERO),
		PreviousPose2(DVec2::ZERO),
	));
	world.resource_mut::<AimOrigin>().0 = DVec2::ZERO;
	world.remove_resource::<ShopVisit>();
	let mut run = world.resource_mut::<Run>();
	run.wave = next;
	run.state = RunState::Wave;
	Ok(())
}

/// Confirmed abandon from Wave or Shop: result is `Abandoned`, the build is discarded.
pub fn abandon(world: &mut World) -> Result<(), RunError> {
	let run = world.get_resource::<Run>().ok_or(RunError::WrongState)?;
	if !matches!(run.state, RunState::Wave | RunState::Shop) {
		return Err(RunError::WrongState);
	}
	settle(world, Outcome::Abandoned);
	world.remove_resource::<ShopVisit>();
	world.resource_mut::<Run>().state = RunState::Summary;
	Ok(())
}

/// Victory or Defeat to Summary on an explicit continue.
pub fn continue_to_summary(world: &mut World) -> Result<(), RunError> {
	let run = world.get_resource::<Run>().ok_or(RunError::WrongState)?;
	if !matches!(run.state, RunState::Victory | RunState::Defeat) {
		return Err(RunError::WrongState);
	}
	world.resource_mut::<Run>().state = RunState::Summary;
	Ok(())
}

fn run_transitions(world: &mut World) {
	let Some(run) = world.get_resource::<Run>() else {
		return;
	};
	if run.state != RunState::Wave {
		return;
	}
	if player_alive(world) == Some(false) {
		settle(world, Outcome::Defeat);
		clear_transients(world);
		world.resource_mut::<Run>().state = RunState::Defeat;
		return;
	}
	let Some(runtime) = world.get_resource::<WaveRuntime>() else {
		return;
	};
	if !runtime.completed {
		return;
	}
	collect_remaining_pickups(world);
	clear_transients(world);
	let (wave, total) = {
		let run = world.resource::<Run>();
		(run.wave, run.total_waves)
	};
	if wave >= total {
		settle(world, Outcome::Victory);
		world.resource_mut::<Run>().state = RunState::Victory;
	} else if let Err(reject) = open_shop(world, wave) {
		// Content and build are validated at run start; a failure here is a defect.
		eprintln!("cannot open shop after wave {wave}: {reject:?}");
	} else {
		world.resource_mut::<Run>().state = RunState::Shop;
	}
}

pub struct RunPlugin;

impl Plugin for RunPlugin {
	fn build(&self, app: &mut App) {
		for set in [
			TickSet::Spawn,
			TickSet::Intent,
			TickSet::Movement,
			TickSet::Emit,
			TickSet::Collide,
			TickSet::Resolve,
			TickSet::CollectHeal,
		] {
			app.configure_sets(GameTick, set.run_if(wave_active));
		}
		app.add_systems(
			GameTick,
			run_transitions
				.after(complete_wave)
				.in_set(TickSet::Transition),
		);
	}
}
