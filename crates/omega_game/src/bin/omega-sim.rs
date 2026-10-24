//! Headless scenario runner (SP-002, SP-016):
//! `omega-sim --scenario NAME --seed HEX64 --ticks N [--trace PATH] [--measure --output PATH]`.
//! Input is validated before any run is created.
//!
//! `omega-sim --scenario balance [--policy NAME | --policies a,b|all] [--seeds N]
//! [--seed-start N] [--label TEXT] [--out PATH] [--md PATH] [--content DIR] [--threads N]`
//! runs complete bot-driven runs and writes the balance JSON (and optional Markdown).

use omega_game::balance::{MatrixSpec, Policy, load_content, markdown, run_matrix};
use omega_game::content::Profile;
use omega_game::diagnostics::{Metric, Report, ReportMetadata};
use omega_game::sim::{RunSeed, SimHarness, TickInput, TraceEntry};
use std::process::ExitCode;
use std::time::Instant;

const USAGE: &str = "usage: omega-sim --scenario smoke|repeatability --seed HEX64 --ticks N [--trace PATH] [--measure --output PATH]";
const MAX_TICKS: u64 = 10_000_000;

struct Args {
	scenario: String,
	seed: RunSeed,
	ticks: u64,
	trace: Option<String>,
	measure: bool,
	output: Option<String>,
}

fn parse() -> Result<Args, String> {
	let mut it = std::env::args().skip(1);
	let (mut scenario, mut seed, mut ticks, mut trace, mut measure, mut output) =
		(None, None, None, None, false, None);
	while let Some(flag) = it.next() {
		let mut value = || it.next().ok_or(format!("{flag} needs a value"));
		match flag.as_str() {
			"--scenario" => scenario = Some(value()?),
			"--seed" => seed = Some(RunSeed::from_hex(&value()?).map_err(|e| e.to_string())?),
			"--ticks" => {
				let n: u64 = value()?
					.parse()
					.map_err(|_| "--ticks must be a positive integer".to_string())?;
				if n == 0 || n > MAX_TICKS {
					return Err(format!("--ticks must be in 1..={MAX_TICKS}"));
				}
				ticks = Some(n);
			}
			"--trace" => trace = Some(value()?),
			"--measure" => measure = true,
			"--output" => output = Some(value()?),
			other => return Err(format!("unknown argument {other}")),
		}
	}
	let scenario = scenario.ok_or("--scenario is required")?;
	if !["smoke", "repeatability"].contains(&scenario.as_str()) {
		return Err(format!(
			"scenario `{scenario}` is not available yet; PERF-02..07 arrive with the combat and encounter specs"
		));
	}
	if measure != output.is_some() {
		return Err("--measure and --output go together".into());
	}
	Ok(Args {
		scenario,
		seed: seed.ok_or("--seed is required")?,
		ticks: ticks.ok_or("--ticks is required")?,
		trace,
		measure,
		output,
	})
}

fn run(seed: RunSeed, ticks: u64, timings: Option<&mut Vec<f64>>) -> Vec<TraceEntry> {
	let mut h = SimHarness::new(seed);
	let mut traces = Vec::with_capacity(ticks as usize);
	let mut timings = timings;
	for _ in 0..ticks {
		let start = Instant::now();
		h.step(TickInput::default());
		if let Some(t) = timings.as_deref_mut() {
			t.push(start.elapsed().as_secs_f64() * 1e6);
		}
		traces.push(h.trace());
		h.drain_presentation();
	}
	traces
}

const BALANCE_USAGE: &str = "usage: omega-sim --scenario balance [--policy NAME | --policies a,b|all] [--seeds N] [--seed-start N] [--label TEXT] [--out PATH] [--md PATH] [--content DIR] [--threads N]";
const MAX_BALANCE_SEEDS: u64 = 100_000;

