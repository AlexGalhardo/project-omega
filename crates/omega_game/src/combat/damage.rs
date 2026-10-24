//! Hit candidates produced by Collide. Damage, death and healing resolution is GAME-P2-005.

use super::weapons::AttackSnapshot;
use crate::sim::{AttackId, SimId};
use bevy::math::DVec2;
use bevy::prelude::*;

/// One player-attack hit on one enemy, decided in Collide and applied in Resolve in
/// attack-then-target order.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hit {
	pub attack: AttackId,
	pub target: Entity,
	pub target_id: SimId,
	pub snapshot: AttackSnapshot,
	pub origin: DVec2,
	pub direction: DVec2,
}

/// Candidates of the current tick; cleared at the start of Collide.
#[derive(Resource, Debug, Default)]
pub struct HitCandidates(pub Vec<Hit>);
