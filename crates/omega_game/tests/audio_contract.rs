//! SP-015 prototype acceptance: cue mapping, grouping, once-only results, mix gain,
//! provenance of every clip and a simulation that does not depend on audio.

use bevy::prelude::*;
use omega_game::audio::*;
use omega_game::combat::actor::*;
use omega_game::content::{Profile, load_dir};
use omega_game::sim::run::*;
use omega_game::sim::*;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::path::PathBuf;
use std::sync::Arc;

fn assets() -> PathBuf {
	PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets")
}

fn record(kind: PresentationKind) -> PresentationRecord {
	PresentationRecord {
		sequence: 0,
		tick: 1,
		kind,
	}
}

#[test]
fn every_cue_has_a_clip_and_a_matching_provenance_record() {
	let manifest: serde_json::Value =
		serde_json::from_slice(&std::fs::read(assets().join("audio/provenance.json")).unwrap())
			.unwrap();
	let records = manifest["assets"].as_array().unwrap();
	for cue in Cue::ALL {
		let id = format!("audio_placeholder_{}", cue.name());
		let entry = records
			.iter()
			.find(|r| r["asset_id"] == id)
			.unwrap_or_else(|| panic!("no provenance for {id}"));
		let bytes = std::fs::read(assets().join(cue.file())).unwrap();
		assert_eq!(entry["sha256"], hex(&Sha256::digest(&bytes)), "{id} hash");
		assert_eq!(entry["bytes"], bytes.len());
		assert_eq!(entry["status"], "placeholder");
		for field in ["creator", "acquired", "license", "allowed_uses"] {
			assert!(
				!entry[field].as_str().unwrap_or("").is_empty(),
				"{id}.{field}"
			);
		}
	}
	assert_eq!(
		records.len(),
		Cue::ALL.len(),
		"no unrecorded or orphan clip"
	);
}

fn hex(bytes: &[u8]) -> String {
	bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[test]
fn accepted_events_map_to_cues_and_bursts_group_into_one_sound() {
	let pickups: Vec<_> = (0..5)
		.map(|_| record(PresentationKind::PickupCollected { value: 2 }))
		.collect();
	assert_eq!(cues_for_records(&pickups), vec![Cue::Pickup]);
	let mixed = [
		record(PresentationKind::AttackEmitted {
			attack: 1,
			slot: 0,
			origin: [0.0; 2],
			direction: [1.0, 0.0],
		}),
		record(PresentationKind::HitAccepted {
			attack: 1,
			target: 2,
			damage: 5,
			crit: false,
			position: [0.0; 2],
		}),
		record(PresentationKind::PlayerDamaged {
			damage: 3,
			hp_after: 10,
		}),
		record(PresentationKind::EnemyDied {
			target: 2,
			position: [0.0; 2],
			scrap: 2,
		}),
		record(PresentationKind::Marker(1)),
	];
	let cues: BTreeSet<_> = cues_for_records(&mixed).into_iter().collect();
	assert_eq!(
		cues,
		BTreeSet::from([Cue::Shot, Cue::HitEnemy, Cue::HitPlayer, Cue::EnemyDeath])
	);
	assert_ne!(
		Cue::HitEnemy,
		Cue::HitPlayer,
		"player hit is distinct from enemy damage"
	);
}

#[test]
fn shop_messages_map_to_purchase_refresh_and_denied() {
	assert_eq!(cue_for_message("Purchased."), Some(Cue::Purchase));
	assert_eq!(cue_for_message("Offers refreshed."), Some(Cue::Refresh));
	assert_eq!(cue_for_message("Need 5 more scrap."), Some(Cue::Denied));
	assert_eq!(cue_for_message(""), None);
}

fn run(state: RunState) -> Run {
	Run {
		state,
		wave: 1,
		total_waves: 3,
		result: None,
		settlements: 0,
	}
}

#[test]
fn ac1_result_cue_plays_once_and_wave_edges_pair_up() {
	let mut tracker = CueTracker::default();
	assert_eq!(tracker.observe(&run(RunState::Wave)), vec![Cue::WaveStart]);
	assert!(tracker.observe(&run(RunState::Wave)).is_empty());
	assert_eq!(
		tracker.observe(&run(RunState::Defeat)),
		vec![Cue::WaveEnd, Cue::Defeat]
	);
	for _ in 0..5 {
		assert!(
			tracker.observe(&run(RunState::Defeat)).is_empty(),
			"duplicate input cannot repeat the result cue"
		);
	}
	assert!(tracker.observe(&run(RunState::Summary)).is_empty());
	tracker.reset();
	assert_eq!(tracker.observe(&run(RunState::Wave)), vec![Cue::WaveStart]);
	let mut shop = CueTracker::default();
	shop.observe(&run(RunState::Wave));
	assert_eq!(shop.observe(&run(RunState::Shop)), vec![Cue::WaveEnd]);
	assert_eq!(shop.observe(&run(RunState::Wave)), vec![Cue::WaveStart]);
}

#[test]
fn mix_gain_is_master_times_sfx_and_mute_or_zero_silences() {
	let mut s = AudioSettings::default();
	assert!((s.sfx_gain() - 0.64).abs() < 1e-6);
	s.muted = true;
	assert_eq!(s.sfx_gain(), 0.0);
	s.muted = false;
	s.master = 0;
	assert_eq!(s.sfx_gain(), 0.0);
	s.master = 200;
	s.sfx = 200;
	assert_eq!(s.sfx_gain(), 1.0, "sliders clamp at 100%");
}

fn traced(with_audio: bool) -> Vec<TraceEntry> {
	let dir = assets().join("content-prototype");
	let content = load_dir(&dir, Profile::Prototype).unwrap();
	let bounds = content.catalogs.arenas["arena_foundry"].bounds;
	let mut app = App::new();
	app.add_plugins(omega_game::GameCorePlugin);
	let mut h = SimHarness::with_app(app, RunSeed::from_hex(&"55".repeat(32)).unwrap());
	h.app_mut().insert_resource(ContentRes(Arc::new(content)));
	h.app_mut().insert_resource(ArenaRes(bounds));
	begin_run(h.app_mut().world_mut(), "char_patch", BTreeSet::new()).unwrap();
	let mut tracker = CueTracker::default();
	let mut out = Vec::new();
	for _ in 0..900 {
		h.step(TickInput::default());
		let records = h.drain_presentation();
		if with_audio {
			let _ = cues_for_records(&records);
			let run = h.app().world().resource::<Run>().clone();
			let _ = tracker.observe(&run);
		}
		out.push(h.trace());
	}
	out
}

#[test]
fn ac2_the_simulation_trace_is_identical_with_and_without_audio() {
	assert_eq!(traced(true), traced(false));
}
