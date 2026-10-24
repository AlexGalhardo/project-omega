//! QA-P2-002: scripted e2e and the QA bot drive the real prototype loop (begin_run, Wave,
//! Shop, Victory/Defeat, Summary) headless through the shared input buffer and the typed
//! UI action layer.

use omega_game::input::{Edge, FrameInput};
use omega_game::qa::bot::{Auditor, BotProfile};
use omega_game::qa::script::Script;
use omega_game::qa::{
	DebugOp, HeadlessHost, QaRun, Snapshot, apply_debug, apply_ui, run_headless_session,
};
use omega_game::sim::RunSeed;
use omega_game::ui::UiAction;
use std::path::PathBuf;

fn repo(path: &str) -> PathBuf {
	PathBuf::from(env!("CARGO_MANIFEST_DIR"))
		.join("../..")
		.join(path)
}

fn scratch(name: &str) -> PathBuf {
	let dir = std::env::temp_dir().join(format!("omega-loop-{name}-{}", std::process::id()));
	std::fs::create_dir_all(&dir).unwrap();
	dir
}

fn seed(byte: u8) -> RunSeed {
	RunSeed::from_hex(&format!("{byte:02x}").repeat(32)).unwrap()
}

fn run_script(name: &str) -> (bool, serde_json::Value) {
	let run = QaRun::E2e {
		script: repo(&format!("tests/e2e/{name}.json")),
		out_dir: scratch(name),
	};
	let finished = run_headless_session(&run).unwrap();
	(
		finished.passed,
		serde_json::from_str(&finished.json).unwrap(),
	)
}

