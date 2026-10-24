//! Shop comparison text (GDD section 15.1): actual damage and fire rate, rates at every
//! point count for an empty slot, item modifiers and replacement details.

use crate::combat::actor::Loadout;
use crate::combat::stats::{EffectiveStats, fire_interval_ticks, player_damage};
use crate::content::ContentSnapshot;
use crate::content::model::{StatId, StatModifier, Weapon};
use crate::shop::Offer;
use crate::sim::TICKS_PER_SECOND;

pub fn rate_text(weapon: &Weapon, points: u8, attack_speed_pct: i32) -> String {
	if points == 0 {
		return "unpowered".to_string();
	}
	match fire_interval_ticks(weapon, points, attack_speed_pct) {
		Some(ticks) => format!("{:.2}/s", f64::from(TICKS_PER_SECOND) / f64::from(ticks)),
		None => "unpowered".to_string(),
	}
}

fn damage_of(weapon: &Weapon, tier: u8, stats: &EffectiveStats) -> u32 {
	player_damage(weapon.base_damage, tier, stats.damage_pct, false, 0).unwrap_or(0)
}

pub fn stat_name(stat: StatId) -> &'static str {
	match stat {
		StatId::MaxHp => "Max HP",
		StatId::Armor => "Armor",
		StatId::DamagePct => "Damage %",
		StatId::AttackSpeedPct => "Attack speed %",
		StatId::MoveSpeedPct => "Move speed %",
		StatId::CritPct => "Crit %",
		StatId::RegenTenths => "Regen (tenths HP/s)",
		StatId::PickupRadius => "Pickup radius LU",
	}
}

pub fn modifier_text(modifier: &StatModifier) -> String {
	format!("{} {:+}", stat_name(modifier.stat), modifier.value)
}

fn rates_by_points(weapon: &Weapon, stats: &EffectiveStats) -> String {
	(1..=3)
		.map(|p| format!("{p} pt {}", rate_text(weapon, p, stats.attack_speed_pct)))
		.collect::<Vec<_>>()
		.join(", ")
}

/// Detail lines for one offer, from the player's actual stats and loadout.
pub fn offer_detail(
	content: &ContentSnapshot,
	loadout: &Loadout,
	stats: &EffectiveStats,
	offer: &Offer,
) -> Vec<String> {
	match offer {
		Offer::Weapon { id, tier, .. } => {
			let Some(weapon) = content.catalogs.weapons.get(id) else {
				return Vec::new();
			};
			let slots = loadout.energy.slots();
			let owned = loadout
				.slots
				.iter()
				.position(|s| s.as_ref().is_some_and(|w| &w.weapon_id == id));
			let damage_new = damage_of(weapon, *tier, stats);
			match owned {
				Some(slot) => {
					let damage_old = damage_of(weapon, tier.saturating_sub(1).max(1), stats);
					let points = slots[slot];
					vec![format!(
						"Slot {}: damage {damage_old} -> {damage_new}, range {} LU, {} at {points} pt (rate changes only with energy)",
						slot + 1,
						weapon.range_lu,
						rate_text(weapon, points, stats.attack_speed_pct)
					)]
				}
				None => {
					let target = loadout.slots.iter().position(Option::is_none);
					let place = match target {
						Some(slot) => format!("goes to slot {} with 0 pt", slot + 1),
						None => {
							"needs you to pick a weapon to replace (it inherits that slot's energy)"
								.to_string()
						}
					};
					vec![
						format!("Damage {damage_new}, range {} LU, {place}", weapon.range_lu),
						format!("Rate: {}", rates_by_points(weapon, stats)),
					]
				}
			}
		}
		Offer::Item { id, .. } => content
			.catalogs
			.items
			.get(id)
			.map_or_else(Vec::new, |item| {
				vec![
					item.modifiers
						.iter()
						.map(modifier_text)
						.collect::<Vec<_>>()
						.join(", "),
				]
			}),
		Offer::Sold | Offer::Empty => Vec::new(),
	}
}

/// One line per slot for the replacement prompt: what is lost and what arrives.
pub fn replacement_lines(
	content: &ContentSnapshot,
	loadout: &Loadout,
	stats: &EffectiveStats,
	incoming: &Offer,
) -> Vec<String> {
	let Offer::Weapon { id, .. } = incoming else {
		return Vec::new();
	};
	let Some(new_weapon) = content.catalogs.weapons.get(id) else {
		return Vec::new();
	};
	let points = loadout.energy.slots();
	loadout
		.slots
		.iter()
		.enumerate()
		.filter_map(|(i, slot)| {
			let slot = slot.as_ref()?;
			let old = content.catalogs.weapons.get(&slot.weapon_id)?;
			Some(format!(
				"Slot {}: lose {} tier {} ({} at {} pt); gain {} tier 1 ({} at {} pt)",
				i + 1,
				old.display_name,
				slot.tier,
				rate_text(old, points[i], stats.attack_speed_pct),
				points[i],
				new_weapon.display_name,
				rate_text(new_weapon, points[i], stats.attack_speed_pct),
				points[i],
			))
		})
		.collect()
}
