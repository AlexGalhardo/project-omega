//! ENG-P2-001: views mirror simulation entities without feeding anything back.
#![cfg(feature = "client")]

use bevy::math::DVec2;
use bevy::prelude::*;
use omega_2d::visuals::VisualCatalog;
use omega_game::client::presentation::*;
use omega_game::combat::actor::{Body, Pickup, Player, Pose2, PreviousPose2};
use omega_game::combat::weapons::Projectile;
use omega_game::sim::{AttackId, PresentationKind, PresentationQueue, SimId, SimTick};
use std::path::PathBuf;

fn app() -> App {
	let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets");
	let bytes = std::fs::read(root.join("presentation/proof-catalog.json")).unwrap();
	let skin = VisualCatalog::from_json(&bytes, |_| true).unwrap();
	let mut app = App::new();
	app.add_plugins((MinimalPlugins, AssetPlugin::default()))
		.init_asset::<Image>()
		.init_resource::<PresentationQueue>()
		.insert_resource(ActiveSkin(skin))
		.add_plugins(LivePresentationPlugin);
	app
}

fn projectile(radius: f64) -> impl Bundle {
	(
		SimId(2),
		Projectile {
			attack: AttackId(1),
			direction: DVec2::X,
			speed_per_tick: 8.0,
			range_left: 100.0,
			pierce_left: 0,
			explosion_radius: None,
			snapshot: omega_game::combat::weapons::AttackSnapshot {
				base_damage: 1,
				tier: 1,
				damage_pct: 0,
				crit_pct: 0,
				knockback_lu: 0.0,
			},
			origin: DVec2::ZERO,
		},
		Pose2(DVec2::new(10.0, 0.0)),
		PreviousPose2(DVec2::new(2.0, 0.0)),
		Body(radius),
	)
}

#[test]
fn views_follow_sim_entities_and_disappear_with_them() {
	let mut app = app();
	let player = app
		.world_mut()
		.spawn((
			SimId(1),
			Player,
			Pose2(DVec2::new(5.0, 6.0)),
			PreviousPose2(DVec2::new(5.0, 6.0)),
		))
		.id();
	let shot = app.world_mut().spawn(projectile(4.0)).id();
	app.world_mut()
		.spawn((SimId(3), Pickup { value: 2 }, Pose2(DVec2::new(-20.0, 0.0))));
	app.update();
	assert_eq!(app.world().resource::<RenderMap>().0.len(), 3);
	app.update();
	assert_eq!(
		app.world().resource::<RenderMap>().0.len(),
		3,
		"no duplicate views"
	);
	app.world_mut().despawn(shot);
	app.update();
	let map = app.world().resource::<RenderMap>();
	assert_eq!(map.0.len(), 2);
	assert!(!map.0.contains_key(&SimId(2)));
	let _ = player;
}

#[test]
fn projectile_core_uses_the_exact_collision_diameter() {
	let mut app = app();
	app.world_mut().spawn(projectile(4.0));
	app.update();
	let view = *app
		.world()
		.resource::<RenderMap>()
		.0
		.get(&SimId(2))
		.unwrap();
	let size = app.world().get::<Sprite>(view).unwrap().custom_size;
	assert_eq!(size, Some(Vec2::splat(8.0)));
}

#[test]
fn queue_is_drained_once_per_frame() {
	let mut app = app();
	app.world_mut()
		.resource_mut::<PresentationQueue>()
		.push(SimTick(1), PresentationKind::Marker(7));
	app.update();
	assert_eq!(app.world().resource::<FrameRecords>().0.len(), 1);
	assert!(app.world().resource::<PresentationQueue>().is_empty());
	app.update();
	assert!(app.world().resource::<FrameRecords>().0.is_empty());
}
