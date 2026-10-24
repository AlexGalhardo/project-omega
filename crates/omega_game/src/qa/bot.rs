//! Seeded human-like input source (ROADMAP rule 1.11). It only produces frames and
//! window commands for the shared input buffer; it never touches the simulation.

use super::{Command, InputDriver, LoadoutFacts, RunFacts, ShopFacts, Snapshot, Step, arena};
use crate::input::{Edge, FrameInput};
use crate::sim::RunSeed;
use crate::ui::UiAction;
use bevy::math::DVec2;
use rand_chacha::ChaCha8Rng;
use rand_chacha::rand_core::{Rng, SeedableRng};
use serde::Serialize;
use std::collections::{BTreeSet, VecDeque};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum BotProfile {
	Cautious,
	Aggressive,
	Chaotic,
}

impl BotProfile {
	pub const ALL: [BotProfile; 3] = [Self::Cautious, Self::Aggressive, Self::Chaotic];

	fn name(self) -> &'static str {
		match self {
			Self::Cautious => "cautious",
			Self::Aggressive => "aggressive",
			Self::Chaotic => "chaotic",
		}
	}

	fn tuning(self) -> Tuning {
		match self {
			Self::Cautious => Tuning {
				reaction: (14, 36),
				aim_error: 0.10,
				hesitate: 0.004,
				mistake: 0.01,
				shop_mistake: 0.08,
				menu_act: 0.05,
				abuse: 0.0005,
				manual_aim: 0.3,
				abandon_ask: 0.0,
				danger: 260.0,
			},
			Self::Aggressive => Tuning {
				reaction: (8, 22),
				aim_error: 0.22,
				hesitate: 0.0008,
				mistake: 0.02,
				shop_mistake: 0.15,
				menu_act: 0.12,
				abuse: 0.0008,
				manual_aim: 0.7,
				abandon_ask: 0.001,
				danger: 60.0,
			},
			Self::Chaotic => Tuning {
				reaction: (4, 60),
				aim_error: 0.6,
				hesitate: 0.01,
				mistake: 0.12,
				shop_mistake: 0.3,
				menu_act: 0.4,
				abuse: 0.01,
				manual_aim: 0.5,
				abandon_ask: 0.006,
				danger: 150.0,
			},
		}
	}
}

impl fmt::Display for BotProfile {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(self.name())
	}
}

impl FromStr for BotProfile {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, String> {
		Self::ALL
			.into_iter()
			.find(|p| p.name() == s)
			.ok_or_else(|| {
				format!("unknown bot profile {s}; expected cautious, aggressive or chaotic")
			})
	}
}

