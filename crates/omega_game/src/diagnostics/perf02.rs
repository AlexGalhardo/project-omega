//! PERF-02 prototype workload: a seeded three-wave run with scripted movement and manual
//! aim, scripted shop visits and per-tick timing. Observation only: the timed region is
//! exactly one `GameTick`, and counts are read between ticks.

use super::{Metric, Report, ReportMetadata};
use crate::combat::actor::{ArenaRes, ContentRes, Enemy, Pickup, Player};
use crate::combat::weapons::Projectile;
use crate::content::{Profile, load_dir};
use crate::shop::{Offer, ShopVisit};
use crate::sim::run::{Run, RunState, begin_run, start_next_wave};
use crate::sim::{AimInput, RunSeed, SimHarness, TickInput};
use crate::ui::{UiAction, apply_action};
use bevy::math::DVec2;
use bevy::prelude::*;
use std::collections::BTreeSet;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;

/// Upper bound that keeps a stuck run from looping forever.
const MAX_TICKS: u32 = 40_000;

/// Eight-direction movement table advanced every 90 ticks.
const DIRECTIONS: [(f64, f64); 8] = [
	(1.0, 0.0),
	(1.0, 1.0),
	(0.0, 1.0),
	(-1.0, 1.0),
	(-1.0, 0.0),
	(-1.0, -1.0),
	(0.0, -1.0),
	(1.0, -1.0),
];

/// The recorded input for tick `n`: strafing in a fixed pattern, manual aim sweeping a
/// full turn every 600 ticks during the second half of each 300-tick block.
pub fn scripted_input(n: u32) -> TickInput {
	let (x, y) = DIRECTIONS[(n / 90) as usize % DIRECTIONS.len()];
	let movement = DVec2::new(x, y).normalize_or_zero();
	let aim = if (n / 300) % 2 == 1 {
		let angle = f64::from(n % 600) / 600.0 * std::f64::consts::TAU;
		AimInput::Manual(DVec2::from_angle(angle))
	} else {
		AimInput::Auto
	};
	TickInput { movement, aim }
}

/// Buys the first affordable offer, then starts the next wave.
fn scripted_shop(h: &mut SimHarness) {
	let world = h.app_mut().world_mut();
	let scrap = world.resource::<crate::combat::actor::Scrap>().0;
	let pick = world
		.resource::<ShopVisit>()
		.offers
		.iter()
		.position(|o| match o {
			Offer::Weapon { price, .. } | Offer::Item { price, .. } => *price <= scrap,
			_ => false,
		});
	if let Some(offer) = pick {
		apply_action(world, UiAction::Buy(offer));
		apply_action(world, UiAction::CancelReplace);
	}
	let _ = start_next_wave(world);
}

fn count<F: bevy::ecs::query::QueryFilter>(world: &mut World) -> f64 {
	world.query_filtered::<Entity, F>().iter(world).count() as f64
}

pub fn run(seed: RunSeed, content_dir: &Path) -> Result<Report, String> {
	let content = load_dir(content_dir, Profile::Prototype).map_err(|e| format!("{e:?}"))?;
	let revision: String = content
		.revision
		.iter()
		.map(|b| format!("{b:02x}"))
		.collect();
	let bounds = content.catalogs.arenas["arena_foundry"].bounds;
	let mut app = App::new();
	app.add_plugins(crate::GameCorePlugin);
	let mut h = SimHarness::with_app(app, seed);
	h.app_mut().insert_resource(ContentRes(Arc::new(content)));
	h.app_mut().insert_resource(ArenaRes(bounds));
	h.app_mut().init_resource::<crate::ui::UiMode>();
	begin_run(h.app_mut().world_mut(), "char_patch", BTreeSet::new())
		.map_err(|e| format!("{e:?}"))?;
	let (mut tick_us, mut enemies, mut projectiles, mut pickups) =
		(Vec::new(), Vec::new(), Vec::new(), Vec::new());
	let mut shop_visits = 0u32;
	let mut n = 0u32;
	while n < MAX_TICKS {
		match h.app().world().resource::<Run>().state {
			RunState::Wave => {
				let input = scripted_input(n);
				let start = Instant::now();
				h.step(input);
				tick_us.push(start.elapsed().as_secs_f64() * 1e6);
				h.drain_presentation();
				let world = h.app_mut().world_mut();
				enemies.push(count::<With<Enemy>>(world));
				projectiles.push(count::<With<Projectile>>(world));
				pickups.push(count::<With<Pickup>>(world));
				n += 1;
			}
			RunState::Shop => {
				shop_visits += 1;
				scripted_shop(&mut h);
			}
			_ => break,
		}
	}
	let world = h.app_mut().world_mut();
	let final_state = world.resource::<Run>().state;
	let player_alive = count::<With<Player>>(world) > 0.0;
	let mut metadata = ReportMetadata::new("PERF-02", seed.to_hex());
	metadata.content_revision = Some(revision);
	metadata.settings = vec![
		("profile".into(), "prototype (three waves)".into()),
		("ticks".into(), tick_us.len().to_string()),
		("shop_visits".into(), shop_visits.to_string()),
		("final_state".into(), format!("{final_state:?}")),
		("player_present".into(), player_alive.to_string()),
		(
			"input".into(),
			"scripted strafing and manual-aim sweep".into(),
		),
	];
	Ok(Report {
		metadata,
		metrics: vec![
			Metric::from_samples("sim_tick_wall_time", "us", &tick_us),
			Metric::from_samples("live_enemies", "count", &enemies),
			Metric::from_samples("live_projectiles", "count", &projectiles),
			Metric::from_samples("live_pickups", "count", &pickups),
		],
		unsupported: vec![
			"render_frame_time".into(),
			"gpu_frame_time".into(),
			"allocation_count".into(),
			"allocation_bytes".into(),
			"working_set".into(),
		],
	})
}
