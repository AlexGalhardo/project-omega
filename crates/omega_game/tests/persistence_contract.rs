//! SP-010 acceptance: idempotent grants with a frozen run pool, two-slot saves that survive a
//! fault at every protocol boundary, recovery that never overwrites unreadable bytes, and
//! settings that outlive a profile reset.

use bevy::prelude::*;
use omega_game::combat::actor::*;
use omega_game::content::{Profile as ContentProfile, load_dir};
use omega_game::persist::*;
use omega_game::profile::settings::SETTINGS_SCHEMA;
use omega_game::profile::{PROFILE_SCHEMA, Profile, Settings, grant_for_wave_completion};
use omega_game::shop::{Offer, ShopVisit};
use omega_game::sim::run::*;
use omega_game::sim::*;
use std::cell::Cell;
use std::collections::BTreeSet;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

fn full_content() -> omega_game::content::ContentSnapshot {
	let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/content/full");
	load_dir(&dir, ContentProfile::Full).expect("full fixture")
}

#[test]
fn ac1_wave_three_grants_the_first_shift_bundle_once() {
	let content = full_content();
	let mut profile = Profile::initial();
	assert!(grant_for_wave_completion(&content, &mut profile, 1).is_empty());
	assert_eq!(
		grant_for_wave_completion(&content, &mut profile, 3),
		vec!["unlock_first_shift".to_string()]
	);
	assert_eq!(
		profile.granted,
		BTreeSet::from(["unlock_first_shift".to_string()])
	);
	let snapshot = profile.unlock_snapshot();
	for (id, character) in &content.catalogs.characters {
		let open = profile.is_available(&character.availability);
		assert_eq!(open, id == "char_patch" || id == "char_coil", "{id}");
	}
	assert!(snapshot.contains("unlock_first_shift"));
}

#[test]
fn ac2_duplicate_evaluation_grants_nothing_more() {
	let content = full_content();
	let mut profile = Profile::initial();
	grant_for_wave_completion(&content, &mut profile, 3);
	let after_first = profile.clone();
	for _ in 0..5 {
		assert!(grant_for_wave_completion(&content, &mut profile, 3).is_empty());
	}
	assert_eq!(profile, after_first);
}

#[test]
fn full_availability_matches_the_gdd_pools() {
	let content = full_content();
	let initial = Profile::initial();
	let mut all = Profile::initial();
	for wave in [3, 5, 8, 10] {
		grant_for_wave_completion(&content, &mut all, wave);
	}
	let count = |p: &Profile| {
		(
			content
				.catalogs
				.characters
				.values()
				.filter(|c| p.is_available(&c.availability))
				.count(),
			content
				.catalogs
				.weapons
				.values()
				.filter(|w| p.is_available(&w.availability))
				.count(),
			content
				.catalogs
				.items
				.values()
				.filter(|i| p.is_available(&i.availability))
				.count(),
		)
	};
	assert_eq!(count(&initial), (1, 6, 20));
	assert_eq!(count(&all), (4, 12, 40));
}