fn balance(args: impl Iterator<Item = String>) -> Result<(), String> {
	let mut spec = MatrixSpec::new(Policy::ALL.to_vec(), 0, 200);
	let (mut out, mut md, mut content_dir) = (None, None, None);
	let mut it = args;
	while let Some(flag) = it.next() {
		let mut value = || it.next().ok_or(format!("{flag} needs a value"));
		let number = |text: String, what: &str| {
			text.parse::<u64>()
				.map_err(|_| format!("{what} must be a non-negative integer"))
		};
		match flag.as_str() {
			"--scenario" => {
				value()?;
			}
			"--policy" => spec.policies = vec![value()?.parse()?],
			"--policies" => {
				let text = value()?;
				spec.policies = if text == "all" {
					Policy::ALL.to_vec()
				} else {
					text.split(',')
						.map(str::parse)
						.collect::<Result<Vec<Policy>, _>>()?
				};
			}
			"--seeds" => {
				spec.seeds = number(value()?, "--seeds")?;
				if spec.seeds == 0 || spec.seeds > MAX_BALANCE_SEEDS {
					return Err(format!("--seeds must be in 1..={MAX_BALANCE_SEEDS}"));
				}
			}
			"--seed-start" => spec.seed_start = number(value()?, "--seed-start")?,
			"--threads" => spec.threads = number(value()?, "--threads")?.max(1) as usize,
			"--label" => spec.label = value()?,
			"--out" => out = Some(value()?),
			"--md" => md = Some(value()?),
			"--content" => content_dir = Some(value()?),
			other => return Err(format!("unknown argument {other}")),
		}
	}
	if spec.policies.is_empty() {
		return Err("at least one policy is required".into());
	}
	let dir = content_dir.map_or_else(
		|| std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/content"),
		Into::into,
	);
	let content = load_content(&dir, Profile::Prototype)?;
	let report = run_matrix(&content, &spec)?;
	let out = out.unwrap_or_else(|| format!("artifacts/balance/{}.json", spec.label));
	let write = |path: &str, text: &str| {
		if let Some(dir) = std::path::Path::new(path).parent() {
			let _ = std::fs::create_dir_all(dir);
		}
		std::fs::write(path, text).map_err(|e| format!("cannot write {path}: {e}"))
	};
	write(&out, &report.to_json())?;
	if let Some(path) = md {
		write(&path, &markdown(&report))?;
	}
	for s in &report.summaries {
		println!(
			"{}: {} runs, win rate {:.1} %",
			s.policy,
			s.runs,
			s.win_rate * 100.0
		);
	}
	println!("balance report: {out}");
	Ok(())
}

fn main() -> ExitCode {
	let argv: Vec<String> = std::env::args().skip(1).collect();
	if argv
		.windows(2)
		.any(|w| w[0] == "--scenario" && w[1] == "balance")
	{
		return match balance(argv.into_iter()) {
			Ok(()) => ExitCode::SUCCESS,
			Err(e) => {
				eprintln!(
					"{e}
{BALANCE_USAGE}"
				);
				ExitCode::from(2)
			}
		};
	}
	let args = match parse() {
		Ok(a) => a,
		Err(e) => {
			eprintln!("{e}\n{USAGE}");
			return ExitCode::from(2);
		}
	};
	let mut timings = Vec::new();
	let traces = run(args.seed, args.ticks, args.measure.then_some(&mut timings));
	if args.scenario == "repeatability" {
		let again = run(args.seed, args.ticks, None);
		if let Some(i) = traces.iter().zip(&again).position(|(a, b)| a != b) {
			eprintln!("divergence at tick {}", traces[i].tick);
			return ExitCode::FAILURE;
		}
		println!("repeatability: {} identical ticks", traces.len());
	}
	if let Some(path) = &args.trace {
		let lines: String = traces
			.iter()
			.map(|t| serde_json::to_string(t).expect("trace serializes") + "\n")
			.collect();
		if let Err(e) = std::fs::write(path, lines) {
			eprintln!("cannot write trace {path}: {e}");
			return ExitCode::FAILURE;
		}
	}
	if let Some(path) = &args.output {
		let mut metadata = ReportMetadata::new(&args.scenario, args.seed.to_hex());
		metadata
			.settings
			.push(("ticks".into(), args.ticks.to_string()));
		let report = Report {
			metadata,
			metrics: vec![Metric::from_samples("sim_tick_wall_time", "us", &timings)],
			unsupported: vec![
				"allocation_count".into(),
				"allocation_bytes".into(),
				"working_set".into(),
				"entities_by_role".into(),
			],
		};
		if let Some(dir) = std::path::Path::new(path).parent() {
			let _ = std::fs::create_dir_all(dir);
		}
		if let Err(e) = std::fs::write(path, report.to_json()) {
			eprintln!("cannot write report {path}: {e}");
			return ExitCode::FAILURE;
		}
	}
	let last = traces.last().expect("at least one tick");
	println!(
		"{}: {} ticks, final tick {}",
		args.scenario,
		traces.len(),
		last.tick
	);
	ExitCode::SUCCESS
}
