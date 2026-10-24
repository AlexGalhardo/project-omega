//! QA-P1-003: scripted e2e and the seeded QA bot run headless through the real
//! `GameTick` schedule and the shared input buffer.

use omega_game::input::Edge;
use omega_game::qa::bot::{Bot, BotProfile};
use omega_game::qa::{
	HeadlessHost, InputDriver, MenuView, QaRun, run_headless, run_headless_session,
};
use omega_game::sim::RunSeed;
use std::collections::BTreeSet;
use std::path::PathBuf;

fn repo(path: &str) -> PathBuf {
	PathBuf::from(env!("CARGO_MANIFEST_DIR"))
		.join("../..")
		.join(path)
}

fn scratch(name: &str) -> PathBuf {
	let dir = std::env::temp_dir().join(format!("omega-e2e-{name}-{}", std::process::id()));
	std::fs::create_dir_all(&dir).unwrap();
	dir
}

fn script_run(file: PathBuf, out: &str) -> QaRun {
	QaRun::E2e {
		script: file,
		out_dir: scratch(out),
	}
}

fn write_script(name: &str, json: &str) -> PathBuf {
	let path = scratch(name).join("script.json");
	std::fs::write(&path, json).unwrap();
	path
}

fn seed(byte: u8) -> RunSeed {
	RunSeed::from_hex(&format!("{byte:02x}").repeat(32)).unwrap()
}

fn bot_run(profile: BotProfile, byte: u8, frames: u64, out: &str) -> QaRun {
	QaRun::Bot {
		profile,
		seed: seed(byte),
		frames,
		resolution: [1280, 720],
		out_dir: scratch(out),
	}
}

fn json(text: &str) -> serde_json::Value {
	serde_json::from_str(text).unwrap()
}

fn checkpoint<'a>(report: &'a serde_json::Value, name: &str) -> &'a serde_json::Value {
	report["checkpoints"]
		.as_array()
		.unwrap()
		.iter()
		.find(|c| c["name"] == name)
		.unwrap_or_else(|| panic!("no checkpoint {name}"))
}

#[test]
fn foundation_script_passes_and_exports_the_camera_checkpoint() {
	let run = script_run(repo("tests/e2e/foundation.json"), "foundation");
	let finished = run_headless_session(&run).unwrap();
	let report = json(&finished.json);
	assert!(finished.passed, "{}", finished.json);
	assert_eq!(report["mode"], "headless");
	let ultrawide = checkpoint(&report, "resize-21-9");
	assert_eq!(ultrawide["snapshot"]["camera"]["letterbox"], true);
	assert_eq!(ultrawide["snapshot"]["camera"]["content_w"], 1920.0);
	assert_eq!(
		ultrawide["snapshot"]["window"],
		serde_json::json!([2560, 1080])
	);
	assert_eq!(run_headless(&run), 0);
}

#[test]
fn no_ticks_are_simulated_while_unfocused() {
	let run = script_run(repo("tests/e2e/foundation.json"), "unfocused");
	let report = json(&run_headless_session(&run).unwrap().json);
	let tick = |name: &str| {
		checkpoint(&report, name)["snapshot"]["tick"]
			.as_u64()
			.unwrap()
	};
	assert_eq!(tick("unfocused-a"), tick("unfocused-b"));
	assert!(tick("refocused") > tick("unfocused-b"));
}

#[test]
fn a_broken_checkpoint_exits_one_and_a_bad_script_exits_two() {
	let broken = write_script(
		"broken",
		r#"{"name":"broken","steps":[
			{"do":"wait","ticks":3},
			{"do":"checkpoint","name":"wrong","expect":{"camera.letterbox":true,"tick":99}}
		]}"#,
	);
	let run = QaRun::E2e {
		script: broken.clone(),
		out_dir: broken.parent().unwrap().to_path_buf(),
	};
	let finished = run_headless_session(&run).unwrap();
	assert!(!finished.passed);
	let report = json(&finished.json);
	let failures = checkpoint(&report, "wrong")["failures"].as_array().unwrap();
	assert_eq!(failures.len(), 2);
	assert_eq!(run_headless(&run), 1);

	let invalid = write_script("invalid", r#"{"name":"x","steps":[{"do":"hover"}]}"#);
	assert_eq!(run_headless(&script_run(invalid, "invalid-out")), 2);
	let missing = script_run(repo("tests/e2e/missing.json"), "missing");
	assert_eq!(run_headless(&missing), 2);
}

#[test]
fn waiting_while_paused_fails_instead_of_hanging() {
	let stuck = write_script(
		"stuck",
		r#"{"name":"stuck","steps":[{"do":"focus_lost"},{"do":"wait","ticks":5}]}"#,
	);
	let finished = run_headless_session(&script_run(stuck, "stuck-out")).unwrap();
	assert!(!finished.passed);
	let report = json(&finished.json);
	assert!(report["error"].as_str().unwrap().contains("cannot finish"));
}