#[test]
fn the_current_runs_pool_stays_frozen_while_the_next_run_gets_the_grant() {
	let content = full_content();
	let arena = content.catalogs.arenas["arena_foundry"].bounds;
	let mut app = App::new();
	app.add_plugins(omega_game::GameCorePlugin);
	let mut h = SimHarness::with_app(app, RunSeed::from_hex(&"88".repeat(32)).unwrap());
	h.app_mut().insert_resource(ContentRes(Arc::new(content)));
	h.app_mut().insert_resource(ArenaRes(arena));
	h.app_mut().insert_resource(Profile::initial());
	let snapshot = h.app().world().resource::<Profile>().unlock_snapshot();
	begin_run(h.app_mut().world_mut(), "char_patch", snapshot).unwrap();
	let player = {
		let w = h.app_mut().world_mut();
		w.query_filtered::<Entity, With<Player>>()
			.single(w)
			.unwrap()
	};
	let mut granted_tick = None;
	for _ in 0..40_000 {
		let state = h.app().world().resource::<Run>().state;
		match state {
			RunState::Wave => {
				h.app_mut()
					.world_mut()
					.get_mut::<Health>(player)
					.unwrap()
					.current = 1_000_000;
				h.step(TickInput::default());
				let records = h.drain_presentation();
				if records.iter().any(
					|r| matches!(&r.kind, PresentationKind::UnlockGranted { id } if id == "unlock_first_shift"),
				) {
					granted_tick = Some(h.tick());
				}
			}
			RunState::Shop => {
				if granted_tick.is_some() {
					break;
				}
				start_next_wave(h.app_mut().world_mut()).unwrap();
			}
			_ => break,
		}
	}
	assert!(
		granted_tick.is_some(),
		"wave 3 completion grants the challenge"
	);
	let world = h.app().world();
	assert!(
		world
			.resource::<Profile>()
			.granted
			.contains("unlock_first_shift")
	);
	assert!(world.contains_resource::<ProfileDirty>(), "a save is owed");
	assert!(
		world
			.resource::<omega_game::shop::RunBuild>()
			.unlocked
			.is_empty(),
		"the running pool is a snapshot"
	);
	let offers = world.resource::<ShopVisit>().offers.clone();
	assert!(
		offers
			.iter()
			.all(|o| !matches!(o, Offer::Weapon { id, .. } if id == "wpn_arc" || id == "wpn_nail")),
		"granted weapons stay out of this run's shop: {offers:?}"
	);
	let next = world.resource::<Profile>().unlock_snapshot();
	assert!(next.contains("unlock_first_shift"));
}

// --- persistence -----------------------------------------------------------------------

fn temp_dir(tag: &str) -> PathBuf {
	use std::sync::atomic::{AtomicU32, Ordering};
	static N: AtomicU32 = AtomicU32::new(0);
	let dir = std::env::temp_dir().join(format!(
		"omega-persist-{}-{tag}-{}",
		std::process::id(),
		N.fetch_add(1, Ordering::Relaxed)
	));
	let _ = std::fs::remove_dir_all(&dir);
	dir
}

fn profile_with(grants: &[&str], runs: u32) -> Profile {
	Profile {
		granted: grants.iter().map(|s| s.to_string()).collect(),
		completed_runs: runs,
		victories: 0,
	}
}

/// Fails (or half-writes) the `fail_at`-th storage operation, then lets the rest through.
struct Faulty {
	inner: DiskStorage,
	ops: Cell<usize>,
	fail_at: Option<usize>,
	half_write: bool,
}

impl Faulty {
	fn new(fail_at: Option<usize>, half_write: bool) -> Self {
		Self {
			inner: DiskStorage,
			ops: Cell::new(0),
			fail_at,
			half_write,
		}
	}

	fn tick(&self) -> bool {
		let n = self.ops.get();
		self.ops.set(n + 1);
		self.fail_at == Some(n)
	}
}

impl Storage for Faulty {
	fn read(&self, path: &Path) -> io::Result<Option<Vec<u8>>> {
		if self.tick() {
			return Err(io::Error::other("injected read fault"));
		}
		self.inner.read(path)
	}

	fn write_synced(&self, path: &Path, bytes: &[u8]) -> io::Result<()> {
		if self.tick() {
			if self.half_write {
				let _ = self.inner.write_synced(path, &bytes[..bytes.len() / 2]);
			}
			return Err(io::Error::other("injected write fault"));
		}
		self.inner.write_synced(path, bytes)
	}

	fn rename(&self, from: &Path, to: &Path) -> io::Result<()> {
		if self.tick() {
			return Err(io::Error::other("injected rename fault"));
		}
		self.inner.rename(from, to)
	}

	fn remove(&self, path: &Path) -> io::Result<()> {
		self.inner.remove(path)
	}

	fn create_dir_all(&self, path: &Path) -> io::Result<()> {
		if self.tick() {
			return Err(io::Error::other("injected mkdir fault"));
		}
		self.inner.create_dir_all(path)
	}
}

fn profile_store(dir: &Path) -> SlotStore<Profile> {
	SlotStore::new(dir, "profile", PROFILE_SCHEMA)
}

