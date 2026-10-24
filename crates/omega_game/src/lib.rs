//! Project Omega game library shared by the native client, content CLI and headless
//! scenario runner. Rules live here; reusable engine contracts live in `omega_2d`.

pub mod combat;
pub mod content;
pub mod diagnostics;
pub mod input;
pub mod sim;

#[cfg(feature = "client")]
pub mod client;

use bevy::prelude::*;

/// Headless game core: engine contracts plus the authoritative simulation.
pub struct GameCorePlugin;

impl Plugin for GameCorePlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((
			omega_2d::Omega2dPlugin,
			sim::GameSimPlugin,
			combat::CombatPlugin,
		));
	}
}
