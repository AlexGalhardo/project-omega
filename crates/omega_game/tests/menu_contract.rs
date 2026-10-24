//! SP-011 / SP-014: Boot validation, Menu, character selection, Unlocks, Settings, Reset
//! Profile and Unsaved/Retry as screen data driven by typed actions.

use bevy::prelude::*;
use omega_game::combat::actor::ContentRes;
use omega_game::content::Profile as ContentProfile;
use omega_game::input::bindings::Action;
use omega_game::profile::{Profile, SaveStatus, Settings, grant_for_wave_completion};
use omega_game::sim::SimPaused;
use omega_game::sim::run::{ProfileDirty, Run, RunState, abandon};
use omega_game::ui::menus::*;
use omega_game::ui::*;
use std::path::PathBuf;

fn fixtures(name: &str) -> PathBuf {
	PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/content/{name}"))
}

fn world_with(dir: PathBuf, profile: ContentProfile) -> World {
	let mut world = World::new();
	world.insert_resource(ContentSource { dir, profile });
	world.insert_resource(Profile::initial());
	world.insert_resource(Settings::default());
	world.insert_resource(UiMode::Normal);
	world.insert_resource(SimPaused(false));
	world.init_resource::<omega_game::combat::actor::Scrap>();
	world.init_resource::<omega_game::sim::IdAllocator>();
	world.init_resource::<omega_game::sim::SimTick>();
	world.init_resource::<omega_game::sim::PresentationQueue>();
	world.init_resource::<omega_game::combat::weapons::EmittedShapes>();
	world.init_resource::<omega_game::combat::weapons::HitLedger>();
	world.init_resource::<omega_game::combat::damage::HitCandidates>();
	world.init_resource::<omega_game::combat::damage::HostileHits>();
	world.init_resource::<omega_game::input::AimOrigin>();
	boot(&mut world);
	world
}

fn screen(world: &mut World) -> ScreenView {
	screen_view(world, UiMode::Normal, false).expect("a screen is showing")
}

fn press(world: &mut World, label: &str) -> String {
	let view = screen(world);
	let button = view
		.buttons
		.iter()
		.find(|b| b.label.starts_with(label))
		.unwrap_or_else(|| panic!("no button {label:?} in {:?}", view.buttons));
	assert!(button.enabled, "{label} is disabled");
	apply_action(world, button.action)
}

#[test]
fn boot_with_valid_data_reaches_the_menu_with_every_entry_reachable() {
	let mut w = world_with(fixtures("full"), ContentProfile::Full);
	assert_eq!(*w.resource::<AppScreen>(), AppScreen::Menu);
	let menu = screen(&mut w);
	assert_eq!(menu.title, "Project Omega");
	for label in ["New run", "Unlocks", "Settings", "Credits", "Quit"] {
		assert!(
			menu.buttons.iter().any(|b| b.label == label && b.enabled),
			"{label}"
		);
	}
}

#[test]
fn sp011_ac5_invalid_data_blocks_start_and_retry_recovers() {
	let empty = std::env::temp_dir().join(format!("omega-boot-empty-{}", std::process::id()));
	std::fs::create_dir_all(&empty).unwrap();
	let mut w = world_with(empty, ContentProfile::Full);
	assert!(matches!(
		w.resource::<AppScreen>(),
		AppScreen::Boot { error: Some(_) }
	));
	assert!(
		!w.contains_resource::<ContentRes>(),
		"no partial content is installed"
	);
	let view = screen(&mut w);
	assert_eq!(view.title, "Cannot start the game");
	assert!(view.buttons.iter().any(|b| b.label == "Retry"));
	assert!(view.buttons.iter().any(|b| b.label == "Quit"));
	assert!(
		view.buttons.iter().all(|b| b.label != "New run"),
		"Start is not offered"
	);
	apply_menu(&mut w, MenuAction::NewRun);
	assert!(!w.contains_resource::<Run>());
	w.insert_resource(ContentSource {
		dir: fixtures("full"),
		profile: ContentProfile::Full,
	});
	press(&mut w, "Retry");
	assert_eq!(*w.resource::<AppScreen>(), AppScreen::Menu);
}

