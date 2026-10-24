//! Phase-driven telegraph cues (ENG-VIS-001, Art Bible section 6).
//!
//! A cue is a presentation copy of an authoritative fact: the shape and anchor come from
//! the simulation and the engine never invents or delays them. A locked cue stays on its
//! world center even when the attacker is knocked back; a follow cue keeps tracking the
//! attacker's current origin. Cancellation removes a cue at once.

use bevy::math::DVec2;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CueShape {
	/// Ground circle of the exact damaging radius.
	Circle { radius: f64 },
	/// Lane of the exact damaging width with rounded ends.
	Capsule { reach: f64, radius: f64 },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CueAnchor {
	/// World center fixed at lock time (ground strikes).
	Locked(DVec2),
	/// Starts at the attacker's current origin and points along `direction` (beams, charges).
	FollowOrigin { direction: DVec2 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EffectsLevel {
	#[default]
	Full,
	/// Drops decorative parts only; shape, progress and timing stay visible.
	Reduced,
}

/// Facts the game hands over when a cue starts. Ticks are simulation ticks.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhaseCue {
	pub shape: CueShape,
	pub anchor: CueAnchor,
	pub total_ticks: u32,
	pub elapsed_ticks: u32,
}

/// What to draw for one cue this frame.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CueView {
	/// Circle center, or the lane start.
	pub origin: DVec2,
	/// Lane direction; `None` for circles.
	pub direction: Option<DVec2>,
	pub shape: CueShape,
	/// Fill progress in `0.0..=1.0`, equal to elapsed over total ticks.
	pub progress: f64,
	/// Sparks, pulses and glows; always `false` under reduced effects.
	pub decorations: bool,
}

impl PhaseCue {
	pub fn progress(&self) -> f64 {
		if self.total_ticks == 0 {
			return 1.0;
		}
		(f64::from(self.elapsed_ticks) / f64::from(self.total_ticks)).clamp(0.0, 1.0)
	}

	/// Projects the cue for drawing; `attacker_origin` is the attacker's current position.
	pub fn view(&self, attacker_origin: DVec2, effects: EffectsLevel) -> CueView {
		let (origin, direction) = match self.anchor {
			CueAnchor::Locked(center) => (center, None),
			CueAnchor::FollowOrigin { direction } => (attacker_origin, Some(direction)),
		};
		CueView {
			origin,
			direction,
			shape: self.shape,
			progress: self.progress(),
			decorations: effects == EffectsLevel::Full,
		}
	}
}

/// Live cues by simulation ID, in stable ID order.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct CueSet {
	cues: BTreeMap<u64, PhaseCue>,
}

impl CueSet {
	pub fn start(&mut self, id: u64, cue: PhaseCue) {
		self.cues.insert(id, cue);
	}

	/// Advances every cue by one simulation tick; finished cues stay until emission or cancel.
	pub fn tick(&mut self) {
		for cue in self.cues.values_mut() {
			cue.elapsed_ticks = (cue.elapsed_ticks + 1).min(cue.total_ticks);
		}
	}

	/// Emission or cancellation: the cue disappears immediately.
	pub fn end(&mut self, id: u64) -> bool {
		self.cues.remove(&id).is_some()
	}

	pub fn get(&self, id: u64) -> Option<&PhaseCue> {
		self.cues.get(&id)
	}

	pub fn len(&self) -> usize {
		self.cues.len()
	}

	pub fn is_empty(&self) -> bool {
		self.cues.is_empty()
	}

	pub fn iter(&self) -> impl Iterator<Item = (u64, &PhaseCue)> {
		self.cues.iter().map(|(id, cue)| (*id, cue))
	}
}
