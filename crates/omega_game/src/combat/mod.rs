//! Combat rules: exact stat, energy and damage evaluators (SP-006) and the actor,
//! movement and attack systems (SP-007).

pub mod actor;
pub mod damage;
pub mod energy;
pub mod movement;
pub mod stats;
pub mod weapons;

use crate::sim::{GameTick, TickSet};
use bevy::prelude::*;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<actor::Scrap>()
			.init_resource::<crate::input::AimOrigin>()
			.init_resource::<weapons::EmittedShapes>()
			.init_resource::<weapons::HitLedger>()
			.init_resource::<damage::HitCandidates>()
			.add_systems(GameTick, movement::move_player.in_set(TickSet::Movement))
			.add_systems(GameTick, weapons::emit_weapons.in_set(TickSet::Emit))
			.add_systems(
				GameTick,
				weapons::collide_player_attacks.in_set(TickSet::Collide),
			);
	}
}
