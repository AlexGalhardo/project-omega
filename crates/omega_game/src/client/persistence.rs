//! Loads the profile and settings at start and saves them whenever they are marked dirty.
//! A failed write keeps the in-memory value and is reported through [`SaveStatus`]; nothing
//! here ever overwrites bytes that could not be read.

use crate::persist::{DiskStorage, LoadProblem, LoadStatus, SaveError, SlotStore, user_dir};
use crate::profile::settings::SETTINGS_SCHEMA;
use crate::profile::{PROFILE_SCHEMA, Profile, SaveStatus, Settings};
use crate::sim::run::ProfileDirty;
use crate::ui::SettingsDirty;
use crate::ui::menus::{ProfileResetRequested, QuarantineRequested};
use bevy::prelude::*;

#[derive(Resource)]
struct ProfileStore(SlotStore<Profile>);

#[derive(Resource)]
struct SettingsStore(SlotStore<Settings>);

pub struct PersistencePlugin;

impl Plugin for PersistencePlugin {
	fn build(&self, app: &mut App) {
		let mut status = SaveStatus::default();
		let (profile, settings, stores) = match user_dir() {
			Some(dir) => {
				let mut profile_store = SlotStore::new(&dir, "profile", PROFILE_SCHEMA);
				let mut settings_store = SlotStore::new(&dir, "settings", SETTINGS_SCHEMA);
				let (profile, profile_status) = profile_store.load(&DiskStorage);
				let (settings, settings_status) = settings_store.load(&DiskStorage);
				for load in [profile_status, settings_status] {
					if let LoadStatus::Problem(problem) = load {
						status.load_problem = Some(describe(&problem));
					}
				}
				(
					profile.unwrap_or_else(Profile::initial),
					settings.unwrap_or_default(),
					Some((profile_store, settings_store)),
				)
			}
			None => {
				status.last_error =
					Some("LOCALAPPDATA is not set, so progress cannot be saved".to_string());
				status.profile_unsaved = true;
				(Profile::initial(), Settings::default(), None)
			}
		};
		app.insert_resource(profile)
			.insert_resource(settings)
			.insert_resource(status)
			.add_systems(Last, (handle_requests, save_when_dirty).chain());
		if let Some((profile_store, settings_store)) = stores {
			app.insert_resource(ProfileStore(profile_store))
				.insert_resource(SettingsStore(settings_store));
		}
	}
}

fn describe(problem: &LoadProblem) -> String {
	match problem {
		LoadProblem::Corrupt { files } => {
			format!(
				"A save file could not be read ({} file(s) kept for recovery)",
				files.len()
			)
		}
		LoadProblem::FutureSchema { found } => {
			format!("The save was written by a newer version (format {found})")
		}
		LoadProblem::Unreadable(e) => format!("The save folder could not be read: {e}"),
	}
}

fn explain(error: &SaveError) -> String {
	match error {
		SaveError::NeedsRecovery => {
			"Saving is paused until the damaged save is handled".to_string()
		}
		other => format!("Save failed: {other:?}"),
	}
}

#[allow(clippy::too_many_arguments)]
fn save_when_dirty(
	mut commands: Commands,
	profile_dirty: Option<Res<ProfileDirty>>,
	settings_dirty: Option<Res<SettingsDirty>>,
	profile: Option<Res<Profile>>,
	settings: Option<Res<Settings>>,
	profile_store: Option<ResMut<ProfileStore>>,
	settings_store: Option<ResMut<SettingsStore>>,
	mut status: ResMut<SaveStatus>,
) {
	if profile_dirty.is_some()
		&& let (Some(profile), Some(mut store)) = (&profile, profile_store)
	{
		let next = store.0.committed_generation() + 1;
		match store.0.save(&DiskStorage, profile, next) {
			Ok(()) => {
				status.profile_unsaved = false;
				commands.remove_resource::<ProfileDirty>();
			}
			Err(e) => {
				status.profile_unsaved = true;
				status.last_error = Some(explain(&e));
				commands.remove_resource::<ProfileDirty>();
			}
		}
	}
	if settings_dirty.is_some()
		&& let (Some(settings), Some(mut store)) = (&settings, settings_store)
	{
		let next = store.0.committed_generation() + 1;
		match store.0.save(&DiskStorage, settings, next) {
			Ok(()) => status.settings_unsaved = false,
			Err(e) => {
				status.settings_unsaved = true;
				status.last_error = Some(explain(&e));
			}
		}
		commands.remove_resource::<SettingsDirty>();
	}
}

/// Menu requests that touch the files: both only move files aside, never delete them.
fn handle_requests(world: &mut World) {
	if world.remove_resource::<QuarantineRequested>().is_some() {
		let mut moved = false;
		if let Some(mut store) = world.get_resource_mut::<ProfileStore>()
			&& store.0.needs_recovery()
		{
			moved |= store.0.quarantine(&DiskStorage).is_ok();
			world.insert_resource(Profile::initial());
			world.insert_resource(ProfileDirty);
		}
		if let Some(mut store) = world.get_resource_mut::<SettingsStore>()
			&& store.0.needs_recovery()
		{
			moved |= store.0.quarantine(&DiskStorage).is_ok();
			world.insert_resource(Settings::default());
			world.insert_resource(SettingsDirty);
		}
		if moved {
			world.resource_mut::<SaveStatus>().load_problem = None;
		}
	}
	if world.remove_resource::<ProfileResetRequested>().is_some()
		&& let Some(mut store) = world.get_resource_mut::<ProfileStore>()
		&& let Err(e) = store.0.quarantine(&DiskStorage)
	{
		world.resource_mut::<SaveStatus>().last_error = Some(explain(&e));
	}
}
