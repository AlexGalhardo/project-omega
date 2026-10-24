//! Shop eligibility, pricing, offer draws and atomic transactions (SP-009, GDD section 8).
//!
//! `ShopVisit` is the only mutable shop state. Every command validates first, stages the
//! result (including a cloned RNG stream) and commits once, so a rejection leaves scrap,
//! build, offers and RNG untouched.

mod transactions;

pub use transactions::{ShopCommand, ShopOutcome, ShopReject, apply, open_shop};

use crate::combat::actor::Loadout;
use crate::content::ContentSnapshot;
use crate::content::model::Availability;
use crate::sim::SimRng;
use bevy::prelude::*;
use std::collections::BTreeSet;

pub const OFFER_COUNT: usize = 4;
const MAX_TIER: u8 = 3;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Offer {
	/// `tier` is the tier the purchase installs (1 for a new weapon, owned tier + 1 for an upgrade).
	Weapon {
		id: String,
		tier: u8,
		price: u32,
	},
	Item {
		id: String,
		price: u32,
	},
	Sold,
	Empty,
}

#[derive(Resource, Debug, Clone, PartialEq, Eq)]
pub struct ShopVisit {
	/// Completed wave number `w` (1-9): drives inflation and refresh cost.
	pub visit: u32,
	pub offers: [Offer; OFFER_COUNT],
	/// Bumped by every committed command; commands carry the revision they were built from.
	pub revision: u64,
	pub refresh_used: bool,
}

/// Run-scoped build facts the shop reads and writes.
#[derive(Resource, Debug, Clone, PartialEq, Eq)]
pub struct RunBuild {
	pub character_id: String,
	pub items: BTreeSet<String>,
	/// Unlocks active in this run's snapshot; content unlocked mid-run is not added.
	pub unlocked: BTreeSet<String>,
}

/// `ceil(base * percent * (100 + 8 * (w - 1)) / 10_000)` with a single ceiling.
pub fn price(base: u32, percent: u32, visit: u32) -> u32 {
	let inflation = 100 + 8 * u64::from(visit.saturating_sub(1));
	let value = (u64::from(base) * u64::from(percent) * inflation).div_ceil(10_000);
	u32::try_from(value).unwrap_or(u32::MAX)
}

fn tier_percent(tier: u8) -> u32 {
	match tier {
		2 => 150,
		3 => 200,
		_ => 100,
	}
}

pub fn refresh_cost(visit: u32) -> u32 {
	8 + 2 * visit.saturating_sub(1)
}

fn available(unlocked: &BTreeSet<String>, availability: &Availability) -> bool {
	match availability {
		Availability::Initial => true,
		Availability::Unlock(id) => unlocked.contains(id),
	}
}

fn pick(pool: &mut Vec<Offer>, rng: &mut SimRng) -> Option<Offer> {
	if pool.is_empty() {
		return None;
	}
	let index = rng.below(u32::try_from(pool.len()).unwrap_or(u32::MAX)) as usize;
	Some(pool.remove(index))
}

/// Draws four offers: two weapons and two items uniformly without replacement, then
/// fills spare positions from the other category, then `Empty`. Pools are ordered by ID.
pub fn draw_offers(
	content: &ContentSnapshot,
	build: &RunBuild,
	loadout: &Loadout,
	visit: u32,
	rng: &mut SimRng,
) -> [Offer; OFFER_COUNT] {
	let mut weapons: Vec<Offer> = Vec::new();
	for (id, weapon) in &content.catalogs.weapons {
		if !available(&build.unlocked, &weapon.availability) {
			continue;
		}
		let owned = loadout
			.slots
			.iter()
			.flatten()
			.find(|slot| &slot.weapon_id == id)
			.map(|slot| slot.tier);
		let tier = match owned {
			Some(t) if t >= MAX_TIER => continue,
			Some(t) => t + 1,
			None => 1,
		};
		weapons.push(Offer::Weapon {
			id: id.clone(),
			tier,
			price: price(weapon.base_price, tier_percent(tier), visit),
		});
	}
	let mut items: Vec<Offer> = content
		.catalogs
		.items
		.iter()
		.filter(|(id, item)| {
			!build.items.contains(*id) && available(&build.unlocked, &item.availability)
		})
		.map(|(id, item)| Offer::Item {
			id: id.clone(),
			price: price(item.base_price, 100, visit),
		})
		.collect();
	let mut offers = [const { Offer::Empty }; OFFER_COUNT];
	let mut spare_weapon_slots: Vec<usize> = Vec::new();
	let mut spare_item_slots: Vec<usize> = Vec::new();
	for (i, slot) in offers.iter_mut().enumerate() {
		let (pool, spare) = if i < 2 {
			(&mut weapons, &mut spare_weapon_slots)
		} else {
			(&mut items, &mut spare_item_slots)
		};
		match pick(pool, rng) {
			Some(offer) => *slot = offer,
			None => spare.push(i),
		}
	}
	for i in spare_weapon_slots {
		if let Some(offer) = pick(&mut items, rng) {
			offers[i] = offer;
		}
	}
	for i in spare_item_slots {
		if let Some(offer) = pick(&mut weapons, rng) {
			offers[i] = offer;
		}
	}
	offers
}