struct Tuning {
	/// Frames between what happens and what the bot reacts to.
	reaction: (u32, u32),
	/// Aim error amplitude in radians.
	aim_error: f64,
	hesitate: f64,
	mistake: f64,
	/// Per shop turn chance of a careless request (unaffordable buy, refresh spam, zero-energy start).
	shop_mistake: f64,
	menu_act: f64,
	/// Per-frame chance to start pause, focus or resize abuse.
	abuse: f64,
	manual_aim: f64,
	/// Per paused frame chance to open the abandon dialog (the bot always cancels it).
	abandon_ask: f64,
	/// Distance at which the bot backs off an enemy.
	danger: f64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct BotCounts {
	pub frames: u64,
	pub decisions: u64,
	pub hesitations: u64,
	pub mistakes: u64,
	pub manual_aim_frames: u64,
	pub menu_actions: u64,
	pub pause_presses: u64,
	pub focus_losses: u64,
	pub resizes: u64,
	pub ui_actions: u64,
	/// Typed requests the bot sent knowing they should be rejected or ignored.
	pub ui_mistakes: u64,
	pub shop_visits: u64,
	pub abandon_dialogs: u64,
}

const SIZES: [(u32, u32); 12] = [
	(1280, 720),
	(1920, 1080),
	(2560, 1080),
	(3440, 1440),
	(1280, 1024),
	(800, 600),
	(640, 360),
	(1366, 768),
	(1, 1),
	(2, 2000),
	(4000, 3),
	(7680, 4320),
];

/// What the bot remembers of an earlier frame; it acts on this, not on the present.
#[derive(Clone, Copy)]
struct Perceived {
	player: DVec2,
	enemy: Option<DVec2>,
}

/// Largest number of typed requests the bot spends in one shop visit before it leaves.
const VISIT_BUDGET: u32 = 24;

/// Frames the chaotic bot keeps poking a finished run, which must stay settled.
const SUMMARY_DWELL: u32 = 30;

struct Awaited {
	action: UiAction,
	enabled: bool,
	before: (&'static str, u64, &'static str, u32),
}

pub struct Bot {
	profile: BotProfile,
	tuning: Tuning,
	rng: ChaCha8Rng,
	max_frames: u64,
	history: VecDeque<Perceived>,
	delay: usize,
	direction: DVec2,
	decide_in: u32,
	manual: bool,
	hesitate_left: u32,
	pause_burst: u32,
	paused_for: u32,
	unfocused_for: u32,
	queue: VecDeque<UiAction>,
	ui_wait: u32,
	visit: Option<u32>,
	visit_actions: u32,
	refreshed: bool,
	summary_dwell: u32,
	awaited: Option<Awaited>,
	/// Run state of the previous frame and whether the bot asked to start a wave since.
	last_state: Option<&'static str>,
	start_sent: bool,
	pub findings: Vec<Finding>,
	pub counts: BotCounts,
}

impl Bot {
	pub fn new(profile: BotProfile, seed: &RunSeed, max_frames: u64) -> Self {
		let mut bytes = seed.0;
		bytes[30] ^= 0xB0 ^ profile as u8;
		let mut bot = Self {
			profile,
			tuning: profile.tuning(),
			rng: ChaCha8Rng::from_seed(bytes),
			max_frames,
			history: VecDeque::new(),
			delay: 0,
			direction: DVec2::ZERO,
			decide_in: 0,
			manual: false,
			hesitate_left: 0,
			pause_burst: 0,
			paused_for: 0,
			unfocused_for: 0,
			queue: VecDeque::new(),
			ui_wait: 0,
			visit: None,
			visit_actions: 0,
			refreshed: false,
			summary_dwell: 0,
			awaited: None,
			last_state: None,
			start_sent: false,
			findings: Vec::new(),
			counts: BotCounts::default(),
		};
		bot.delay = bot.reaction() as usize;
		bot
	}

	fn unit(&mut self) -> f64 {
		f64::from(self.rng.next_u32()) / 4_294_967_296.0
	}

	fn chance(&mut self, p: f64) -> bool {
		self.unit() < p
	}

	fn between(&mut self, lo: u32, hi: u32) -> u32 {
		lo + (self.rng.next_u32() % (hi - lo + 1))
	}

	fn reaction(&mut self) -> u32 {
		let (lo, hi) = self.tuning.reaction;
		self.between(lo, hi)
	}

	/// Roughly normal error with the profile's amplitude.
	fn aim_noise(&mut self) -> f64 {
		(self.unit() + self.unit() + self.unit() - 1.5) * self.tuning.aim_error
	}

	fn perceive(&mut self, obs: &Snapshot) -> Option<Perceived> {
		let player = obs.player.as_ref()?;
		self.history.push_back(Perceived {
			player: DVec2::new(player.x, player.y),
			enemy: obs.nearest_enemy.map(|e| DVec2::new(e[0], e[1])),
		});
		while self.history.len() > 90 {
			self.history.pop_front();
		}
		let index = self.history.len().saturating_sub(1 + self.delay);
		self.history.get(index).copied()
	}

