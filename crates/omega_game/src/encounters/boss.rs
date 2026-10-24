//! Boss encounters (SP-008, GDD section 10.2): a fixed-position warning that postpones while
//! the player stands on it, then an alternating attack schedule with recovery movement.
//! A pending or live boss blocks wave completion; killing it releases the block.

use super::ai::{cue_for, emit, unit};
use super::{BossPhase, POSTPONE_TICKS, WaveRuntime};
use crate::combat::actor::{
	ArenaRes, Body, ContentRes, Enemy, EnemyStats, Health, PLAYER_RADIUS_LU, Player, Pose2,
	PreviousPose2,
};
use crate::combat::damage::HostileHits;
use crate::content::model::{BossAttack, HostilePayload, RecoveryMovement};
use crate::sim::{
	IdAllocator, PresentationKind, PresentationQueue, SimId, SimTick, TICKS_PER_SECOND,
};
use bevy::math::DVec2;
use bevy::prelude::*;

/// Extra clearance in the occupied-warning test, shared with common spawns.
const OCCUPIED_MARGIN_LU: f64 = 30.0;
const KEEP_DISTANCE_BAND_LU: f64 = 20.0;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct BossWarning {
	pub boss_id: String,
	pub position: DVec2,
	pub materialize_tick: u64,
	pub postponements: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BossState {
	Recovery {
		left: u32,
	},
	Windup {
		left: u32,
		aim: DVec2,
		center: DVec2,
		attack: usize,
	},
	Charging {
		left: u32,
		direction: DVec2,
		attack: usize,
	},
}

impl BossAi {
	/// Recovery ticks of the attack that ran last (the one before `next`).
	fn last_recovery(&self) -> u32 {
		let last = (self.next + self.attacks.len() - 1) % self.attacks.len();
		self.attacks[last].recovery_ticks
	}
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct BossAi {
	pub attacks: Vec<BossAttack>,
	pub movement: RecoveryMovement,
	pub next: usize,
	pub state: BossState,
	pub base_contact: u32,
}

/// Requests the boss warning at the authored wave tick.
pub fn schedule_boss(
	tick: Res<SimTick>,
	content: Option<Res<ContentRes>>,
	runtime: Option<ResMut<WaveRuntime>>,
	mut ids: ResMut<IdAllocator>,
	mut queue: ResMut<PresentationQueue>,
	mut commands: Commands,
) {
	let (Some(content), Some(mut runtime)) = (content, runtime) else {
		return;
	};
	if runtime.boss != BossPhase::Pending || runtime.completed {
		return;
	}
	let Some(wave) = content.0.catalogs.waves.get(&runtime.number) else {
		return;
	};
	let Some(spawn) = &wave.boss else {
		return;
	};
	if tick.0.saturating_sub(runtime.tick0) < u64::from(spawn.request_tick) {
		return;
	}
	let Some(boss) = content.0.catalogs.bosses.get(&spawn.boss_id) else {
		return;
	};
	runtime.boss = BossPhase::Warning;
	let sim_id = ids.sim_id();
	queue.push(
		*tick,
		PresentationKind::SpawnWarned {
			sim_id: sim_id.0,
			position: [boss.spawn_position.x, boss.spawn_position.y],
			ticks: boss.warning_ticks,
		},
	);
	commands.spawn((
		sim_id,
		BossWarning {
			boss_id: boss.id.clone(),
			position: boss.spawn_position,
			materialize_tick: runtime.tick0
				+ u64::from(spawn.request_tick)
				+ u64::from(boss.warning_ticks),
			postponements: 0,
		},
	));
}

/// Materializes the boss when its warning expires and the spawn is clear, postponing in
/// 30-tick steps without a cap while the player occupies it.
pub fn resolve_boss_warning(world: &mut World) {
	let tick = world.resource::<SimTick>().0;
	if !world.contains_resource::<WaveRuntime>() {
		return;
	}
	let Some((player, alive)) = world
		.query_filtered::<(&Pose2, &Health), With<Player>>()
		.iter(world)
		.next()
		.map(|(p, h)| (p.0, h.current > 0))
	else {
		return;
	};
	let content = world.resource::<ContentRes>().0.clone();
	let due: Vec<(Entity, BossWarning, SimId)> = world
		.query::<(Entity, &BossWarning, &SimId)>()
		.iter(world)
		.filter(|(_, w, _)| w.materialize_tick <= tick)
		.map(|(e, w, id)| (e, w.clone(), *id))
		.collect();
	for (entity, warning, warning_id) in due {
		let Some(boss) = content.catalogs.bosses.get(&warning.boss_id) else {
			continue;
		};
		let occupied = alive
			&& warning.position.distance(player)
				<= boss.radius_lu + PLAYER_RADIUS_LU + OCCUPIED_MARGIN_LU;
		if occupied {
			if let Some(mut w) = world.get_mut::<BossWarning>(entity) {
				w.postponements += 1;
				w.materialize_tick = tick + POSTPONE_TICKS;
			}
			continue;
		}
		world.despawn(entity);
		let sim_id = world.resource_mut::<IdAllocator>().sim_id();
		let speed = match boss.recovery_movement {
			RecoveryMovement::Chase { speed_lu_s }
			| RecoveryMovement::KeepDistance { speed_lu_s, .. } => speed_lu_s,
		};
		world.spawn((
			sim_id,
			Enemy {
				id: boss.id.clone(),
			},
			Pose2(boss.spawn_position),
			PreviousPose2(boss.spawn_position),
			Body(boss.radius_lu),
			Health {
				current: boss.hp,
				max: boss.hp,
			},
			EnemyStats {
				armor: boss.armor,
				contact_damage: boss.contact_damage,
				scrap: boss.scrap,
				knockback_multiplier: 0.0,
				speed_lu_s: speed,
			},
			BossAi {
				attacks: boss.attacks.clone(),
				movement: boss.recovery_movement.clone(),
				next: 0,
				state: BossState::Recovery {
					left: boss.initial_recovery_ticks,
				},
				base_contact: boss.contact_damage,
			},
		));
		world.resource_mut::<WaveRuntime>().boss = BossPhase::Alive;
		world.resource_mut::<PresentationQueue>().push(
			SimTick(tick),
			PresentationKind::EnemySpawned {
				sim_id: sim_id.0,
				warning: warning_id.0,
				position: [boss.spawn_position.x, boss.spawn_position.y],
			},
		);
	}
}

#[allow(clippy::type_complexity)]
pub fn move_boss(
	arena: Option<Res<ArenaRes>>,
	players: Query<&Pose2, (With<Player>, Without<BossAi>)>,
	mut bosses: Query<(
		&mut Pose2,
		&mut PreviousPose2,
		&Body,
		&EnemyStats,
		&mut BossAi,
	)>,
) {
	let Ok(target) = players.single() else {
		return;
	};
	for (mut pose, mut previous, body, stats, mut ai) in &mut bosses {
		previous.0 = pose.0;
		let per_tick = stats.speed_lu_s / f64::from(TICKS_PER_SECOND);
		match ai.state {
			BossState::Recovery { .. } => {
				let to = target.0 - pose.0;
				let direction = match &ai.movement {
					RecoveryMovement::Chase { .. } => {
						(to.length_squared() > 0.0).then(|| unit(to, DVec2::ZERO))
					}
					RecoveryMovement::KeepDistance {
						desired_distance_lu,
						..
					} => {
						let distance = to.length();
						if distance > desired_distance_lu + KEEP_DISTANCE_BAND_LU {
							Some(unit(to, DVec2::ZERO))
						} else if distance < desired_distance_lu - KEEP_DISTANCE_BAND_LU {
							Some(-unit(to, DVec2::ZERO))
						} else {
							None
						}
					}
				};
				if let Some(direction) = direction {
					let mut next = pose.0 + direction * per_tick;
					if let Some(arena) = &arena {
						next = arena.0.clamp_circle_center(next, body.0);
					}
					pose.0 = next;
				}
			}
			BossState::Charging {
				left,
				direction,
				attack,
			} => {
				let HostilePayload::Charge { speed_lu_s, .. } = ai.attacks[attack].payload else {
					continue;
				};
				let wanted = pose.0 + direction * speed_lu_s / f64::from(TICKS_PER_SECOND);
				let reached = arena
					.as_ref()
					.map_or(wanted, |a| a.0.clamp_circle_center(wanted, body.0));
				pose.0 = reached;
				let wall = reached != wanted;
				ai.state = if wall || left <= 1 {
					BossState::Recovery {
						left: ai.attacks[attack].recovery_ticks + 1,
					}
				} else {
					BossState::Charging {
						left: left - 1,
						direction,
						attack,
					}
				};
			}
			BossState::Windup { .. } => {}
		}
	}
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn run_boss_attacks(
	tick: Res<SimTick>,
	mut ids: ResMut<IdAllocator>,
	mut queue: ResMut<PresentationQueue>,
	mut hits: ResMut<HostileHits>,
	mut commands: Commands,
	players: Query<(&Pose2, &Health), (With<Player>, Without<BossAi>)>,
	mut bosses: Query<(&SimId, &Pose2, &Body, &mut EnemyStats, &mut BossAi)>,
) {
	let Ok((target, health)) = players.single() else {
		return;
	};
	let mut order: Vec<_> = bosses.iter_mut().collect();
	order.sort_by_key(|(id, ..)| **id);
	for (sim_id, pose, body, mut stats, mut ai) in order {
		match ai.state {
			BossState::Recovery { left } => {
				if stats.contact_damage != ai.base_contact && left <= ai.last_recovery() {
					stats.contact_damage = ai.base_contact;
				}
				if left > 0 || health.current == 0 {
					ai.state = BossState::Recovery {
						left: left.saturating_sub(1),
					};
					continue;
				}
				let index = ai.next;
				let attack = ai.attacks[index].clone();
				let to_player = target.0 - pose.0;
				let aim = unit(to_player, DVec2::X);
				let (shape, anchor) = cue_for(&attack.payload, body.0, aim, target.0);
				queue.push(
					*tick,
					PresentationKind::CueStarted {
						sim_id: sim_id.0,
						shape,
						anchor,
						ticks: attack.windup_ticks,
					},
				);
				ai.state = BossState::Windup {
					left: attack.windup_ticks,
					aim,
					center: target.0,
					attack: index,
				};
			}
			BossState::Windup {
				left,
				aim,
				center,
				attack,
			} => {
				let left = left.saturating_sub(1);
				if left > 0 {
					ai.state = BossState::Windup {
						left,
						aim,
						center,
						attack,
					};
					continue;
				}
				queue.push(
					*tick,
					PresentationKind::CueEnded {
						sim_id: sim_id.0,
						cancelled: false,
					},
				);
				let spec = ai.attacks[attack].clone();
				ai.next = (attack + 1) % ai.attacks.len();
				if let HostilePayload::Charge {
					ticks,
					contact_damage,
					..
				} = spec.payload
				{
					stats.contact_damage = contact_damage;
					ai.state = BossState::Charging {
						left: ticks,
						direction: aim,
						attack,
					};
					continue;
				}
				emit(
					&spec.payload,
					1,
					pose.0,
					aim,
					center,
					&mut ids,
					&mut queue,
					*tick,
					&mut hits,
					&mut commands,
					target.0,
				);
				ai.state = BossState::Recovery {
					left: spec.recovery_ticks,
				};
			}
			BossState::Charging { .. } => {}
		}
	}
}

/// A boss that existed and is gone was killed; this releases the wave-completion block.
pub fn settle_boss(mut runtime: Option<ResMut<WaveRuntime>>, bosses: Query<(), With<BossAi>>) {
	let Some(runtime) = runtime.as_mut() else {
		return;
	};
	if runtime.boss == BossPhase::Alive && bosses.is_empty() {
		runtime.boss = BossPhase::Defeated;
		runtime.boss_blocking = false;
	}
}