#[test]
fn character_selection_locks_unearned_characters_with_the_reason() {
	let mut w = world_with(fixtures("full"), ContentProfile::Full);
	press(&mut w, "New run");
	let view = screen(&mut w);
	assert_eq!(view.title, "Choose a character");
	let open: Vec<_> = view
		.buttons
		.iter()
		.filter(|b| b.enabled && b.label != "Back")
		.collect();
	assert_eq!(open.len(), 1, "only Patch is unlocked at the start");
	let text = view.lines.join("\n");
	assert!(
		text.contains("Coil") && text.contains("locked") && text.contains("First Shift"),
		"{text}"
	);
	// A locked pick is refused even if the action is sent anyway.
	apply_menu(&mut w, MenuAction::SelectCharacter(0));
	assert!(!w.contains_resource::<Run>());
}

#[test]
fn selecting_an_unlocked_character_starts_a_run_and_leaves_the_menu() {
	let mut w = world_with(fixtures("full"), ContentProfile::Full);
	press(&mut w, "New run");
	let view = screen(&mut w);
	let patch = view
		.buttons
		.iter()
		.find(|b| b.enabled && b.label.starts_with("Patch"))
		.unwrap();
	apply_action(&mut w, patch.action);
	assert_eq!(w.resource::<Run>().state, RunState::Wave);
	assert!(!w.contains_resource::<AppScreen>());
	assert!(
		screen_view(&mut w, UiMode::Normal, false).is_none(),
		"live combat shows no panel"
	);
}

#[test]
fn an_earned_character_becomes_selectable() {
	let mut w = world_with(fixtures("full"), ContentProfile::Full);
	let content = w.resource::<ContentRes>().0.clone();
	grant_for_wave_completion(&content, &mut w.resource_mut::<Profile>(), 3);
	press(&mut w, "New run");
	let view = screen(&mut w);
	assert_eq!(
		view.buttons
			.iter()
			.filter(|b| b.enabled && b.label != "Back")
			.count(),
		2
	);
}

#[test]
fn unlocks_screen_shows_conditions_rewards_and_earned_status() {
	let mut w = world_with(fixtures("full"), ContentProfile::Full);
	press(&mut w, "Unlocks");
	let text = screen(&mut w).lines.join("\n");
	for name in ["First Shift", "Press Operator", "Overtime"] {
		assert!(text.contains(name), "{name}");
	}
	assert!(text.contains("finish wave 3 alive") && text.contains("Rewards:"));
	assert!(!text.contains("EARNED"));
	let content = w.resource::<ContentRes>().0.clone();
	grant_for_wave_completion(&content, &mut w.resource_mut::<Profile>(), 3);
	assert!(screen(&mut w).lines.join("\n").contains("EARNED"));
}

#[test]
fn reset_profile_needs_a_separate_confirmation_and_keeps_settings() {
	let mut w = world_with(fixtures("full"), ContentProfile::Full);
	let content = w.resource::<ContentRes>().0.clone();
	grant_for_wave_completion(&content, &mut w.resource_mut::<Profile>(), 3);
	w.resource_mut::<Settings>().master = 35;
	press(&mut w, "Unlocks");
	press(&mut w, "Reset profile");
	let confirm = screen(&mut w);
	assert_eq!(confirm.title, "Reset profile?");
	assert!(
		confirm.lines.join("\n").contains("First Shift"),
		"lists what is lost"
	);
	assert!(
		!w.resource::<Profile>().granted.is_empty(),
		"nothing changes before confirming"
	);
	press(&mut w, "Keep my profile");
	assert!(!w.resource::<Profile>().granted.is_empty());
	press(&mut w, "Reset profile");
	press(&mut w, "Reset profile");
	assert_eq!(*w.resource::<Profile>(), Profile::initial());
	assert!(w.contains_resource::<ProfileResetRequested>());
	assert_eq!(w.resource::<Settings>().master, 35, "settings survive");
}

