//! Profile progression and settings data (SP-010, GDD sections 13-14).
//!
//! Grant IDs are the authority; availability always derives from the initial pool plus
//! those grants. Persistence of these values lives in `crate::persist`.

pub mod settings;

use crate::content::ContentSnapshot;
use crate::content::model::{Availability, UnlockCondition};
use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub use settings::Settings;

pub const PROFILE_SCHEMA: u32 = 1;

#[derive(Resource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Profile {
	pub granted: BTreeSet<String>,
	pub completed_runs: u32,
	pub victories: u32,
}

impl Profile {
	pub fn initial() -> Self {
		Self {
			granted: BTreeSet::new(),
			completed_runs: 0,
			victories: 0,
		}
	}

	/// The unlock snapshot a new run starts with; later grants never change a running run.
	pub fn unlock_snapshot(&self) -> BTreeSet<String> {
		self.granted.clone()
	}

	pub fn is_available(&self, availability: &Availability) -> bool {
		match availability {
			Availability::Initial => true,
			Availability::Unlock(id) => self.granted.contains(id),
		}
	}
}

/// Challenges completed by finishing `wave` alive, granted once. Returns the new grants in ID
/// order; repeating the same completion grants nothing and changes nothing.
pub fn grant_for_wave_completion(
	content: &ContentSnapshot,
	profile: &mut Profile,
	wave: u32,
) -> Vec<String> {
	let mut granted = Vec::new();
	for (id, unlock) in &content.catalogs.unlocks {
		let UnlockCondition::CompleteWave(target) = unlock.condition;
		if target == wave && profile.granted.insert(id.clone()) {
			granted.push(id.clone());
		}
	}
	granted
}

/// Ids that appear in the profile but not in the content (retired content): kept, reported.
pub fn unknown_grants(content: &ContentSnapshot, profile: &Profile) -> Vec<String> {
	profile
		.granted
		.iter()
		.filter(|id| !content.catalogs.unlocks.contains_key(*id))
		.cloned()
		.collect()
}
