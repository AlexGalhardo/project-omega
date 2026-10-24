//! SP-009 acceptance: eligibility, prices, offer draws and atomic transactions.

use bevy::math::DVec2;
use bevy::prelude::*;
use omega_game::combat::actor::*;
use omega_game::combat::energy::EnergyError;
use omega_game::content::{ContentSnapshot, Profile, load_dir};
use omega_game::shop::*;
use omega_game::sim::*;
use std::collections::BTreeSet;
use std::path::PathBuf;
use std::sync::Arc;

fn fixture() -> ContentSnapshot {
	let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/content/full");
	load_dir(&dir, Profile::Full).expect("full fixture")
}

fn harness_with(content: ContentSnapshot) -> (SimHarness, Entity) {
	let arena = content.catalogs.arenas["arena_foundry"].bounds;
	let mut app = App::new();
	app.add_plugins(omega_game::GameCorePlugin);
	let mut h = SimHarness::with_app(app, RunSeed::from_hex(&"22".repeat(32)).unwrap());
	h.app_mut().insert_resource(ContentRes(Arc::new(content)));
	h.app_mut().insert_resource(ArenaRes(arena));
	let player = spawn_player(h.app_mut().world_mut(), "char_patch", DVec2::ZERO).unwrap();
	h.app_mut().world_mut().insert_resource(RunBuild {
		character_id: "char_patch".into(),
		items: BTreeSet::new(),
		unlocked: BTreeSet::new(),
	});
	(h, player)
}

fn shop(visit: u32, scrap: u32) -> (SimHarness, Entity) {
	let (mut h, p) = harness_with(fixture());
	h.app_mut().world_mut().resource_mut::<Scrap>().0 = scrap;
	open_shop(h.app_mut().world_mut(), visit).unwrap();
	(h, p)
}

fn world(h: &mut SimHarness) -> &mut World {
	h.app_mut().world_mut()
}

fn visit(h: &SimHarness) -> ShopVisit {
	h.app().world().resource::<ShopVisit>().clone()
}

fn scrap(h: &SimHarness) -> u32 {
	h.app().world().resource::<Scrap>().0
}

fn shop_rng(h: &SimHarness) -> u128 {
	h.app().world().resource::<RngStreams>().shop.position()
}

/// Snapshot of everything a rejected command must leave alone.
fn state(h: &SimHarness, p: Entity) -> (u32, ShopVisit, Loadout, u128, RunBuild) {
	let w = h.app().world();
	(
		scrap(h),
		visit(h),
		w.get::<Loadout>(p).unwrap().clone(),
		shop_rng(h),
		w.resource::<RunBuild>().clone(),
	)
}

fn set_offers(h: &mut SimHarness, offers: [Offer; OFFER_COUNT]) {
	world(h).resource_mut::<ShopVisit>().offers = offers;
}

fn slug_upgrade(visit: u32) -> Offer {
	Offer::Weapon {
		id: "wpn_slug".into(),
		tier: 2,
		price: price(24, 150, visit),
	}
}

fn equip(h: &mut SimHarness, p: Entity, slot: usize, id: &str, tier: u8) {
	world(h).get_mut::<Loadout>(p).unwrap().slots[slot] = Some(EquippedWeapon {
		weapon_id: id.into(),
		tier,
	});
}

#[test]
fn ac1_prices_use_one_final_ceiling() {
	assert_eq!(price(24, 100, 3), 28);
	assert_eq!(price(24, 150, 3), 42);
	assert_eq!(price(24, 200, 3), 56);
	assert_eq!(price(20, 100, 1), 20);
	assert_eq!(refresh_cost(1), 8);
	assert_eq!(refresh_cost(3), 12);
}

#[test]
fn offers_are_two_weapons_and_two_items_from_unlocked_content() {
	let (h, _) = shop(3, 0);
	let v = visit(&h);
	let content = h.app().world().resource::<ContentRes>().0.clone();
	assert!(matches!(v.offers[0], Offer::Weapon { .. }));
	assert!(matches!(v.offers[1], Offer::Weapon { .. }));
	assert!(matches!(v.offers[2], Offer::Item { .. }));
	assert!(matches!(v.offers[3], Offer::Item { .. }));
	let ids: Vec<&String> = v
		.offers
		.iter()
		.map(|o| match o {
			Offer::Weapon { id, .. } | Offer::Item { id, .. } => id,
			_ => unreachable!(),
		})
		.collect();
	assert_eq!(
		ids.iter().collect::<BTreeSet<_>>().len(),
		4,
		"no duplicates"
	);
	for id in ids {
		let initial = content
			.catalogs
			.weapons
			.get(id)
			.map(|w| w.availability.is_initial())
			.or_else(|| {
				content
					.catalogs
					.items
					.get(id)
					.map(|i| i.availability.is_initial())
			})
			.unwrap();
		assert!(initial, "{id} is locked");
	}
}