#[test]
fn first_save_load_round_trip_and_a_missing_profile_is_fresh() {
	let dir = temp_dir("roundtrip");
	let mut store = profile_store(&dir);
	assert_eq!(store.load(&DiskStorage), (None, LoadStatus::Fresh));
	let p = profile_with(&["unlock_first_shift"], 2);
	store.save(&DiskStorage, &p, 1).unwrap();
	let mut again = profile_store(&dir);
	assert_eq!(
		again.load(&DiskStorage),
		(Some(p), LoadStatus::Loaded { generation: 1 })
	);
}

#[test]
fn ac3_a_fault_at_every_protocol_boundary_keeps_a_readable_profile() {
	for half_write in [false, true] {
		for fail_at in 0..8 {
			let dir = temp_dir("fault");
			let first = profile_with(&["unlock_first_shift"], 1);
			let second = profile_with(&["unlock_first_shift", "unlock_overtime"], 2);
			let mut store = profile_store(&dir);
			store.save(&DiskStorage, &first, 1).unwrap();
			let faulty = Faulty::new(Some(fail_at), half_write);
			let outcome = store.save(&faulty, &second, 2);
			let mut fresh = profile_store(&dir);
			let (loaded, status) = fresh.load(&DiskStorage);
			let loaded =
				loaded.unwrap_or_else(|| panic!("fault {fail_at}: lost the profile: {status:?}"));
			match outcome {
				Ok(()) => assert_eq!(loaded, second, "fault {fail_at} never fired"),
				Err(_) => assert_eq!(loaded, first, "fault {fail_at}: previous generation kept"),
			}
			assert!(matches!(status, LoadStatus::Loaded { .. }), "{status:?}");
			// A failed save can be retried and then wins.
			let mut retry = profile_store(&dir);
			retry.load(&DiskStorage);
			let next = retry.committed_generation() + 1;
			retry.save(&DiskStorage, &second, next).unwrap();
		}
	}
}

#[test]
fn a_damaged_newest_slot_falls_back_to_the_previous_generation() {
	let dir = temp_dir("fallback");
	let mut store = profile_store(&dir);
	let (a, b) = (profile_with(&[], 1), profile_with(&["unlock_overtime"], 2));
	store.save(&DiskStorage, &a, 1).unwrap();
	store.save(&DiskStorage, &b, 2).unwrap();
	let newest = dir.join("profile.b.json");
	let bytes = std::fs::read(&newest).unwrap();
	std::fs::write(&newest, &bytes[..bytes.len() / 2]).unwrap();
	let mut fresh = profile_store(&dir);
	let (loaded, status) = fresh.load(&DiskStorage);
	assert_eq!(loaded, Some(a));
	assert_eq!(status, LoadStatus::RecoveredPrevious { generation: 1 });
	let c = profile_with(&["unlock_shutdown"], 3);
	fresh.save(&DiskStorage, &c, 2).unwrap();
	let mut last = profile_store(&dir);
	assert_eq!(last.load(&DiskStorage).0, Some(c));
}

#[test]
fn ac3_unreadable_slots_are_preserved_and_never_replaced_silently() {
	let dir = temp_dir("corrupt");
	std::fs::create_dir_all(&dir).unwrap();
	std::fs::write(dir.join("profile.a.json"), b"{ not json").unwrap();
	std::fs::write(dir.join("profile.b.json"), b"\x00\x01garbage").unwrap();
	let mut store = profile_store(&dir);
	let (loaded, status) = store.load(&DiskStorage);
	assert_eq!(loaded, None);
	assert!(
		matches!(status, LoadStatus::Problem(LoadProblem::Corrupt { ref files }) if files.len() == 2)
	);
	assert_eq!(
		store.save(&DiskStorage, &Profile::initial(), 1),
		Err(SaveError::NeedsRecovery),
		"no fresh profile over corrupt bytes"
	);
	assert_eq!(
		std::fs::read(dir.join("profile.a.json")).unwrap(),
		b"{ not json"
	);
	let moved = store.quarantine(&DiskStorage).unwrap();
	assert_eq!(moved.len(), 2);
	assert_eq!(
		std::fs::read(&moved[0]).unwrap(),
		b"{ not json",
		"bytes kept for recovery"
	);
	store.save(&DiskStorage, &Profile::initial(), 1).unwrap();
}

