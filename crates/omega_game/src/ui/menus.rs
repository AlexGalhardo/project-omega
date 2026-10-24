//! Boot, Menu, character selection, Unlocks, Settings, Credits and Reset Profile screens
//! (SP-011, SP-014, GDD sections 4, 13-15). Screens are plain `ScreenView` data and every
//! change goes through `apply_menu`, so the whole flow runs headless.

use super::{Button, ScreenView, UiAction, button};
use crate::combat::actor::{ArenaRes, ContentRes};
use crate::content::{Profile as ContentProfile, load_dir};
use crate::input::bindings::{Action as BindAction, Bindings};
use crate::profile::settings::ManualAimMode;
use crate::profile::{Profile, SaveStatus, Settings};
use crate::sim::run::{ProfileDirty, Run, begin_run};
use crate::ui::{SettingsDirty, hints};
use bevy::prelude::*;
use std::path::PathBuf;

/// Where Boot loads the game data from and which count profile it must satisfy.
#[derive(Resource, Debug, Clone)]
pub struct ContentSource {
	pub dir: PathBuf,
	pub profile: ContentProfile,
}

/// The screen shown while no run exists.
#[derive(Resource, Debug, Clone, PartialEq, Eq)]
pub enum AppScreen {
	Boot { error: Option<String> },
	Menu,
	CharacterSelect,
	Unlocks,
	ResetConfirm,
	Settings,
	Credits,
}

/// Requested by the Quit button; the client turns it into an app exit.
#[derive(Resource, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct QuitRequested;

/// Requested by "Start new profile": the persistence layer quarantines the damaged files.
#[derive(Resource, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct QuarantineRequested;

/// Requested by a confirmed Reset Profile: the persistence layer moves the old files aside.
#[derive(Resource, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ProfileResetRequested;

