//! GAME-P2-013: the balance scenario is repeatable, `idle` loses early, and policies and
//! metrics only observe.

use bevy::prelude::*;
use omega_game::GameCorePlugin;
use omega_game::balance::{
	MatrixSpec, Observer, Policy, bands, load_content, markdown, run_matrix,
};
use omega_game::combat::actor::{ArenaRes, ContentRes};
use omega_game::content::{ContentSnapshot, Profile};
use omega_game::sim::run::begin_run;
use omega_game::sim::{RunSeed, SimHarness, TickInput};
use std::collections::BTreeSet;
use std::path::PathBuf;
use std::sync::Arc;

fn content() -> Arc<ContentSnapshot> {
	let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/content-prototype");
	load_content(&dir, Profile::Prototype).expect("prototype content")
}

fn spec(policies: &[Policy], seed_start: u64, seeds: u64) -> MatrixSpec {
	let mut spec = MatrixSpec::new(policies.to_vec(), seed_start, seeds);
	spec.threads = 2;
	spec
}

#[test]
fn a_fixed_seed_balance_scenario_is_repeatable() {
	let content = content();
	let spec = spec(&[Policy::Idle, Policy::Greedy, Policy::HumanLike], 7, 2);
	let first = run_matrix(&content, &spec).unwrap().to_json();
	let second = run_matrix(&content, &spec).unwrap().to_json();
	assert_eq!(first, second, "same seeds and policies give identical JSON");
	let other = run_matrix(&content, &self::spec(&spec.policies, 8, 2))
		.unwrap()
		.to_json();
	assert_ne!(first, other, "a different seed range changes the report");
}

#[test]
fn the_idle_policy_dies_in_waves_one_or_two() {
	let report = run_matrix(&content(), &spec(&[Policy::Idle], 0, 6)).unwrap();
	let idle = report.summary(Policy::Idle).unwrap();
	assert_eq!(idle.wins, 0);
	let late: u32 = idle.deaths_by_wave.range(3..).map(|(_, n)| n).sum();
	assert_eq!(late, 0, "deaths by wave: {:?}", idle.deaths_by_wave);
	assert_eq!(idle.deaths_by_wave.values().sum::<u32>(), 6);
}

#[test]
fn every_policy_finishes_a_run_and_records_its_metrics() {
	let report = run_matrix(&content(), &spec(&Policy::ALL, 100, 2)).unwrap();
	assert_eq!(report.runs.len(), Policy::ALL.len() * 2);
	for run in &report.runs {
		assert!(run.combat_ticks > 0 && !run.waves.is_empty(), "{run:?}");
		assert_eq!(run.waves.last().unwrap().cleared, run.victory);
	}
	let md = markdown(&report);
	assert!(md.contains("## Prototype bands") && md.contains("human_like"));
	assert!(!bands(&report).is_empty());
}

#[test]
fn observing_never_changes_the_simulation() {
	let content = content();
	let arena = content.catalogs.arenas["arena_foundry"].bounds;
	let build = |observe: bool| {
		let mut app = App::new();
		app.add_plugins(GameCorePlugin);
		let mut h = SimHarness::with_app(app, RunSeed([9; 32]));
		h.app_mut()
			.insert_resource(ContentRes(content.clone()))
			.insert_resource(ArenaRes(arena));
		begin_run(h.app_mut().world_mut(), "char_patch", BTreeSet::new()).unwrap();
		let mut observer = Observer::new(h.app_mut().world_mut());
		for _ in 0..900 {
			if observe {
				observer.observe(h.app().world());
				omega_game::qa::capture(h.app_mut().world_mut(), [1280, 720]);
			}
			h.step(TickInput::default());
		}
		h.trace()
	};
	assert_eq!(build(true), build(false));
}
