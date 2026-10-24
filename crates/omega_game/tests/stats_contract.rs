//! SP-006 acceptance: exact stat, fire-rate, damage, energy and regeneration rules.
//! Numbers come from the full content fixture, never retyped constants.

use omega_game::combat::energy::{EnergyAllocation, EnergyError};
use omega_game::combat::stats::*;
use omega_game::content::model::{Item, StatId, StatModifier, Weapon};
use omega_game::content::{Profile, load_dir};
use std::path::PathBuf;

fn catalogs() -> omega_game::content::Catalogs {
	let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/content/full");
	load_dir(&dir, Profile::Full)
		.expect("full fixture")
		.catalogs
}

fn weapon(id: &str) -> Weapon {
	catalogs().weapons[id].clone()
}

fn m(stat: StatId, value: i32) -> StatModifier {
	StatModifier { stat, value }
}

#[test]
fn ac1_fire_interval_worked_examples() {
	let bolt = weapon("wpn_bolt");
	assert_eq!(bolt.base_rate_centi, 200);
	// 2.0/s, 2 points, +25%: requested 4/s -> 15 ticks.
	assert_eq!(fire_interval_ticks(&bolt, 2, 25), Some(15));
	// Three points, no speed modifier: also 15.
	assert_eq!(fire_interval_ticks(&bolt, 3, 0), Some(15));
	// One point, +15%: requested 2.3/s -> 27 ticks.
	assert_eq!(fire_interval_ticks(&bolt, 1, 15), Some(27));
	// Zero energy cannot attack.
	assert_eq!(fire_interval_ticks(&bolt, 0, 0), None);
}

#[test]
fn ac1_hammer_animation_floor_and_rate_cap() {
	let hammer = weapon("wpn_hammer");
	assert_eq!(hammer.windup_ticks + hammer.recovery_ticks, 24);
	assert_eq!(fire_interval_ticks(&hammer, 3, 150), Some(24));
	// The 10 attacks/s cap: needle at 3 points and +150% requests far above 10.
	let needle = weapon("wpn_needle");
	assert_eq!(fire_interval_ticks(&needle, 3, 150), Some(6));
}

#[test]
fn ac1_damage_worked_examples() {
	let bolt = weapon("wpn_bolt");
	assert_eq!(bolt.base_damage, 12);
	assert_eq!(player_damage(bolt.base_damage, 2, 25, false, 0), Ok(21));
	assert_eq!(player_damage(bolt.base_damage, 2, 25, true, 20), Ok(26));
	assert_eq!(enemy_damage(18, 20), Ok(15));
	assert_eq!(player_damage(1, 1, -75, false, 80), Ok(1));
	assert_eq!(player_damage(1, 4, 0, false, 0), Err(StatError::Domain));
}

#[test]
fn ac1_hp_limits_from_character_and_clamps() {
	let c = catalogs();
	let patch = &c.characters["char_patch"];
	let stats = effective_stats([patch.modifier]).unwrap();
	assert_eq!(stats.max_hp, 120);
	assert_eq!(stats.crit_pct, 5);
	assert_eq!(stats.pickup_radius, 60);
	let hi = effective_stats([m(StatId::MaxHp, 1000)]).unwrap();
	assert_eq!(hi.max_hp, 300);
	let lo = effective_stats([m(StatId::MaxHp, -1000)]).unwrap();
	assert_eq!(lo.max_hp, 1);
	// Armor +70 and +20 sum to 90 before the single clamp to 80.
	let armor = effective_stats([m(StatId::Armor, 70), m(StatId::Armor, 20)]).unwrap();
	assert_eq!(armor.armor, 80);
	assert_eq!(
		effective_stats([m(StatId::Armor, i32::MAX), m(StatId::Armor, i32::MAX)])
			.unwrap()
			.armor,
		80
	);
}

