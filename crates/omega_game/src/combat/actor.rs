//! Actor components and run setup shared by every combat system (SP-007).

use super::energy::EnergyAllocation;
use super::stats::{EffectiveStats, RegenAccumulator, StatError, effective_stats};
use super::weapons::WeaponTimers;
use crate::content::ContentSnapshot;
use crate::sim::IdAllocator;
use bevy::math::DVec2;
use bevy::prelude::*;
use omega_2d::geometry::Bounds2;
use std::sync::Arc;

/// Player hurtbox radius in LU (GDD-004).
pub const PLAYER_RADIUS_LU: f64 = 14.0;
/// Base movement speed in LU/s (GDD section 7.3).
pub const BASE_SPEED_LU_S: f64 = 180.0;

/// Validated content the run reads; replaced only between runs.
#[derive(Resource, Debug, Clone)]
pub struct ContentRes(pub Arc<ContentSnapshot>);

/// Playable rectangle of the current wave's arena.
#[derive(Resource, Debug, Clone, Copy)]
pub struct ArenaRes(pub Bounds2);

#[derive(Resource, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Scrap(pub u32);

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Pose2(pub DVec2);

/// Pose at the start of the tick, kept only so rendering can interpolate.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct PreviousPose2(pub DVec2);

/// Circular hurtbox radius in LU.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Body(pub f64);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Health {
	pub current: u32,
	pub max: u32,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Player;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerStats(pub EffectiveStats);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EquippedWeapon {
	pub weapon_id: String,
	pub tier: u8,
}

/// Four weapon slots and the six-point reactor allocation.
#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct Loadout {
	pub slots: [Option<EquippedWeapon>; 4],
	pub energy: EnergyAllocation,
}

impl Loadout {
	pub fn occupied(&self) -> [bool; 4] {
		std::array::from_fn(|i| self.slots[i].is_some())
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpawnError {
	UnknownCharacter,
	UnknownWeapon,
	UnknownEnemy,
	Stats(StatError),
}

/// Spawns the player for a new run: starting weapon in slot 1 at tier 1 with three points.
pub fn spawn_player(
	world: &mut World,
	character_id: &str,
	spawn: DVec2,
) -> Result<Entity, SpawnError> {
	let content = world.resource::<ContentRes>().0.clone();
	let character = content
		.catalogs
		.characters
		.get(character_id)
		.ok_or(SpawnError::UnknownCharacter)?;
	if !content
		.catalogs
		.weapons
		.contains_key(&character.starting_weapon_id)
	{
		return Err(SpawnError::UnknownWeapon);
	}
	let stats = effective_stats([character.modifier]).map_err(SpawnError::Stats)?;
	let hp = u32::try_from(stats.max_hp).map_err(|_| SpawnError::Stats(StatError::Domain))?;
	let id = world.resource_mut::<IdAllocator>().sim_id();
	let loadout = Loadout {
		slots: [
			Some(EquippedWeapon {
				weapon_id: character.starting_weapon_id.clone(),
				tier: 1,
			}),
			None,
			None,
			None,
		],
		energy: EnergyAllocation::new([3, 0, 0, 0]),
	};
	Ok(world
		.spawn((
			id,
			Player,
			Pose2(spawn),
			PreviousPose2(spawn),
			Body(PLAYER_RADIUS_LU),
			Health {
				current: hp,
				max: hp,
			},
			PlayerStats(stats),
			loadout,
			WeaponTimers::ready(),
			Invulnerability::default(),
			RegenAccumulator::default(),
		))
		.id())
}

/// Hostile marker with the content ID it came from.
#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct Enemy {
	pub id: String,
}

/// Unscaled-per-spawn values the combat rules read (armor, speed and reward do not scale).
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct EnemyStats {
	pub armor: u32,
	pub contact_damage: u32,
	pub scrap: u32,
	pub knockback_multiplier: f64,
	pub speed_lu_s: f64,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pickup {
	pub value: u32,
}

/// Global incoming-hit window (GDD section 7.3): a hit is accepted on or after this tick.
#[derive(Component, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Invulnerability {
	pub next_accept_tick: u64,
}

/// Spawns one enemy with its already wave-scaled HP and contact damage.
pub fn spawn_enemy(
	world: &mut World,
	enemy_id: &str,
	position: DVec2,
	hp: u32,
	contact_damage: u32,
) -> Result<Entity, SpawnError> {
	let content = world.resource::<ContentRes>().0.clone();
	let def = content
		.catalogs
		.enemies
		.get(enemy_id)
		.ok_or(SpawnError::UnknownEnemy)?;
	let id = world.resource_mut::<IdAllocator>().sim_id();
	Ok(world
		.spawn((
			id,
			Enemy { id: def.id.clone() },
			Pose2(position),
			PreviousPose2(position),
			Body(def.radius_lu),
			Health {
				current: hp,
				max: hp,
			},
			EnemyStats {
				armor: def.armor,
				contact_damage,
				scrap: def.scrap,
				knockback_multiplier: def.knockback_multiplier,
				speed_lu_s: def.speed_lu_s,
			},
		))
		.id())
}
