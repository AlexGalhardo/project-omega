//! Event audio mapping (SP-015): presentation facts to cue IDs, mix settings and the
//! once-only result latch. Nothing here touches simulation state or feeds back into rules.

#[cfg(feature = "client")]
pub mod client;

use crate::sim::run::{Run, RunState};
use crate::sim::{PresentationKind, PresentationRecord};
use bevy::prelude::Resource;
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Cue {
	Shot,
	HitEnemy,
	HitPlayer,
	EnemyDeath,
	Pickup,
	Purchase,
	Denied,
	Refresh,
	EnergyTick,
	WaveStart,
	WaveEnd,
	Victory,
	Defeat,
}

impl Cue {
	pub const ALL: [Cue; 13] = [
		Cue::Shot,
		Cue::HitEnemy,
		Cue::HitPlayer,
		Cue::EnemyDeath,
		Cue::Pickup,
		Cue::Purchase,
		Cue::Denied,
		Cue::Refresh,
		Cue::EnergyTick,
		Cue::WaveStart,
		Cue::WaveEnd,
		Cue::Victory,
		Cue::Defeat,
	];

	/// Path under `assets/audio/` and the suffix of the provenance asset ID.
	pub fn name(self) -> &'static str {
		match self {
			Cue::Shot => "shot",
			Cue::HitEnemy => "hit_enemy",
			Cue::HitPlayer => "hit_player",
			Cue::EnemyDeath => "enemy_death",
			Cue::Pickup => "pickup",
			Cue::Purchase => "purchase",
			Cue::Denied => "denied",
			Cue::Refresh => "refresh",
			Cue::EnergyTick => "energy_tick",
			Cue::WaveStart => "wave_start",
			Cue::WaveEnd => "wave_end",
			Cue::Victory => "victory",
			Cue::Defeat => "defeat",
		}
	}

	pub fn file(self) -> String {
		format!("audio/placeholder/{}.ogg", self.name())
	}
}

/// Cues for the facts of one rendered frame. Each cue plays at most once per frame, so a
/// burst of pickups or hits is one sound while every logical event stays in the records.
pub fn cues_for_records(records: &[PresentationRecord]) -> Vec<Cue> {
	let mut seen = BTreeSet::new();
	for record in records {
		let cue = match record.kind {
			PresentationKind::AttackEmitted { .. } => Cue::Shot,
			PresentationKind::HitAccepted { .. } => Cue::HitEnemy,
			PresentationKind::PlayerDamaged { .. } => Cue::HitPlayer,
			PresentationKind::EnemyDied { .. } => Cue::EnemyDeath,
			PresentationKind::PickupCollected { .. } => Cue::Pickup,
			_ => continue,
		};
		seen.insert(cue);
	}
	seen.into_iter().collect()
}

/// Cue for the one-line outcome message the UI shows after an action.
pub fn cue_for_message(message: &str) -> Option<Cue> {
	match message {
		"" => None,
		"Purchased." => Some(Cue::Purchase),
		"Offers refreshed." => Some(Cue::Refresh),
		_ => Some(Cue::Denied),
	}
}

/// Run-state edges to wave and result cues; a repeated state never repeats a cue.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CueTracker {
	last: Option<RunState>,
}

impl CueTracker {
	pub fn observe(&mut self, run: &Run) -> Vec<Cue> {
		let previous = self.last.replace(run.state);
		if previous == Some(run.state) {
			return Vec::new();
		}
		let mut cues = Vec::new();
		if previous == Some(RunState::Wave) {
			cues.push(Cue::WaveEnd);
		}
		match run.state {
			RunState::Wave => cues.push(Cue::WaveStart),
			RunState::Victory => cues.push(Cue::Victory),
			RunState::Defeat => cues.push(Cue::Defeat),
			_ => {}
		}
		cues
	}

	/// Teardown at run end: the next run starts from a clean latch.
	pub fn reset(&mut self) {
		self.last = None;
	}
}

/// Percent sliders from GDD-015; master multiplies the channel gain.
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioSettings {
	pub master: u8,
	pub music: u8,
	pub sfx: u8,
	pub muted: bool,
}

impl Default for AudioSettings {
	fn default() -> Self {
		Self {
			master: 80,
			music: 40,
			sfx: 80,
			muted: false,
		}
	}
}

impl AudioSettings {
	/// Linear gain for sound effects in `0.0..=1.0`.
	pub fn sfx_gain(&self) -> f32 {
		if self.muted {
			return 0.0;
		}
		f32::from(self.master.min(100)) / 100.0 * f32::from(self.sfx.min(100)) / 100.0
	}
}