	fn decide(&mut self, seen: Perceived) {
		self.counts.decisions += 1;
		self.delay = self.reaction() as usize;
		self.decide_in = self.reaction();
		self.manual = self.chance(self.tuning.manual_aim);
		self.direction = match seen.enemy.map(|e| e - seen.player) {
			Some(v) if v.length() < 1.0 => DVec2::ZERO,
			Some(v) if v.length() < self.tuning.danger => -v.normalize(),
			// Aggressive bots close in; the others circle the enemy.
			Some(v) if self.tuning.danger < 100.0 => v.normalize(),
			Some(v) => {
				let dir = v.normalize();
				DVec2::new(-dir.y, dir.x) * if self.chance(0.5) { 1.0 } else { -1.0 }
			}
			None => {
				let angle = self.unit() * std::f64::consts::TAU;
				DVec2::new(angle.cos(), angle.sin()) * f64::from(u8::from(self.chance(0.7)))
			}
		};
	}

	fn menu_frame(&mut self, items: u8, selected: u8) -> FrameInput {
		let mut frame = FrameInput::default();
		if !self.chance(self.tuning.menu_act) {
			return frame;
		}
		self.counts.menu_actions += 1;
		match self.between(0, 5) {
			0 if selected + 1 < items => frame.down = true,
			1 if selected > 0 => frame.up = true,
			2 | 3 => frame.edges.push(Edge::Confirm),
			4 => frame.edges.push(Edge::Cancel),
			5 => frame.edges.push(Edge::OpenEnergy),
			_ => {}
		}
		frame
	}

	fn abuse(&mut self, obs: &Snapshot, commands: &mut Vec<Command>) -> Option<FrameInput> {
		if self.unfocused_for > 0 {
			self.unfocused_for -= 1;
			if self.unfocused_for == 0 {
				commands.push(Command::Resume);
			}
			return Some(FrameInput::default());
		}
		if self.pause_burst > 0 {
			self.pause_burst -= 1;
			self.counts.pause_presses += 1;
			return Some(FrameInput {
				edges: vec![Edge::Cancel],
				..FrameInput::default()
			});
		}
		if obs.paused {
			if obs.run.is_some() && self.chance(self.tuning.abandon_ask) {
				self.counts.abandon_dialogs += 1;
				self.send(obs, UiAction::AskAbandon, false, commands);
				return Some(FrameInput::default());
			}
			self.paused_for += 1;
			if self.paused_for > self.between(5, 60) {
				self.paused_for = 0;
				commands.push(Command::Resume);
			}
			return Some(FrameInput::default());
		}
		if !self.chance(self.tuning.abuse) {
			return None;
		}
		match self.between(0, 2) {
			0 => {
				self.pause_burst = self.between(2, 12);
			}
			1 => {
				self.counts.focus_losses += 1;
				self.unfocused_for = self.between(5, 120);
				commands.push(Command::FocusLost);
			}
			_ => {
				self.counts.resizes += 1;
				let (width, height) = SIZES[self.between(0, SIZES.len() as u32 - 1) as usize];
				commands.push(Command::Resize { width, height });
			}
		}
		Some(FrameInput::default())
	}

	fn gameplay_frame(&mut self, obs: &Snapshot) -> FrameInput {
		let Some(seen) = self.perceive(obs) else {
			return FrameInput::default();
		};
		if self.hesitate_left > 0 {
			self.hesitate_left -= 1;
			return FrameInput::default();
		}
		if self.chance(self.tuning.hesitate) {
			self.counts.hesitations += 1;
			self.hesitate_left = self.between(10, 60);
			return FrameInput::default();
		}
		if self.decide_in == 0 {
			self.decide(seen);
		}
		self.decide_in = self.decide_in.saturating_sub(1);
		let mut frame = FrameInput {
			up: self.direction.y > 0.38,
			down: self.direction.y < -0.38,
			left: self.direction.x < -0.38,
			right: self.direction.x > 0.38,
			..FrameInput::default()
		};
		if self.chance(self.tuning.mistake) {
			self.counts.mistakes += 1;
			match self.between(0, 3) {
				0 => std::mem::swap(&mut frame.left, &mut frame.right),
				1 => std::mem::swap(&mut frame.up, &mut frame.down),
				2 => frame.edges.push(Edge::Confirm),
				_ => frame.edges.push(Edge::OpenEnergy),
			}
		}
		if self.manual {
			self.counts.manual_aim_frames += 1;
			frame.manual_aim_held = true;
			frame.cursor_world = self.aim_point(obs, seen);
		}
		frame
	}

