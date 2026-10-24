//! SP-001-AC2: a headless app with the engine plugin creates no window, renderer or
//! audio resources.

use bevy::prelude::*;
use omega_2d::Omega2dPlugin;

#[test]
fn headless_engine_app_has_no_device_resources() {
	let mut app = App::new();
	app.add_plugins(Omega2dPlugin);
	app.update();
	let forbidden = ["Window", "Winit", "Render", "Audio", "Gilrs"];
	for (_, info, _) in app.world().iter_resources() {
		let name = info.name().to_string();
		assert!(
			!forbidden.iter().any(|f| name.contains(f)),
			"headless app created device resource {name}"
		);
	}
}