#[test]
fn owned_weapon_appears_only_as_its_next_tier_and_tier3_is_excluded() {
	let mut content = fixture();
	content
		.catalogs
		.weapons
		.retain(|id, _| id == "wpn_bolt" || id == "wpn_slug");
	let (mut h, p) = harness_with(content);
	equip(&mut h, p, 1, "wpn_slug", 3);
	open_shop(world(&mut h), 2).unwrap();
	let offers = visit(&h).offers;
	let weapons: Vec<_> = offers
		.iter()
		.filter(|o| matches!(o, Offer::Weapon { .. }))
		.collect();
	assert_eq!(weapons.len(), 1, "tier-3 slug excluded: {offers:?}");
	assert_eq!(
		weapons[0],
		&Offer::Weapon {
			id: "wpn_bolt".into(),
			tier: 2,
			price: price(20, 150, 2)
		}
	);
}

#[test]
fn exhaustion_fills_from_the_other_category_then_shows_empty() {
	let mut content = fixture();
	content.catalogs.weapons.retain(|id, _| id == "wpn_bolt");
	let (mut h, p) = harness_with(content.clone());
	equip(&mut h, p, 0, "wpn_bolt", 3);
	open_shop(world(&mut h), 1).unwrap();
	let offers = visit(&h).offers;
	assert!(
		offers.iter().all(|o| matches!(o, Offer::Item { .. })),
		"no weapon left: four items fill the row: {offers:?}"
	);
	let mut bare = content;
	bare.catalogs.items.clear();
	let (mut h, p) = harness_with(bare);
	equip(&mut h, p, 0, "wpn_bolt", 3);
	open_shop(world(&mut h), 1).unwrap();
	assert_eq!(visit(&h).offers, [const { Offer::Empty }; OFFER_COUNT]);
}

#[test]
fn owned_items_are_never_offered_again() {
	let (mut h, _) = harness_with(fixture());
	let all: BTreeSet<String> = h
		.app()
		.world()
		.resource::<ContentRes>()
		.0
		.catalogs
		.items
		.iter()
		.filter(|(_, i)| i.availability.is_initial())
		.map(|(id, _)| id.clone())
		.collect();
	let keep = all.iter().next().unwrap().clone();
	let owned: BTreeSet<String> = all.iter().filter(|id| **id != keep).cloned().collect();
	world(&mut h).resource_mut::<RunBuild>().items = owned;
	open_shop(world(&mut h), 1).unwrap();
	let items: Vec<_> = visit(&h)
		.offers
		.into_iter()
		.filter_map(|o| match o {
			Offer::Item { id, .. } => Some(id),
			_ => None,
		})
		.collect();
	assert_eq!(items, vec![keep], "only the single unowned item remains");
}

#[test]
fn unlocked_content_joins_the_pool_only_via_the_run_snapshot() {
	let (mut h, _) = harness_with(fixture());
	open_shop(world(&mut h), 1).unwrap();
	let locked_seen = visit(&h).offers.iter().any(|o| {
		matches!(o, Offer::Weapon { id, .. } if id == "wpn_mortar")
			|| matches!(o, Offer::Item { id, .. } if id == "item_chassis")
	});
	assert!(!locked_seen);
	world(&mut h).resource_mut::<RunBuild>().unlocked = ["unlock_overtime".to_string()].into();
	let mut seen = false;
	for _ in 0..40 {
		open_shop(world(&mut h), 1).unwrap();
		seen |= visit(&h)
			.offers
			.iter()
			.any(|o| matches!(o, Offer::Weapon { id, .. } if id == "wpn_mortar"));
	}
	assert!(seen, "an unlocked weapon becomes drawable");
}

