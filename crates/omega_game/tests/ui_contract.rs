//! SP-014 prototype acceptance: HUD/shop view models, disabled reasons in text and typed
//! actions completing the loop without any widget touching rules state directly.

use bevy::math::DVec2;
use bevy::prelude::*;
use omega_game::combat::actor::*;
use omega_game::content::{Profile, load_dir};
use omega_game::shop::{Offer, ShopVisit};
use omega_game::sim::run::*;
use omega_game::sim::*;
use omega_game::ui::*;
use std::collections::BTreeSet;
use std::path::PathBuf;
use std::sync::Arc;

fn run_in_shop() -> SimHarness {
	let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/content");
	let content = load_dir(&dir, Profile::Prototype).expect("prototype content");
	let arena = content.catalogs.arenas["arena_foundry"].bounds;
	let mut app = App::new();
	app.add_plugins(omega_game::GameCorePlugin);
	let mut h = SimHarness::with_app(app, RunSeed::from_hex(&"44".repeat(32)).unwrap());
	h.app_mut().insert_resource(ContentRes(Arc::new(content)));
	h.app_mut().insert_resource(ArenaRes(arena));
	h.app_mut().init_resource::<UiMode>();
	let world = h.app_mut().world_mut();
	begin_run(world, "char_patch", BTreeSet::new()).unwrap();
	let player = world
		.query_filtered::<Entity, With<Player>>()
		.single(world)
		.unwrap();
	world.get_mut::<Health>(player).unwrap().current = 1_000_000;
	for _ in 0..4000 {
		if h.app().world().resource::<Run>().state == RunState::Shop {
			break;
		}
		h.step(TickInput::default());
	}
	assert_eq!(h.app().world().resource::<Run>().state, RunState::Shop);
	h
}

fn world(h: &mut SimHarness) -> &mut World {
	h.app_mut().world_mut()
}

fn screen(h: &mut SimHarness) -> ScreenView {
	screen_view(world(h), UiMode::Normal, false).expect("shop screen")
}

#[test]
fn hud_shows_hp_wave_timer_scrap_and_slots() {
	let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/content");
	let content = load_dir(&dir, Profile::Prototype).unwrap();
	let mut app = App::new();
	app.add_plugins(omega_game::GameCorePlugin);
	let mut h = SimHarness::with_app(app, RunSeed::from_hex(&"44".repeat(32)).unwrap());
	h.app_mut().insert_resource(ContentRes(Arc::new(content)));
	begin_run(world(&mut h), "char_patch", BTreeSet::new()).unwrap();
	h.step(TickInput::default());
	let hud = hud_view(world(&mut h)).unwrap();
	assert_eq!((hud.hp, hud.max_hp), (120, 120));
	assert_eq!((hud.wave, hud.total_waves), (1, 3));
	assert_eq!(hud.timer, "60s");
	assert_eq!(hud.slots[0].name.as_deref(), Some("Bolt Driver"));
	assert_eq!(hud.slots[0].tier, 1);
	assert_eq!(hud.slots[0].points, 3);
	assert_eq!(
		hud.slots[0].rate,
		Some(4.0),
		"base 2.0/s at three points is 4.0/s"
	);
	assert_eq!(hud.unallocated, 3);
	let text = hud_text(&hud);
	assert!(
		text.contains("HP 120/120")
			&& text.contains("Wave 1/3")
			&& text.contains("[1] Bolt Driver")
	);
	assert!(text.contains("[2] empty"));
}

#[test]
fn shop_screen_lists_offers_costs_refresh_power_and_start() {
	let mut h = run_in_shop();
	let view = screen(&mut h);
	assert_eq!(view.title, "Shop");
	let labels: Vec<_> = view.buttons.iter().map(|b| b.label.as_str()).collect();
	for expected in [
		"[1]",
		"[2]",
		"[3]",
		"[4]",
		"[F] Refresh offers - 8 scrap",
		"Start next wave",
	] {
		assert!(
			labels.iter().any(|l| l.contains(expected)),
			"{expected} missing in {labels:?}"
		);
	}
	assert!(view.lines.iter().any(|l| l.contains("Reactor: 3 of 6")));
}