	/// Cursor point near the enemy with human error; `None` models a click in a bar.
	fn aim_point(&mut self, obs: &Snapshot, seen: Perceived) -> Option<DVec2> {
		let target = seen
			.enemy
			.or_else(|| obs.nearest_enemy.map(|e| DVec2::new(e[0], e[1])))?;
		let delta = target - seen.player;
		let (sin, cos) = self.aim_noise().sin_cos();
		let rotated = DVec2::new(delta.x * cos - delta.y * sin, delta.x * sin + delta.y * cos);
		let point = seen.player + rotated;
		let bounds = arena();
		Some(point.clamp(bounds.min(), bounds.max()))
	}
}

impl InputDriver for Bot {
	fn next(&mut self, obs: &Snapshot) -> Step {
		if self.counts.frames >= self.max_frames {
			return Step {
				done: true,
				..Step::default()
			};
		}
		self.counts.frames += 1;
		self.check_awaited(obs);
		let state = obs.run.as_ref().map(|r| r.state);
		if self.last_state == Some("shop") && state == Some("wave") && !self.start_sent {
			self.findings.push(Finding {
				frame: self.counts.frames,
				tick: obs.tick,
				kind: "shop-left-without-start",
				detail: format!("wave {}", obs.run.as_ref().map_or(0, |r| r.wave)),
			});
		}
		if state != Some("shop") {
			self.start_sent = false;
		}
		self.last_state = state;
		if state == Some("summary")
			&& (self.profile != BotProfile::Chaotic || self.summary_dwell >= SUMMARY_DWELL)
		{
			return Step {
				done: true,
				..Step::default()
			};
		}
		let mut commands = Vec::new();
		let screen = state.is_some_and(|s| s != "wave");
		let frame = if let Some(menu) = obs.menu {
			self.menu_frame(menu.items, menu.selected)
		} else if screen {
			self.screen_frame(obs, &mut commands)
		} else if obs.ui.mode == "confirm_abandon" {
			self.abandon_dialog(obs, &mut commands)
		} else if let Some(frame) = self.abuse(obs, &mut commands) {
			frame
		} else {
			self.gameplay_frame(obs)
		};
		Step {
			frame,
			commands,
			done: false,
		}
	}
}

/// Run state, shop revision, modal and wave: what a request that took effect must change.
fn effect_key(obs: &Snapshot) -> (&'static str, u64, &'static str, u32) {
	let run = obs.run.as_ref();
	(
		run.map_or("none", |r| r.state),
		obs.shop.as_ref().map_or(0, |s| s.revision),
		obs.ui.mode,
		run.map_or(0, |r| r.wave),
	)
}

fn energy_target(lo: &LoadoutFacts, concentrate: bool) -> Option<usize> {
	if lo.unallocated == 0 {
		return None;
	}
	lo.slots
		.iter()
		.enumerate()
		.filter(|(_, s)| s.weapon.is_some() && s.points < 3)
		.max_by_key(|(i, s)| {
			let points = i32::from(s.points);
			(if concentrate { points } else { -points }, -(*i as i32))
		})
		.map(|(i, _)| i)
}

/// Index of the affordable offer of `kind` with the lowest (or highest) price.
fn pick_offer(shop: &ShopFacts, scrap: u32, kind: &str, cheapest: bool) -> Option<usize> {
	let affordable = shop
		.offers
		.iter()
		.enumerate()
		.filter(|(_, o)| o.kind == kind && o.price <= scrap);
	if cheapest {
		affordable.min_by_key(|(_, o)| o.price).map(|(i, _)| i)
	} else {
		affordable.max_by_key(|(_, o)| o.price).map(|(i, _)| i)
	}
}

fn add_energy(slot: usize) -> UiAction {
	UiAction::Energy { slot, add: true }
}

impl Bot {
	fn send(
		&mut self,
		obs: &Snapshot,
		action: UiAction,
		mistaken: bool,
		commands: &mut Vec<Command>,
	) {
		self.counts.ui_actions += 1;
		self.start_sent |= action == UiAction::Start;
		self.counts.ui_mistakes += u64::from(mistaken);
		let enabled = obs
			.screen
			.as_ref()
			.is_some_and(|s| s.buttons.iter().any(|b| b.action == action && b.enabled));
		self.awaited = Some(Awaited {
			action,
			enabled,
			before: effect_key(obs),
		});
		self.ui_wait = self.reaction();
		commands.push(Command::Ui(action));
	}

