//! Pure stat, fire-rate, damage and regeneration evaluators (GDD-006/007, CR-002).
//! All arithmetic is checked integer math; floors and ceilings apply exactly once.

use crate::content::model::{StatId, StatModifier, Weapon};
use bevy::prelude::Component;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatError {
	Overflow,
	/// An input outside its documented domain (for example a tier above 3).
	Domain,
}

/// Fully summed and clamped player stats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EffectiveStats {
	pub max_hp: i32,
	pub armor: i32,
	pub damage_pct: i32,
	pub attack_speed_pct: i32,
	pub move_speed_pct: i32,
	pub crit_pct: i32,
	/// Tenths of HP per second.
	pub regen_tenths: i32,
	pub pickup_radius: i32,
}

/// (base, min, max) per stat, GDD-006.
const fn bounds(stat: StatId) -> (i64, i64, i64) {
	match stat {
		StatId::MaxHp => (100, 1, 300),
		StatId::Armor => (0, 0, 80),
		StatId::DamagePct => (0, -75, 200),
		StatId::AttackSpeedPct => (0, -50, 150),
		StatId::MoveSpeedPct => (0, -40, 80),
		StatId::CritPct => (5, 0, 60),
		StatId::RegenTenths => (0, 0, 50),
		StatId::PickupRadius => (60, 24, 180),
	}
}

const ALL: [StatId; 8] = [
	StatId::MaxHp,
	StatId::Armor,
	StatId::DamagePct,
	StatId::AttackSpeedPct,
	StatId::MoveSpeedPct,
	StatId::CritPct,
	StatId::RegenTenths,
	StatId::PickupRadius,
];

/// Sums base plus every modifier, then clamps once. Order of `modifiers` is irrelevant.
pub fn effective_stats(
	modifiers: impl IntoIterator<Item = StatModifier>,
) -> Result<EffectiveStats, StatError> {
	let mut sums = ALL.map(|s| bounds(s).0);
	for modifier in modifiers {
		let slot = ALL
			.iter()
			.position(|&s| s == modifier.stat)
			.ok_or(StatError::Domain)?;
		sums[slot] = sums[slot]
			.checked_add(i64::from(modifier.value))
			.ok_or(StatError::Overflow)?;
	}
	let at = |stat: StatId| {
		let (_, lo, hi) = bounds(stat);
		let slot = ALL.iter().position(|&s| s == stat).expect("listed stat");
		sums[slot].clamp(lo, hi) as i32
	};
	Ok(EffectiveStats {
		max_hp: at(StatId::MaxHp),
		armor: at(StatId::Armor),
		damage_pct: at(StatId::DamagePct),
		attack_speed_pct: at(StatId::AttackSpeedPct),
		move_speed_pct: at(StatId::MoveSpeedPct),
		crit_pct: at(StatId::CritPct),
		regen_tenths: at(StatId::RegenTenths),
		pickup_radius: at(StatId::PickupRadius),
	})
}

/// Energy rate multiplier `E(p)` as integer percent.
const ENERGY_PCT: [u64; 4] = [0, 100, 160, 200];

/// Fire interval in ticks (GDD-007, CR-002); `None` when the weapon has no energy.
/// `attack_speed_pct` is an already-clamped effective stat.
pub fn fire_interval_ticks(weapon: &Weapon, points: u8, attack_speed_pct: i32) -> Option<u32> {
	let energy = *ENERGY_PCT.get(usize::from(points))?;
	if energy == 0 {
		return None;
	}
	let speed = u64::try_from(100 + i64::from(attack_speed_pct)).ok()?;
	let n = u64::from(weapon.base_rate_centi)
		.checked_mul(energy)?
		.checked_mul(speed)?;
	let by_rate = 60_000_000u64.div_ceil(n.clamp(1, 10_000_000));
	let floor = u64::from(weapon.windup_ticks) + u64::from(weapon.recovery_ticks);
	u32::try_from(by_rate.max(floor).max(1)).ok()
}

const TIER_PCT: [u64; 3] = [100, 140, 190];

/// Player-weapon hit damage (GDD §7.2): one floor, minimum 1.
pub fn player_damage(
	base_damage: u32,
	tier: u8,
	damage_pct: i32,
	crit: bool,
	target_armor: u32,
) -> Result<u32, StatError> {
	let tier_pct = *TIER_PCT
		.get(usize::from(tier).wrapping_sub(1))
		.ok_or(StatError::Domain)?;
	let damage_factor =
		u64::try_from(100 + i64::from(damage_pct)).map_err(|_| StatError::Domain)?;
	let crit_factor = if crit { 150 } else { 100 };
	let numerator = u64::from(base_damage)
		.checked_mul(tier_pct)
		.and_then(|v| v.checked_mul(damage_factor))
		.and_then(|v| v.checked_mul(crit_factor))
		.and_then(|v| v.checked_mul(100))
		.ok_or(StatError::Overflow)?;
	let denominator = 1_000_000u64 * (100 + u64::from(target_armor));
	u32::try_from((numerator / denominator).max(1)).map_err(|_| StatError::Overflow)
}

/// Damage an armor value leaves of an authored raw hit (enemy attacks, no tier or crit).
pub fn enemy_damage(raw: u32, target_armor: u32) -> Result<u32, StatError> {
	let scaled = u64::from(raw) * 100 / (100 + u64::from(target_armor));
	u32::try_from(scaled.max(1)).map_err(|_| StatError::Overflow)
}

/// Exact fractional regeneration, stored as 1/600 HP units (tenths of HP/s over 60 ticks).
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RegenAccumulator(u32);

impl RegenAccumulator {
	const UNIT: u32 = 600;

	/// One tick of regeneration; returns the new HP. Death is final, full HP clears the remainder.
	pub fn tick(&mut self, hp: u32, max_hp: u32, regen_tenths: i32) -> u32 {
		if hp == 0 || hp >= max_hp {
			*self = Self::default();
			return hp;
		}
		self.0 += u32::try_from(regen_tenths).unwrap_or(0);
		let healed = self.0 / Self::UNIT;
		self.0 %= Self::UNIT;
		let hp = (hp + healed).min(max_hp);
		if hp == max_hp {
			*self = Self::default();
		}
		hp
	}
}
