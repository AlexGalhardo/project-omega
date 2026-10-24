//! Seeded human-like input source (ROADMAP rule 1.11). It only produces frames and
//! window commands for the shared input buffer; it never touches the simulation.

use super::{Command, InputDriver, Snapshot, Step, arena};
use crate::input::{Edge, FrameInput};
use crate::sim::RunSeed;
use bevy::math::DVec2;
use rand_chacha::ChaCha8Rng;
use rand_chacha::rand_core::{Rng, SeedableRng};
use serde::Serialize;
use std::collections::VecDeque;
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
				menu_act: 0.05,
				abuse: 0.0005,
				manual_aim: 0.3,
				danger: 260.0,
			},
			Self::Aggressive => Tuning {
				reaction: (8, 22),
				aim_error: 0.22,
				hesitate: 0.0008,
				mistake: 0.02,
				menu_act: 0.12,
				abuse: 0.0008,
				manual_aim: 0.7,
				danger: 60.0,
			},
			Self::Chaotic => Tuning {
				reaction: (4, 60),
				aim_error: 0.6,
				hesitate: 0.01,
				mistake: 0.12,
				menu_act: 0.4,
				abuse: 0.01,
				manual_aim: 0.5,
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
	menu_act: f64,
	/// Per-frame chance to start pause, focus or resize abuse.
	abuse: f64,
	manual_aim: f64,
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

pub struct Bot {
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
	pub counts: BotCounts,
}

impl Bot {
	pub fn new(profile: BotProfile, seed: &RunSeed, max_frames: u64) -> Self {
		let mut bytes = seed.0;
		bytes[30] ^= 0xB0 ^ profile as u8;
		let mut bot = Self {
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
		let mut commands = Vec::new();
		let frame = if let Some(menu) = obs.menu {
			self.menu_frame(menu.items, menu.selected)
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
	pub findings: Vec<Finding>,
}

const MAX_FINDINGS: usize = 50;

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

	pub fn observe(&mut self, snap: &Snapshot) {
		self.frame += 1;
		if let Some(prev) = self.prev.take() {
			if snap.tick < prev.tick || snap.tick > prev.tick + 1 {
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
	pub passed: bool,
	pub final_snapshot: Option<Snapshot>,
}