/// The next key press is captured as the new binding for this action.
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub struct RebindCapture(pub BindAction);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingKind {
	MasterUp,
	MasterDown,
	SfxUp,
	SfxDown,
	MusicUp,
	MusicDown,
	ToggleMute,
	CycleScale,
	ToggleReduced,
	ToggleShake,
	ToggleDamageNumbers,
	ToggleAimMode,
	RestoreBindings,
	ResetTutorials,
	Rebind(BindAction),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuAction {
	RetryBoot,
	NewRun,
	SelectCharacter(usize),
	OpenUnlocks,
	OpenSettings,
	OpenCredits,
	Back,
	PlayAgain,
	BackToMenu,
	AskReset,
	ConfirmReset,
	CancelReset,
	Adjust(SettingKind),
	RetrySave,
	StartNewProfile,
	Quit,
}

fn menu_button(label: impl Into<String>, action: MenuAction, enabled: bool) -> Button {
	button(label, UiAction::Menu(action), enabled)
}

/// Validates the game data. A failure leaves no content installed, so Start stays disabled.
pub fn boot(world: &mut World) {
	let Some(source) = world.get_resource::<ContentSource>().cloned() else {
		world.insert_resource(AppScreen::Boot {
			error: Some("No game data location is configured".to_string()),
		});
		return;
	};
	match load_dir(&source.dir, source.profile) {
		Ok(content) => {
			if let Some(arena) = content.catalogs.arenas.get("arena_foundry") {
				world.insert_resource(ArenaRes(arena.bounds));
			}
			world.insert_resource(ContentRes(std::sync::Arc::new(content)));
			world.insert_resource(AppScreen::Menu);
		}
		Err(errors) => {
			world.remove_resource::<ContentRes>();
			let first: Vec<String> = errors.0.iter().take(3).map(ToString::to_string).collect();
			world.insert_resource(AppScreen::Boot {
				error: Some(first.join("\n")),
			});
		}
	}
}

fn challenge_name_for(world: &World, reward: &str) -> Option<String> {
	let content = world.get_resource::<ContentRes>()?;
	content
		.0
		.catalogs
		.unlocks
		.values()
		.find(|u| u.reward_ids.iter().any(|r| r == reward))
		.map(|u| {
			format!(
				"{} (complete wave {})",
				u.display_name,
				wave_of(&u.condition)
			)
		})
}

fn wave_of(condition: &crate::content::model::UnlockCondition) -> u32 {
	let crate::content::model::UnlockCondition::CompleteWave(wave) = condition;
	*wave
}

fn reward_name(world: &World, id: &str) -> String {
	let Some(content) = world.get_resource::<ContentRes>() else {
		return id.to_string();
	};
	let c = &content.0.catalogs;
	c.characters
		.get(id)
		.map(|x| format!("{} (character)", x.display_name))
		.or_else(|| {
			c.weapons
				.get(id)
				.map(|x| format!("{} (weapon)", x.display_name))
		})
		.or_else(|| {
			c.items
				.get(id)
				.map(|x| format!("{} (item)", x.display_name))
		})
		.unwrap_or_else(|| id.to_string())
}

fn save_lines(world: &World) -> Vec<String> {
	let Some(status) = world.get_resource::<SaveStatus>() else {
		return Vec::new();
	};
	let mut lines = Vec::new();
	if status.profile_unsaved {
		lines.push("Progress is NOT saved. It stays in memory for this session.".to_string());
		if let Some(e) = &status.last_error {
			lines.push(e.clone());
		}
	} else {
		lines.push("Progress saved.".to_string());
	}
	lines
}

/// Screens for the states without a run, plus the Summary extras.
pub fn menu_screen(world: &mut World) -> Option<ScreenView> {
	let screen = world.get_resource::<AppScreen>()?.clone();
	Some(match screen {
		AppScreen::Boot { error } => ScreenView {
			title: "Cannot start the game".into(),
			lines: std::iter::once("The game data failed validation, so no run can start.".to_string())
				.chain(error.into_iter().flat_map(|e| e.lines().map(str::to_string).collect::<Vec<_>>()))
				.collect(),
			buttons: vec![
				menu_button("Retry", MenuAction::RetryBoot, true),
				menu_button("Quit", MenuAction::Quit, true),
			],
		},
		AppScreen::Menu => {
			let ready = world.contains_resource::<ContentRes>();
			let mut lines = vec![String::new()];
			let mut buttons = vec![
				menu_button("New run", MenuAction::NewRun, ready),
				menu_button("Unlocks", MenuAction::OpenUnlocks, ready),
				menu_button("Settings", MenuAction::OpenSettings, true),
				menu_button("Credits", MenuAction::OpenCredits, true),
				menu_button("Quit", MenuAction::Quit, true),
			];
			if let Some(problem) = world
				.get_resource::<SaveStatus>()
				.and_then(|s| s.load_problem.clone())
			{
				lines.push(format!("Save problem: {problem}"));
				lines.push("Your damaged files are kept. Starting a new profile moves them aside.".into());
				buttons.insert(
					4,
					menu_button("Start new profile", MenuAction::StartNewProfile, true),
				);
			}
			ScreenView {
				title: "Project Omega".into(),
				lines,
				buttons,
			}
		}
		AppScreen::CharacterSelect => character_screen(world),
		AppScreen::Unlocks => unlocks_screen(world),
		AppScreen::ResetConfirm => {
			let granted: Vec<String> = world
				.get_resource::<Profile>()
				.map(|p| p.granted.iter().cloned().collect())
				.unwrap_or_default();
			let mut lines = vec![
				"This erases your unlock progress. Settings are not changed.".to_string(),
				"You would lose these unlocks:".to_string(),
			];
			if granted.is_empty() {
				lines.push("  (nothing earned yet)".into());
			}
			for id in granted {
				let name = world
					.get_resource::<ContentRes>()
					.and_then(|c| c.0.catalogs.unlocks.get(&id).map(|u| u.display_name.clone()))
					.unwrap_or(id);
				lines.push(format!("  {name}"));
			}
			ScreenView {
				title: "Reset profile?".into(),
				lines,
				buttons: vec![
					menu_button("Reset profile", MenuAction::ConfirmReset, true),
					menu_button("Keep my profile", MenuAction::CancelReset, true),
				],
			}
		}
		AppScreen::Settings => settings_screen(world),
		AppScreen::Credits => ScreenView {
			title: "Credits".into(),
			lines: vec![
				"Project Omega is an original game and 2D engine layer by the project owner.".into(),
				"All shipped art, sounds and data are original and generated from project scripts.".into(),
				"Built with the Bevy engine (MIT or Apache-2.0); license files and the software bill of materials ship in the release archive.".into(),
				"The default interface font is Fira Mono (SIL Open Font License), bundled by Bevy.".into(),
			],
			buttons: vec![menu_button("Back", MenuAction::Back, true)],
		},
	})
}

fn character_screen(world: &World) -> ScreenView {
	let mut lines = Vec::new();
	let mut buttons = Vec::new();
	let profile = world
		.get_resource::<Profile>()
		.cloned()
		.unwrap_or_else(Profile::initial);
	if let Some(content) = world.get_resource::<ContentRes>() {
		for (i, (id, character)) in content.0.catalogs.characters.iter().enumerate() {
			let weapon = content
				.0
				.catalogs
				.weapons
				.get(&character.starting_weapon_id)
				.map_or("?", |w| w.display_name.as_str());
			let open = profile.is_available(&character.availability);
			buttons.push(menu_button(
				format!("{} - starts with {weapon}", character.display_name),
				MenuAction::SelectCharacter(i),
				open,
			));
			if !open {
				let how = challenge_name_for(world, id).unwrap_or_else(|| "a challenge".into());
				lines.push(format!("{} is locked: earn {how}.", character.display_name));
			}
		}
	}
	buttons.push(menu_button("Back", MenuAction::Back, true));
	ScreenView {
		title: "Choose a character".into(),
		lines,
		buttons,
	}
}

fn unlocks_screen(world: &World) -> ScreenView {
	let profile = world
		.get_resource::<Profile>()
		.cloned()
		.unwrap_or_else(Profile::initial);
	let mut lines = Vec::new();
	if let Some(content) = world.get_resource::<ContentRes>() {
		for unlock in content.0.catalogs.unlocks.values() {
			let status = if profile.granted.contains(&unlock.id) {
				"EARNED"
			} else {
				"not earned"
			};
			lines.push(format!(
				"{}: finish wave {} alive in one run - {status}",
				unlock.display_name,
				wave_of(&unlock.condition)
			));
			let rewards: Vec<String> = unlock
				.reward_ids
				.iter()
				.map(|r| reward_name(world, r))
				.collect();
			lines.push(format!("  Rewards: {}", rewards.join(", ")));
		}
	}
	lines.push(
		"Each challenge is earned once; progress never carries between runs as a count.".into(),
	);
	ScreenView {
		title: "Unlocks".into(),
		lines,
		buttons: vec![
			menu_button("Reset profile...", MenuAction::AskReset, true),
			menu_button("Back", MenuAction::Back, true),
		],
	}
}

fn settings_screen(world: &World) -> ScreenView {
	let s = world
		.get_resource::<Settings>()
		.cloned()
		.unwrap_or_default();
	let lines = vec![
		format!(
			"Master volume {}%   Music {}%   Effects {}%   Muted: {}",
			s.master, s.music, s.sfx, s.muted
		),
		format!(
			"UI scale {}%   Manual aim: {:?}",
			s.ui_scale_pct, s.manual_aim
		),
		format!(
			"Reduced effects: {}   Screen shake: {}   Damage numbers: {}",
			s.reduced_effects, s.camera_shake, s.damage_numbers
		),
		"Rebinding: pick an action, then press the new key (Escape cancels).".to_string(),
	];
	let adjust =
		|label: &str, kind: SettingKind| menu_button(label, MenuAction::Adjust(kind), true);
	let mut buttons = vec![
		adjust("Master -", SettingKind::MasterDown),
		adjust("Master +", SettingKind::MasterUp),
		adjust("Music -", SettingKind::MusicDown),
		adjust("Music +", SettingKind::MusicUp),
		adjust("Effects -", SettingKind::SfxDown),
		adjust("Effects +", SettingKind::SfxUp),
		adjust("Toggle mute", SettingKind::ToggleMute),
		adjust("Cycle UI scale (100/125/150)", SettingKind::CycleScale),
		adjust("Toggle reduced effects", SettingKind::ToggleReduced),
		adjust("Toggle screen shake", SettingKind::ToggleShake),
		adjust("Toggle damage numbers", SettingKind::ToggleDamageNumbers),
		adjust("Toggle manual aim hold/toggle", SettingKind::ToggleAimMode),
	];
	let (bindings, _) = Bindings::from_settings(&s.bindings);
	for action in BindAction::ALL {
		buttons.push(adjust(
			&format!(
				"Rebind {} ({})",
				action.name(),
				bindings.inputs(action).join(" / ")
			),
			SettingKind::Rebind(action),
		));
	}
	buttons.push(adjust(
		"Restore default bindings",
		SettingKind::RestoreBindings,
	));
	buttons.push(adjust("Reset tutorials", SettingKind::ResetTutorials));
	buttons.push(menu_button("Back", MenuAction::Back, true));
	ScreenView {
		title: "Settings".into(),
		lines,
		buttons,
	}
}

/// Extra lines and buttons for the Summary and result screens.
pub fn result_extras(world: &World, summary: bool) -> (Vec<String>, Vec<Button>) {
	let mut lines = save_lines(world);
	let mut buttons = Vec::new();
	if world
		.get_resource::<SaveStatus>()
		.is_some_and(|s| s.profile_unsaved)
	{
		buttons.push(menu_button("Retry save", MenuAction::RetrySave, true));
		lines.push("You can keep playing unsaved; retry any time.".into());
	}
	if summary {
		buttons.push(menu_button("Play again", MenuAction::PlayAgain, true));
		buttons.push(menu_button("Back to menu", MenuAction::BackToMenu, true));
	}
	(lines, buttons)
}

fn step(value: u8, up: bool) -> u8 {
	if up {
		value.saturating_add(10).min(100)
	} else {
		value.saturating_sub(10)
	}
}

fn adjust(world: &mut World, kind: SettingKind) -> String {
	if let SettingKind::Rebind(action) = kind {
		world.insert_resource(RebindCapture(action));
		return format!("Press the new key for {} (Escape cancels).", action.name());
	}
	let Some(mut s) = world.get_resource_mut::<Settings>() else {
		return String::new();
	};
	match kind {
		SettingKind::MasterUp => s.master = step(s.master, true),
		SettingKind::MasterDown => s.master = step(s.master, false),
		SettingKind::SfxUp => s.sfx = step(s.sfx, true),
		SettingKind::SfxDown => s.sfx = step(s.sfx, false),
		SettingKind::MusicUp => s.music = step(s.music, true),
		SettingKind::MusicDown => s.music = step(s.music, false),
		SettingKind::ToggleMute => s.muted = !s.muted,
		SettingKind::CycleScale => {
			s.ui_scale_pct = match s.ui_scale_pct {
				100 => 125,
				125 => 150,
				_ => 100,
			}
		}
		SettingKind::ToggleReduced => s.reduced_effects = !s.reduced_effects,
		SettingKind::ToggleShake => s.camera_shake = !s.camera_shake,
		SettingKind::ToggleDamageNumbers => s.damage_numbers = !s.damage_numbers,
		SettingKind::ToggleAimMode => {
			s.manual_aim = match s.manual_aim {
				ManualAimMode::Hold => ManualAimMode::Toggle,
				ManualAimMode::Toggle => ManualAimMode::Hold,
			}
		}
		SettingKind::RestoreBindings => s.bindings.clear(),
		SettingKind::ResetTutorials => hints::reset_tutorials(&mut s),
		SettingKind::Rebind(_) => {}
	}
	world.insert_resource(SettingsDirty);
	String::new()
}

/// Applies a captured key as a rebinding; reports conflicts and keeps the old binding.
pub fn apply_rebind(world: &mut World, action: BindAction, input: &str) -> String {
	world.remove_resource::<RebindCapture>();
	let Some(mut settings) = world.get_resource_mut::<Settings>() else {
		return String::new();
	};
	let (mut bindings, _) = Bindings::from_settings(&settings.bindings);
	match bindings.rebind(action, input) {
		Ok(()) => {
			settings.bindings = bindings.to_settings();
			world.insert_resource(SettingsDirty);
			format!("{} is now {input}.", action.name())
		}
		Err(crate::input::bindings::BindError::Conflict { with }) => {
			format!(
				"{input} already triggers {}; pick another key.",
				with.name()
			)
		}
		Err(_) => "That binding is not possible.".to_string(),
	}
}

pub fn apply_menu(world: &mut World, action: MenuAction) -> String {
	let set = |world: &mut World, screen: AppScreen| world.insert_resource(screen);
	match action {
		MenuAction::RetryBoot => boot(world),
		MenuAction::NewRun => {
			if world.contains_resource::<ContentRes>() {
				set(world, AppScreen::CharacterSelect);
			}
		}
		MenuAction::SelectCharacter(index) => return start_run(world, index),
		MenuAction::OpenUnlocks => set(world, AppScreen::Unlocks),
		MenuAction::OpenSettings => set(world, AppScreen::Settings),
		MenuAction::OpenCredits => set(world, AppScreen::Credits),
		MenuAction::Back => {
			let next = match world.get_resource::<AppScreen>() {
				Some(AppScreen::ResetConfirm) => AppScreen::Unlocks,
				_ => AppScreen::Menu,
			};
			set(world, next);
		}
		MenuAction::PlayAgain => {
			world.remove_resource::<Run>();
			set(world, AppScreen::CharacterSelect);
		}
		MenuAction::BackToMenu => {
			world.remove_resource::<Run>();
			set(world, AppScreen::Menu);
		}
		MenuAction::AskReset => set(world, AppScreen::ResetConfirm),
		MenuAction::CancelReset => set(world, AppScreen::Unlocks),
		MenuAction::ConfirmReset => {
			world.insert_resource(Profile::initial());
			world.insert_resource(ProfileResetRequested);
			world.insert_resource(ProfileDirty);
			set(world, AppScreen::Unlocks);
		}
		MenuAction::Adjust(kind) => return adjust(world, kind),
		MenuAction::RetrySave => {
			world.insert_resource(ProfileDirty);
		}
		MenuAction::StartNewProfile => {
			world.insert_resource(QuarantineRequested);
		}
		MenuAction::Quit => {
			world.insert_resource(QuitRequested);
		}
	}
	String::new()
}

fn start_run(world: &mut World, index: usize) -> String {
	let Some(content) = world.get_resource::<ContentRes>().map(|c| c.0.clone()) else {
		return "The game data is not loaded.".to_string();
	};
	let Some(id) = content.catalogs.characters.keys().nth(index).cloned() else {
		return String::new();
	};
	let unlocked = world
		.get_resource::<Profile>()
		.map(Profile::unlock_snapshot)
		.unwrap_or_default();
	match begin_run(world, &id, unlocked) {
		Ok(()) => {
			world.remove_resource::<AppScreen>();
			world.insert_resource(crate::ui::UiMode::Normal);
			String::new()
		}
		Err(crate::sim::run::BeginError::Locked) => "That character is still locked.".to_string(),
		Err(other) => format!("Cannot start: {other:?}"),
	}
}
