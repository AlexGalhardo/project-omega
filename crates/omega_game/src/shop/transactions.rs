//! Atomic shop commands (SP-009): validate, stage, commit once.

use super::{Offer, RunBuild, ShopVisit, draw_offers, refresh_cost};
use crate::combat::actor::{
	ContentRes, EquippedWeapon, Health, Loadout, Player, PlayerStats, Scrap,
};
use crate::combat::energy::EnergyError;
use crate::combat::stats::effective_stats;
use crate::sim::RngStreams;
use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShopCommand {
	/// Buy `offer`; a full loadout needs `replace_slot` for a new weapon.
	Buy {
		offer: usize,
		revision: u64,
		replace_slot: Option<usize>,
	},
	Refresh {
		revision: u64,
	},
	AddEnergy {
		slot: usize,
		revision: u64,
	},
	RemoveEnergy {
		slot: usize,
		revision: u64,
	},
}

impl ShopCommand {
	fn revision(&self) -> u64 {
		match self {
			Self::Buy { revision, .. }
			| Self::Refresh { revision }
			| Self::AddEnergy { revision, .. }
			| Self::RemoveEnergy { revision, .. } => *revision,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShopOutcome {
	Bought {
		offer: usize,
		price: u32,
	},
	Replaced {
		offer: usize,
		slot: usize,
		price: u32,
	},
	Refreshed {
		price: u32,
	},
	EnergyChanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShopReject {
	NoShop,
	StaleRevision,
	BadOffer,
	NotForSale,
	Insufficient { price: u32, have: u32 },
	NeedsReplacementSlot,
	BadSlot,
	RefreshUsed,
	Energy(EnergyError),
	Content,
}

fn player_entity(world: &mut World) -> Result<Entity, ShopReject> {
	world
		.query_filtered::<Entity, With<Player>>()
		.iter(world)
		.next()
		.ok_or(ShopReject::NoShop)
}

fn player_loadout(world: &mut World) -> Result<Loadout, ShopReject> {
	let player = player_entity(world)?;
	world
		.get::<Loadout>(player)
		.cloned()
		.ok_or(ShopReject::NoShop)
}

/// Draws offers on a cloned stream and commits the stream only after the draw.
fn draw(world: &mut World, visit: u32) -> Result<[Offer; super::OFFER_COUNT], ShopReject> {
	let content = world.resource::<ContentRes>().0.clone();
	let build = world
		.get_resource::<RunBuild>()
		.cloned()
		.ok_or(ShopReject::NoShop)?;
	let loadout = player_loadout(world)?;
	let mut rng = world.resource::<RngStreams>().shop.clone();
	let offers = draw_offers(&content, &build, &loadout, visit, &mut rng);
	world.resource_mut::<RngStreams>().shop = rng;
	Ok(offers)
}

/// Enters a shop visit for completed wave `visit` and draws its first offers.
pub fn open_shop(world: &mut World, visit: u32) -> Result<(), ShopReject> {
	let offers = draw(world, visit)?;
	world.insert_resource(ShopVisit {
		visit,
		offers,
		revision: 0,
		refresh_used: false,
	});
	Ok(())
}

/// Applies one command. On `Err` nothing observable has changed.
pub fn apply(world: &mut World, command: ShopCommand) -> Result<ShopOutcome, ShopReject> {
	let visit = world
		.get_resource::<ShopVisit>()
		.cloned()
		.ok_or(ShopReject::NoShop)?;
	if command.revision() != visit.revision {
		return Err(ShopReject::StaleRevision);
	}
	let outcome = match command {
		ShopCommand::Buy {
			offer,
			replace_slot,
			..
		} => buy(world, &visit, offer, replace_slot)?,
		ShopCommand::Refresh { .. } => refresh(world, &visit)?,
		ShopCommand::AddEnergy { slot, .. } => energy(world, slot, true)?,
		ShopCommand::RemoveEnergy { slot, .. } => energy(world, slot, false)?,
	};
	world.resource_mut::<ShopVisit>().revision += 1;
	Ok(outcome)
}

fn energy(world: &mut World, slot: usize, add: bool) -> Result<ShopOutcome, ShopReject> {
	let player = player_entity(world)?;
	let mut loadout = world.get_mut::<Loadout>(player).ok_or(ShopReject::NoShop)?;
	let occupied = loadout.occupied();
	let next = if add {
		loadout.energy.increment(slot, occupied)
	} else {
		loadout.energy.decrement(slot, occupied)
	}
	.map_err(ShopReject::Energy)?;
	loadout.energy = next;
	Ok(ShopOutcome::EnergyChanged)
}

fn refresh(world: &mut World, visit: &ShopVisit) -> Result<ShopOutcome, ShopReject> {
	if visit.refresh_used {
		return Err(ShopReject::RefreshUsed);
	}
	let cost = refresh_cost(visit.visit);
	let have = world.resource::<Scrap>().0;
	if have < cost {
		return Err(ShopReject::Insufficient { price: cost, have });
	}
	let offers = draw(world, visit.visit)?;
	world.resource_mut::<Scrap>().0 = have - cost;
	let mut shop = world.resource_mut::<ShopVisit>();
	shop.offers = offers;
	shop.refresh_used = true;
	Ok(ShopOutcome::Refreshed { price: cost })
}

enum Staged {
	Item(String),
	Weapon(String, u8),
}

fn buy(
	world: &mut World,
	visit: &ShopVisit,
	index: usize,
	replace_slot: Option<usize>,
) -> Result<ShopOutcome, ShopReject> {
	let (price, staged) = match visit.offers.get(index).ok_or(ShopReject::BadOffer)? {
		Offer::Sold | Offer::Empty => return Err(ShopReject::NotForSale),
		Offer::Item { id, price } => (*price, Staged::Item(id.clone())),
		Offer::Weapon { id, tier, price } => (*price, Staged::Weapon(id.clone(), *tier)),
	};
	let have = world.resource::<Scrap>().0;
	if have < price {
		return Err(ShopReject::Insufficient { price, have });
	}
	let content = world.resource::<ContentRes>().0.clone();
	let player = player_entity(world)?;
	let mut loadout = player_loadout(world)?;
	let mut build = world
		.get_resource::<RunBuild>()
		.cloned()
		.ok_or(ShopReject::NoShop)?;
	let mut replaced_slot = None;
	match staged {
		Staged::Item(id) => {
			build.items.insert(id);
		}
		Staged::Weapon(id, tier) => {
			let owned = loadout
				.slots
				.iter()
				.position(|s| s.as_ref().is_some_and(|w| w.weapon_id == id));
			let target = match owned {
				Some(slot) => slot,
				None => match loadout.slots.iter().position(Option::is_none) {
					Some(empty) => empty,
					None => {
						let slot = replace_slot.ok_or(ShopReject::NeedsReplacementSlot)?;
						if slot >= loadout.slots.len() {
							return Err(ShopReject::BadSlot);
						}
						replaced_slot = Some(slot);
						slot
					}
				},
			};
			loadout.slots[target] = Some(EquippedWeapon {
				weapon_id: id,
				tier,
			});
		}
	}
	let character = content
		.catalogs
		.characters
		.get(&build.character_id)
		.ok_or(ShopReject::Content)?;
	let modifiers = std::iter::once(character.modifier).chain(
		build
			.items
			.iter()
			.filter_map(|id| content.catalogs.items.get(id))
			.flat_map(|item| item.modifiers.iter().copied()),
	);
	let stats = effective_stats(modifiers).map_err(|_| ShopReject::Content)?;

	world.resource_mut::<Scrap>().0 = have - price;
	world.insert_resource(build);
	world
		.get_mut::<Loadout>(player)
		.ok_or(ShopReject::NoShop)?
		.slots = loadout.slots;
	world
		.get_mut::<PlayerStats>(player)
		.ok_or(ShopReject::NoShop)?
		.0 = stats;
	if let Some(mut health) = world.get_mut::<Health>(player) {
		health.max = u32::try_from(stats.max_hp).unwrap_or(health.max);
		health.current = health.current.min(health.max);
	}
	world.resource_mut::<ShopVisit>().offers[index] = Offer::Sold;
	Ok(match replaced_slot {
		Some(slot) => ShopOutcome::Replaced {
			offer: index,
			slot,
			price,
		},
		None => ShopOutcome::Bought {
			offer: index,
			price,
		},
	})
}
