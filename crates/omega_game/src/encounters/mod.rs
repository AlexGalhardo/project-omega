//! Wave scheduling, spawn warnings and chase behavior (SP-008, GDD sections 10-11).
//!
//! Bosses and the ranged/charge/strike profiles belong to later tasks; a wave that
//! authors a boss is held open here by `boss_blocking` until GAME-P3-004 settles it.

use crate::combat::actor::{
	ArenaRes, Body, ContentRes, Enemy, EnemyStats, Health, PLAYER_RADIUS_LU, Player, Pose2,
	PreviousPose2, spawn_enemy,
};
use crate::content::model::Behavior;
use crate::sim::{
	GameTick, IdAllocator, PresentationKind, PresentationQueue, RngStreams, SimId, SimTick,
	TICKS_PER_SECOND, TickSet, advance_tick,
};
use bevy::math::DVec2;
use bevy::prelude::*;

/// Ticks a spawn warning is visible before materialization (GDD section 11).
pub const WARNING_TICKS: u64 = 45;
/// Ticks added when the player occupies a warning at expiry.
pub const POSTPONE_TICKS: u64 = 30;
/// Postponements after which a common spawn is cancelled without reward.
pub const POSTPONE_CAP: u32 = 10;
/// Anchors closer than this to the player center are not eligible.
pub const ANCHOR_SAFE_LU: f64 = 180.0;
/// Extra clearance added to the radius sum in the occupied-warning test.
pub const OCCUPIED_MARGIN_LU: f64 = 30.0;

