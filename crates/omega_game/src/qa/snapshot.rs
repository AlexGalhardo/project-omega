use crate::combat::actor::{Enemy, Health, Player, Pose2, Scrap};
use crate::input::AimOrigin;
use crate::sim::{SimPaused, SimTick};
use bevy::math::DVec2;
use bevy::prelude::*;
use omega_2d::camera::ViewportMapping;
use omega_2d::geometry::Bounds2;
use serde::Serialize;

/// Arena `arena_foundry` playable rectangle (GDD-004), shared with the client camera.
const ARENA_MIN: DVec2 = DVec2::new(-600.0, -337.5);
const ARENA_MAX: DVec2 = DVec2::new(600.0, 337.5);

pub fn arena() -> Bounds2 {
	Bounds2::new(ARENA_MIN, ARENA_MAX).expect("static arena bounds")
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PlayerFacts {
	pub x: f64,
	pub y: f64,
	pub hp: u32,
	pub hp_max: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CameraFacts {
	pub content_x: f64,
	pub content_y: f64,
	pub content_w: f64,
	pub content_h: f64,
	pub pixels_per_lu: f64,
	pub letterbox: bool,
}

/// Stand-in for a future menu or modal, so bot menu exploration is testable now.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MenuView {
	pub items: u8,
	pub selected: u8,
}

/// Authoritative state an e2e checkpoint exports and a driver observes.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Snapshot {
	pub tick: u64,
	pub paused: bool,
	pub window: [u32; 2],
	pub player: Option<PlayerFacts>,
	pub enemy_count: u32,
	pub scrap: u32,
	/// `None` when the window size is rejected by the viewport mapping.
	pub camera: Option<CameraFacts>,
	pub nearest_enemy: Option<[f64; 2]>,
	pub aim_origin: [f64; 2],
	#[serde(skip)]
	pub menu: Option<MenuView>,
}

pub fn camera_facts(window: [u32; 2]) -> Option<CameraFacts> {
	let mapping = ViewportMapping::new(arena(), f64::from(window[0]), f64::from(window[1])).ok()?;
	let rect = mapping.content_rect();
	let screen = mapping.screen_size();
	Some(CameraFacts {
		content_x: rect.origin.x,
		content_y: rect.origin.y,
		content_w: rect.size.x,
		content_h: rect.size.y,
		pixels_per_lu: mapping.pixels_per_lu(),
		letterbox: (screen - rect.size).abs().max_element() > 0.5,
	})
}

pub fn capture(world: &mut World, window: [u32; 2]) -> Snapshot {
	let player = world
		.query_filtered::<(&Pose2, &Health), With<Player>>()
		.iter(world)
		.next()
		.map(|(p, h)| PlayerFacts {
			x: p.0.x,
			y: p.0.y,
			hp: h.current,
			hp_max: h.max,
		});
	let origin = player
		.as_ref()
		.map_or(DVec2::ZERO, |p| DVec2::new(p.x, p.y));
	let mut enemy_count = 0;
	let mut nearest: Option<(f64, DVec2)> = None;
	for pose in world.query_filtered::<&Pose2, With<Enemy>>().iter(world) {
		enemy_count += 1;
		let d = pose.0.distance(origin);
		if nearest.is_none_or(|(best, _)| d < best) {
			nearest = Some((d, pose.0));
		}
	}
	Snapshot {
		tick: world.resource::<SimTick>().0,
		paused: world.resource::<SimPaused>().0,
		window,
		player,
		enemy_count,
		scrap: world.get_resource::<Scrap>().map_or(0, |s| s.0),
		camera: camera_facts(window),
		nearest_enemy: nearest.map(|(_, p)| [p.x, p.y]),
		aim_origin: world
			.get_resource::<AimOrigin>()
			.map_or([0.0, 0.0], |a| [a.0.x, a.0.y]),
		menu: None,
	}
}
