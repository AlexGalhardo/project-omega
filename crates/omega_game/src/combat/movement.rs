//! Player movement in `TickSet::Movement` (GDD-004, GDD section 7.3).

use super::actor::{
	ArenaRes, BASE_SPEED_LU_S, Body, Health, Player, PlayerStats, Pose2, PreviousPose2,
};
use crate::input::AimOrigin;
use crate::sim::{TICKS_PER_SECOND, TickInput};
use bevy::prelude::*;

pub fn move_player(
	input: Res<TickInput>,
	arena: Option<Res<ArenaRes>>,
	mut origin: ResMut<AimOrigin>,
	mut players: Query<
		(&mut Pose2, &mut PreviousPose2, &Body, &PlayerStats, &Health),
		With<Player>,
	>,
) {
	for (mut pose, mut previous, body, stats, health) in &mut players {
		previous.0 = pose.0;
		if health.current == 0 {
			continue;
		}
		let speed = BASE_SPEED_LU_S * (1.0 + f64::from(stats.0.move_speed_pct) / 100.0)
			/ f64::from(TICKS_PER_SECOND);
		let mut next = pose.0 + input.movement * speed;
		if let Some(arena) = &arena {
			next = arena.0.clamp_circle_center(next, body.0);
		}
		pose.0 = next;
		origin.0 = next;
	}
}
