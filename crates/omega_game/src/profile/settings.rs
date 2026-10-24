//! Settings persisted separately from the profile (GDD section 14, GDD-015 baseline).

use crate::audio::AudioSettings;
use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const SETTINGS_SCHEMA: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ManualAimMode {
	#[default]
	Hold,
	Toggle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum WindowMode {
	#[default]
	Windowed,
	Fullscreen,
}

#[derive(Resource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Settings {
	pub master: u8,
	pub music: u8,
	pub sfx: u8,
	pub muted: bool,
	pub window_mode: WindowMode,
	/// 100, 125 or 150.
	pub ui_scale_pct: u16,
	pub manual_aim: ManualAimMode,
	pub reduced_effects: bool,
	pub camera_shake: bool,
	pub damage_numbers: bool,
	/// Action name to key or button name; empty means the defaults.
	pub bindings: BTreeMap<String, String>,
	pub acknowledged_hints: Vec<String>,
}

impl Default for Settings {
	fn default() -> Self {
		let audio = AudioSettings::default();
		Self {
			master: audio.master,
			music: audio.music,
			sfx: audio.sfx,
			muted: false,
			window_mode: WindowMode::default(),
			ui_scale_pct: 100,
			manual_aim: ManualAimMode::default(),
			reduced_effects: false,
			camera_shake: false,
			damage_numbers: true,
			bindings: BTreeMap::new(),
			acknowledged_hints: Vec::new(),
		}
	}
}

impl Settings {
	pub fn audio(&self) -> AudioSettings {
		AudioSettings {
			master: self.master,
			music: self.music,
			sfx: self.sfx,
			muted: self.muted,
		}
	}
}
