//! Bot policies: producers of `TickInput` and shop commands from player-visible facts.

use super::observe::Observation;
use crate::combat::actor::Loadout;
use crate::content::ContentSnapshot;
use crate::content::model::StatId;
use crate::input::InputBuffer;
use crate::qa::bot::{Bot, BotProfile};
use crate::qa::{InputDriver, capture};
use crate::shop::{Offer, ShopVisit};
use crate::sim::{AimInput, RunSeed, TickInput};
use bevy::math::DVec2;
use bevy::prelude::World;
use rand_chacha::ChaCha8Rng;
use rand_chacha::rand_core::{Rng, SeedableRng};
use serde::Serialize;
use std::fmt;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Policy {
	Idle,
	Kiter,
	Greedy,
	Defensive,
	Random,
	HumanLike,
}

impl Policy {
	pub const ALL: [Policy; 6] = [
		Self::Idle,
		Self::Kiter,
		Self::Greedy,
		Self::Defensive,
		Self::Random,
		Self::HumanLike,
	];

	pub fn name(self) -> &'static str {
		match self {
			Self::Idle => "idle",
			Self::Kiter => "kiter",
			Self::Greedy => "greedy",
			Self::Defensive => "defensive",
			Self::Random => "random",
			Self::HumanLike => "human_like",
		}
	}
}

impl fmt::Display for Policy {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(self.name())
	}
}

impl FromStr for Policy {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, String> {
		Self::ALL
			.into_iter()
			.find(|p| p.name() == s)
			.ok_or_else(|| {
				format!(
					"unknown policy {s}; expected idle, kiter, greedy, defensive, random or human_like"
				)
			})
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShopAction {
	Buy {
		offer: usize,
		replace_slot: Option<usize>,
	},
	Refresh,
	Energy {
		slot: usize,
		add: bool,
	},
}

/// What the shop screen shows.
pub struct ShopObs<'a> {
	pub scrap: u32,
	pub visit: &'a ShopVisit,
	pub loadout: &'a Loadout,
}

/// Purchase and reactor preferences of a scoring policy.
struct Style {
	/// Score per stat unit, in `StatId` order: hp, armor, damage, attack speed, move
	/// speed, crit, regen, pickup radius.
	stat: [f64; 8],
	new_weapon: f64,
	upgrade: f64,
	/// Only weapons reaching at least this far are bought as new weapons.
	min_new_range: f64,
	/// Most points put on one weapon.
	energy_cap: u8,
}

fn style(policy: Policy) -> Style {
	match policy {
		Policy::Greedy => Style {
			stat: [0.1, 0.3, 3.0, 3.0, 0.5, 1.5, 0.3, 0.15],
			new_weapon: 60.0,
			upgrade: 40.0,
			min_new_range: 0.0,
			energy_cap: 3,
		},
		Policy::Defensive => Style {
			stat: [0.5, 3.0, 0.5, 0.5, 1.0, 0.2, 4.0, 0.1],
			new_weapon: 10.0,
			upgrade: 25.0,
			min_new_range: 0.0,
			energy_cap: 2,
		},
		Policy::Kiter => Style {
			stat: [0.2, 0.5, 2.0, 2.0, 2.5, 1.0, 1.0, 1.0],
			new_weapon: 40.0,
			upgrade: 30.0,
			min_new_range: 200.0,
			energy_cap: 3,
		},
		_ => Style {
			stat: [0.3, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0, 0.5],
			new_weapon: 45.0,
			upgrade: 30.0,
			min_new_range: 0.0,
			energy_cap: 3,
		},
	}
}

const SHOP_ACTION_CAP: u32 = 40;
const REFRESH_MARGIN: u32 = 16;

struct Human {
	bot: Bot,
	buffer: InputBuffer,
}

pub struct Agent {
	policy: Policy,
	rng: ChaCha8Rng,
	content: Arc<ContentSnapshot>,
	seed: RunSeed,
	seed_index: u64,
	min: DVec2,
	max: DVec2,
	heading: DVec2,
	heading_left: u32,
	human: Option<Human>,
	shop_steps: u32,
}

impl Agent {
	/// Input style within a policy (human-like players alternate profiles per seed).
	pub fn style_name(&self) -> Option<String> {
		(self.policy == Policy::HumanLike).then(|| human_profile(self.seed_index).to_string())
	}
}

fn human_profile(seed_index: u64) -> BotProfile {
	// Players differ in style; alternate the QA bot's cautious and aggressive profiles.
	if seed_index.is_multiple_of(2) {
		BotProfile::Cautious
	} else {
		BotProfile::Aggressive
	}
}

impl Agent {
	pub fn new(
		policy: Policy,
		content: Arc<ContentSnapshot>,
		seed: RunSeed,
		seed_index: u64,
		bounds: (DVec2, DVec2),
	) -> Self {
		let mut bytes = seed.0;
		bytes[29] ^= 0xBA ^ policy as u8;
		Self {
			policy,
			rng: ChaCha8Rng::from_seed(bytes),
			content,
			seed,
			seed_index,
			min: bounds.0,
			max: bounds.1,
			heading: DVec2::ZERO,
			heading_left: 0,
			human: None,
			shop_steps: 0,
		}
	}

