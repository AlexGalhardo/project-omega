//! SP-016-AC1/AC2 (reporting subset): reports carry metadata, sample counts,
//! percentiles and explicit unsupported metrics; measuring does not change traces.

use omega_game::diagnostics::{Metric, Report, ReportMetadata};
use omega_game::sim::{RunSeed, SimHarness, TickInput};
use std::time::Instant;

#[test]
fn ac1_report_has_metadata_percentiles_and_unsupported_metrics() {
	let samples: Vec<f64> = (1..=200).map(f64::from).collect();
	let report = Report {
		metadata: ReportMetadata::new("smoke", "00".repeat(32)),
		metrics: vec![
			Metric::from_samples("sim_tick_wall_time", "us", &samples),
			Metric::from_samples("gpu", "ms", &[]),
		],
		unsupported: vec!["allocation_count".into()],
	};
	let v: serde_json::Value = serde_json::from_str(&report.to_json()).unwrap();
	assert_eq!(v["metadata"]["scenario"], "smoke");
	assert_eq!(
		v["metadata"]["percentile_method"],
		"nearest-rank, index ceil(p*N)-1"
	);
	assert_eq!(v["metrics"][0]["samples"], 200);
	assert_eq!(v["metrics"][0]["percentiles"]["p95"], 190.0);
	assert_eq!(v["metrics"][0]["percentiles"]["max"], 200.0);
	assert!(
		v["metrics"][1]["percentiles"].is_null(),
		"no samples is null, never zero"
	);
	assert_eq!(v["unsupported"][0], "allocation_count");
}

#[test]
fn ac2_measuring_does_not_change_the_trace() {
	let seed = RunSeed([9; 32]);
	let plain = SimHarness::new(seed).run(&vec![TickInput::default(); 300]);
	let mut measured = SimHarness::new(seed);
	let mut timings = Vec::new();
	let mut traced = Vec::new();
	for _ in 0..300 {
		let t = Instant::now();
		measured.step(TickInput::default());
		timings.push(t.elapsed().as_secs_f64());
		traced.push(measured.trace());
	}
	assert_eq!(plain, traced);
	assert_eq!(timings.len(), 300);
}
