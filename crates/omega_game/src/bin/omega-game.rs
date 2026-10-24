//! Native client entry point.
//! `omega-game [--resolution WxH] [--catalog FILE] [--screenshot PATH]
//!             [--benchmark PERF-ID --duration-seconds N --output PATH]
//!             [--e2e-script FILE | --qa-bot PROFILE --seed HEX [--frames N]]
//!             [--headless] [--out-dir DIR]`

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::app::AppExit;
use omega_game::client::{BenchmarkOptions, ClientOptions, run};
use omega_game::qa::{QaRun, bot::BotProfile, run_headless};
use omega_game::sim::RunSeed;
use std::path::PathBuf;
use std::process::ExitCode;

const USAGE: &str = "usage: omega-game [--resolution WxH] [--catalog FILE] [--screenshot PATH] [--play] [--benchmark PERF-ID --duration-seconds N --output PATH] [--e2e-script FILE | --qa-bot cautious|aggressive|chaotic --seed HEX64 [--frames N]] [--headless] [--out-dir DIR]";

struct Parsed {
	client: ClientOptions,
	headless: bool,
}

fn parse() -> Result<Parsed, String> {
	let mut o = ClientOptions::default();
	let (mut bench, mut duration, mut output) = (None, None, None);
	let (mut script, mut profile, mut seed, mut headless) = (None, None, None, false);
	let (mut frames, mut out_dir) = (3600u64, PathBuf::from("artifacts/e2e"));
	let mut it = std::env::args().skip(1);
	while let Some(flag) = it.next() {
		let mut value = || it.next().ok_or(format!("{flag} needs a value"));
		match flag.as_str() {
			"--resolution" => {
				let v = value()?;
				let (w, h) = v.split_once('x').ok_or("--resolution expects WxH")?;
				o.width = w.parse().map_err(|_| "bad width")?;
				o.height = h.parse().map_err(|_| "bad height")?;
				if o.width == 0 || o.height == 0 {
					return Err("resolution must be positive".into());
				}
			}
			"--catalog" => o.catalog = value()?,
			"--play" => o.play = true,
			"--screenshot" => o.screenshot = Some(PathBuf::from(value()?)),
			"--benchmark" => bench = Some(value()?),
			"--duration-seconds" => {
				let d: f64 = value()?.parse().map_err(|_| "bad duration")?;
				if !(d.is_finite() && d > 0.0) {
					return Err("duration must be positive".into());
				}
				duration = Some(d);
			}
			"--output" => output = Some(PathBuf::from(value()?)),
			"--e2e-script" => script = Some(PathBuf::from(value()?)),
			"--qa-bot" => profile = Some(value()?.parse::<BotProfile>()?),
			"--seed" => seed = Some(RunSeed::from_hex(&value()?).map_err(|e| e.to_string())?),
			"--frames" => frames = value()?.parse().map_err(|_| "bad frame count")?,
			"--out-dir" => out_dir = PathBuf::from(value()?),
			"--headless" => headless = true,
			other => return Err(format!("unknown argument {other}")),
		}
	}
	match (bench, duration, output) {
		(None, None, None) => {}
		(Some(id), Some(duration_seconds), Some(output)) => {
			if id != "PERF-01" {
				return Err(format!(
					"{id} is not available yet; Foundation supports PERF-01"
				));
			}
			o.benchmark = Some(BenchmarkOptions {
				id,
				duration_seconds,
				output,
			});
		}
		_ => return Err("--benchmark, --duration-seconds and --output go together".into()),
	}
	o.qa = match (script, profile, seed) {
		(None, None, None) => None,
		(Some(script), None, None) => Some(QaRun::E2e { script, out_dir }),
		(None, Some(profile), Some(seed)) => Some(QaRun::Bot {
			profile,
			seed,
			frames,
			resolution: [o.width, o.height],
			out_dir,
		}),
		_ => return Err("use --e2e-script FILE, or --qa-bot PROFILE with --seed HEX".into()),
	};
	if headless && o.qa.is_none() {
		return Err("--headless needs --e2e-script or --qa-bot".into());
	}
	Ok(Parsed {
		client: o,
		headless,
	})
}

fn main() -> ExitCode {
	match parse() {
		Ok(Parsed {
			client: ClientOptions { qa: Some(qa), .. },
			headless: true,
		}) => ExitCode::from(run_headless(&qa)),
		Ok(Parsed { client, .. }) => match run(client) {
			AppExit::Success => ExitCode::SUCCESS,
			AppExit::Error(_) => ExitCode::FAILURE,
		},
		Err(e) => {
			eprintln!("{e}\n{USAGE}");
			ExitCode::from(2)
		}
	}
}