	fn unit(&mut self) -> f64 {
		f64::from(self.rng.next_u32()) / 4_294_967_296.0
	}

	fn below(&mut self, n: usize) -> usize {
		(self.rng.next_u32() as usize) % n.max(1)
	}

	/// The human-like bot restarts every wave so it never reacts to the previous arena.
	pub fn begin_wave(&mut self, wave: u32) {
		if self.policy != Policy::HumanLike {
			return;
		}
		let mut bytes = self.seed.0;
		bytes[28] ^= wave as u8;
		let profile = human_profile(self.seed_index);
		self.human = Some(Human {
			bot: Bot::new(profile, &RunSeed(bytes), u64::MAX),
			buffer: InputBuffer::default(),
		});
	}

	pub fn begin_shop(&mut self) {
		self.shop_steps = 0;
	}

	pub fn tick(&mut self, world: &mut World, obs: &Observation) -> TickInput {
		let movement = match self.policy {
			Policy::Idle => DVec2::ZERO,
			Policy::HumanLike => return self.human_tick(world, obs),
			Policy::Kiter => self.kite(obs, 300.0, true),
			Policy::Defensive => self.kite(obs, 380.0, false),
			Policy::Greedy => self.greedy(obs),
			Policy::Random => self.wander(),
		};
		TickInput {
			movement: movement.normalize_or_zero(),
			aim: AimInput::Auto,
		}
	}

	fn human_tick(&mut self, world: &mut World, obs: &Observation) -> TickInput {
		let Some(human) = self.human.as_mut() else {
			return TickInput::default();
		};
		let snapshot = capture(world, [1280, 720]);
		let step = human.bot.next(&snapshot);
		human.buffer.begin_frame(step.frame);
		human.buffer.sample_tick(obs.player).input
	}

	fn wall_push(&self, at: DVec2) -> DVec2 {
		const MARGIN: f64 = 110.0;
		let push = |distance: f64| ((MARGIN - distance) / MARGIN).max(0.0);
		DVec2::new(
			push(at.x - self.min.x) - push(self.max.x - at.x),
			push(at.y - self.min.y) - push(self.max.y - at.y),
		)
	}

	/// Moves away from nearby enemies, slides along walls, and (when `collect`) walks to
	/// scrap while nothing is close.
	fn kite(&mut self, obs: &Observation, radius: f64, collect: bool) -> DVec2 {
		let mut away = DVec2::ZERO;
		for &enemy in &obs.enemies {
			let offset = obs.player - enemy;
			let distance = offset.length();
			if distance < radius && distance > 1e-6 {
				away += offset / distance * ((radius - distance) / radius);
			}
		}
		if away.length() > 0.05 {
			let walls = self.wall_push(obs.player) * 1.5;
			let steer = away + walls;
			// Pinned against a wall: run along it instead of into the corner.
			if steer.length() < 0.2 && away.length() > 0.3 {
				return DVec2::new(-away.y, away.x);
			}
			return steer;
		}
		if collect && let Some((pickup, _)) = obs.nearest_pickup() {
			return pickup - obs.player;
		}
		let center = (self.min + self.max) / 2.0;
		if obs.player.distance(center) > 200.0 {
			return center - obs.player;
		}
		DVec2::ZERO
	}

	/// Chases scrap, then walks into weapon range of the nearest enemy and stands there.
	fn greedy(&mut self, obs: &Observation) -> DVec2 {
		if let Some((pickup, distance)) = obs.nearest_pickup()
			&& distance < 350.0
		{
			return pickup - obs.player;
		}
		match obs.nearest_enemy() {
			Some((enemy, distance)) if distance > 60.0 => enemy - obs.player,
			_ => DVec2::ZERO,
		}
	}

	fn wander(&mut self) -> DVec2 {
		if self.heading_left == 0 {
			self.heading_left = 20 + self.below(50) as u32;
			self.heading = if self.unit() < 0.25 {
				DVec2::ZERO
			} else {
				let angle = self.unit() * std::f64::consts::TAU;
				DVec2::new(angle.cos(), angle.sin())
			};
		}
		self.heading_left -= 1;
		self.heading
	}

