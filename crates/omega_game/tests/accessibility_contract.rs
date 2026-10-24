//! GAME-P3-014: UI scale, reduced effects and first-run hints that survive a settings reload.

use bevy::prelude::*;
use omega_2d::visuals::cues::EffectsLevel;
use omega_game::persist::{DiskStorage, SlotStore};
use omega_game::profile::Settings;
use omega_game::profile::settings::SETTINGS_SCHEMA;
use omega_game::sim::SimPaused;
use omega_game::sim::run::RunState;
use omega_game::ui::hints::*;
use omega_game::ui::{SettingsDirty, UiAction, UiMode, apply_action};

#[test]
fn ui_scale_accepts_only_the_three_supported_values() {
	let mut s = Settings::default();
	assert_eq!(ui_scale(&s), 1.0);
	for (pct, scale) in [(125, 1.25), (150, 1.5), (100, 1.0), (110, 1.0), (0, 1.0)] {
		s.ui_scale_pct = pct;
		assert_eq!(ui_scale(&s), scale, "{pct}");
	}
}

#[test]
fn reduced_effects_map_to_the_decoration_free_level() {
	let mut s = Settings::default();
	assert_eq!(s.effects(), EffectsLevel::Full);
	s.reduced_effects = true;
	assert_eq!(s.effects(), EffectsLevel::Reduced);
	assert!(!s.camera_shake, "shake defaults off");
}

#[test]
fn hints_show_once_on_their_screens_and_never_after_acknowledgment() {
	let mut s = Settings::default();
	assert_eq!(
		pending(&s, RunState::Wave, false),
		None,
		"not on later ticks"
	);
	assert_eq!(pending(&s, RunState::Wave, true), Some(Hint::Controls));
	assert_eq!(pending(&s, RunState::Shop, false), Some(Hint::FirstShop));
	assert_eq!(pending(&s, RunState::Victory, true), None);
	acknowledge(&mut s, Hint::Controls);
	acknowledge(&mut s, Hint::Controls);
	assert_eq!(s.acknowledged_hints.len(), 1, "no duplicate entries");
	assert_eq!(pending(&s, RunState::Wave, true), None);
	assert_eq!(pending(&s, RunState::Shop, false), Some(Hint::FirstShop));
	skip_all(&mut s);
	assert_eq!(pending(&s, RunState::Shop, false), None);
	reset_tutorials(&mut s);
	assert_eq!(pending(&s, RunState::Wave, true), Some(Hint::Controls));
}

#[test]
fn hint_texts_cover_the_gdd_topics() {
	let shop = Hint::FirstShop.lines().join(" ");
	for topic in [
		"six points",
		"four weapon slots",
		"zero points",
		"unused points",
		"one paid refresh",
	] {
		assert!(shop.contains(topic), "{topic} missing: {shop}");
	}
	let controls = Hint::Controls.lines().join(" ");
	assert!(controls.contains("automatically") && controls.contains("aim by hand"));
}

#[test]
fn dismissing_a_hint_records_it_resumes_combat_and_owes_a_save() {
	let mut world = World::new();
	world.insert_resource(Settings::default());
	world.insert_resource(UiMode::Hint(Hint::Controls));
	world.insert_resource(SimPaused(true));
	apply_action(&mut world, UiAction::DismissHint);
	assert!(
		world
			.resource::<Settings>()
			.acknowledged_hints
			.contains(&"hint_controls".to_string())
	);
	assert_eq!(*world.resource::<UiMode>(), UiMode::Normal);
	assert!(
		!world.resource::<SimPaused>().0,
		"combat resumes only after the note is dismissed"
	);
	assert!(world.contains_resource::<SettingsDirty>());
}

#[test]
fn skip_all_and_reset_tutorials_through_actions() {
	let mut world = World::new();
	world.insert_resource(Settings::default());
	world.insert_resource(UiMode::Hint(Hint::FirstShop));
	world.insert_resource(SimPaused(true));
	apply_action(&mut world, UiAction::SkipAllHints);
	assert_eq!(world.resource::<Settings>().acknowledged_hints.len(), 2);
	apply_action(&mut world, UiAction::ResetTutorials);
	assert!(world.resource::<Settings>().acknowledged_hints.is_empty());
}

#[test]
fn settings_and_acknowledgments_survive_a_reload() {
	let dir = std::env::temp_dir().join(format!("omega-a11y-{}", std::process::id()));
	let _ = std::fs::remove_dir_all(&dir);
	let mut settings = Settings {
		ui_scale_pct: 150,
		reduced_effects: true,
		muted: true,
		..Settings::default()
	};
	acknowledge(&mut settings, Hint::Controls);
	let mut store: SlotStore<Settings> = SlotStore::new(&dir, "settings", SETTINGS_SCHEMA);
	store.save(&DiskStorage, &settings, 1).unwrap();
	let mut again: SlotStore<Settings> = SlotStore::new(&dir, "settings", SETTINGS_SCHEMA);
	let (loaded, _) = again.load(&DiskStorage);
	assert_eq!(loaded, Some(settings));
}