/// Wave progress; `tick0` is the `SimTick` at wave entry, so wave tick = tick - tick0.
#[derive(Resource, Debug, Clone, PartialEq, Eq)]
pub struct WaveRuntime {
	pub number: u32,
	pub tick0: u64,
	pub duration: u64,
	/// Next batch to request.
	pub cursor: usize,
	/// A boss is authored for this wave and not yet settled.
	pub boss_blocking: bool,
	/// Eligibility fact only: the lifecycle decides what completion means.
	pub completed: bool,
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct SpawnWarning {
	pub enemy_id: String,
	pub position: DVec2,
	pub materialize_tick: u64,
	pub postponements: u32,
	pub hp: u32,
	pub contact_damage: u32,
}

/// Marker for the chase profile (chaser, runner, bulwark).
#[derive(Component, Debug, Clone, Copy)]
pub struct ChaseAi;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WaveError {
	UnknownWave,
}

/// `ceil(base * (100 + per_wave * (w - 1)) / 100)` (GDD section 10.1).
pub fn scaled(base: u32, per_wave: u32, wave: u32) -> u32 {
	let factor = 100 + u64::from(per_wave) * u64::from(wave.saturating_sub(1));
	u32::try_from((u64::from(base) * factor).div_ceil(100)).unwrap_or(u32::MAX)
}

/// Enters a wave: resets the runtime and removes leftovers of the previous one.
pub fn start_wave(world: &mut World, number: u32) -> Result<(), WaveError> {
	let content = world.resource::<ContentRes>().0.clone();
	let wave = content
		.catalogs
		.waves
		.get(&number)
		.ok_or(WaveError::UnknownWave)?;
	let tick0 = world.resource::<SimTick>().0;
	world.insert_resource(WaveRuntime {
		number,
		tick0,
		duration: u64::from(wave.duration_ticks),
		cursor: 0,
		boss_blocking: wave.boss.is_some(),
		completed: false,
	});
	clear_hostiles(world);
	Ok(())
}

fn clear_hostiles(world: &mut World) {
	let stale: Vec<Entity> = world
		.query_filtered::<Entity, Or<(With<SpawnWarning>, With<Enemy>)>>()
		.iter(world)
		.collect();
	for entity in stale {
		world.despawn(entity);
	}
}

fn anchors(min: DVec2, max: DVec2, inset: f64) -> [DVec2; 8] {
	let (lo, hi) = (min + DVec2::splat(inset), max - DVec2::splat(inset));
	let mid = (lo + hi) * 0.5;
	// Clockwise from top-center; the top edge is +Y.
	[
		DVec2::new(mid.x, hi.y),
		DVec2::new(hi.x, hi.y),
		DVec2::new(hi.x, mid.y),
		DVec2::new(hi.x, lo.y),
		DVec2::new(mid.x, lo.y),
		DVec2::new(lo.x, lo.y),
		DVec2::new(lo.x, mid.y),
		DVec2::new(lo.x, hi.y),
	]
}

/// Requests the batches whose tick has arrived and creates their 45-tick warnings.
#[allow(clippy::too_many_arguments)]
pub fn schedule_batches(
	tick: Res<SimTick>,
	content: Option<Res<ContentRes>>,
	arena: Option<Res<ArenaRes>>,
	runtime: Option<ResMut<WaveRuntime>>,
	mut rng: ResMut<RngStreams>,
	mut ids: ResMut<IdAllocator>,
	mut queue: ResMut<PresentationQueue>,
	mut commands: Commands,
	players: Query<(&Pose2, &Health), With<Player>>,
) {
	let (Some(content), Some(arena), Some(mut runtime)) = (content, arena, runtime) else {
		return;
	};
	let Ok((player, health)) = players.single() else {
		return;
	};
	let Some(wave) = content.0.catalogs.waves.get(&runtime.number) else {
		return;
	};
	let wave_tick = tick.0.saturating_sub(runtime.tick0);
	if runtime.completed || health.current == 0 {
		return;
	}
	while let Some(batch) = wave.batches.get(runtime.cursor) {
		if wave_tick < u64::from(batch.request_tick) {
			break;
		}
		runtime.cursor += 1;
		let total: u32 = batch.enemies.iter().map(|(_, w)| *w).sum();
		let mut used = [false; 8];
		for _ in 0..batch.count {
			let mut pick = rng.spawn_types.below(total);
			let Some((enemy_id, _)) = batch.enemies.iter().find(|(_, w)| {
				if pick < *w {
					true
				} else {
					pick -= *w;
					false
				}
			}) else {
				continue;
			};
			let Some(def) = content.0.catalogs.enemies.get(enemy_id) else {
				continue;
			};
			let points = anchors(arena.0.min(), arena.0.max(), def.radius_lu + 4.0);
			let far = |i: usize| points[i].distance(player.0) >= ANCHOR_SAFE_LU;
			let mut eligible: Vec<usize> = (0..8).filter(|&i| far(i) && !used[i]).collect();
			if eligible.is_empty() {
				// Every eligible anchor was used in this batch: reuse is allowed again.
				used = [false; 8];
				eligible = (0..8).filter(|&i| far(i)).collect();
			}
			if eligible.is_empty() {
				continue;
			}
			let chosen = eligible[rng.spawn_anchors.below(eligible.len() as u32) as usize];
			used[chosen] = true;
			let sim_id = ids.sim_id();
			queue.push(
				*tick,
				PresentationKind::SpawnWarned {
					sim_id: sim_id.0,
					position: [points[chosen].x, points[chosen].y],
					ticks: WARNING_TICKS as u32,
				},
			);
			commands.spawn((
				sim_id,
				SpawnWarning {
					enemy_id: enemy_id.clone(),
					position: points[chosen],
					materialize_tick: tick.0 + WARNING_TICKS,
					postponements: 0,
					hp: scaled(def.base_hp, 18, runtime.number),
					contact_damage: scaled(def.contact_damage, 8, runtime.number),
				},
			));
		}
	}
}

/// Materializes expired warnings in `SimId` order, postponing or cancelling occupied ones.
pub fn resolve_warnings(world: &mut World) {
	let tick = world.resource::<SimTick>().0;
	if !world.contains_resource::<WaveRuntime>() || world.resource::<WaveRuntime>().completed {
		return;
	}
	let Some(player) = world
		.query_filtered::<(&Pose2, &Health), With<Player>>()
		.iter(world)
		.next()
		.map(|(p, h)| (p.0, h.current))
	else {
		return;
	};
	let content = world.resource::<ContentRes>().0.clone();
	let mut due: Vec<(SimId, Entity)> = world
		.query::<(Entity, &SimId, &SpawnWarning)>()
		.iter(world)
		.filter(|(_, _, w)| w.materialize_tick <= tick)
		.map(|(e, id, _)| (*id, e))
		.collect();
	due.sort_by_key(|(id, _)| *id);
	for (id, entity) in due {
		let warning = world.get::<SpawnWarning>(entity).cloned().unwrap();
		let radius = content
			.catalogs
			.enemies
			.get(&warning.enemy_id)
			.map_or(0.0, |d| d.radius_lu);
		let occupied = player.1 > 0
			&& warning.position.distance(player.0)
				<= radius + PLAYER_RADIUS_LU + OCCUPIED_MARGIN_LU;
		if occupied {
			if warning.postponements >= POSTPONE_CAP {
				world.despawn(entity);
				world.resource_mut::<PresentationQueue>().push(
					SimTick(tick),
					PresentationKind::SpawnCancelled { sim_id: id.0 },
				);
			} else {
				let mut w = world.get_mut::<SpawnWarning>(entity).unwrap();
				w.postponements += 1;
				w.materialize_tick = tick + POSTPONE_TICKS;
			}
			continue;
		}
		world.despawn(entity);
		if let Ok(enemy) = spawn_enemy(
			world,
			&warning.enemy_id,
			warning.position,
			warning.hp,
			warning.contact_damage,
		) {
			let def = &content.catalogs.enemies[&warning.enemy_id];
			if def.behavior == Behavior::Chase {
				world.entity_mut(enemy).insert(ChaseAi);
			}
			let sim_id = world.get::<SimId>(enemy).copied().unwrap();
			world.resource_mut::<PresentationQueue>().push(
				SimTick(tick),
				PresentationKind::EnemySpawned {
					sim_id: sim_id.0,
					warning: id.0,
					position: [warning.position.x, warning.position.y],
				},
			);
		}
	}
}

/// Chase: normalized velocity toward the player each tick; equal centers do not move.
pub fn chase_player(
	arena: Option<Res<ArenaRes>>,
	players: Query<&Pose2, (With<Player>, Without<ChaseAi>)>,
	mut enemies: Query<(&mut Pose2, &mut PreviousPose2, &Body, &EnemyStats), With<ChaseAi>>,
) {
	let Ok(target) = players.single() else {
		return;
	};
	for (mut pose, mut previous, body, stats) in &mut enemies {
		previous.0 = pose.0;
		let to = target.0 - pose.0;
		if to.length_squared() == 0.0 {
			continue;
		}
		let mut next = pose.0 + to.normalize() * stats.speed_lu_s / f64::from(TICKS_PER_SECOND);
		if let Some(arena) = &arena {
			next = arena.0.clamp_circle_center(next, body.0);
		}
		pose.0 = next;
	}
}

/// Timer completion: with the player alive, no pending boss and the duration elapsed,
/// the wave is complete; survivors and pending warnings are removed without rewards.
pub fn complete_wave(world: &mut World) {
	let tick = world.resource::<SimTick>().0;
	let Some(runtime) = world.get_resource::<WaveRuntime>() else {
		return;
	};
	if runtime.completed || runtime.boss_blocking || tick - runtime.tick0 < runtime.duration {
		return;
	}
	let alive = world
		.query_filtered::<&Health, With<Player>>()
		.iter(world)
		.any(|h| h.current > 0);
	if !alive {
		return;
	}
	world.resource_mut::<WaveRuntime>().completed = true;
	clear_hostiles(world);
}

pub struct EncounterPlugin;

impl Plugin for EncounterPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(
			GameTick,
			(schedule_batches, resolve_warnings.after(schedule_batches))
				.after(advance_tick)
				.in_set(TickSet::Spawn),
		)
		.add_systems(GameTick, chase_player.in_set(TickSet::Movement))
		.add_systems(GameTick, complete_wave.in_set(TickSet::Transition));
	}
}