	fn style_score(&self, style: &Style, offer: &Offer, loadout: &Loadout) -> f64 {
		match offer {
			Offer::Weapon { id, tier, .. } => {
				if *tier > 1 {
					return style.upgrade;
				}
				let range = self
					.content
					.catalogs
					.weapons
					.get(id)
					.map_or(0.0, |w| w.range_lu);
				let free = loadout.slots.iter().any(Option::is_none);
				if range >= style.min_new_range && free {
					style.new_weapon
				} else {
					0.0
				}
			}
			Offer::Item { id, .. } => self.content.catalogs.items.get(id).map_or(0.0, |item| {
				item.modifiers
					.iter()
					.map(|m| style.stat[stat_index(m.stat)] * f64::from(m.value))
					.sum()
			}),
			Offer::Sold | Offer::Empty => 0.0,
		}
	}

	fn target_energy(&self, style: &Style, loadout: &Loadout) -> [u8; 4] {
		let mut order: Vec<usize> = (0..4).filter(|&i| loadout.slots[i].is_some()).collect();
		let reach = |slot: usize| {
			loadout.slots[slot]
				.as_ref()
				.and_then(|w| self.content.catalogs.weapons.get(&w.weapon_id))
				.map_or(0.0, |w| w.range_lu)
		};
		if self.policy != Policy::Greedy {
			order.sort_by(|&a, &b| reach(b).total_cmp(&reach(a)));
		}
		let mut left = 6u8;
		let mut target = [0u8; 4];
		for slot in order {
			let give = style.energy_cap.min(left);
			target[slot] = give;
			left -= give;
		}
		target
	}

	/// Next shop command, or `None` when the bot is done and starts the wave.
	pub fn shop_action(&mut self, shop: &ShopObs) -> Option<ShopAction> {
		if self.policy == Policy::Idle || self.shop_steps >= SHOP_ACTION_CAP {
			return None;
		}
		self.shop_steps += 1;
		if self.policy == Policy::Random {
			return self.random_action(shop);
		}
		let style = style(self.policy);
		let mut best: Option<(usize, f64)> = None;
		for (index, offer) in shop.visit.offers.iter().enumerate() {
			let price = match offer {
				Offer::Weapon { price, .. } | Offer::Item { price, .. } => *price,
				Offer::Sold | Offer::Empty => continue,
			};
			let score = self.style_score(&style, offer, shop.loadout);
			if price <= shop.scrap && score > 0.0 && best.is_none_or(|(_, s)| score > s) {
				best = Some((index, score));
			}
		}
		if let Some((offer, _)) = best {
			return Some(ShopAction::Buy {
				offer,
				replace_slot: self.replace_slot(shop),
			});
		}
		let cost = crate::shop::refresh_cost(shop.visit.visit);
		if !shop.visit.refresh_used && shop.scrap >= cost + REFRESH_MARGIN {
			return Some(ShopAction::Refresh);
		}
		let target = self.target_energy(&style, shop.loadout);
		let now = shop.loadout.energy.slots();
		if let Some(slot) = (0..4).find(|&i| now[i] > target[i]) {
			return Some(ShopAction::Energy { slot, add: false });
		}
		if shop.loadout.energy.remainder() > 0
			&& let Some(slot) = (0..4).find(|&i| now[i] < target[i])
		{
			return Some(ShopAction::Energy { slot, add: true });
		}
		None
	}

	/// Full loadout: replace the lowest-tier slot.
	fn replace_slot(&self, shop: &ShopObs) -> Option<usize> {
		if shop.loadout.slots.iter().any(Option::is_none) {
			return None;
		}
		(0..4).min_by_key(|&i| shop.loadout.slots[i].as_ref().map_or(0, |w| w.tier))
	}

	fn random_action(&mut self, shop: &ShopObs) -> Option<ShopAction> {
		let mut legal: Vec<Option<ShopAction>> = vec![None];
		for (offer, entry) in shop.visit.offers.iter().enumerate() {
			if let Offer::Weapon { price, .. } | Offer::Item { price, .. } = entry
				&& *price <= shop.scrap
			{
				legal.push(Some(ShopAction::Buy {
					offer,
					replace_slot: self.replace_slot(shop),
				}));
			}
		}
		if !shop.visit.refresh_used && shop.scrap >= crate::shop::refresh_cost(shop.visit.visit) {
			legal.push(Some(ShopAction::Refresh));
		}
		let occupied = shop.loadout.occupied();
		let points = shop.loadout.energy.slots();
		for slot in 0..4 {
			if occupied[slot] && points[slot] < 3 && shop.loadout.energy.remainder() > 0 {
				legal.push(Some(ShopAction::Energy { slot, add: true }));
			}
			if occupied[slot] && points[slot] > 0 {
				legal.push(Some(ShopAction::Energy { slot, add: false }));
			}
		}
		let pick = self.below(legal.len());
		legal[pick]
	}
}

fn stat_index(stat: StatId) -> usize {
	match stat {
		StatId::MaxHp => 0,
		StatId::Armor => 1,
		StatId::DamagePct => 2,
		StatId::AttackSpeedPct => 3,
		StatId::MoveSpeedPct => 4,
		StatId::CritPct => 5,
		StatId::RegenTenths => 6,
		StatId::PickupRadius => 7,
	}
}
