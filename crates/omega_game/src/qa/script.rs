//! JSON e2e timeline: input actions plus checkpoints over the authoritative snapshot.

use super::{Command, InputDriver, Snapshot, Step, arena};
use crate::input::{Edge, FrameInput};
use crate::sim::RunSeed;
use bevy::math::DVec2;
use omega_2d::camera::ViewportMapping;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Key {
	W,
	A,
	S,
	D,
	Up,
	Down,
	Left,
	Right,
	Escape,
	Enter,
	Space,
	E,
}

impl Key {
	fn edge(self) -> Option<Edge> {
		match self {
			Key::Escape => Some(Edge::Cancel),
			Key::Enter | Key::Space => Some(Edge::Confirm),
			Key::E => Some(Edge::OpenEnergy),
			_ => None,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Button {
	Left,
	Right,
}

/// Expected value of one dotted snapshot path, for example `camera.letterbox`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum Expect {
	Flag(bool),
	Exact(f64),
	Range {
		#[serde(default)]
		min: Option<f64>,
		#[serde(default)]
		max: Option<f64>,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub struct Checkpoint {
	pub name: String,
	pub expect: BTreeMap<String, Expect>,
	/// Windowed runs write a PNG next to the report; headless runs record `null`.
	pub screenshot: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "do", rename_all = "snake_case", deny_unknown_fields)]
pub enum Action {
	KeyDown {
		key: Key,
	},
	KeyUp {
		key: Key,
	},
	KeyTap {
		key: Key,
	},
	MouseMove {
		x: f64,
		y: f64,
	},
	MouseDown {
		button: Button,
	},
	MouseUp {
		button: Button,
	},
	MouseClick {
		button: Button,
		x: f64,
		y: f64,
	},
	/// Waits until `ticks` more simulation ticks ran; fails if the sim is paused.
	Wait {
		ticks: u32,
	},
	/// Waits `count` rendered frames whether or not the sim ticks.
	Frames {
		count: u32,
	},
	Resize {
		width: u32,
		height: u32,
	},
	FocusLost,
	FocusGained,
	/// Escape tap outside any modal.
	Pause,
	/// Explicit resume, as the pause menu will offer.
	Resume,
	Checkpoint {
		name: String,
		#[serde(default)]
		expect: BTreeMap<String, Expect>,
		#[serde(default)]
		screenshot: bool,
	},
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Script {
	pub name: String,
	#[serde(default)]
	pub seed: Option<String>,
	#[serde(default = "default_resolution")]
	pub resolution: [u32; 2],
	pub steps: Vec<Action>,
}

fn default_resolution() -> [u32; 2] {
	[1280, 720]
}

impl Script {
	pub fn parse(text: &str) -> Result<Self, String> {
		let script: Script = serde_json::from_str(text).map_err(|e| format!("bad script: {e}"))?;
		script.validate()?;
		Ok(script)
	}

	pub fn run_seed(&self) -> Result<RunSeed, String> {
		let hex = self.seed.clone().unwrap_or_else(|| "5a".repeat(32));
		RunSeed::from_hex(&hex).map_err(|e| e.to_string())
	}

	fn validate(&self) -> Result<(), String> {
		if self.name.is_empty()
			|| !self
				.name
				.chars()
				.all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
		{
			return Err("script name must be non-empty [A-Za-z0-9_-]".into());
		}
		if self.steps.is_empty() {
			return Err("script has no steps".into());
		}
		if self.resolution.contains(&0) {
			return Err("script resolution must be positive".into());
		}
		self.run_seed()?;
		let mut names = BTreeSet::new();
		for (i, step) in self.steps.iter().enumerate() {
			let bad = match step {
				Action::Wait { ticks } => *ticks == 0,
				Action::Frames { count } => *count == 0,
				Action::Resize { width, height } => *width == 0 || *height == 0,
				Action::MouseMove { x, y } | Action::MouseClick { x, y, .. } => {
					!(x.is_finite() && y.is_finite())
				}
				Action::Checkpoint { name, .. } => name.is_empty() || !names.insert(name.clone()),
				_ => false,
			};
			if bad {
				return Err(format!(
					"step {i} is invalid or duplicates a checkpoint name"
				));
			}
		}
		Ok(())
	}
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CheckpointResult {
	pub name: String,
	pub passed: bool,
	pub failures: Vec<String>,
	pub snapshot: Snapshot,
	pub screenshot: Option<String>,
}

pub fn evaluate(cp: &Checkpoint, snap: &Snapshot) -> CheckpointResult {
	let json = serde_json::to_value(snap).unwrap_or(serde_json::Value::Null);
	let mut failures = Vec::new();
	for (path, expect) in &cp.expect {
		let found = path.split('.').try_fold(&json, |v, key| v.get(key));
		let Some(value) = found.filter(|v| !v.is_null()) else {
			failures.push(format!("{path}: missing in snapshot"));
			continue;
		};
		let ok = match expect {
			Expect::Flag(want) => value.as_bool() == Some(*want),
			Expect::Exact(want) => value.as_f64().is_some_and(|v| (v - want).abs() <= 1e-6),
			Expect::Range { min, max } => value
				.as_f64()
				.is_some_and(|v| min.is_none_or(|m| v >= m) && max.is_none_or(|m| v <= m)),
		};
		if !ok {
			failures.push(format!("{path}: expected {expect:?}, found {value}"));
		}
	}
	CheckpointResult {
		name: cp.name.clone(),
		passed: failures.is_empty(),
		failures,
		snapshot: snap.clone(),
		screenshot: None,
	}
}

enum Pending {
	None,
	Frames(u32),
	UntilTick { target: u64, budget: u64 },
}

pub struct ScriptDriver {
	steps: Vec<Action>,
	pc: usize,
	pending: Pending,
	keys: BTreeSet<Key>,
	left: bool,
	right: bool,
	cursor: Option<DVec2>,
	release_next: Vec<Key>,
	release_buttons_next: Vec<Button>,
	failure: Option<String>,
}

impl ScriptDriver {
	pub fn new(script: &Script) -> Self {
		Self {
			steps: script.steps.clone(),
			pc: 0,
			pending: Pending::None,
			keys: BTreeSet::new(),
			left: false,
			right: false,
			cursor: None,
			release_next: Vec::new(),
			release_buttons_next: Vec::new(),
			failure: None,
		}
	}

	fn held(&self, codes: [Key; 2]) -> bool {
		codes.iter().any(|k| self.keys.contains(k))
	}

	fn frame(&self, window: [u32; 2], edges: Vec<Edge>) -> FrameInput {
		let cursor_world = self.cursor.and_then(|c| {
			ViewportMapping::new(arena(), f64::from(window[0]), f64::from(window[1]))
				.ok()?
				.screen_to_world(c)
		});
		FrameInput {
			up: self.held([Key::W, Key::Up]),
			down: self.held([Key::S, Key::Down]),
			left: self.held([Key::A, Key::Left]),
			right: self.held([Key::D, Key::Right]),
			manual_aim_held: self.right,
			cursor_world,
			edges,
		}
	}

	fn press(&mut self, key: Key, edges: &mut Vec<Edge>) {
		if self.keys.insert(key)
			&& let Some(edge) = key.edge()
		{
			edges.push(edge);
		}
	}

	fn button(&mut self, button: Button, down: bool) {
		match button {
			Button::Left => self.left = down,
			Button::Right => self.right = down,
		}
	}

	fn finish(&mut self, commands: Vec<Command>) -> Step {
		Step {
			commands,
			done: true,
			..Step::default()
		}
	}
}

impl InputDriver for ScriptDriver {
	fn next(&mut self, obs: &Snapshot) -> Step {
		let mut window = obs.window;
		// Commands run after this call, so the driver tracks the pause state it causes.
		let mut paused = obs.paused;
		let mut edges = Vec::new();
		let mut commands = Vec::new();
		for key in std::mem::take(&mut self.release_next) {
			self.keys.remove(&key);
		}
		for button in std::mem::take(&mut self.release_buttons_next) {
			self.button(button, false);
		}
		loop {
			let emit = match &mut self.pending {
				Pending::Frames(n) if *n > 0 => {
					*n -= 1;
					true
				}
				Pending::UntilTick { target, budget } if obs.tick < *target => {
					if paused || *budget == 0 {
						self.failure = Some(format!(
							"wait for tick {target} cannot finish (paused: {paused}, tick: {})",
							obs.tick
						));
						return self.finish(commands);
					}
					*budget -= 1;
					true
				}
				_ => {
					self.pending = Pending::None;
					false
				}
			};
			if emit {
				return Step {
					frame: self.frame(window, edges),
					commands,
					done: false,
				};
			}
			let Some(action) = self.steps.get(self.pc).cloned() else {
				return self.finish(commands);
			};
			self.pc += 1;
			match action {
				Action::KeyDown { key } => self.press(key, &mut edges),
				Action::KeyUp { key } => {
					self.keys.remove(&key);
				}
				Action::KeyTap { key } => {
					self.press(key, &mut edges);
					self.release_next.push(key);
					self.pending = Pending::Frames(1);
				}
				Action::MouseMove { x, y } => self.cursor = Some(DVec2::new(x, y)),
				Action::MouseDown { button } => self.button(button, true),
				Action::MouseUp { button } => self.button(button, false),
				Action::MouseClick { button, x, y } => {
					self.cursor = Some(DVec2::new(x, y));
					self.button(button, true);
					self.release_buttons_next.push(button);
					self.pending = Pending::Frames(1);
				}
				Action::Wait { ticks } => {
					self.pending = Pending::UntilTick {
						target: obs.tick + u64::from(ticks),
						budget: u64::from(ticks) * 4 + 600,
					};
				}
				Action::Frames { count } => self.pending = Pending::Frames(count),
				Action::Resize { width, height } => {
					window = [width, height];
					commands.push(Command::Resize { width, height });
				}
				Action::FocusLost => {
					paused = true;
					commands.push(Command::FocusLost);
				}
				Action::FocusGained => {}
				Action::Pause => {
					self.press(Key::Escape, &mut edges);
					self.release_next.push(Key::Escape);
					self.pending = Pending::Frames(1);
				}
				Action::Resume => {
					paused = false;
					commands.push(Command::Resume);
				}
				Action::Checkpoint {
					name,
					expect,
					screenshot,
				} => commands.push(Command::Checkpoint(Checkpoint {
					name,
					expect,
					screenshot,
				})),
			}
		}
	}

	fn failure(&self) -> Option<&str> {
		self.failure.as_deref()
	}
}
