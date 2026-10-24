//! SP-001-AC2: the headless game core creates no window, renderer or audio
//! resources, and registers the engine and simulation plugins.

use bevy::prelude::*;
use omega_game::GameCorePlugin;
use omega_game::sim::{GameSimPlugin, SimTick};

#[test]
fn headless_core_has_expected_plugins_and_no_devices() {
	let mut app = App::new();
	app.add_plugins(GameCorePlugin);
	app.update();
	assert!(app.is_plugin_added::<omega_2d::Omega2dPlugin>());
	assert!(app.is_plugin_added::<GameSimPlugin>());
	assert!(app.world().get_resource::<SimTick>().is_some());
	for (_, info, _) in app.world().iter_resources() {
		let name = info.name().to_string();
		assert!(
			!["Window", "Winit", "Render", "Audio", "Gilrs"]
				.iter()
				.any(|f| name.contains(f)),
			"headless core created device resource {name}"
		);
	}
}
