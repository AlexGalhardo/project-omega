//! Enemy behavior profiles (SP-008, GDD section 10.1): keep-distance, orbit, stationary,
//! chase-then-strike and charge, all driven by one attack state machine.
//!
//! Movement runs in `TickSet::Movement` from the state left by the previous tick; the
//! timers, windup starts and emissions run in `TickSet::Emit`. Hostile damage is queued as
//! `HostileHit`s and settled by `combat::damage::resolve_damage`.

use super::scaled;
use crate::combat::actor::{
	ArenaRes, Body, EnemyStats, Health, PLAYER_RADIUS_LU, Player, Pose2, PreviousPose2,
};
use crate::combat::damage::{HostileHit, HostileHits};
use crate::content::model::{AimFrame, Behavior, EnemyAttack, HostilePayload};
use crate::sim::{
	CueAnchorFact, CueShapeFact, IdAllocator, PresentationKind, PresentationQueue, SimId, SimTick,
	TICKS_PER_SECOND,
};
use bevy::math::DVec2;
use bevy::prelude::*;
use omega_2d::geometry::{Capsule, Circle, capsule_circle_overlap, circles_overlap};
use std::collections::BTreeMap;

use super::hostile::HostileProjectile;

/// Tolerance band around a keep-distance target (GDD section 10.1).
pub const KEEP_DISTANCE_BAND_LU: f64 = 20.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AiState {
	Moving,
	Windup {
		left: u32,
		total: u32,
		/// Aim locked at windup start.
		aim: DVec2,
		/// Ground-circle center locked at windup start.
		center: DVec2,
	},
	Charging {
		left: u32,
		direction: DVec2,
	},
	Recovery {
		left: u32,
	},
}

/// Non-chase behavior profile plus the attack timers it owns.
#[derive(Component, Debug, Clone, PartialEq)]
pub struct EnemyAi {
	pub behavior: Behavior,
	pub state: AiState,
	/// Ticks since the last windup start (or materialization), capped at the interval.
	pub interval_elapsed: u32,
	/// Charge profile: ticks of approach since materialization or the last recovery.
	pub wait_elapsed: u32,
	pub wave: u32,
	/// Contact damage outside a charge.
	pub base_contact: u32,
}

impl EnemyAi {
	pub fn new(behavior: Behavior, wave: u32, base_contact: u32) -> Self {
		Self {
			behavior,
			state: AiState::Moving,
			interval_elapsed: 0,
			wait_elapsed: 0,
			wave,
			base_contact,
		}
	}

	fn attack(&self) -> Option<&EnemyAttack> {
		match &self.behavior {
			Behavior::KeepDistance { attack, .. }
			| Behavior::Orbit { attack, .. }
			| Behavior::Stationary { attack }
			| Behavior::ChaseStrike { attack } => Some(attack),
			Behavior::Chase | Behavior::Charge { .. } => None,
		}
	}
}

/// Attackers whose windup cue is live, so a cancelled one can be reported once.
#[derive(Resource, Debug, Default)]
pub struct LiveCues(BTreeMap<Entity, SimId>);

fn pair(v: DVec2) -> [f64; 2] {
	[v.x, v.y]
}

fn unit(v: DVec2, fallback: DVec2) -> DVec2 {
	if v.length_squared() > 0.0 {
		v.normalize()
	} else {
		fallback
	}
}

/// Fraction of `motion` that fits inside the arena (1.0 when it all fits).
pub fn wall_fraction(arena: &ArenaRes, start: DVec2, motion: DVec2) -> f64 {
	let (min, max) = (arena.0.min(), arena.0.max());
	let axis = |p: f64, m: f64, lo: f64, hi: f64| {
		if m > 0.0 && p + m > hi {
			(hi - p) / m
		} else if m < 0.0 && p + m < lo {
			(lo - p) / m
		} else {
			1.0
		}
	};
	axis(start.x, motion.x, min.x, max.x)
		.min(axis(start.y, motion.y, min.y, max.y))
		.clamp(0.0, 1.0)
}

