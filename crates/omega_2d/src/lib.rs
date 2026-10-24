//! Reusable 2D layer over upstream Bevy: geometry (ENG-GEOM-001), logical camera
//! (ENG-CAM-001) and visual catalog (ENG-VIS-001). No game rules live here.

pub mod camera;
pub mod geometry;
pub mod visuals;

use bevy::prelude::*;

/// Engine entry point. Headless builds register nothing that needs a window,
/// renderer or audio device; the `client` feature adds camera integration.
pub struct Omega2dPlugin;

impl Plugin for Omega2dPlugin {
	fn build(&self, _app: &mut App) {
		#[cfg(feature = "client")]
		_app.add_plugins(camera::LogicalCameraPlugin);
	}
}
