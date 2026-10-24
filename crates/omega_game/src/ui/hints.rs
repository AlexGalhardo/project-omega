//! First-run hints and UI scale (GDD section 15.2). Hints pause combat while shown, are
//! dismissed once, persist their acknowledgment in settings and can be reset.

use crate::profile::Settings;
use crate::sim::run::RunState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hint {
	/// Movement and automatic fire, shown on the very first wave.
	Controls,
	/// Reactor points, slots, unused energy and the single paid refresh.
	FirstShop,
}

impl Hint {
	pub const ALL: [Hint; 2] = [Hint::Controls, Hint::FirstShop];

	/// Stable key stored in settings.
	pub fn key(self) -> &'static str {
		match self {
			Hint::Controls => "hint_controls",
			Hint::FirstShop => "hint_first_shop",
		}
	}

	pub fn title(self) -> &'static str {
		match self {
			Hint::Controls => "How to play",
			Hint::FirstShop => "The shop and the reactor",
		}
	}

	pub fn lines(self) -> Vec<String> {
		match self {
			Hint::Controls => vec![
				"Move with WASD, the arrow keys or the left stick.".into(),
				"Your weapons fire automatically at the nearest enemy.".into(),
				"Hold the right mouse button or push the right stick to aim by hand.".into(),
				"Escape pauses. Combat is paused while this note is open.".into(),
			],
			Hint::FirstShop => vec![
				"The reactor has six points for four weapon slots, at most three per slot.".into(),
				"A weapon with zero points does not fire; unused points grant nothing.".into(),
				"Buying the same weapon again upgrades it one tier.".into(),
				"You get one paid refresh of the offers per visit.".into(),
			],
		}
	}
}

/// The hint to show now, if its screen is open and it was not acknowledged.
pub fn pending(settings: &Settings, state: RunState, first_wave_tick: bool) -> Option<Hint> {
	let wanted = match state {
		RunState::Wave if first_wave_tick => Hint::Controls,
		RunState::Shop => Hint::FirstShop,
		_ => return None,
	};
	(!settings
		.acknowledged_hints
		.iter()
		.any(|k| k == wanted.key()))
	.then_some(wanted)
}

pub fn acknowledge(settings: &mut Settings, hint: Hint) {
	if !settings.acknowledged_hints.iter().any(|k| k == hint.key()) {
		settings.acknowledged_hints.push(hint.key().to_string());
	}
}

pub fn skip_all(settings: &mut Settings) {
	for hint in Hint::ALL {
		acknowledge(settings, hint);
	}
}

pub fn reset_tutorials(settings: &mut Settings) {
	settings.acknowledged_hints.clear();
}

/// Supported scales are 100, 125 and 150 percent; anything else falls back to 100.
pub fn ui_scale(settings: &Settings) -> f32 {
	match settings.ui_scale_pct {
		125 => 1.25,
		150 => 1.5,
		_ => 1.0,
	}
}