#[test]
fn settings_adjust_clamp_cycle_and_mark_a_save() {
	let mut w = world_with(fixtures("full"), ContentProfile::Full);
	press(&mut w, "Settings");
	for _ in 0..12 {
		press(&mut w, "Master +");
	}
	assert_eq!(w.resource::<Settings>().master, 100);
	for _ in 0..12 {
		press(&mut w, "Music -");
	}
	assert_eq!(w.resource::<Settings>().music, 0);
	let scales: Vec<u16> = (0..3)
		.map(|_| {
			press(&mut w, "Cycle UI scale");
			w.resource::<Settings>().ui_scale_pct
		})
		.collect();
	assert_eq!(scales, vec![125, 150, 100]);
	press(&mut w, "Toggle reduced effects");
	assert!(w.resource::<Settings>().reduced_effects);
	assert!(w.contains_resource::<SettingsDirty>());
	press(&mut w, "Back");
	assert_eq!(*w.resource::<AppScreen>(), AppScreen::Menu);
}

#[test]
fn rebinding_reports_conflicts_and_restore_defaults_clears_overrides() {
	let mut w = world_with(fixtures("full"), ContentProfile::Full);
	press(&mut w, "Settings");
	let msg = apply_rebind(&mut w, Action::MoveUp, "KeyS");
	assert!(msg.contains("already triggers move_down"), "{msg}");
	assert!(
		w.resource::<Settings>().bindings.is_empty(),
		"a conflict changes nothing"
	);
	let ok = apply_rebind(&mut w, Action::OpenEnergy, "KeyQ");
	assert!(ok.contains("now KeyQ"), "{ok}");
	assert_eq!(w.resource::<Settings>().bindings.len(), 1);
	press(&mut w, "Restore default bindings");
	assert!(w.resource::<Settings>().bindings.is_empty());
	let started = press(&mut w, "Rebind back");
	assert!(started.contains("Escape cancels"));
	assert!(w.contains_resource::<RebindCapture>());
}

fn run_to_summary(w: &mut World) {
	if *w.resource::<AppScreen>() == AppScreen::Menu {
		press(w, "New run");
	}
	let view = screen(w);
	let patch = view
		.buttons
		.iter()
		.find(|b| b.enabled && b.label.starts_with("Patch"))
		.unwrap();
	apply_action(w, patch.action);
	abandon(w).unwrap();
}

#[test]
fn summary_offers_play_again_and_back_with_an_honest_save_line() {
	let mut w = world_with(fixtures("full"), ContentProfile::Full);
	w.insert_resource(SaveStatus::default());
	run_to_summary(&mut w);
	let view = screen(&mut w);
	assert_eq!(view.title, "Summary");
	assert!(view.lines.iter().any(|l| l == "Progress saved."));
	press(&mut w, "Play again");
	assert!(!w.contains_resource::<Run>());
	assert_eq!(*w.resource::<AppScreen>(), AppScreen::CharacterSelect);
	run_to_summary(&mut w);
	press(&mut w, "Back to menu");
	assert_eq!(*w.resource::<AppScreen>(), AppScreen::Menu);
}

#[test]
fn a_failed_save_is_shown_as_unsaved_with_retry_and_never_as_saved() {
	let mut w = world_with(fixtures("full"), ContentProfile::Full);
	w.insert_resource(SaveStatus {
		profile_unsaved: true,
		last_error: Some("Save failed: Io(\"disk full\")".into()),
		..SaveStatus::default()
	});
	run_to_summary(&mut w);
	let view = screen(&mut w);
	let text = view.lines.join("\n");
	assert!(
		text.contains("NOT saved") && text.contains("disk full"),
		"{text}"
	);
	assert!(!text.contains("Progress saved."));
	press(&mut w, "Retry save");
	assert!(w.contains_resource::<ProfileDirty>(), "retry owes a save");
}

#[test]
fn a_load_problem_offers_a_new_profile_and_never_silently_resets() {
	let mut w = world_with(fixtures("full"), ContentProfile::Full);
	w.insert_resource(SaveStatus {
		load_problem: Some("A save file could not be read".into()),
		..SaveStatus::default()
	});
	let menu = screen(&mut w);
	assert!(menu.lines.join("\n").contains("damaged files are kept"));
	press(&mut w, "Start new profile");
	assert!(w.contains_resource::<QuarantineRequested>());
}

#[test]
fn credits_and_quit_work() {
	let mut w = world_with(fixtures("full"), ContentProfile::Full);
	press(&mut w, "Credits");
	assert!(screen(&mut w).lines.join("\n").contains("Bevy"));
	press(&mut w, "Back");
	press(&mut w, "Quit");
	assert!(w.contains_resource::<QuitRequested>());
}
