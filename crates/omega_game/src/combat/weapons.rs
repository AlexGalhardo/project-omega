//! Weapon timers, attack emission and swept player-attack collision (SP-007, GDD 6.1-6.2).
//!
//! Emit starts and emits attacks; Collide moves projectiles and computes every hit
//! candidate. Candidates are applied later, in attack-then-target order, by `damage`.

use super::actor::{ArenaRes, ContentRes, Enemy, Health, Loadout, Player, PlayerStats, Pose2};
use super::actor::{Body, PreviousPose2};
use super::damage::{Hit, HitCandidates};
use super::stats::fire_interval_ticks;
use crate::content::model::{Weapon, WeaponPayload};
use crate::sim::{
	AimInput, AttackId, IdAllocator, PresentationKind, PresentationQueue, SimId, SimTick,
	TICKS_PER_SECOND, TickInput,
};
use bevy::math::DVec2;
use bevy::prelude::*;
use omega_2d::geometry::{Capsule, Circle, capsule_circle_overlap, circles_overlap, sweep_circle};
use std::collections::{BTreeSet, HashSet};

/// Value that makes a weapon ready regardless of its (later) interval.
const READY: u32 = u32::MAX / 2;
const EPS: f64 = 1e-9;

/// Values captured at attack start (GDD 6.1): later stat changes never affect the attack.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AttackSnapshot {
	pub base_damage: u32,
	pub tier: u8,
	pub damage_pct: i32,
	pub crit_pct: i32,
	pub knockback_lu: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PendingAttack {
	pub attack: AttackId,
	pub windup_left: u32,
	pub direction: DVec2,
	pub snapshot: AttackSnapshot,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WeaponTimer {
	/// Ticks since the most recent attack start, capped at the current interval.
	pub elapsed: u32,
	pub pending: Option<PendingAttack>,
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct WeaponTimers(pub [WeaponTimer; 4]);

impl WeaponTimers {
	/// Wave entry: every powered weapon starts ready, nothing is pending.
	pub fn ready() -> Self {
		Self(
			[WeaponTimer {
				elapsed: READY,
				pending: None,
			}; 4],
		)
	}
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Projectile {
	pub attack: AttackId,
	pub direction: DVec2,
	pub speed_per_tick: f64,
	pub range_left: f64,
	pub pierce_left: u32,
	pub explosion_radius: Option<f64>,
	pub snapshot: AttackSnapshot,
	pub origin: DVec2,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InstantShape {
	Capsule {
		start: DVec2,
		end: DVec2,
		radius: f64,
	},
	Circle {
		center: DVec2,
		radius: f64,
	},
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InstantAttack {
	pub attack: AttackId,
	pub shape: InstantShape,
	pub snapshot: AttackSnapshot,
	pub origin: DVec2,
	pub direction: DVec2,
}

/// One-tick attacks emitted this tick, resolved by `collide_player_attacks`.
#[derive(Resource, Debug, Default)]
pub struct EmittedShapes(pub Vec<InstantAttack>);

/// (attack, target) pairs already damaged, so one attack hits a target at most once.
#[derive(Resource, Debug, Default)]
pub struct HitLedger(pub BTreeSet<(AttackId, SimId)>);

fn rotate(direction: DVec2, degrees: f64) -> DVec2 {
	DVec2::from_angle(degrees.to_radians()).rotate(direction)
}

fn pair(v: DVec2) -> [f64; 2] {
	[v.x, v.y]
}

#[allow(clippy::too_many_arguments)]
fn emit_attack(
	weapon: &Weapon,
	slot: u8,
	pending: &PendingAttack,
	origin: DVec2,
	ids: &mut IdAllocator,
	queue: &mut PresentationQueue,
	tick: SimTick,
	shapes: &mut EmittedShapes,
	commands: &mut Commands,
) {
	let direction = pending.direction;
	let snapshot = pending.snapshot;
	queue.push(
		tick,
		PresentationKind::AttackEmitted {
			attack: pending.attack.0,
			slot,
			origin: pair(origin),
			direction: pair(direction),
		},
	);
	let mut spawn_projectiles =
		|speed: f64, radius: f64, pierce: u32, explosion: Option<f64>, angles: &[f64]| {
			for &angle in angles {
				let dir = rotate(direction, angle);
				let sim_id = ids.sim_id();
				queue.push(
					tick,
					PresentationKind::ProjectileSpawned {
						sim_id: sim_id.0,
						attack: pending.attack.0,
						origin: pair(origin),
						direction: pair(dir),
						radius,
					},
				);
				commands.spawn((
					sim_id,
					Projectile {
						attack: pending.attack,
						direction: dir,
						speed_per_tick: speed / f64::from(TICKS_PER_SECOND),
						range_left: weapon.range_lu,
						pierce_left: pierce,
						explosion_radius: explosion,
						snapshot,
						origin,
					},
					Pose2(origin),
					PreviousPose2(origin),
					Body(radius),
				));
			}
		};
	match &weapon.payload {
		WeaponPayload::Projectile {
			speed_lu_s,
			radius_lu,
			pierce,
			angles_degrees,
		} => spawn_projectiles(*speed_lu_s, *radius_lu, *pierce, None, angles_degrees),
		WeaponPayload::ExplosiveProjectile {
			speed_lu_s,
			radius_lu,
			explosion_radius_lu,
			angles_degrees,
		} => spawn_projectiles(
			*speed_lu_s,
			*radius_lu,
			0,
			Some(*explosion_radius_lu),
			angles_degrees,
		),
		WeaponPayload::Capsule {
			reach_lu,
			radius_lu,
		} => shapes.0.push(InstantAttack {
			attack: pending.attack,
			shape: InstantShape::Capsule {
				start: origin,
				end: origin + direction * *reach_lu,
				radius: *radius_lu,
			},
			snapshot,
			origin,
			direction,
		}),
		WeaponPayload::Circle {
			offset_lu,
			radius_lu,
		} => shapes.0.push(InstantAttack {
			attack: pending.attack,
			shape: InstantShape::Circle {
				center: origin + direction * *offset_lu,
				radius: *radius_lu,
			},
			snapshot,
			origin,
			direction,
		}),
	}
}

#[allow(clippy::too_many_arguments)]
pub fn emit_weapons(
	input: Res<TickInput>,
	content: Res<ContentRes>,
	tick: Res<SimTick>,
	mut ids: ResMut<IdAllocator>,
	mut queue: ResMut<PresentationQueue>,
	mut shapes: ResMut<EmittedShapes>,
	mut commands: Commands,
	mut players: Query<(&Pose2, &Health, &PlayerStats, &Loadout, &mut WeaponTimers), With<Player>>,
	enemies: Query<(&SimId, &Pose2, &Health), With<Enemy>>,
) {
	let mut targets: Vec<(SimId, DVec2)> = enemies
		.iter()
		.filter(|(_, _, h)| h.current > 0)
		.map(|(id, pose, _)| (*id, pose.0))
		.collect();
	targets.sort_by_key(|(id, _)| *id);
	for (pose, health, stats, loadout, mut timers) in &mut players {
		if health.current == 0 {
			// A dead attacker cancels attacks that have not emitted.
			timers.0.iter_mut().for_each(|t| t.pending = None);
			continue;
		}
		let origin = pose.0;
		for slot in 0..4 {
			let Some(equipped) = &loadout.slots[slot] else {
				continue;
			};
			let Some(weapon) = content.0.catalogs.weapons.get(&equipped.weapon_id) else {
				continue;
			};
			let Some(interval) = fire_interval_ticks(
				weapon,
				loadout.energy.slots()[slot],
				stats.0.attack_speed_pct,
			) else {
				continue;
			};
			let timer = &mut timers.0[slot];
			timer.elapsed = timer.elapsed.saturating_add(1).min(interval);
			if let Some(pending) = &mut timer.pending {
				pending.windup_left = pending.windup_left.saturating_sub(1);
				if pending.windup_left == 0 {
					let due = *pending;
					timer.pending = None;
					emit_attack(
						weapon,
						slot as u8,
						&due,
						origin,
						&mut ids,
						&mut queue,
						*tick,
						&mut shapes,
						&mut commands,
					);
				}
			}
			if timer.pending.is_some() || timer.elapsed < interval {
				continue;
			}
			let direction = match input.aim {
				AimInput::Manual(dir) => Some(dir),
				AimInput::Auto => {
					let range2 = weapon.range_lu * weapon.range_lu;
					targets
						.iter()
						.map(|(id, pos)| ((*pos - origin).length_squared(), *id, *pos))
						.filter(|(d2, _, _)| *d2 <= range2)
						.min_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)))
						.map(|(_, _, pos)| (pos - origin).try_normalize().unwrap_or(DVec2::X))
				}
			};
			let Some(direction) = direction else {
				continue;
			};
			timer.elapsed = 0;
			let pending = PendingAttack {
				attack: ids.attack_id(),
				windup_left: weapon.windup_ticks,
				direction,
				snapshot: AttackSnapshot {
					base_damage: weapon.base_damage,
					tier: equipped.tier,
					damage_pct: stats.0.damage_pct,
					crit_pct: stats.0.crit_pct,
					knockback_lu: weapon.knockback_lu,
				},
			};
			if weapon.windup_ticks == 0 {
				emit_attack(
					weapon,
					slot as u8,
					&pending,
					origin,
					&mut ids,
					&mut queue,
					*tick,
					&mut shapes,
					&mut commands,
				);
			} else {
				timer.pending = Some(pending);
			}
		}
	}
}

type EnemyFilter = (With<Enemy>, Without<Projectile>);

struct Target {
	entity: Entity,
	id: SimId,
	circle: Circle,
}

fn live_targets(
	enemies: &Query<(Entity, &SimId, &Pose2, &Body, &Health), EnemyFilter>,
) -> Vec<Target> {
	let mut out: Vec<Target> = enemies
		.iter()
		.filter(|(_, _, _, _, h)| h.current > 0)
		.filter_map(|(entity, id, pose, body, _)| {
			Some(Target {
				entity,
				id: *id,
				circle: Circle::new(pose.0, body.0).ok()?,
			})
		})
		.collect();
	out.sort_by_key(|t| t.id);
	out
}

fn push_hit(
	hits: &mut HitCandidates,
	ledger: &mut HitLedger,
	attack: AttackId,
	target: &Target,
	snapshot: AttackSnapshot,
	origin: DVec2,
	direction: DVec2,
) -> bool {
	if !ledger.0.insert((attack, target.id)) {
		return false;
	}
	hits.0.push(Hit {
		attack,
		target: target.entity,
		target_id: target.id,
		snapshot,
		origin,
		direction,
	});
	true
}

#[allow(clippy::too_many_arguments)]
pub fn collide_player_attacks(
	arena: Option<Res<ArenaRes>>,
	tick: Res<SimTick>,
	mut shapes: ResMut<EmittedShapes>,
	mut ledger: ResMut<HitLedger>,
	mut hits: ResMut<HitCandidates>,
	mut queue: ResMut<PresentationQueue>,
	mut commands: Commands,
	enemies: Query<(Entity, &SimId, &Pose2, &Body, &Health), EnemyFilter>,
	mut projectiles: Query<(
		Entity,
		&SimId,
		&mut Pose2,
		&mut PreviousPose2,
		&Body,
		&mut Projectile,
	)>,
) {
	hits.0.clear();
	let targets = live_targets(&enemies);
	let mut instants = std::mem::take(&mut shapes.0);
	instants.sort_by_key(|i| i.attack);
	for attack in instants {
		for target in &targets {
			let overlaps = match attack.shape {
				InstantShape::Capsule { start, end, radius } => Capsule::new(start, end, radius)
					.is_ok_and(|c| capsule_circle_overlap(&c, &target.circle)),
				InstantShape::Circle { center, radius } => {
					Circle::new(center, radius).is_ok_and(|c| circles_overlap(&c, &target.circle))
				}
			};
			if overlaps {
				push_hit(
					&mut hits,
					&mut ledger,
					attack.attack,
					target,
					attack.snapshot,
					attack.origin,
					attack.direction,
				);
			}
		}
	}

	let mut order: Vec<_> = projectiles.iter().map(|(e, id, ..)| (e, *id)).collect();
	order.sort_by_key(|(_, id)| *id);
	let mut live_attacks: HashSet<AttackId> = HashSet::new();
	for (entity, sim_id) in order {
		let Ok((_, _, mut pose, mut previous, body, mut proj)) = projectiles.get_mut(entity) else {
			continue;
		};
		previous.0 = pose.0;
		let start = pose.0;
		let travel = (proj.speed_per_tick).min(proj.range_left);
		let mut motion = proj.direction * travel;
		let mut wall = false;
		if let Some(arena) = &arena {
			let (min, max) = (arena.0.min(), arena.0.max());
			let axis_t = |p: f64, m: f64, lo: f64, hi: f64| {
				if m > 0.0 && p + m > hi {
					(hi - p) / m
				} else if m < 0.0 && p + m < lo {
					(lo - p) / m
				} else {
					1.0
				}
			};
			let t = axis_t(start.x, motion.x, min.x, max.x)
				.min(axis_t(start.y, motion.y, min.y, max.y))
				.clamp(0.0, 1.0);
			if t < 1.0 {
				wall = true;
				motion *= t;
			}
		}
		let Ok(moving) = Circle::new(start, body.0) else {
			continue;
		};
		let mut candidates: Vec<(f64, &Target)> = targets
			.iter()
			.filter(|t| !ledger.0.contains(&(proj.attack, t.id)))
			.filter_map(|t| sweep_circle(&moving, motion, &t.circle).map(|time| (time, t)))
			.collect();
		candidates.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.id.cmp(&b.1.id)));
		let mut ended: Option<DVec2> = None;
		for (time, target) in candidates {
			if let Some(radius) = proj.explosion_radius {
				ended = Some(start + motion * time);
				explode(
					&mut hits,
					&mut ledger,
					&mut queue,
					*tick,
					&proj,
					ended.unwrap(),
					radius,
					&targets,
				);
				break;
			}
			if push_hit(
				&mut hits,
				&mut ledger,
				proj.attack,
				target,
				proj.snapshot,
				proj.origin,
				proj.direction,
			) {
				if proj.pierce_left == 0 {
					ended = Some(start + motion * time);
					break;
				}
				proj.pierce_left -= 1;
			}
		}
		let end = start + motion;
		proj.range_left -= travel;
		if ended.is_none() && (wall || proj.range_left <= EPS) {
			if let Some(radius) = proj.explosion_radius {
				explode(
					&mut hits,
					&mut ledger,
					&mut queue,
					*tick,
					&proj,
					end,
					radius,
					&targets,
				);
			}
			ended = Some(end);
		}
		match ended {
			Some(at) => {
				queue.push(
					*tick,
					PresentationKind::ProjectileEnded {
						sim_id: sim_id.0,
						position: pair(at),
					},
				);
				commands.entity(entity).despawn();
			}
			None => {
				pose.0 = end;
				live_attacks.insert(proj.attack);
			}
		}
	}
	ledger.0.retain(|(attack, _)| live_attacks.contains(attack));
}

#[allow(clippy::too_many_arguments)]
fn explode(
	hits: &mut HitCandidates,
	ledger: &mut HitLedger,
	queue: &mut PresentationQueue,
	tick: SimTick,
	proj: &Projectile,
	center: DVec2,
	radius: f64,
	targets: &[Target],
) {
	queue.push(
		tick,
		PresentationKind::Explosion {
			attack: proj.attack.0,
			center: pair(center),
			radius,
		},
	);
	let Ok(blast) = Circle::new(center, radius) else {
		return;
	};
	for target in targets {
		if circles_overlap(&blast, &target.circle) {
			push_hit(
				hits,
				ledger,
				proj.attack,
				target,
				proj.snapshot,
				center,
				proj.direction,
			);
		}
	}
}