#[test]
fn da08_upgrade_charges_once_and_rejection_changes_nothing() {
	let (mut h, p) = shop(3, 41);
	equip(&mut h, p, 0, "wpn_slug", 1);
	set_offers(
		&mut h,
		[slug_upgrade(3), Offer::Empty, Offer::Empty, Offer::Empty],
	);
	let before = state(&h, p);
	let rev = visit(&h).revision;
	let result = apply(
		world(&mut h),
		ShopCommand::Buy {
			offer: 0,
			revision: rev,
			replace_slot: None,
		},
	);
	assert_eq!(
		result,
		Err(ShopReject::Insufficient {
			price: 42,
			have: 41
		})
	);
	assert_eq!(state(&h, p), before, "41 scrap: no state or RNG change");
	world(&mut h).resource_mut::<Scrap>().0 = 42;
	let result = apply(
		world(&mut h),
		ShopCommand::Buy {
			offer: 0,
			revision: rev,
			replace_slot: None,
		},
	);
	assert_eq!(
		result,
		Ok(ShopOutcome::Bought {
			offer: 0,
			price: 42
		})
	);
	assert_eq!(scrap(&h), 0);
	let loadout = h.app().world().get::<Loadout>(p).unwrap().clone();
	assert_eq!(loadout.slots[0].as_ref().unwrap().tier, 2);
	assert_eq!(visit(&h).offers[0], Offer::Sold);
	let again = apply(
		world(&mut h),
		ShopCommand::Buy {
			offer: 0,
			revision: rev + 1,
			replace_slot: None,
		},
	);
	assert_eq!(again, Err(ShopReject::NotForSale));
}

#[test]
fn stale_revision_is_rejected_without_effects() {
	let (mut h, p) = shop(1, 100);
	let before = state(&h, p);
	let stale = visit(&h).revision + 5;
	assert_eq!(
		apply(world(&mut h), ShopCommand::Refresh { revision: stale }),
		Err(ShopReject::StaleRevision)
	);
	assert_eq!(state(&h, p), before);
}

#[test]
fn new_weapon_fills_the_lowest_empty_slot_with_zero_energy() {
	let (mut h, p) = shop(1, 100);
	set_offers(
		&mut h,
		[
			Offer::Weapon {
				id: "wpn_slug".into(),
				tier: 1,
				price: 24,
			},
			Offer::Empty,
			Offer::Empty,
			Offer::Empty,
		],
	);
	let rev = visit(&h).revision;
	apply(
		world(&mut h),
		ShopCommand::Buy {
			offer: 0,
			revision: rev,
			replace_slot: None,
		},
	)
	.unwrap();
	let loadout = h.app().world().get::<Loadout>(p).unwrap();
	assert_eq!(loadout.slots[1].as_ref().unwrap().weapon_id, "wpn_slug");
	assert_eq!(loadout.slots[1].as_ref().unwrap().tier, 1);
	assert_eq!(loadout.energy.slots(), [3, 0, 0, 0]);
	assert_eq!(scrap(&h), 76);
}

#[test]
fn da09_replacement_cancel_changes_nothing_and_confirm_keeps_slot_energy() {
	let (mut h, p) = shop(1, 100);
	for (slot, id) in ["wpn_slug", "wpn_needle", "wpn_saw"].iter().enumerate() {
		equip(&mut h, p, slot + 1, id, 1);
	}
	{
		let mut l = h.app_mut().world_mut().get_mut::<Loadout>(p).unwrap();
		l.energy = omega_game::combat::energy::EnergyAllocation::new([2, 2, 1, 1]);
	}
	set_offers(
		&mut h,
		[
			Offer::Weapon {
				id: "wpn_hammer".into(),
				tier: 1,
				price: 26,
			},
			Offer::Empty,
			Offer::Empty,
			Offer::Empty,
		],
	);
	let before = state(&h, p);
	let rev = visit(&h).revision;
	let cancelled = apply(
		world(&mut h),
		ShopCommand::Buy {
			offer: 0,
			revision: rev,
			replace_slot: None,
		},
	);
	assert_eq!(cancelled, Err(ShopReject::NeedsReplacementSlot));
	assert_eq!(
		state(&h, p),
		before,
		"cancelling the prompt changes nothing"
	);
	let bad = apply(
		world(&mut h),
		ShopCommand::Buy {
			offer: 0,
			revision: rev,
			replace_slot: Some(9),
		},
	);
	assert_eq!(bad, Err(ShopReject::BadSlot));
	assert_eq!(state(&h, p), before);
	let done = apply(
		world(&mut h),
		ShopCommand::Buy {
			offer: 0,
			revision: rev,
			replace_slot: Some(2),
		},
	);
	assert_eq!(
		done,
		Ok(ShopOutcome::Replaced {
			offer: 0,
			slot: 2,
			price: 26
		})
	);
	let loadout = h.app().world().get::<Loadout>(p).unwrap();
	let slot = loadout.slots[2].as_ref().unwrap();
	assert_eq!((slot.weapon_id.as_str(), slot.tier), ("wpn_hammer", 1));
	assert_eq!(
		loadout.energy.slots(),
		[2, 2, 1, 1],
		"slot energy is preserved"
	);
	assert_eq!(scrap(&h), 74);
}

