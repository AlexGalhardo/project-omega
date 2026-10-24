use crate::combat::actor::{Enemy, Health, Loadout, Player, Pose2, Scrap};
use crate::input::AimOrigin;
use crate::shop::{Offer, RunBuild, ShopVisit, refresh_cost};
use crate::sim::run::{Outcome, Run, RunState};
use crate::sim::{SimPaused, SimTick};
use crate::ui::{ScreenView, UiMode, screen_view};
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

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RunFacts {
	/// Lowercase `RunState`: wave, shop, victory, defeat or summary.
	pub state: &'static str,
	pub wave: u32,
	pub total_waves: u32,
	/// Set once the run settles: victory, defeat or abandoned.
	pub outcome: Option<&'static str>,
	pub waves_completed: Option<u32>,
	pub settlements: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct OfferFacts {
	/// weapon, item, sold or empty.
	pub kind: &'static str,
	pub id: String,
	pub tier: u8,
	pub price: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ShopFacts {
	pub visit: u32,
	pub revision: u64,
	pub refresh_used: bool,
	pub refresh_cost: u32,
	pub offers: Vec<OfferFacts>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SlotFacts {
	pub weapon: Option<String>,
	pub tier: u8,
	pub points: u8,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct LoadoutFacts {
	pub slots: Vec<SlotFacts>,
	pub occupied: u8,
	pub energy_used: u8,
	pub items: Vec<String>,
	pub item_count: u32,
	pub unallocated: u8,
	pub powered: bool,
}

/// What the last typed UI request did, as a person would read it.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct UiFacts {
	pub mode: &'static str,
	pub message: String,
	pub actions: u32,
}

/// Message and counter of the last [`crate::ui::UiAction`] the QA harness applied.
#[derive(Resource, Debug, Clone, Default)]
pub struct QaUi {
	pub message: String,
	pub actions: u32,
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
	pub run: Option<RunFacts>,
	pub shop: Option<ShopFacts>,
	pub loadout: Option<LoadoutFacts>,
	pub ui: UiFacts,
	#[serde(skip)]
	pub menu: Option<MenuView>,
	/// The legal-action layer: the panel the player sees, `None` during live combat.
	#[serde(skip)]
	pub screen: Option<ScreenView>,
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

fn state_name(state: RunState) -> &'static str {
	match state {
		RunState::Wave => "wave",
		RunState::Shop => "shop",
		RunState::Victory => "victory",
		RunState::Defeat => "defeat",
		RunState::Summary => "summary",
	}
}

fn run_facts(world: &World) -> Option<RunFacts> {
	let run = world.get_resource::<Run>()?;
	Some(RunFacts {
		state: state_name(run.state),
		wave: run.wave,
		total_waves: run.total_waves,
		outcome: run.result.map(|r| match r.outcome {
			Outcome::Victory => "victory",
			Outcome::Defeat => "defeat",
			Outcome::Abandoned => "abandoned",
		}),
		waves_completed: run.result.map(|r| r.waves_completed),
		settlements: run.settlements,
	})
}

fn shop_facts(visit: &ShopVisit) -> ShopFacts {
	let offers = visit
		.offers
		.iter()
		.map(|offer| match offer {
			Offer::Weapon { id, tier, price } => OfferFacts {
				kind: "weapon",
				id: id.clone(),
				tier: *tier,
				price: *price,
			},
			Offer::Item { id, price } => OfferFacts {
				kind: "item",
				id: id.clone(),
				tier: 0,
				price: *price,
			},
			Offer::Sold => OfferFacts {
				kind: "sold",
				id: String::new(),
				tier: 0,
				price: 0,
			},
			Offer::Empty => OfferFacts {
				kind: "empty",
				id: String::new(),
				tier: 0,
				price: 0,
			},
		})
		.collect();
	ShopFacts {
		visit: visit.visit,
		revision: visit.revision,
		refresh_used: visit.refresh_used,
		refresh_cost: refresh_cost(visit.visit),
		offers,
	}
}

fn loadout_facts(loadout: &Loadout, items: Vec<String>) -> LoadoutFacts {
	let points = loadout.energy.slots();
	let occupied = loadout.occupied();
	LoadoutFacts {
		slots: loadout
			.slots
			.iter()
			.zip(points)
			.map(|(slot, points)| SlotFacts {
				weapon: slot.as_ref().map(|w| w.weapon_id.clone()),
				tier: slot.as_ref().map_or(0, |w| w.tier),
				points,
			})
			.collect(),
		occupied: occupied.iter().filter(|o| **o).count() as u8,
		energy_used: points.iter().sum(),
		item_count: items.len() as u32,
		items,
		unallocated: loadout.energy.remainder(),
		powered: loadout.energy.any_powered(occupied),
	}
}

pub fn capture(world: &mut World, window: [u32; 2]) -> Snapshot {
	let mode = world.get_resource::<UiMode>().copied().unwrap_or_default();
	let paused = world.resource::<SimPaused>().0;
	let screen = screen_view(world, mode, paused);
	let items = world
		.get_resource::<RunBuild>()
		.map_or_else(Vec::new, |b| b.items.iter().cloned().collect());
	let loadout = world
		.query_filtered::<&Loadout, With<Player>>()
		.iter(world)
		.next()
		.map(|l| loadout_facts(l, items));
	let ui = world.get_resource::<QaUi>().cloned().unwrap_or_default();
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
		run: run_facts(world),
		shop: world.get_resource::<ShopVisit>().map(shop_facts),
		loadout,
		ui: UiFacts {
			mode: match mode {
				UiMode::Normal => "normal",
				UiMode::Replace(_) => "replace",
				UiMode::ConfirmAbandon => "confirm_abandon",
			},
			message: ui.message,
			actions: ui.actions,
		},
		menu: None,
		screen,
	}
}