#[test]
fn bot_is_deterministic_per_seed() {
	for profile in BotProfile::ALL {
		let run = |byte| {
			run_headless_session(&bot_run(profile, byte, 600, "det"))
				.unwrap()
				.json
		};
		assert_eq!(run(7), run(7), "{profile} diverged for one seed");
		assert_ne!(run(7), run(8), "{profile} ignored the seed");
	}
}

#[test]
fn bot_reports_for_three_profiles_and_five_seeds_have_no_panics_or_findings() {
	for profile in BotProfile::ALL {
		for byte in 1..=5u8 {
			let finished = run_headless_session(&bot_run(profile, byte, 1800, "matrix")).unwrap();
			let report = json(&finished.json);
			assert_eq!(report["panics"], 0, "{profile} seed {byte}");
			assert_eq!(report["profile"], profile.to_string());
			assert_eq!(report["seed"], seed(byte).to_hex());
			assert!(
				finished.passed,
				"{profile} seed {byte}: {}",
				report["findings"]
			);
		}
	}
}

#[test]
fn chaotic_bot_abuses_pause_focus_and_resize() {
	let finished = run_headless_session(&bot_run(BotProfile::Chaotic, 3, 6000, "abuse")).unwrap();
	let report = json(&finished.json);
	for key in [
		"pause_presses",
		"focus_losses",
		"resizes",
		"mistakes",
		"hesitations",
	] {
		assert!(
			report["counts"][key].as_u64().unwrap() > 0,
			"{key} never happened"
		);
	}
	assert!(finished.passed, "{}", report["findings"]);
}

#[test]
fn bot_explores_a_stub_menu_without_leaving_its_bounds() {
	let mut host = HeadlessHost::new(seed(1), [1280, 720]).unwrap();
	let mut bot = Bot::new(BotProfile::Chaotic, &seed(9), 4000);
	let mut selected = 0u8;
	let (mut visited, mut edges) = (BTreeSet::new(), BTreeSet::new());
	loop {
		let mut obs = host.snapshot();
		obs.menu = Some(MenuView { items: 5, selected });
		let step = bot.next(&obs);
		if step.done {
			break;
		}
		if step.frame.down && selected + 1 < 5 {
			selected += 1;
		}
		if step.frame.up && selected > 0 {
			selected -= 1;
		}
		visited.insert(selected);
		edges.extend(step.frame.edges.iter().copied().map(|e| e as u8));
		assert!(!step.frame.manual_aim_held, "menu frames never aim");
	}
	assert!(visited.len() >= 3, "visited {visited:?}");
	assert!(edges.contains(&(Edge::Confirm as u8)));
	assert!(edges.contains(&(Edge::Cancel as u8)));
}

#[test]
fn manual_aim_points_stay_inside_the_arena() {
	let mut host = HeadlessHost::new(seed(2), [1280, 720]).unwrap();
	let mut bot = Bot::new(BotProfile::Aggressive, &seed(2), 3000);
	let mut aimed = 0;
	loop {
		let step = bot.next(&host.snapshot());
		if step.done {
			break;
		}
		let cursor = step.frame.cursor_world;
		host.advance(step.frame);
		if let Some(p) = cursor {
			aimed += 1;
			assert!(p.x.abs() <= 600.0 && p.y.abs() <= 337.5, "{p:?}");
		}
	}
	assert!(aimed > 0, "the bot never aimed manually");
}

#[cfg(feature = "client")]
#[test]
fn the_binary_exits_zero_one_and_two_headless() {
	use std::process::Command;
	let bin = env!("CARGO_BIN_EXE_omega-game");
	let out = scratch("bin");
	let run = |script: &std::path::Path| {
		Command::new(bin)
			.args(["--headless", "--e2e-script"])
			.arg(script)
			.arg("--out-dir")
			.arg(&out)
			.status()
			.unwrap()
			.code()
			.unwrap()
	};
	assert_eq!(run(&repo("tests/e2e/foundation.json")), 0);
	let broken = write_script(
		"bin-broken",
		r#"{"name":"b","steps":[{"do":"checkpoint","name":"c","expect":{"tick":5}}]}"#,
	);
	assert_eq!(run(&broken), 1);
	assert_eq!(run(&repo("tests/e2e/missing.json")), 2);
	let bot = Command::new(bin)
		.args(["--headless", "--qa-bot", "cautious", "--seed"])
		.arg("11".repeat(32))
		.args(["--frames", "300", "--out-dir"])
		.arg(&out)
		.status()
		.unwrap();
	assert_eq!(bot.code(), Some(0));
}
