//! Native client entry point.
//! `omega-game [--resolution WxH] [--catalog FILE] [--screenshot PATH]
//!             [--benchmark PERF-ID --duration-seconds N --output PATH]`

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::app::AppExit;
use omega_game::client::{BenchmarkOptions, ClientOptions, run};
use std::path::PathBuf;
use std::process::ExitCode;

const USAGE: &str = "usage: omega-game [--resolution WxH] [--catalog FILE] [--screenshot PATH] [--benchmark PERF-ID --duration-seconds N --output PATH]";

fn parse() -> Result<ClientOptions, String> {
	let mut o = ClientOptions::default();
	let (mut bench, mut duration, mut output) = (None, None, None);
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
	Ok(o)
}

fn main() -> ExitCode {
	match parse() {
		Ok(options) => match run(options) {
			AppExit::Success => ExitCode::SUCCESS,
			AppExit::Error(_) => ExitCode::FAILURE,
		},
		Err(e) => {
			eprintln!("{e}\n{USAGE}");
			ExitCode::from(2)
		}
	}
}