	/// A request from an enabled button must change something; a no-op is a UI defect.
	fn check_awaited(&mut self, obs: &Snapshot) {
		let Some(awaited) = self.awaited.take() else {
			return;
		};
		if awaited.enabled && effect_key(obs) == awaited.before {
			self.findings.push(Finding {
				frame: self.counts.frames,
				tick: obs.tick,
				kind: "enabled-action-had-no-effect",
				detail: format!("{:?}: {}", awaited.action, obs.ui.message),
			});
		}
	}

	fn abandon_dialog(&mut self, obs: &Snapshot, commands: &mut Vec<Command>) -> FrameInput {
		if self.ui_wait > 0 {
			self.ui_wait -= 1;
		} else {
			self.send(obs, UiAction::CancelAbandon, false, commands);
		}
		FrameInput::default()
	}

	fn screen_frame(&mut self, obs: &Snapshot, commands: &mut Vec<Command>) -> FrameInput {
		self.history.clear();
		if self.ui_wait > 0 {
			self.ui_wait -= 1;
			return FrameInput::default();
		}
		match obs.run.as_ref().map(|r| r.state) {
			Some("shop") => self.shop_turn(obs, commands),
			Some("victory" | "defeat") => {
				let action = if self.chance(self.tuning.mistake) {
					self.random_poke()
				} else {
					UiAction::Continue
				};
				self.send(obs, action, action != UiAction::Continue, commands);
			}
			Some("summary") => {
				self.summary_dwell += 1;
				let action = self.random_poke();
				self.send(obs, action, true, commands);
			}
			_ => {}
		}
		FrameInput::default()
	}

	/// A request the keyboard can send on a result screen; none may change a settled run.
	fn random_poke(&mut self) -> UiAction {
		match self.between(0, 4) {
			0 => UiAction::Buy(self.between(0, 3) as usize),
			1 => UiAction::Refresh,
			2 => UiAction::Start,
			3 => UiAction::Energy {
				slot: self.between(0, 3) as usize,
				add: self.chance(0.5),
			},
			_ => UiAction::Continue,
		}
	}

	fn shop_turn(&mut self, obs: &Snapshot, commands: &mut Vec<Command>) {
		let (Some(shop), Some(lo)) = (&obs.shop, &obs.loadout) else {
			return;
		};
		if self.visit != Some(shop.visit) {
			self.visit = Some(shop.visit);
			self.visit_actions = 0;
			self.refreshed = false;
			self.queue.clear();
			self.counts.shop_visits += 1;
		}
		if let Some(action) = self.queue.pop_front() {
			self.visit_actions += 1;
			return self.send(obs, action, true, commands);
		}
		if self.visit_actions < VISIT_BUDGET && self.chance(self.tuning.shop_mistake) {
			self.mistaken_requests(lo);
			return;
		}
		self.visit_actions += 1;
		let action = if self.visit_actions > VISIT_BUDGET {
			self.leave(lo)
		} else {
			self.policy(obs, shop, lo)
		};
		self.send(obs, action, false, commands);
	}

