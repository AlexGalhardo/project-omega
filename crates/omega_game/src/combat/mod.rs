//! Combat rules: exact stat, energy and damage evaluators (SP-006) and the actor,
//! movement and attack systems (SP-007).

pub mod actor;
pub mod energy;
pub mod movement;
pub mod stats;

use crate::sim::{GameTick, TickSet};
use bevy::prelude::*;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<actor::Scrap>()
			.init_resource::<crate::input::AimOrigin>()
			.add_systems(GameTick, movement::move_player.in_set(TickSet::Movement));
	}
}
