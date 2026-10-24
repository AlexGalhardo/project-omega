use crate::combat::actor::{Enemy, Health, Pickup, Player, Pose2};
use crate::sim::SimId;
use bevy::ecs::query::QueryState;
use bevy::math::DVec2;
use bevy::prelude::*;

/// What a player sees on screen during a wave. Nothing hidden (RNG, unspawned batches,
/// enemy HP) is exposed.
#[derive(Debug, Clone, Default)]
pub struct Observation {
	pub player: DVec2,
	pub hp: u32,
	pub max_hp: u32,
	pub enemies: Vec<DVec2>,
	pub pickups: Vec<DVec2>,
}

impl Observation {
	pub fn nearest_enemy(&self) -> Option<(DVec2, f64)> {
		self.enemies
			.iter()
			.map(|&e| (e, e.distance(self.player)))
			.min_by(|a, b| a.1.total_cmp(&b.1))
	}

	pub fn nearest_pickup(&self) -> Option<(DVec2, f64)> {
		self.pickups
			.iter()
			.map(|&p| (p, p.distance(self.player)))
			.min_by(|a, b| a.1.total_cmp(&b.1))
	}
}

/// Read-only world access with cached query state.
pub struct Observer {
	player: QueryState<(&'static Pose2, &'static Health), With<Player>>,
	enemies: QueryState<&'static Pose2, With<Enemy>>,
	pickups: QueryState<&'static Pose2, With<Pickup>>,
	kinds: QueryState<(&'static SimId, &'static Enemy)>,
}

impl Observer {
	pub fn new(world: &mut World) -> Self {
		Self {
			player: world.query_filtered(),
			enemies: world.query_filtered(),
			pickups: world.query_filtered(),
			kinds: world.query(),
		}
	}

	pub fn observe(&mut self, world: &World) -> Observation {
		let mut seen = Observation::default();
		if let Some((pose, health)) = self.player.iter(world).next() {
			seen.player = pose.0;
			seen.hp = health.current;
			seen.max_hp = health.max;
		}
		seen.enemies = self.enemies.iter(world).map(|p| p.0).collect();
		seen.pickups = self.pickups.iter(world).map(|p| p.0).collect();
		// Query order follows archetype storage; sort so policies never see an arbitrary order.
		let key = |v: &DVec2| (v.x.to_bits(), v.y.to_bits());
		seen.enemies.sort_by_key(key);
		seen.pickups.sort_by_key(key);
		seen
	}

	/// Content ID of the live enemy with simulation ID `id`.
	pub fn enemy_kind(&mut self, world: &World, id: u64) -> Option<String> {
		self.kinds
			.iter(world)
			.find(|(sim, _)| sim.0 == id)
			.map(|(_, enemy)| enemy.id.clone())
	}
}