/// Velocity direction for the movement part of a behavior; `None` means hold position.
fn desired_direction(behavior: &Behavior, to_player: DVec2) -> Option<DVec2> {
	let distance = to_player.length();
	match behavior {
		Behavior::Chase | Behavior::ChaseStrike { .. } | Behavior::Charge { .. } => {
			(distance > 0.0).then(|| to_player / distance)
		}
		Behavior::KeepDistance {
			desired_distance_lu,
			..
		} => {
			if distance > desired_distance_lu + KEEP_DISTANCE_BAND_LU {
				Some(unit(to_player, DVec2::ZERO))
			} else if distance < desired_distance_lu - KEEP_DISTANCE_BAND_LU {
				Some(-unit(to_player, DVec2::ZERO))
			} else {
				None
			}
		}
		Behavior::Orbit {
			orbit_radius_lu, ..
		} => {
			if distance == 0.0 {
				return Some(DVec2::X);
			}
			let radial_out = -to_player / distance;
			// Clockwise tangent around the player for a Y-up plane.
			let tangent = DVec2::new(radial_out.y, -radial_out.x);
			let pull = ((distance - orbit_radius_lu) / 60.0).clamp(-1.0, 1.0);
			Some(unit(tangent + (-radial_out) * pull, tangent))
		}
		Behavior::Stationary { .. } => None,
	}
}

#[allow(clippy::type_complexity)]
pub fn move_enemies(
	arena: Option<Res<ArenaRes>>,
	players: Query<&Pose2, (With<Player>, Without<EnemyAi>)>,
	mut enemies: Query<
		(
			&SimId,
			&mut Pose2,
			&mut PreviousPose2,
			&Body,
			&EnemyStats,
			&mut EnemyAi,
		),
		Without<Player>,
	>,
) {
	let Ok(target) = players.single() else {
		return;
	};
	let mut order: Vec<_> = enemies.iter_mut().collect();
	order.sort_by_key(|(id, ..)| **id);
	for (_, mut pose, mut previous, body, stats, mut ai) in order {
		previous.0 = pose.0;
		let to_player = target.0 - pose.0;
		let per_tick = stats.speed_lu_s / f64::from(TICKS_PER_SECOND);
		match ai.state {
			AiState::Moving => {
				let Some(direction) = desired_direction(&ai.behavior, to_player) else {
					continue;
				};
				let mut next = pose.0 + direction * per_tick;
				if let Some(arena) = &arena {
					next = arena.0.clamp_circle_center(next, body.0);
				}
				pose.0 = next;
			}
			AiState::Charging { left, direction } => {
				let Behavior::Charge {
					charge: HostilePayload::Charge { speed_lu_s, .. },
					recovery_ticks,
					..
				} = &ai.behavior
				else {
					continue;
				};
				let wanted = pose.0 + direction * speed_lu_s / f64::from(TICKS_PER_SECOND);
				let reached = arena
					.as_ref()
					.map_or(wanted, |a| a.0.clamp_circle_center(wanted, body.0));
				pose.0 = reached;
				let wall = reached != wanted;
				ai.state = if wall || left <= 1 {
					AiState::Recovery {
						left: *recovery_ticks + 1,
					}
				} else {
					AiState::Charging {
						left: left - 1,
						direction,
					}
				};
			}
			AiState::Windup { .. } | AiState::Recovery { .. } => {}
		}
	}
}