fn assert_script_passes(name: &str) -> serde_json::Value {
	let (passed, report) = run_script(name);
	let failed: Vec<_> = report["checkpoints"]
		.as_array()
		.unwrap()
		.iter()
		.filter(|c| c["passed"] == false)
		.map(|c| (c["name"].clone(), c["failures"].clone()))
		.collect();
	assert!(
		passed,
		"{name}: error {} failed {failed:?}",
		report["error"]
	);
	report
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
fn the_full_prototype_loop_script_plays_three_waves_to_the_summary() {
	let report = assert_script_passes("prototype-loop");
	assert_eq!(report["mode"], "headless");
	let summary = &checkpoint(&report, "summary")["snapshot"]["run"];
	assert_eq!(summary["state"], "summary");
	assert_eq!(summary["total_waves"], 3);
}

#[test]
fn double_purchase_is_rejected_and_changes_nothing() {
	assert_script_passes("shop-double-purchase");
}

#[test]
fn refresh_spam_is_rejected_after_the_first_refresh() {
	assert_script_passes("shop-refresh-spam");
}

#[test]
fn unaffordable_and_invalid_purchases_are_rejected_with_reasons() {
	assert_script_passes("shop-unaffordable");
}

#[test]
fn zero_energy_start_is_rejected_with_the_reason() {
	assert_script_passes("shop-zero-energy-start");
}

#[test]
fn energy_allocation_respects_the_reactor_limits() {
	assert_script_passes("shop-energy-allocation");
}

#[test]
fn death_in_the_completion_tick_settles_defeat_and_opens_no_shop() {
	assert_script_passes("death-at-wave-completion");
}

#[test]
fn death_inside_the_shop_does_not_end_the_run() {
	assert_script_passes("death-in-shop");
}

#[test]
fn abandon_cancel_changes_nothing() {
	assert_script_passes("abandon-cancel");
}

#[test]
fn abandon_confirm_settles_once() {
	assert_script_passes("abandon-confirm");
}

#[test]
fn script_errors_in_the_new_actions_are_rejected_at_parse_time() {
	for bad in [
		r#"{"name":"x","steps":[{"do":"wait_state","states":["lobby"]}]}"#,
		r#"{"name":"x","steps":[{"do":"wait_state","states":[]}]}"#,
		r#"{"name":"x","steps":[{"do":"ui","action":"sell"}]}"#,
		r#"{"name":"x","steps":[{"do":"debug_headless","ops":[]}]}"#,
		r#"{"name":"x","mode":"arcade","steps":[{"do":"wait","ticks":1}]}"#,
	] {
		assert!(Script::parse(bad).is_err(), "{bad}");
	}
}

#[test]
fn the_prototype_host_exposes_run_shop_and_loadout_facts() {
	let mut host = HeadlessHost::prototype(seed(1), [1280, 720]).unwrap();
	let snap = host.snapshot();
	let run = snap.run.expect("run facts");
	assert_eq!((run.state, run.wave, run.total_waves), ("wave", 1, 3));
	assert!(snap.shop.is_none());
	let loadout = snap.loadout.expect("loadout facts");
	assert!(loadout.powered && loadout.occupied == 1 && loadout.energy_used == 3);
}

#[test]
fn bots_play_the_prototype_loop_to_the_summary_without_findings() {
	for profile in BotProfile::ALL {
		for byte in [1u8, 2] {
			let run = QaRun::Bot {
				profile,
				seed: seed(byte),
				frames: 60_000,
				resolution: [1280, 720],
				out_dir: scratch("bots"),
			};
			let finished = run_headless_session(&run).unwrap();
			let report: serde_json::Value = serde_json::from_str(&finished.json).unwrap();
			assert!(
				finished.passed && report["completed"] == true,
				"{profile} seed {byte}: findings {} completed {}",
				report["findings"],
				report["completed"]
			);
		}
	}
}

fn host_in_shop() -> HeadlessHost {
	let mut host = HeadlessHost::prototype(seed(1), [1280, 720]).unwrap();
	for _ in 0..4000 {
		if host.snapshot().run.is_some_and(|r| r.state == "shop") {
			return host;
		}
		host.advance(FrameInput::default());
	}
	panic!("never reached the shop");
}

fn enabled_energy_adds(host: &mut HeadlessHost) -> Vec<usize> {
	host.snapshot()
		.screen
		.unwrap()
		.buttons
		.iter()
		.filter_map(|b| match b.action {
			UiAction::Energy { slot, add: true } if b.enabled => Some(slot),
			_ => None,
		})
		.collect()
}

#[test]
fn energy_add_button_is_disabled_when_the_reactor_cannot_take_a_point() {
	let mut host = host_in_shop();
	assert!(enabled_energy_adds(&mut host).is_empty(), "slot 0 is full");
	apply_debug(host.world_mut(), DebugOp::SetScrap(1000)).unwrap();
	apply_ui(host.world_mut(), UiAction::Buy(0));
	apply_ui(host.world_mut(), UiAction::Buy(1));
	assert_eq!(enabled_energy_adds(&mut host), vec![1]);
	for _ in 0..3 {
		apply_ui(host.world_mut(), UiAction::Energy { slot: 1, add: true });
	}
	assert!(enabled_energy_adds(&mut host).is_empty(), "reactor is full");
	apply_ui(
		host.world_mut(),
		UiAction::Energy {
			slot: 1,
			add: false,
		},
	);
	assert_eq!(enabled_energy_adds(&mut host), vec![1]);
}

#[test]
fn confirm_pressed_during_combat_does_not_start_the_next_wave_from_the_shop() {
	let mut host = HeadlessHost::prototype(seed(1), [1280, 720]).unwrap();
	host.advance(FrameInput {
		edges: vec![Edge::Confirm, Edge::OpenEnergy],
		..FrameInput::default()
	});
	for _ in 0..4000 {
		if host.snapshot().run.is_some_and(|r| r.state == "shop") {
			break;
		}
		host.advance(FrameInput::default());
	}
	for _ in 0..30 {
		host.advance(FrameInput::default());
	}
	let run = host.snapshot().run.unwrap();
	assert_eq!((run.state, run.wave), ("shop", 1));
}

fn kinds(auditor: &Auditor) -> Vec<&'static str> {
	auditor.findings.iter().map(|f| f.kind).collect()
}

