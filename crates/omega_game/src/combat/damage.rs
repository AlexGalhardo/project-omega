//! Hit candidates produced by Collide and their resolution (SP-007, GAME-P2-005).

use super::actor::{
	ArenaRes, Body, Enemy, EnemyStats, Health, Invulnerability, Pickup, Player, PlayerStats, Pose2,
	PreviousPose2, Scrap,
};
use super::stats::{RegenAccumulator, enemy_damage, player_damage};
use super::weapons::AttackSnapshot;
use crate::sim::{
	AttackId, IdAllocator, PresentationKind, PresentationQueue, RngStreams, SimId, SimTick,
};
use bevy::math::DVec2;
use bevy::prelude::*;
use omega_2d::geometry::{Circle, circles_overlap};

/// One player-attack hit on one enemy, decided in Collide and applied in Resolve in
/// attack-then-target order.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hit {
	pub attack: AttackId,
	pub target: Entity,
	pub target_id: SimId,
	pub snapshot: AttackSnapshot,
	pub origin: DVec2,
	pub direction: DVec2,
}

/// Candidates of the current tick; cleared at the start of Collide.
#[derive(Resource, Debug, Default)]
pub struct HitCandidates(pub Vec<Hit>);

type TargetPlayer = (
	&'static Pose2,
	&'static Body,
	&'static PlayerStats,
	&'static mut Health,
	&'static mut Invulnerability,
);

/// Ticks a hit blocks later incoming damage (GDD section 7.3).
pub const INVULNERABILITY_TICKS: u64 = 36;

fn pair(v: DVec2) -> [f64; 2] {
	[v.x, v.y]
}

/// Applies player-attack candidates, then body contact, in stable attack/target order
/// (SP-007, GDD sections 6, 7.2, 7.3). Death is immediate: a target at zero HP is skipped
/// by every later hit in the same tick and yields exactly one scrap pickup.
#[allow(clippy::too_many_arguments)]
pub fn resolve_damage(
	tick: Res<SimTick>,
	arena: Option<Res<ArenaRes>>,
	hits: Res<HitCandidates>,
	mut rng: ResMut<RngStreams>,
	mut ids: ResMut<IdAllocator>,
	mut queue: ResMut<PresentationQueue>,
	mut commands: Commands,
	mut enemies: Query<(&SimId, &mut Pose2, &Body, &mut Health, &EnemyStats), With<Enemy>>,
	mut players: Query<TargetPlayer, (With<Player>, Without<Enemy>)>,
) {
	// Contact candidates are fixed before any death this tick, so a simultaneous lethal
	// hit on both sides kills both (GDD section 6, DA-13).
	let mut contacts: Vec<_> = enemies
		.iter()
		.filter(|(_, _, _, h, _)| h.current > 0)
		.map(|(id, pose, body, _, stats)| (*id, pose.0, body.0, stats.contact_damage))
		.collect();
	contacts.sort_by_key(|c| c.0);
	let mut ordered: Vec<Hit> = hits.0.clone();
	ordered.sort_by_key(|h| (h.attack, h.target_id));
	for hit in ordered {
		let Ok((_, mut pose, body, mut health, stats)) = enemies.get_mut(hit.target) else {
			continue;
		};
		if health.current == 0 {
			continue;
		}
		let snap = hit.snapshot;
		let crit = rng.crit.below(10_000) < 100 * u32::try_from(snap.crit_pct).unwrap_or(0);
		let Ok(damage) = player_damage(
			snap.base_damage,
			snap.tier,
			snap.damage_pct,
			crit,
			stats.armor,
		) else {
			continue;
		};
		health.current = health.current.saturating_sub(damage);
		queue.push(
			*tick,
			PresentationKind::HitAccepted {
				attack: hit.attack.0,
				target: hit.target_id.0,
				damage,
				crit,
				position: pair(pose.0),
			},
		);
		if health.current == 0 {
			let position = pose.0;
			queue.push(
				*tick,
				PresentationKind::EnemyDied {
					target: hit.target_id.0,
					position: pair(position),
					scrap: stats.scrap,
				},
			);
			commands.entity(hit.target).despawn();
			commands.spawn((
				ids.sim_id(),
				Pickup { value: stats.scrap },
				Pose2(position),
				PreviousPose2(position),
			));
			continue;
		}
		let away = pose.0 - hit.origin;
		let direction = if away.length_squared() > 0.0 {
			away.normalize()
		} else {
			hit.direction
		};
		let mut next = pose.0 + direction * snap.knockback_lu * stats.knockback_multiplier;
		if let Some(arena) = &arena {
			next = arena.0.clamp_circle_center(next, body.0);
		}
		pose.0 = next;
	}

	let Ok((player_pose, player_body, player_stats, mut player_hp, mut invuln)) =
		players.single_mut()
	else {
		return;
	};
	for (_, position, radius, raw) in contacts {
		if player_hp.current == 0 || tick.0 < invuln.next_accept_tick {
			break;
		}
		let (Ok(a), Ok(b)) = (
			Circle::new(player_pose.0, player_body.0),
			Circle::new(position, radius),
		) else {
			continue;
		};
		if !circles_overlap(&a, &b) {
			continue;
		}
		let Ok(damage) = enemy_damage(raw, u32::try_from(player_stats.0.armor).unwrap_or(0)) else {
			continue;
		};
		player_hp.current = player_hp.current.saturating_sub(damage);
		invuln.next_accept_tick = tick.0 + INVULNERABILITY_TICKS;
		queue.push(
			*tick,
			PresentationKind::PlayerDamaged {
				damage,
				hp_after: player_hp.current,
			},
		);
	}
}

/// Pickup settlement then regeneration for a living player (GDD section 7.3).
pub fn collect_and_heal(
	tick: Res<SimTick>,
	mut scrap: ResMut<Scrap>,
	mut queue: ResMut<PresentationQueue>,
	mut commands: Commands,
	pickups: Query<(Entity, &SimId, &Pose2, &Pickup)>,
	mut players: Query<(&Pose2, &PlayerStats, &mut Health, &mut RegenAccumulator), With<Player>>,
) {
	let Ok((pose, stats, mut health, mut regen)) = players.single_mut() else {
		return;
	};
	if health.current == 0 {
		return;
	}
	let radius = f64::from(stats.0.pickup_radius);
	let mut order: Vec<_> = pickups.iter().collect();
	order.sort_by_key(|(_, id, ..)| **id);
	for (entity, _, at, pickup) in order {
		if at.0.distance_squared(pose.0) <= radius * radius {
			scrap.0 = scrap.0.saturating_add(pickup.value);
			queue.push(
				*tick,
				PresentationKind::PickupCollected {
					value: pickup.value,
				},
			);
			commands.entity(entity).despawn();
		}
	}
	health.current = regen.tick(health.current, health.max, stats.0.regen_tenths);
}
