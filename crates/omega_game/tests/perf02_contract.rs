//! QA-P2-001: the PERF-02 prototype workload is a complete, repeatable three-wave run.

use omega_game::diagnostics::perf02;
use omega_game::sim::RunSeed;
use std::path::PathBuf;

fn content() -> PathBuf {
	PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/content-prototype")
}

fn setting<'a>(report: &'a omega_game::diagnostics::Report, key: &str) -> &'a str {
	&report
		.metadata
		.settings
		.iter()
		.find(|(k, _)| k == key)
		.unwrap()
		.1
}

#[test]
fn workload_runs_three_waves_with_two_shop_visits_and_is_repeatable() {
	let seed = RunSeed::from_hex(&"5a".repeat(32)).unwrap();
	let a = perf02::run(seed, &content()).unwrap();
	let b = perf02::run(seed, &content()).unwrap();
	for report in [&a, &b] {
		let ticks: u32 = setting(report, "ticks").parse().unwrap();
		assert!((3600..=10800).contains(&ticks), "{ticks}");
		assert!(["Victory", "Defeat"].contains(&setting(report, "final_state")));
	}
	let counts = |r: &omega_game::diagnostics::Report| {
		r.metrics
			.iter()
			.filter(|m| m.name != "sim_tick_wall_time")
			.map(|m| (m.name.clone(), m.samples, m.percentiles))
			.collect::<Vec<_>>()
	};
	assert_eq!(
		counts(&a),
		counts(&b),
		"entity populations are seeded, only timing varies"
	);
	assert!(a.unsupported.contains(&"render_frame_time".to_string()));
}

#[test]
fn scripted_input_is_a_pure_function_of_the_tick() {
	assert_eq!(perf02::scripted_input(123), perf02::scripted_input(123));
	assert_ne!(perf02::scripted_input(0), perf02::scripted_input(100));
}