#[test]
fn disabled_actions_state_their_reason_in_text() {
	let mut h = run_in_shop();
	world(&mut h).resource_mut::<Scrap>().0 = 0;
	let view = screen(&mut h);
	let buy = view
		.buttons
		.iter()
		.find(|b| b.action == UiAction::Buy(0))
		.unwrap();
	assert!(!buy.enabled);
	assert!(
		view.lines
			.iter()
			.any(|l| l.contains("[1] unavailable: Need"))
	);
	assert!(
		view.lines
			.iter()
			.any(|l| l.contains("Refresh unavailable: Need 8 more scrap"))
	);
	let p = player(&mut h);
	world(&mut h).get_mut::<Loadout>(p).unwrap().energy =
		omega_game::combat::energy::EnergyAllocation::new([0, 0, 0, 0]);
	let view = screen(&mut h);
	let start = view
		.buttons
		.iter()
		.find(|b| b.action == UiAction::Start)
		.unwrap();
	assert!(!start.enabled);
	assert!(
		view.lines
			.iter()
			.any(|l| l.contains("Cannot start: Give at least one weapon"))
	);
}

fn player(h: &mut SimHarness) -> Entity {
	let w = world(h);
	w.query_filtered::<Entity, With<Player>>()
		.single(w)
		.unwrap()
}

#[test]
fn actions_buy_refresh_allocate_and_start_through_typed_requests() {
	let mut h = run_in_shop();
	world(&mut h).resource_mut::<Scrap>().0 = 500;
	let weapon_offer = world(&mut h)
		.resource::<ShopVisit>()
		.offers
		.iter()
		.position(|o| matches!(o, Offer::Weapon { .. }))
		.unwrap();
	assert_eq!(
		apply_action(world(&mut h), UiAction::Buy(weapon_offer)),
		"Purchased."
	);
	assert!(world(&mut h).resource::<Scrap>().0 < 500);
	let p = player(&mut h);
	assert_eq!(
		world(&mut h)
			.get::<Loadout>(p)
			.unwrap()
			.slots
			.iter()
			.flatten()
			.count(),
		2
	);
	assert_eq!(
		apply_action(world(&mut h), UiAction::Refresh),
		"Offers refreshed."
	);
	assert_eq!(
		apply_action(world(&mut h), UiAction::Refresh),
		"Refresh already used this visit."
	);
	apply_action(
		world(&mut h),
		UiAction::Energy {
			slot: 0,
			add: false,
		},
	);
	apply_action(world(&mut h), UiAction::Energy { slot: 1, add: true });
	assert_eq!(
		world(&mut h).get::<Loadout>(p).unwrap().energy.slots(),
		[2, 1, 0, 0]
	);
	apply_action(world(&mut h), UiAction::Start);
	assert_eq!(world(&mut h).resource::<Run>().state, RunState::Wave);
}

#[test]
fn a_full_loadout_prompts_for_a_replacement_and_cancel_changes_nothing() {
	let mut h = run_in_shop();
	let p = player(&mut h);
	{
		let mut l = world(&mut h).get_mut::<Loadout>(p).unwrap();
		for (i, id) in ["wpn_saw", "wpn_saw", "wpn_saw"].iter().enumerate() {
			l.slots[i + 1] = Some(EquippedWeapon {
				weapon_id: (*id).into(),
				tier: 1,
			});
		}
	}
	world(&mut h).resource_mut::<Scrap>().0 = 500;
	let target = Offer::Weapon {
		id: "wpn_bolt".into(),
		tier: 1,
		price: 20,
	};
	world(&mut h).resource_mut::<ShopVisit>().offers[0] = target;
	// Bolt is owned in slot 0, so this would be an upgrade, not a replacement: use a saw offer.
	world(&mut h).get_mut::<Loadout>(p).unwrap().slots[0] = Some(EquippedWeapon {
		weapon_id: "wpn_saw".into(),
		tier: 1,
	});
	let before = world(&mut h).get::<Loadout>(p).unwrap().clone();
	assert_eq!(apply_action(world(&mut h), UiAction::Buy(0)), "");
	assert_eq!(*world(&mut h).resource::<UiMode>(), UiMode::Replace(0));
	let view = screen_view(world(&mut h), UiMode::Replace(0), false).unwrap();
	assert_eq!(view.title, "Replace a weapon");
	apply_action(world(&mut h), UiAction::CancelReplace);
	assert_eq!(*world(&mut h).resource::<UiMode>(), UiMode::Normal);
	assert_eq!(world(&mut h).get::<Loadout>(p).unwrap(), &before);
	assert_eq!(world(&mut h).resource::<Scrap>().0, 500);
	apply_action(world(&mut h), UiAction::Buy(0));
	apply_action(world(&mut h), UiAction::ReplaceSlot(2));
	let l = world(&mut h).get::<Loadout>(p).unwrap();
	assert_eq!(l.slots[2].as_ref().unwrap().weapon_id, "wpn_bolt");
	assert_eq!(world(&mut h).resource::<Scrap>().0, 480);
}