fn permutation(len: usize, mut seed: u64) -> Vec<usize> {
	let mut order: Vec<usize> = (0..len).collect();
	for i in (1..len).rev() {
		seed = seed
			.wrapping_mul(6364136223846793005)
			.wrapping_add(1442695040888963407);
		order.swap(i, (seed >> 33) as usize % (i + 1));
	}
	order
}

#[test]
fn ac2_item_order_independence() {
	let c = catalogs();
	let items: Vec<&Item> = c.items.values().collect();
	let base: Vec<StatModifier> = items.iter().flat_map(|i| i.modifiers.clone()).collect();
	let expected = effective_stats(base.iter().copied()).unwrap();
	for seed in 0..200 {
		let order = permutation(items.len(), seed);
		let mods = order.iter().flat_map(|&i| items[i].modifiers.clone());
		assert_eq!(effective_stats(mods).unwrap(), expected, "seed {seed}");
	}
}

#[test]
fn ac3_energy_allocation_invariants() {
	let occupied = [true, true, true, false];
	let a = EnergyAllocation::new([3, 0, 0, 0]);
	assert_eq!(a.remainder(), 3);
	let a = a
		.increment(1, occupied)
		.unwrap()
		.increment(2, occupied)
		.unwrap();
	assert_eq!(a.slots(), [3, 1, 1, 0]);
	assert_eq!(a.remainder(), 1);
	let a = a.increment(1, occupied).unwrap();
	assert_eq!(a.remainder(), 0);
	// DA-02: no free point, increment rejected and value preserved.
	assert_eq!(a.increment(2, occupied), Err(EnergyError::NoFreePoints));
	assert_eq!(a.slots(), [3, 2, 1, 0]);
	// Empty slot rejects points; per-slot cap is 3.
	let b = EnergyAllocation::new([0, 0, 0, 0]);
	assert_eq!(b.increment(3, occupied), Err(EnergyError::EmptySlot));
	let b = b
		.increment(0, occupied)
		.unwrap()
		.increment(0, occupied)
		.unwrap()
		.increment(0, occupied)
		.unwrap();
	assert_eq!(b.increment(0, occupied), Err(EnergyError::SlotFull));
	assert_eq!(b.decrement(1, occupied), Err(EnergyError::AlreadyZero));
	assert_eq!(b.decrement(0, occupied).unwrap().slots(), [2, 0, 0, 0]);
	for slots in [[3, 3, 0, 0], [1, 1, 1, 1], [3, 0, 0, 0]] {
		let a = EnergyAllocation::new(slots);
		assert_eq!(
			a.slots().iter().map(|&p| u32::from(p)).sum::<u32>() + u32::from(a.remainder()),
			6
		);
	}
	assert!(!EnergyAllocation::new([0, 0, 0, 0]).any_powered(occupied));
	assert!(EnergyAllocation::new([0, 2, 0, 0]).any_powered(occupied));
	assert!(!EnergyAllocation::new([0, 0, 0, 2]).any_powered(occupied));
}

#[test]
fn ac4_regen_accumulator_and_death() {
	let mut regen = RegenAccumulator::default();
	let mut hp = 50;
	for tick in 1..=120 {
		hp = regen.tick(hp, 120, 5);
		assert_eq!(hp, if tick < 120 { 50 } else { 51 }, "tick {tick}");
	}
	// Dead actors never revive; accumulator stays clear.
	assert_eq!(regen.tick(0, 120, 50), 0);
	// Full HP clears the remainder.
	let mut regen = RegenAccumulator::default();
	regen.tick(10, 120, 5);
	assert_ne!(regen, RegenAccumulator::default());
	assert_eq!(regen.tick(120, 120, 5), 120);
	assert_eq!(regen, RegenAccumulator::default());
	// Heal never exceeds max HP.
	let mut regen = RegenAccumulator::default();
	let hp = (0..12).fold(119, |hp, _| regen.tick(hp, 120, 50));
	assert_eq!(hp, 120);
}