#[test]
fn the_auditor_flags_every_documented_invariant_violation() {
	let mut host = HeadlessHost::prototype(seed(1), [1280, 720]).unwrap();
	let base = host.snapshot();
	let check = |mutate: &dyn Fn(&mut Snapshot), expected: &str| {
		let mut auditor = Auditor::default();
		auditor.observe(&base);
		let mut next = base.clone();
		mutate(&mut next);
		auditor.observe(&next);
		assert!(
			kinds(&auditor).contains(&expected),
			"{expected} not flagged: {:?}",
			kinds(&auditor)
		);
	};
	check(
		&|s| s.loadout.as_mut().unwrap().energy_used = 7,
		"energy-over-budget",
	);
	check(
		&|s| s.loadout.as_mut().unwrap().slots[1].points = 2,
		"energy-on-empty-slot",
	);
	check(
		&|s| s.loadout.as_mut().unwrap().slots[0].points = 4,
		"slot-energy-over-limit",
	);
	check(
		&|s| s.loadout.as_mut().unwrap().occupied = 5,
		"slots-over-limit",
	);
	check(&|s| s.player.as_mut().unwrap().hp = 999, "hp-over-max");
	check(&|s| s.scrap = u32::MAX, "scrap-overflow");
	check(&|s| s.player.as_mut().unwrap().hp = 0, "death-unresolved");
	check(
		&|s| s.run.as_mut().unwrap().settlements = 2,
		"multiple-settlements",
	);
	check(
		&|s| s.run.as_mut().unwrap().wave = 2,
		"wave-advanced-without-shop",
	);
	check(&|s| s.run.as_mut().unwrap().wave = 0, "wave-went-back");
	check(
		&|s| s.run.as_mut().unwrap().state = "victory",
		"result-missing",
	);
	check(&|s| s.tick += 5, "tick-jump");
}

#[test]
fn a_wave_scrap_loss_and_a_state_change_by_a_rejected_command_are_flagged() {
	let mut host = host_in_shop();
	let shop = host.snapshot();
	let mut auditor = Auditor::default();
	auditor.observe(&shop);
	let mut changed = shop.clone();
	changed.scrap += 1;
	auditor.observe(&changed);
	assert!(kinds(&auditor).contains(&"rejected-command-changed-state"));

	let mut auditor = Auditor::default();
	let mut wave = HeadlessHost::prototype(seed(1), [1280, 720])
		.unwrap()
		.snapshot();
	wave.scrap = 10;
	auditor.observe(&wave);
	wave.scrap = 4;
	auditor.observe(&wave);
	assert!(kinds(&auditor).contains(&"scrap-lost-in-combat"));
}

#[cfg(feature = "client")]
#[test]
fn the_binary_plays_the_prototype_loop_headless_for_a_script_and_a_bot() {
	use std::process::Command;
	let bin = env!("CARGO_BIN_EXE_omega-game");
	let out = scratch("bin");
	let script = Command::new(bin)
		.args(["--headless", "--e2e-script"])
		.arg(repo("tests/e2e/prototype-loop.json"))
		.arg("--out-dir")
		.arg(&out)
		.status()
		.unwrap();
	assert_eq!(script.code(), Some(0));
	let bot = Command::new(bin)
		.args(["--headless", "--qa-bot", "chaotic", "--seed"])
		.arg("22".repeat(32))
		.arg("--out-dir")
		.arg(&out)
		.status()
		.unwrap();
	assert_eq!(bot.code(), Some(0));
	let report: serde_json::Value = serde_json::from_str(
		&std::fs::read_to_string(out.join("qa-bot-chaotic-22222222.json")).unwrap(),
	)
	.unwrap();
	assert_eq!(
		report["completed"], true,
		"the default frame budget covers a whole run"
	);
}

#[test]
fn windowed_catch_up_ticks_are_not_a_tick_jump_but_a_regression_still_is() {
	let mut host = HeadlessHost::prototype(seed(1), [1280, 720]).unwrap();
	let base = host.snapshot();
	let mut later = base.clone();
	later.tick += 3;
	let mut strict = Auditor::default();
	strict.observe(&base);
	strict.observe(&later);
	assert_eq!(kinds(&strict), vec!["tick-jump"]);
	let mut windowed = Auditor::default();
	windowed.tick_slack = 15;
	windowed.observe(&base);
	windowed.observe(&later);
	assert!(windowed.findings.is_empty());
	windowed.observe(&base);
	assert_eq!(kinds(&windowed), vec!["tick-jump"]);
}
