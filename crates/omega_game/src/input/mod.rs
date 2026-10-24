//! Keyboard/mouse action buffer and aim (SP-005, GDD-004).
//!
//! The render loop feeds one [`FrameInput`] per frame; the fixed loop samples one
//! [`TickSample`] per tick. Held state applies to every tick of a frame, digital edges
//! are delivered exactly once, and a frame with zero ticks keeps its edges pending.

pub mod bindings;
pub mod gamepad;

#[cfg(feature = "client")]
mod client;
#[cfg(feature = "client")]
pub use client::{ActiveSource, InputClientPlugin, PadNav};

use crate::sim::{AimInput, SimPaused, TickInput};
use bevy::math::DVec2;
use bevy::prelude::*;

/// Digital press edges the buffer tracks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Edge {
	/// Escape: pauses combat, or is Back while a modal owns input.
	Cancel,
	Confirm,
	OpenEnergy,
}

/// Raw sampled device state for one rendered frame.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FrameInput {
	pub up: bool,
	pub down: bool,
	pub left: bool,
	pub right: bool,
	pub manual_aim_held: bool,
	/// Cursor mapped to world LU; `None` in letterbox bars or outside the window.
	pub cursor_world: Option<DVec2>,
	pub edges: Vec<Edge>,
}

/// What the sim sees for one tick.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TickSample {
	pub input: TickInput,
	/// Escape pressed outside any modal; the client turns this into [`SimPaused`].
	pub pause_requested: bool,
}

/// Edges delivered to menus and modals, never to combat.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct UiEdges {
	pub confirm: bool,
	pub back: bool,
	pub open_energy: bool,
}

/// Where manual aim is measured from (the player center once the player exists).
#[derive(Resource, Debug, Clone, Copy, Default, PartialEq)]
pub struct AimOrigin(pub DVec2);

#[derive(Resource, Debug, Clone, PartialEq)]
pub struct InputBuffer {
	held: FrameInput,
	pending: Vec<Edge>,
	last_aim: DVec2,
	/// Analog movement after the dead zone, magnitude at most one (controller).
	pad_move: DVec2,
	/// Normalized aim direction while the aim stick is beyond its dead zone.
	pad_aim: Option<DVec2>,
	/// True while a menu or modal owns Confirm and Back.
	pub ui_focus: bool,
}

impl Default for InputBuffer {
	fn default() -> Self {
		Self {
			held: FrameInput::default(),
			pending: Vec::new(),
			last_aim: DVec2::X,
			pad_move: DVec2::ZERO,
			pad_aim: None,
			ui_focus: false,
		}
	}
}

impl InputBuffer {
	/// Replaces held state and queues the frame's new edges.
	pub fn begin_frame(&mut self, mut frame: FrameInput) {
		self.pending.append(&mut frame.edges);
		self.held = frame;
	}

	/// Controller contribution for the current frame; replaces the previous frame's analog state.
	pub fn set_pad(&mut self, pad: &gamepad::PadFrame) {
		self.pad_move = gamepad::radial_deadzone(pad.left_stick, gamepad::MOVE_DEADZONE);
		let aim = gamepad::radial_deadzone(pad.right_stick, gamepad::AIM_DEADZONE);
		self.pad_aim = (aim != DVec2::ZERO).then(|| aim.normalize());
		self.pending.extend(pad.edges());
	}

	/// Samples one fixed tick. Held state repeats; the pause edge is consumed once.
	pub fn sample_tick(&mut self, origin: DVec2) -> TickSample {
		let mut pause_requested = false;
		let ui_focus = self.ui_focus;
		self.pending.retain(|edge| match edge {
			Edge::Cancel if !ui_focus => {
				pause_requested = true;
				false
			}
			_ => true,
		});
		let held = &self.held;
		let axis = |pos: bool, neg: bool| f64::from(i8::from(pos) - i8::from(neg));
		let raw = DVec2::new(axis(held.right, held.left), axis(held.up, held.down));
		let movement = if raw != DVec2::ZERO {
			raw.normalize()
		} else {
			self.pad_move.clamp_length_max(1.0)
		};
		let aim = if let Some(direction) = self.pad_aim {
			self.last_aim = direction;
			AimInput::Manual(direction)
		} else if held.manual_aim_held {
			if let Some(cursor) = held.cursor_world {
				let delta = cursor - origin;
				if delta.length() >= 1.0 {
					self.last_aim = delta.normalize();
				}
			}
			AimInput::Manual(self.last_aim)
		} else {
			AimInput::Auto
		};
		TickSample {
			input: TickInput { movement, aim },
			pause_requested,
		}
	}

	/// Drains Confirm/Back/OpenEnergy edges for the UI; each press is delivered once.
	/// Edges are only drained while the UI has focus, so combat input cannot leak into it
	/// and a Confirm that started a wave cannot fire again in the next screen.
	pub fn take_ui_edges(&mut self) -> UiEdges {
		let mut out = UiEdges::default();
		if !self.ui_focus {
			return out;
		}
		self.pending.retain(|edge| {
			match edge {
				Edge::Confirm => out.confirm = true,
				Edge::Cancel => out.back = true,
				Edge::OpenEnergy => out.open_energy = true,
			}
			false
		});
		out
	}

	/// Focus or device loss: pause before another tick, drop stale held state and edges.
	pub fn focus_lost(&mut self, paused: &mut SimPaused) {
		paused.0 = true;
		self.held = FrameInput::default();
		self.pending.clear();
	}

	/// Explicit resume discards edges pressed while paused.
	pub fn resume(&mut self, paused: &mut SimPaused) {
		self.pending.clear();
		paused.0 = false;
	}

	pub fn pending_len(&self) -> usize {
		self.pending.len()
	}
}
