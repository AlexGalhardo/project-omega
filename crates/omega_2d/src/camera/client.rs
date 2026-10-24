use super::ViewportMapping;
use crate::geometry::Bounds2;
use bevy::camera::{ScalingMode, Viewport};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

/// Marks the camera that shows a fixed world rectangle with letterboxing.
#[derive(Component, Debug, Clone, Copy)]
pub struct LogicalCamera {
	pub world: Bounds2,
}

/// Mapping of the primary window's logical camera, updated when the window changes.
#[derive(Resource, Debug, Clone, Copy)]
pub struct CurrentViewport(pub ViewportMapping);

pub struct LogicalCameraPlugin;

impl Plugin for LogicalCameraPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(
			PostUpdate,
			fit_logical_cameras.before(TransformSystems::Propagate),
		);
	}
}

fn fit_logical_cameras(
	mut commands: Commands,
	window: Single<&Window, (With<PrimaryWindow>, Changed<Window>)>,
	mut cameras: Query<(&LogicalCamera, &mut Camera, &mut Projection, &mut Transform)>,
) {
	let size = window.physical_size();
	for (logical, mut camera, mut projection, mut transform) in &mut cameras {
		let Ok(mapping) = ViewportMapping::new(logical.world, f64::from(size.x), f64::from(size.y))
		else {
			continue;
		};
		let rect = mapping.content_rect();
		camera.viewport = Some(Viewport {
			physical_position: rect.origin.round().as_uvec2(),
			physical_size: rect.size.round().as_uvec2().max(UVec2::ONE),
			..default()
		});
		let world = logical.world.size().as_vec2();
		if let Projection::Orthographic(ortho) = projection.as_mut() {
			ortho.scaling_mode = ScalingMode::Fixed {
				width: world.x,
				height: world.y,
			};
		}
		let center = ((logical.world.min() + logical.world.max()) * 0.5).as_vec2();
		transform.translation.x = center.x;
		transform.translation.y = center.y;
		commands.insert_resource(CurrentViewport(mapping));
	}
}