	/// Queues requests a hurried or careless person sends: unaffordable or sold buys,
	/// refresh spam, a start with every energy point removed.
	fn mistaken_requests(&mut self, lo: &LoadoutFacts) {
		match self.between(0, 4) {
			0 => {
				let offer = self.between(0, 5) as usize;
				self.queue.push_back(UiAction::Buy(offer));
			}
			1 => self.queue.extend([UiAction::Refresh; 3]),
			2 => {
				let slot = self.between(0, 3) as usize;
				let add = self.chance(0.5);
				self.queue.push_back(UiAction::Energy { slot, add });
			}
			3 => self.queue.push_back(UiAction::Start),
			_ => {
				for (slot, facts) in lo.slots.iter().enumerate() {
					for _ in 0..facts.points {
						self.queue.push_back(UiAction::Energy { slot, add: false });
					}
				}
				self.queue.push_back(UiAction::Start);
			}
		}
	}

	/// Shortest legal way out of the shop: power one weapon, then start.
	fn leave(&self, lo: &LoadoutFacts) -> UiAction {
		if lo.powered {
			return UiAction::Start;
		}
		let slot = lo
			.slots
			.iter()
			.position(|s| s.weapon.is_some())
			.unwrap_or(0);
		add_energy(slot)
	}

	fn policy(&mut self, obs: &Snapshot, shop: &ShopFacts, lo: &LoadoutFacts) -> UiAction {
		if !lo.powered {
			return self.leave(lo);
		}
		if obs.ui.mode == "replace" {
			return UiAction::CancelReplace;
		}
		let scrap = obs.scrap;
		let buy = |index: Option<usize>| index.map(UiAction::Buy);
		match self.profile {
			BotProfile::Cautious => buy(pick_offer(shop, scrap, "item", true))
				.or_else(|| buy(pick_offer(shop, scrap, "weapon", true)))
				.or_else(|| energy_target(lo, false).map(add_energy))
				.unwrap_or(UiAction::Start),
			BotProfile::Aggressive => {
				let bought = buy(pick_offer(shop, scrap, "weapon", false))
					.or_else(|| buy(pick_offer(shop, scrap, "item", false)));
				if bought.is_none()
					&& !self.refreshed
					&& !shop.refresh_used
					&& scrap >= shop.refresh_cost
				{
					self.refreshed = true;
					return UiAction::Refresh;
				}
				bought
					.or_else(|| energy_target(lo, true).map(add_energy))
					.unwrap_or(UiAction::Start)
			}
			BotProfile::Chaotic => {
				let enabled: Vec<UiAction> = obs.screen.as_ref().map_or_else(Vec::new, |s| {
					s.buttons
						.iter()
						.filter(|b| b.enabled)
						.map(|b| b.action)
						.collect()
				});
				if enabled.is_empty() {
					return UiAction::Start;
				}
				enabled[self.between(0, enabled.len() as u32 - 1) as usize]
			}
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Finding {
	pub frame: u64,
	pub tick: u64,
	pub kind: &'static str,
	pub detail: String,
}

/// Cross-frame invariants every profile must keep; each violation is a finding.
#[derive(Default)]
pub struct Auditor {
	prev: Option<Snapshot>,
	frame: u64,
	/// Run state, wave and shop visit last seen, with the frames spent in them.
	same_since: (Option<StateKey>, u32),
	/// Extra ticks a frame may advance: zero headless, the catch-up cap in a window.
	pub tick_slack: u64,
	pub findings: Vec<Finding>,
}

const MAX_FINDINGS: usize = 50;

type StateKey = (&'static str, u32, Option<u32>);

/// No honest run holds this much scrap; a wrapped subtraction would exceed it.
const SCRAP_SANITY: u32 = 100_000_000;

impl Auditor {
	fn report(&mut self, snap: &Snapshot, kind: &'static str, detail: String) {
		if self.findings.len() < MAX_FINDINGS {
			self.findings.push(Finding {
				frame: self.frame,
				tick: snap.tick,
				kind,
				detail,
			});
		}
	}

	/// Run lifecycle, shop and loadout invariants (SP-009, SP-011, DA-03, DA-13).
	fn lifecycle(&mut self, prev: Option<&Snapshot>, snap: &Snapshot) {
		let Some(run) = &snap.run else {
			return;
		};
		if let Some(lo) = &snap.loadout {
			let mut weapons = BTreeSet::new();
			if lo.slots.len() > 4 || lo.occupied > 4 {
				self.report(snap, "slots-over-limit", format!("{}", lo.slots.len()));
			}
			if lo.energy_used > 6 {
				self.report(snap, "energy-over-budget", format!("{}", lo.energy_used));
			}
			for (i, slot) in lo.slots.iter().enumerate() {
				if slot.points > 3 {
					self.report(snap, "slot-energy-over-limit", format!("slot {i}"));
				}
				if slot.weapon.is_none() && slot.points > 0 {
					self.report(snap, "energy-on-empty-slot", format!("slot {i}"));
				}
				if let Some(id) = &slot.weapon
					&& !weapons.insert(id.clone())
				{
					self.report(snap, "duplicate-weapon", id.clone());
				}
			}
		}
		if snap.scrap > SCRAP_SANITY {
			self.report(snap, "scrap-overflow", format!("{}", snap.scrap));
		}
		if run.state == "wave" && snap.player.as_ref().is_some_and(|p| p.hp == 0) {
			self.report(snap, "death-unresolved", format!("wave {}", run.wave));
		}
		if run.settlements > 1 {
			self.report(snap, "multiple-settlements", format!("{}", run.settlements));
		}
		if run.state == "shop" {
			if snap.enemy_count > 0 {
				self.report(snap, "enemies-in-shop", format!("{}", snap.enemy_count));
			}
			if run.wave >= run.total_waves {
				self.report(snap, "shop-after-final-wave", format!("{}", run.wave));
			}
			if snap.shop.is_none() {
				self.report(snap, "shop-without-visit", String::new());
			}
		}
		if matches!(run.state, "victory" | "defeat" | "summary") && run.outcome.is_none() {
			self.report(snap, "result-missing", run.state.to_owned());
		}
		if let Some(prev) = prev
			&& let Some(was) = &prev.run
		{
			self.transition(prev, was, snap, run);
		}
		let key = (run.state, run.wave, snap.shop.as_ref().map(|s| s.visit));
		if self.same_since.0 == Some(key) {
			if !snap.paused {
				self.same_since.1 += 1;
			}
			let limit = match run.state {
				"wave" => 12_000,
				"shop" => 6_000,
				_ => 3_000,
			};
			if self.same_since.1 == limit {
				self.report(
					snap,
					"state-stuck",
					format!("{} frames in {}", limit, run.state),
				);
			}
		} else {
			self.same_since = (Some(key), 0);
		}
	}

	fn transition(&mut self, prev: &Snapshot, was: &RunFacts, snap: &Snapshot, run: &RunFacts) {
		let legal = matches!(
			(was.state, run.state),
			("wave", "wave" | "shop" | "victory" | "defeat" | "summary")
				| ("shop", "shop" | "wave" | "summary")
				| ("victory", "victory" | "summary")
				| ("defeat", "defeat" | "summary")
				| ("summary", "summary")
		);
		if !legal {
			self.report(
				snap,
				"illegal-transition",
				format!("{} -> {}", was.state, run.state),
			);
		}
		if run.wave < was.wave {
			self.report(
				snap,
				"wave-went-back",
				format!("{} -> {}", was.wave, run.wave),
			);
		}
		if was.outcome.is_some() && was.outcome != run.outcome {
			self.report(
				snap,
				"result-changed",
				format!("{:?} -> {:?}", was.outcome, run.outcome),
			);
		}
		match (was.state, run.state) {
			("wave", "wave") if run.wave != was.wave => self.report(
				snap,
				"wave-advanced-without-shop",
				format!("{} -> {}", was.wave, run.wave),
			),
			("wave", "wave") if snap.scrap < prev.scrap => self.report(
				snap,
				"scrap-lost-in-combat",
				format!("{} -> {}", prev.scrap, snap.scrap),
			),
			("shop", "shop") => {
				let (before, after) = (prev.shop.as_ref(), snap.shop.as_ref());
				let same_revision = before
					.zip(after)
					.is_some_and(|(b, a)| b.visit == a.visit && b.revision == a.revision);
				if same_revision
					&& (before != after || prev.scrap != snap.scrap || prev.loadout != snap.loadout)
				{
					self.report(
						snap,
						"rejected-command-changed-state",
						snap.ui.message.clone(),
					);
				}
			}
			("shop", "wave") => {
				if !prev.loadout.as_ref().is_some_and(|l| l.powered) {
					self.report(snap, "started-unpowered", String::new());
				}
				if run.wave != was.wave + 1 {
					self.report(
						snap,
						"wave-skipped",
						format!("{} -> {}", was.wave, run.wave),
					);
				}
				if snap.player.as_ref().is_some_and(|p| p.hp != p.hp_max) {
					self.report(snap, "wave-start-not-healed", String::new());
				}
			}
			_ => {}
		}
	}

	pub fn observe(&mut self, snap: &Snapshot) {
		self.frame += 1;
		let before = self.prev.take();
		if let Some(prev) = &before {
			if snap.tick < prev.tick || snap.tick > prev.tick + 1 + self.tick_slack {
				self.report(
					snap,
					"tick-jump",
					format!("tick went {} -> {}", prev.tick, snap.tick),
				);
			}
			if prev.paused && snap.paused && snap.tick != prev.tick {
				self.report(snap, "tick-while-paused", format!("tick {}", snap.tick));
			}
		}
		if let Some(p) = &snap.player {
			let b = arena();
			if p.hp > p.hp_max {
				self.report(snap, "hp-over-max", format!("{}/{}", p.hp, p.hp_max));
			}
			let (min, max) = (b.min(), b.max());
			if !(p.x.is_finite() && p.y.is_finite())
				|| p.x < min.x - 1e-6
				|| p.x > max.x + 1e-6
				|| p.y < min.y - 1e-6
				|| p.y > max.y + 1e-6
			{
				self.report(snap, "player-outside-arena", format!("({}, {})", p.x, p.y));
			}
		}
		match &snap.camera {
			None => self.report(
				snap,
				"camera-rejected-size",
				format!("{}x{}", snap.window[0], snap.window[1]),
			),
			Some(c) => {
				let [w, h] = snap.window.map(f64::from);
				if c.content_x < -1e-6
					|| c.content_y < -1e-6
					|| c.content_x + c.content_w > w + 1e-6
					|| c.content_y + c.content_h > h + 1e-6
				{
					self.report(snap, "camera-content-outside-screen", format!("{c:?}"));
				}
			}
		}
		self.lifecycle(before.as_ref(), snap);
		self.prev = Some(snap.clone());
	}
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct QaReport {
	pub kind: &'static str,
	pub profile: BotProfile,
	pub seed: String,
	pub frames: u64,
	pub ticks: u64,
	pub counts: BotCounts,
	pub findings: Vec<Finding>,
	pub panics: u32,
	/// The run reached its Summary screen before the frame budget ran out.
	pub completed: bool,
	pub outcome: Option<&'static str>,
	pub max_wave: u32,
	pub passed: bool,
	pub final_snapshot: Option<Snapshot>,
}
