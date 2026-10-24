//! Plays cue clips through Bevy audio. Missing clips or a missing output device only
//! cost sound: the simulation never reads anything from here.

use super::{AudioSettings, Cue, CueTracker, cue_for_message, cues_for_records};
use crate::client::presentation::FrameRecords;
use crate::profile::Settings;
use crate::sim::run::Run;
use crate::ui::SettingsDirty;
use crate::ui::client::UiMessage;
use bevy::audio::{AudioPlayer, AudioSource, PlaybackSettings, Volume};
use bevy::prelude::*;
use std::collections::BTreeMap;

#[derive(Resource, Default)]
struct CueCatalog(BTreeMap<Cue, Handle<AudioSource>>);

#[derive(Resource, Default)]
struct Latch {
	tracker: CueTracker,
	last_message: String,
}

pub struct AudioCuePlugin;

impl Plugin for AudioCuePlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<AudioSettings>()
			.init_resource::<CueCatalog>()
			.init_resource::<Latch>()
			.add_systems(Startup, load_cues)
			.add_systems(PostUpdate, (toggle_mute, play_cues).chain());
	}
}

fn load_cues(assets: Res<AssetServer>, mut catalog: ResMut<CueCatalog>) {
	for cue in Cue::ALL {
		catalog.0.insert(cue, assets.load(cue.file()));
	}
}

/// Settings drive the mix; the mute key flips it and records the choice for saving.
fn toggle_mute(
	mut commands: Commands,
	keys: Res<ButtonInput<KeyCode>>,
	mut audio: ResMut<AudioSettings>,
	stored: Option<ResMut<Settings>>,
) {
	if let Some(mut stored) = stored {
		if keys.just_pressed(KeyCode::KeyM) {
			stored.muted = !stored.muted;
			commands.insert_resource(SettingsDirty);
		}
		if stored.is_changed() {
			*audio = stored.audio();
		}
	} else if keys.just_pressed(KeyCode::KeyM) {
		audio.muted = !audio.muted;
	}
}

fn play_cues(
	mut commands: Commands,
	settings: Res<AudioSettings>,
	catalog: Res<CueCatalog>,
	records: Res<FrameRecords>,
	message: Res<UiMessage>,
	run: Option<Res<Run>>,
	mut latch: ResMut<Latch>,
) {
	let mut cues = cues_for_records(&records.0);
	if message.0 != latch.last_message {
		latch.last_message.clone_from(&message.0);
		cues.extend(cue_for_message(&message.0));
	}
	match run {
		Some(run) => cues.extend(latch.tracker.observe(&run)),
		None => latch.tracker.reset(),
	}
	let gain = settings.sfx_gain();
	if gain <= 0.0 {
		return;
	}
	for cue in cues {
		if let Some(handle) = catalog.0.get(&cue) {
			commands.spawn((
				AudioPlayer::new(handle.clone()),
				PlaybackSettings::DESPAWN.with_volume(Volume::Linear(gain)),
			));
		}
	}
}
