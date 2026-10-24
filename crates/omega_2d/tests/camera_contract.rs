//! SP-012-AC1: ENG-CAM-001 round trips, letterbox rejection and invalid sizes.

use bevy::math::DVec2;
use omega_2d::camera::{CameraError, ViewportMapping};
use omega_2d::geometry::Bounds2;

fn arena() -> Bounds2 {
	Bounds2::new(DVec2::new(-600.0, -337.5), DVec2::new(600.0, 337.5)).unwrap()
}

const SIZES: [(f64, f64); 5] = [
	(1280.0, 720.0),
	(1920.0, 1080.0),
	(2560.0, 1080.0),
	(1280.0, 1024.0),
	(800.0, 600.0),
];

#[test]
fn ac1_center_and_corners_round_trip_at_supported_sizes() {
	let points = [
		DVec2::ZERO,
		DVec2::new(-600.0, -337.5),
		DVec2::new(600.0, -337.5),
		DVec2::new(-600.0, 337.5),
		DVec2::new(600.0, 337.5),
		DVec2::new(123.25, -45.5),
	];
	for (w, h) in SIZES {
		let m = ViewportMapping::new(arena(), w, h).unwrap();
		for p in points {
			let back = m
				.screen_to_world(m.world_to_screen(p))
				.expect("inside content");
			assert!(back.distance(p) < 1e-9, "{w}x{h}: {p} -> {back}");
		}
		let c = m.world_to_screen(DVec2::ZERO);
		assert!(
			c.distance(DVec2::new(w / 2.0, h / 2.0)) < 1e-9,
			"center maps to screen center"
		);
	}
}

#[test]
fn ac1_world_extent_is_identical_across_resolutions() {
	for (w, h) in SIZES {
		let m = ViewportMapping::new(arena(), w, h).unwrap();
		let size = m.content_rect().size / m.pixels_per_lu();
		assert!(size.distance(DVec2::new(1200.0, 675.0)) < 1e-9);
	}
}

#[test]
fn ac1_letterbox_and_pillarbox_bars_reject_aim() {
	let wide = ViewportMapping::new(arena(), 2560.0, 1080.0).unwrap();
	let bar = wide.content_rect().origin.x;
	assert!(bar > 0.0);
	assert_eq!(wide.screen_to_world(DVec2::new(bar - 1.0, 540.0)), None);
	assert!(wide.screen_to_world(DVec2::new(bar + 1.0, 540.0)).is_some());
	let tall = ViewportMapping::new(arena(), 1280.0, 1024.0).unwrap();
	let top = tall.content_rect().origin.y;
	assert!(top > 0.0);
	assert_eq!(tall.screen_to_world(DVec2::new(640.0, top - 1.0)), None);
	assert_eq!(tall.screen_to_world(DVec2::new(640.0, 1023.0)), None);
	assert_eq!(tall.screen_to_world(DVec2::new(f64::NAN, 10.0)), None);
}

#[test]
fn invalid_dimensions_return_typed_errors() {
	assert_eq!(
		ViewportMapping::new(arena(), 0.0, 720.0),
		Err(CameraError::InvalidScreenSize)
	);
	assert_eq!(
		ViewportMapping::new(arena(), 1280.0, f64::NAN),
		Err(CameraError::InvalidScreenSize)
	);
	let flat = Bounds2::new(DVec2::ZERO, DVec2::new(10.0, 0.0)).unwrap();
	assert_eq!(
		ViewportMapping::new(flat, 1280.0, 720.0),
		Err(CameraError::EmptyWorld)
	);
}