#[test]
fn pause_screen_shows_stats_with_units_and_abandon_needs_confirmation() {
	let mut h = run_in_shop();
	apply_action(world(&mut h), UiAction::Start);
	let view = screen_view(world(&mut h), UiMode::Normal, true).unwrap();
	assert_eq!(view.title, "Paused");
	assert!(
		view.lines
			.iter()
			.any(|l| l.contains("Max HP") && l.contains("Armor"))
	);
	assert!(
		view.lines
			.iter()
			.any(|l| l.contains("Pickup radius") && l.contains("LU"))
	);
	assert!(
		screen_view(world(&mut h), UiMode::Normal, false).is_none(),
		"live combat shows no panel"
	);
	apply_action(world(&mut h), UiAction::AskAbandon);
	assert_eq!(*world(&mut h).resource::<UiMode>(), UiMode::ConfirmAbandon);
	apply_action(world(&mut h), UiAction::CancelAbandon);
	assert_eq!(world(&mut h).resource::<Run>().state, RunState::Wave);
	apply_action(world(&mut h), UiAction::AskAbandon);
	apply_action(world(&mut h), UiAction::ConfirmAbandon);
	assert_eq!(world(&mut h).resource::<Run>().state, RunState::Summary);
	let summary = screen_view(world(&mut h), UiMode::Normal, false).unwrap();
	assert_eq!(summary.title, "Summary");
	let _ = DVec2::ZERO;
}

#[test]
fn upgrade_offer_shows_old_and_new_damage_and_the_slots_actual_rate() {
	let mut h = run_in_shop();
	world(&mut h).resource_mut::<Scrap>().0 = 500;
	let upgrade = Offer::Weapon {
		id: "wpn_bolt".into(),
		tier: 2,
		price: 30,
	};
	world(&mut h).resource_mut::<ShopVisit>().offers[0] = upgrade;
	let view = omega_game::ui::shop_view(world(&mut h)).unwrap();
	let line = &view.offers[0].detail[0];
	// Bolt base 12: tier 1 = 12, tier 2 = 16 (12 * 140 / 100 = 16.8 floored).
	assert!(line.contains("damage 12 -> 16"), "{line}");
	assert!(line.contains("4.00/s at 3 pt"), "{line}");
	assert!(line.contains("range 300 LU"), "{line}");
}

#[test]
fn new_weapon_in_an_empty_slot_shows_rates_at_every_point_count() {
	let mut h = run_in_shop();
	let view = omega_game::ui::shop_view(world(&mut h)).unwrap();
	let weapon = view
		.offers
		.iter()
		.find(|o| o.label.starts_with("New "))
		.expect("a new weapon is offered");
	let text = weapon.detail.join(" | ");
	assert!(text.contains("goes to slot 2 with 0 pt"), "{text}");
	for points in ["1 pt", "2 pt", "3 pt"] {
		assert!(text.contains(points), "{text}");
	}
	assert!(
		!text.contains("unpowered"),
		"an empty slot is not shown as powered or unpowered: {text}"
	);
}

#[test]
fn item_offers_list_their_modifiers_with_units() {
	let mut h = run_in_shop();
	let view = omega_game::ui::shop_view(world(&mut h)).unwrap();
	let item = view
		.offers
		.iter()
		.find(|o| o.label.starts_with("Item "))
		.unwrap();
	assert!(
		item.detail[0].contains('+') || item.detail[0].contains('-'),
		"{:?}",
		item.detail
	);
}

#[test]
fn replacement_prompt_names_what_is_lost_and_gained_per_slot() {
	let mut h = run_in_shop();
	let p = player(&mut h);
	{
		let mut l = world(&mut h).get_mut::<Loadout>(p).unwrap();
		for i in 1..4 {
			l.slots[i] = Some(EquippedWeapon {
				weapon_id: "wpn_saw".into(),
				tier: 1,
			});
		}
	}
	world(&mut h).resource_mut::<Scrap>().0 = 500;
	world(&mut h).resource_mut::<ShopVisit>().offers[0] = Offer::Weapon {
		id: "wpn_bolt".into(),
		tier: 1,
		price: 20,
	};
	world(&mut h).get_mut::<Loadout>(p).unwrap().slots[0] = Some(EquippedWeapon {
		weapon_id: "wpn_saw".into(),
		tier: 2,
	});
	let screen = screen_view(world(&mut h), UiMode::Replace(0), false).unwrap();
	let text = screen.lines.join("\n");
	assert!(text.contains("Slot 1: lose Chain Saw tier 2"), "{text}");
	assert!(text.contains("gain Bolt Driver tier 1"), "{text}");
	assert_eq!(
		screen
			.buttons
			.iter()
			.filter(|b| b.label.starts_with("Replace ["))
			.count(),
		4
	);
	assert!(
		screen
			.buttons
			.iter()
			.any(|b| b.action == UiAction::CancelReplace)
	);
}