fn cue_for(
	attack_payload: &HostilePayload,
	body_radius: f64,
	aim: DVec2,
	center: DVec2,
) -> (CueShapeFact, CueAnchorFact) {
	match attack_payload {
		HostilePayload::Projectiles { .. } => (
			CueShapeFact::Capsule {
				reach: 0.0,
				radius: 0.0,
			},
			CueAnchorFact::Follow {
				direction: pair(aim),
			},
		),
		HostilePayload::Beam {
			reach_lu,
			radius_lu,
			..
		} => (
			CueShapeFact::Capsule {
				reach: *reach_lu,
				radius: *radius_lu,
			},
			CueAnchorFact::Follow {
				direction: pair(aim),
			},
		),
		HostilePayload::GroundCircle { radius_lu, .. } => (
			CueShapeFact::Circle { radius: *radius_lu },
			CueAnchorFact::Locked(pair(center)),
		),
		HostilePayload::Charge {
			speed_lu_s, ticks, ..
		} => (
			CueShapeFact::Capsule {
				reach: speed_lu_s * f64::from(*ticks) / f64::from(TICKS_PER_SECOND),
				radius: body_radius,
			},
			CueAnchorFact::Follow {
				direction: pair(aim),
			},
		),
	}
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn run_attacks(
	tick: Res<SimTick>,
	mut ids: ResMut<IdAllocator>,
	mut queue: ResMut<PresentationQueue>,
	mut live: ResMut<LiveCues>,
	mut hits: ResMut<HostileHits>,
	mut commands: Commands,
	players: Query<(&Pose2, &Health), (With<Player>, Without<EnemyAi>)>,
	mut enemies: Query<
		(Entity, &SimId, &Pose2, &Body, &mut EnemyStats, &mut EnemyAi),
		Without<Player>,
	>,
) {
	let Ok((target, health)) = players.single() else {
		return;
	};
	let mut order: Vec<_> = enemies.iter_mut().collect();
	order.sort_by_key(|(_, id, ..)| **id);
	for (entity, sim_id, pose, body, mut stats, mut ai) in order {
		let interval = ai.attack().map_or(0, |a| a.interval_ticks);
		ai.interval_elapsed = (ai.interval_elapsed + 1).min(interval + 1);
		let to_player = target.0 - pose.0;
		let distance = to_player.length();
		match ai.state {
			AiState::Moving => {
				if health.current == 0 {
					continue;
				}
				if let Behavior::Charge {
					initial_wait_ticks,
					trigger_range_lu,
					windup_ticks,
					charge,
					..
				} = ai.behavior.clone()
				{
					ai.wait_elapsed += 1;
					if ai.wait_elapsed > initial_wait_ticks && distance <= trigger_range_lu {
						let aim = unit(to_player, DVec2::X);
						let (shape, anchor) = cue_for(&charge, body.0, aim, target.0);
						queue.push(
							*tick,
							PresentationKind::CueStarted {
								sim_id: sim_id.0,
								shape,
								anchor,
								ticks: windup_ticks,
							},
						);
						live.0.insert(entity, *sim_id);
						ai.state = AiState::Windup {
							left: windup_ticks,
							total: windup_ticks,
							aim,
							center: target.0,
						};
					}
					continue;
				}
				let Some(attack) = ai.attack() else {
					continue;
				};
				let in_range = attack.start_range_lu.is_none_or(|r| distance <= r);
				if ai.interval_elapsed <= attack.interval_ticks || !in_range {
					continue;
				}
				let windup = attack.windup_ticks;
				let aim = unit(to_player, DVec2::X);
				let (shape, anchor) = cue_for(&attack.payload, body.0, aim, target.0);
				queue.push(
					*tick,
					PresentationKind::CueStarted {
						sim_id: sim_id.0,
						shape,
						anchor,
						ticks: windup,
					},
				);
				live.0.insert(entity, *sim_id);
				ai.interval_elapsed = 1;
				ai.state = AiState::Windup {
					left: windup,
					total: windup,
					aim,
					center: target.0,
				};
			}
			AiState::Windup {
				left,
				total,
				aim,
				center,
			} => {
				let left = left.saturating_sub(1);
				if left > 0 {
					ai.state = AiState::Windup {
						left,
						total,
						aim,
						center,
					};
					continue;
				}
				live.0.remove(&entity);
				queue.push(
					*tick,
					PresentationKind::CueEnded {
						sim_id: sim_id.0,
						cancelled: false,
					},
				);
				if let Behavior::Charge { charge, .. } = ai.behavior.clone() {
					let HostilePayload::Charge {
						ticks,
						contact_damage,
						..
					} = charge
					else {
						continue;
					};
					stats.contact_damage = scaled(contact_damage, 8, ai.wave);
					ai.state = AiState::Charging {
						left: ticks,
						direction: aim,
					};
					continue;
				}
				let Some(attack) = ai.attack().cloned() else {
					continue;
				};
				emit(
					&attack.payload,
					ai.wave,
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
				ai.state = AiState::Recovery {
					left: attack.recovery_ticks,
				};
			}
			AiState::Recovery { left } => {
				if let Behavior::Charge { recovery_ticks, .. } = ai.behavior
					&& left <= recovery_ticks
				{
					stats.contact_damage = ai.base_contact;
				}
				let left = left.saturating_sub(1);
				if left > 0 {
					ai.state = AiState::Recovery { left };
				} else {
					ai.state = AiState::Moving;
					ai.wait_elapsed = 0;
				}
			}
			AiState::Charging { .. } => {}
		}
	}
}

#[allow(clippy::too_many_arguments)]
fn emit(
	payload: &HostilePayload,
	wave: u32,
	origin: DVec2,
	aim: DVec2,
	center: DVec2,
	ids: &mut IdAllocator,
	queue: &mut PresentationQueue,
	tick: SimTick,
	hits: &mut HostileHits,
	commands: &mut Commands,
	player: DVec2,
) {
	let attack = ids.attack_id();
	match payload {
		HostilePayload::Projectiles {
			aim: frame,
			angles_degrees,
			speed_lu_s,
			radius_lu,
			range_lu,
			damage,
		} => {
			let raw = scaled(*damage, 8, wave);
			for angle in angles_degrees {
				let base = match frame {
					AimFrame::Locked => aim,
					AimFrame::World => DVec2::X,
				};
				let direction = DVec2::from_angle(angle.to_radians()).rotate(base);
				let sim_id = ids.sim_id();
				queue.push(
					tick,
					PresentationKind::HostileProjectileSpawned {
						sim_id: sim_id.0,
						attack: attack.0,
						origin: pair(origin),
						direction: pair(direction),
						radius: *radius_lu,
					},
				);
				commands.spawn((
					sim_id,
					HostileProjectile {
						attack,
						direction,
						speed_per_tick: speed_lu_s / f64::from(TICKS_PER_SECOND),
						range_left: *range_lu,
						damage: raw,
					},
					Pose2(origin),
					PreviousPose2(origin),
					Body(*radius_lu),
				));
			}
		}
		HostilePayload::Beam {
			reach_lu,
			radius_lu,
			damage,
		} => {
			let capsule = Capsule::new(origin, origin + aim * *reach_lu, *radius_lu);
			let body = Circle::new(player, PLAYER_RADIUS_LU);
			if let (Ok(capsule), Ok(body)) = (capsule, body)
				&& capsule_circle_overlap(&capsule, &body)
			{
				hits.pending.push(HostileHit {
					attack,
					raw: scaled(*damage, 8, wave),
				});
			}
		}
		HostilePayload::GroundCircle { radius_lu, damage } => {
			let blast = Circle::new(center, *radius_lu);
			let body = Circle::new(player, PLAYER_RADIUS_LU);
			if let (Ok(blast), Ok(body)) = (blast, body)
				&& circles_overlap(&blast, &body)
			{
				hits.pending.push(HostileHit {
					attack,
					raw: scaled(*damage, 8, wave),
				});
			}
		}
		HostilePayload::Charge { .. } => {}
	}
}

/// Reports windups whose attacker died or left the arena before emission.
pub fn cancel_orphaned_cues(
	tick: Res<SimTick>,
	mut live: ResMut<LiveCues>,
	mut queue: ResMut<PresentationQueue>,
	attackers: Query<(), With<EnemyAi>>,
) {
	live.0.retain(|entity, sim_id| {
		let alive = attackers.contains(*entity);
		if !alive {
			queue.push(
				*tick,
				PresentationKind::CueEnded {
					sim_id: sim_id.0,
					cancelled: true,
				},
			);
		}
		alive
	});
}