#[test]
fn ac4_future_schema_and_bad_hash_block_saving_and_keep_the_files() {
	let dir = temp_dir("future");
	let mut store = profile_store(&dir);
	store
		.save(&DiskStorage, &profile_with(&["unlock_overtime"], 1), 1)
		.unwrap();
	let path = dir.join("profile.a.json");
	let original = std::fs::read(&path).unwrap();

	let tampered = String::from_utf8(original.clone())
		.unwrap()
		.replace("unlock_overtime", "unlock_shutdown");
	std::fs::write(&path, &tampered).unwrap();
	let mut s = profile_store(&dir);
	assert!(
		matches!(
			s.load(&DiskStorage).1,
			LoadStatus::Problem(LoadProblem::Corrupt { .. })
		),
		"bad hash"
	);
	assert_eq!(std::fs::read(&path).unwrap(), tampered.as_bytes());

	let future = String::from_utf8(original)
		.unwrap()
		.replace("\"schema\": 1", "\"schema\": 99");
	std::fs::write(&path, &future).unwrap();
	let mut s = profile_store(&dir);
	assert_eq!(
		s.load(&DiskStorage).1,
		LoadStatus::Problem(LoadProblem::FutureSchema { found: 99 })
	);
	assert_eq!(
		s.save(&DiskStorage, &Profile::initial(), 5),
		Err(SaveError::NeedsRecovery)
	);
	assert_eq!(std::fs::read(&path).unwrap(), future.as_bytes());
}

#[test]
fn stale_generations_are_rejected() {
	let dir = temp_dir("stale");
	let mut store = profile_store(&dir);
	store.save(&DiskStorage, &profile_with(&[], 1), 5).unwrap();
	assert_eq!(
		store.save(&DiskStorage, &profile_with(&[], 2), 5),
		Err(SaveError::Stale { committed: 5 })
	);
	assert_eq!(
		store.save(&DiskStorage, &profile_with(&[], 2), 3),
		Err(SaveError::Stale { committed: 5 })
	);
	store.save(&DiskStorage, &profile_with(&[], 2), 6).unwrap();
}

#[test]
fn ac5_resetting_the_profile_keeps_the_settings() {
	let dir = temp_dir("reset");
	let mut profile = profile_store(&dir);
	let mut settings: SlotStore<Settings> = SlotStore::new(&dir, "settings", SETTINGS_SCHEMA);
	let custom = Settings {
		master: 30,
		ui_scale_pct: 150,
		reduced_effects: true,
		..Settings::default()
	};
	profile
		.save(&DiskStorage, &profile_with(&["unlock_overtime"], 4), 1)
		.unwrap();
	settings.save(&DiskStorage, &custom, 1).unwrap();
	profile.quarantine(&DiskStorage).unwrap();
	profile.save(&DiskStorage, &Profile::initial(), 2).unwrap();
	let mut fresh_settings: SlotStore<Settings> = SlotStore::new(&dir, "settings", SETTINGS_SCHEMA);
	assert_eq!(fresh_settings.load(&DiskStorage).0, Some(custom));
	let mut fresh_profile = profile_store(&dir);
	assert_eq!(fresh_profile.load(&DiskStorage).0, Some(Profile::initial()));
}

#[test]
fn a_failed_directory_creation_reports_io_and_changes_nothing() {
	let dir = temp_dir("mkdir");
	let mut store = profile_store(&dir);
	let faulty = Faulty::new(Some(0), false);
	assert!(matches!(
		store.save(&faulty, &Profile::initial(), 1),
		Err(SaveError::Io(_))
	));
	assert_eq!(store.committed_generation(), 0);
	assert!(profile_store(&dir).load(&DiskStorage).0.is_none());
}

#[test]
fn the_save_directory_is_under_localappdata_without_an_install_fallback() {
	if let Some(base) = std::env::var_os("LOCALAPPDATA") {
		let dir = user_dir().unwrap();
		assert_eq!(dir, PathBuf::from(base).join("ProjectOmega"));
	}
}
