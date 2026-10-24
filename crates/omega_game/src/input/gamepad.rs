//! Controller model (SP-005, GDD-004): dead zones, aim stick, buttons to edges, active-source
//! tracking and UI navigation repeat. Pure data; the Bevy sampling lives in `client.rs`.

use super::Edge;
use bevy::math::DVec2;

/// Radial dead zone for movement (GDD section 5).
pub const MOVE_DEADZONE: f64 = 0.20;
/// Radial dead zone for the aim stick.
pub const AIM_DEADZONE: f64 = 0.25;
/// UI navigation repeat: first delay, then the faster interval (seconds).
pub const REPEAT_FIRST_S: f64 = 0.35;
pub const REPEAT_NEXT_S: f64 = 0.12;

/// Maps the remaining magnitude linearly to 0..1 and caps at one; inside the dead zone is zero.
pub fn radial_deadzone(raw: DVec2, deadzone: f64) -> DVec2 {
	let magnitude = raw.length();
	if magnitude <= deadzone {
		return DVec2::ZERO;
	}
	let scaled = ((magnitude - deadzone) / (1.0 - deadzone)).min(1.0);
	raw / magnitude * scaled
}

/// One frame of controller state.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct PadFrame {
	pub left_stick: DVec2,
	pub right_stick: DVec2,
	pub dpad_up: bool,
	pub dpad_down: bool,
	pub dpad_left: bool,
	pub dpad_right: bool,
	/// Face South: confirm.
	pub south_pressed: bool,
	/// Face East or Menu: pause or back.
	pub back_pressed: bool,
	/// Shoulder Right: open the energy panel.
	pub shoulder_right_pressed: bool,
}

impl PadFrame {
	/// A deliberate action: a button press or a stick beyond its dead zone.
	pub fn is_active(&self) -> bool {
		self.south_pressed
			|| self.back_pressed
			|| self.shoulder_right_pressed
			|| self.dpad_up
			|| self.dpad_down
			|| self.dpad_left
			|| self.dpad_right
			|| radial_deadzone(self.left_stick, MOVE_DEADZONE) != DVec2::ZERO
			|| radial_deadzone(self.right_stick, AIM_DEADZONE) != DVec2::ZERO
	}

	/// Button edges of this frame.
	pub fn edges(&self) -> Vec<Edge> {
		let mut edges = Vec::new();
		if self.south_pressed {
			edges.push(Edge::Confirm);
		}
		if self.back_pressed {
			edges.push(Edge::Cancel);
		}
		if self.shoulder_right_pressed {
			edges.push(Edge::OpenEnergy);
		}
		edges
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputSource {
	#[default]
	KeyboardMouse,
	Controller,
}

impl InputSource {
	/// Switches on deliberate actions only, never on passive mouse movement.
	pub fn after(self, keyboard_action: bool, controller_action: bool) -> Self {
		match (keyboard_action, controller_action) {
			(true, false) => InputSource::KeyboardMouse,
			(false, true) => InputSource::Controller,
			_ => self,
		}
	}
}

/// Direction of a UI navigation request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavDirection {
	Up,
	Down,
	Left,
	Right,
}

/// Fires once on press, again after 0.35 s, then every 0.12 s while held.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct NavRepeat {
	held: Option<(NavDirection, f64)>,
	next_at: f64,
}

impl NavRepeat {
	/// Advances to `now_s`; returns the direction to act on this frame, if any.
	pub fn update(&mut self, direction: Option<NavDirection>, now_s: f64) -> Option<NavDirection> {
		let Some(direction) = direction else {
			self.held = None;
			return None;
		};
		match self.held {
			Some((held, _)) if held == direction => {
				if now_s + 1e-9 >= self.next_at {
					self.next_at += REPEAT_NEXT_S;
					return Some(direction);
				}
				None
			}
			_ => {
				self.held = Some((direction, now_s));
				self.next_at = now_s + REPEAT_FIRST_S;
				Some(direction)
			}
		}
	}
}
