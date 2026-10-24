//! Hostile projectiles (SP-008): swept against the player only, consumed on impact even
//! when invulnerability rejects the damage, removed at range end or the arena wall.

use super::ai::wall_fraction;
use crate::combat::actor::{
	ArenaRes, Body, Health, PLAYER_RADIUS_LU, Player, Pose2, PreviousPose2,
};
use crate::combat::damage::{HostileHit, HostileHits};
use crate::sim::{AttackId, PresentationKind, PresentationQueue, SimId, SimTick};
use bevy::math::DVec2;
use bevy::prelude::*;
use omega_2d::geometry::{Circle, sweep_circle};

const EPS: f64 = 1e-9;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct HostileProjectile {
	pub attack: AttackId,
	pub direction: DVec2,
	pub speed_per_tick: f64,
	pub range_left: f64,
	/// Wave-scaled raw damage, mitigated by the player's armor when it lands.
	pub damage: u32,
}

#[allow(clippy::type_complexity)]
pub fn move_hostile_projectiles(
	arena: Option<Res<ArenaRes>>,
	tick: Res<SimTick>,
	mut queue: ResMut<PresentationQueue>,
	mut hits: ResMut<HostileHits>,
	mut commands: Commands,
	players: Query<(&Pose2, &Health), (With<Player>, Without<HostileProjectile>)>,
	mut projectiles: Query<(
		Entity,
		&SimId,
		&mut Pose2,
		&mut PreviousPose2,
		&Body,
		&mut HostileProjectile,
	)>,
) {
	let target = players
		.single()
		.ok()
		.filter(|(_, h)| h.current > 0)
		.and_then(|(p, _)| Circle::new(p.0, PLAYER_RADIUS_LU).ok());
	let mut order: Vec<_> = projectiles.iter().map(|(e, id, ..)| (e, *id)).collect();
	order.sort_by_key(|(_, id)| *id);
	for (entity, sim_id) in order {
		let Ok((_, _, mut pose, mut previous, body, mut proj)) = projectiles.get_mut(entity) else {
			continue;
		};
		previous.0 = pose.0;
		let start = pose.0;
		let travel = proj.speed_per_tick.min(proj.range_left);
		let mut motion = proj.direction * travel;
		let mut wall = false;
		if let Some(arena) = &arena {
			let fraction = wall_fraction(arena, start, motion);
			if fraction < 1.0 {
				wall = true;
				motion *= fraction;
			}
		}
		let mut ended: Option<DVec2> = None;
		if let (Some(player), Ok(moving)) = (&target, Circle::new(start, body.0))
			&& let Some(time) = sweep_circle(&moving, motion, player)
		{
			ended = Some(start + motion * time);
			if hits.landed.insert(proj.attack) {
				hits.pending.push(HostileHit {
					attack: proj.attack,
					raw: proj.damage,
				});
			}
		}
		proj.range_left -= travel;
		if ended.is_none() && (wall || proj.range_left <= EPS) {
			ended = Some(start + motion);
		}
		match ended {
			Some(at) => {
				queue.push(
					*tick,
					PresentationKind::HostileProjectileEnded {
						sim_id: sim_id.0,
						position: [at.x, at.y],
					},
				);
				commands.entity(entity).despawn();
			}
			None => pose.0 = start + motion,
		}
	}
}