#[test]
fn da10_one_refresh_per_visit_and_rejections_keep_rng_and_scrap() {
	let (mut h, p) = shop(3, 40);
	let rev = visit(&h).revision;
	let rng_before = shop_rng(&h);
	let result = apply(world(&mut h), ShopCommand::Refresh { revision: rev });
	assert_eq!(result, Ok(ShopOutcome::Refreshed { price: 12 }));
	assert_eq!(scrap(&h), 28);
	assert!(
		shop_rng(&h) > rng_before,
		"a refresh consumes the shop stream"
	);
	assert!(visit(&h).refresh_used);
	let before = state(&h, p);
	let rev = visit(&h).revision;
	assert_eq!(
		apply(world(&mut h), ShopCommand::Refresh { revision: rev }),
		Err(ShopReject::RefreshUsed)
	);
	assert_eq!(state(&h, p), before);
	open_shop(world(&mut h), 4).unwrap();
	assert!(
		!visit(&h).refresh_used,
		"the next shop permits one new refresh"
	);
}

#[test]
fn refresh_without_funds_leaves_rng_and_offers_alone() {
	let (mut h, p) = shop(3, 11);
	let before = state(&h, p);
	let rev = visit(&h).revision;
	assert_eq!(
		apply(world(&mut h), ShopCommand::Refresh { revision: rev }),
		Err(ShopReject::Insufficient {
			price: 12,
			have: 11
		})
	);
	assert_eq!(state(&h, p), before);
}

#[test]
fn da02_energy_limits_and_empty_slots_reject_points() {
	let (mut h, p) = shop(1, 0);
	{
		let mut l = h.app_mut().world_mut().get_mut::<Loadout>(p).unwrap();
		l.slots = [
			l.slots[0].clone(),
			Some(EquippedWeapon {
				weapon_id: "wpn_slug".into(),
				tier: 1,
			}),
			Some(EquippedWeapon {
				weapon_id: "wpn_saw".into(),
				tier: 1,
			}),
			Some(EquippedWeapon {
				weapon_id: "wpn_fan".into(),
				tier: 1,
			}),
		];
		l.energy = omega_game::combat::energy::EnergyAllocation::new([3, 1, 1, 1]);
	}
	let before = state(&h, p);
	let rev = visit(&h).revision;
	assert_eq!(
		apply(
			world(&mut h),
			ShopCommand::AddEnergy {
				slot: 1,
				revision: rev
			}
		),
		Err(ShopReject::Energy(EnergyError::NoFreePoints))
	);
	assert_eq!(state(&h, p), before);
	apply(
		world(&mut h),
		ShopCommand::RemoveEnergy {
			slot: 1,
			revision: rev,
		},
	)
	.unwrap();
	let rev = visit(&h).revision;
	apply(
		world(&mut h),
		ShopCommand::AddEnergy {
			slot: 2,
			revision: rev,
		},
	)
	.unwrap();
	assert_eq!(
		h.app().world().get::<Loadout>(p).unwrap().energy.slots(),
		[3, 0, 2, 1]
	);
	let (mut h, _) = shop(1, 0);
	let rev = visit(&h).revision;
	assert_eq!(
		apply(
			world(&mut h),
			ShopCommand::AddEnergy {
				slot: 3,
				revision: rev
			}
		),
		Err(ShopReject::Energy(EnergyError::EmptySlot))
	);
}

#[test]
fn buying_an_item_applies_its_modifiers_without_healing() {
	let (mut h, p) = shop(1, 100);
	let content = h.app().world().resource::<ContentRes>().0.clone();
	let (id, item) = content
		.catalogs
		.items
		.iter()
		.find(|(_, i)| {
			i.modifiers
				.iter()
				.any(|m| matches!(m.stat, omega_game::content::model::StatId::MaxHp) && m.value > 0)
		})
		.expect("a max-HP item exists");
	set_offers(
		&mut h,
		[
			Offer::Item {
				id: id.clone(),
				price: price(item.base_price, 100, 1),
			},
			Offer::Empty,
			Offer::Empty,
			Offer::Empty,
		],
	);
	let hp_before = h.app().world().get::<Health>(p).unwrap().current;
	let max_before = h.app().world().get::<Health>(p).unwrap().max;
	let rev = visit(&h).revision;
	apply(
		world(&mut h),
		ShopCommand::Buy {
			offer: 0,
			revision: rev,
			replace_slot: None,
		},
	)
	.unwrap();
	let health = h.app().world().get::<Health>(p).unwrap();
	assert!(health.max > max_before);
	assert_eq!(health.current, hp_before, "raising max HP does not heal");
	assert!(h.app().world().resource::<RunBuild>().items.contains(id));
}
