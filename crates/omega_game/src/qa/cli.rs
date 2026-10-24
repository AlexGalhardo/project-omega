//! Session bookkeeping shared by the headless host and the native client, plus the
//! headless entry point behind `omega-game --headless`.

use super::bot::{Auditor, Bot, BotProfile, Finding, QaReport};
use super::script::{Checkpoint, CheckpointResult, Script, ScriptDriver, ScriptMode, evaluate};
use super::{HeadlessHost, InputDriver, Snapshot, Step};
use crate::sim::RunSeed;
use serde::Serialize;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::{Path, PathBuf};

/// Hard stop for runaway scripts: about 9 hours of 60 Hz frames.
const MAX_FRAMES: u64 = 2_000_000;

#[derive(Debug, Clone)]
pub enum QaRun {
	E2e {
		script: PathBuf,
		out_dir: PathBuf,
	},
	Bot {
		profile: BotProfile,
		seed: RunSeed,
		frames: u64,
		resolution: [u32; 2],
		out_dir: PathBuf,
	},
}

enum Driver {
	Script(ScriptDriver),
	Bot(Box<Bot>),
}

impl Driver {
	fn as_dyn(&mut self) -> &mut dyn InputDriver {
		match self {
			Driver::Script(d) => d,
			Driver::Bot(d) => d.as_mut(),
		}
	}
}

#[derive(Serialize)]
struct E2eReport<'a> {
	kind: &'static str,
	script: &'a str,
	seed: String,
	mode: &'a str,
	passed: bool,
	frames: u64,
	ticks: u64,
	error: Option<String>,
	checkpoints: &'a [CheckpointResult],
}

pub struct Finished {
	pub file: PathBuf,
	pub json: String,
	pub passed: bool,
}

/// One scripted or bot-driven run: owns the driver, the checkpoints and the audit.
pub struct Session {
	driver: Driver,
	name: String,
	profile: Option<BotProfile>,
	seed: RunSeed,
	pub resolution: [u32; 2],
	pub out_dir: PathBuf,
	checkpoints: Vec<CheckpointResult>,
	auditor: Auditor,
	frames: u64,
	last: Option<Snapshot>,
	panic: Option<String>,
	prototype: bool,
	rejected: Option<String>,
}

impl Session {
	pub fn from_run(run: &QaRun) -> Result<Self, String> {
		match run {
			QaRun::E2e { script, out_dir } => {
				let text = std::fs::read_to_string(script)
					.map_err(|e| format!("cannot read {}: {e}", script.display()))?;
				let parsed = Script::parse(&text)?;
				let mut session = Self::new(
					Driver::Script(ScriptDriver::new(&parsed)),
					parsed.name.clone(),
					None,
					parsed.run_seed()?,
					parsed.resolution,
					out_dir.clone(),
				);
				session.prototype = parsed.mode == ScriptMode::Prototype;
				Ok(session)
			}
			QaRun::Bot {
				profile,
				seed,
				frames,
				resolution,
				out_dir,
			} => {
				if *frames == 0 || resolution.contains(&0) {
					return Err("bot frames and resolution must be positive".into());
				}
				let mut session = Self::new(
					Driver::Bot(Box::new(Bot::new(*profile, seed, *frames))),
					format!("qa-bot-{profile}-{}", &seed.to_hex()[..8]),
					Some(*profile),
					*seed,
					*resolution,
					out_dir.clone(),
				);
				session.prototype = true;
				Ok(session)
			}
		}
	}

	fn new(
		driver: Driver,
		name: String,
		profile: Option<BotProfile>,
		seed: RunSeed,
		resolution: [u32; 2],
		out_dir: PathBuf,
	) -> Self {
		Self {
			driver,
			name,
			profile,
			seed,
			resolution,
			out_dir,
			checkpoints: Vec::new(),
			auditor: Auditor::default(),
			frames: 0,
			last: None,
			panic: None,
			prototype: false,
			rejected: None,
		}
	}

	/// A window runs the fixed loop on the wall clock, so one rendered frame can carry up to
	/// the catch-up cap of ticks (0.25 s at 60 Hz); headless frames carry exactly one.
	pub fn allow_catch_up_ticks(&mut self) {
		self.auditor.tick_slack = 15;
	}

	/// True when the session drives the real prototype loop rather than the sandbox.
	pub fn is_prototype(&self) -> bool {
		self.prototype
	}

	/// A command the host could not honour (for example a debug op in a windowed run).
	pub fn reject(&mut self, message: String) {
		self.rejected.get_or_insert(message);
	}

	pub fn seed(&self) -> RunSeed {
		self.seed
	}

	/// Headless frame budget; bots stop on their own frame count.
	pub fn frame_budget(&self) -> u64 {
		MAX_FRAMES
	}

	pub fn next(&mut self, snap: &Snapshot) -> Step {
		self.frames += 1;
		if self.profile.is_some() {
			self.auditor.observe(snap);
		}
		self.last = Some(snap.clone());
		self.driver.as_dyn().next(snap)
	}

	pub fn checkpoint(&mut self, cp: &Checkpoint, snap: &Snapshot) -> &mut CheckpointResult {
		let previous = self.checkpoints.last().map(|c| c.snapshot.clone());
		self.checkpoints.push(evaluate(cp, snap, previous.as_ref()));
		self.checkpoints.last_mut().expect("just pushed")
	}

	pub fn record_panic(&mut self, message: String) {
		self.panic = Some(message);
	}

	pub fn finish(mut self, last: Snapshot, mode: &str, error: Option<String>) -> Finished {
		let error = error
			.or_else(|| self.rejected.take())
			.or_else(|| self.driver.as_dyn().failure().map(str::to_owned));
		let ticks = last.tick;
		let (json, passed) = match (self.profile, self.driver) {
			(Some(profile), Driver::Bot(bot)) => {
				let mut findings = self.auditor.findings;
				findings.extend(bot.findings.iter().cloned());
				let panics = u32::from(self.panic.is_some());
				for detail in self.panic.into_iter().chain(error) {
					findings.push(Finding {
						frame: self.frames,
						tick: ticks,
						kind: "run-error",
						detail,
					});
				}
				let run = last.run.as_ref();
				let (completed, outcome, max_wave) = (
					run.is_some_and(|r| r.state == "summary"),
					run.and_then(|r| r.outcome),
					run.map_or(0, |r| r.wave),
				);
				let passed = findings.is_empty();
				let report = QaReport {
					kind: "qa-bot",
					profile,
					seed: self.seed.to_hex(),
					frames: self.frames,
					ticks,
					counts: bot.counts,
					findings,
					panics,
					completed,
					outcome,
					max_wave,
					passed,
					final_snapshot: Some(last),
				};
				(to_json(&report), passed)
			}
			_ => {
				let error = self.panic.or(error);
				let passed = error.is_none() && self.checkpoints.iter().all(|c| c.passed);
				let report = E2eReport {
					kind: "e2e",
					script: &self.name,
					seed: self.seed.to_hex(),
					mode,
					passed,
					frames: self.frames,
					ticks,
					error,
					checkpoints: &self.checkpoints,
				};
				(to_json(&report), passed)
			}
		};
		Finished {
			file: self.out_dir.join(format!("{}.json", self.name)),
			json,
			passed,
		}
	}
}

fn to_json<T: Serialize>(value: &T) -> String {
	serde_json::to_string_pretty(value).unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"))
}

pub fn write_report(finished: &Finished) -> Result<(), String> {
	let dir = finished.file.parent().unwrap_or(Path::new("."));
	std::fs::create_dir_all(dir).map_err(|e| format!("cannot create {}: {e}", dir.display()))?;
	std::fs::write(&finished.file, &finished.json)
		.map_err(|e| format!("cannot write {}: {e}", finished.file.display()))
}

/// Runs the session without a window and returns its finished report.
pub fn run_headless_session(run: &QaRun) -> Result<Finished, String> {
	let mut session = Session::from_run(run)?;
	let mut host = if session.is_prototype() {
		HeadlessHost::prototype(session.seed(), session.resolution)?
	} else {
		HeadlessHost::new(session.seed(), session.resolution)?
	};
	let budget = session.frame_budget();
	let outcome = catch_unwind(AssertUnwindSafe(|| host.run(&mut session, budget)));
	let error = match outcome {
		Ok(error) => error,
		Err(payload) => {
			let message = payload
				.downcast_ref::<&str>()
				.map(|s| (*s).to_owned())
				.or_else(|| payload.downcast_ref::<String>().cloned())
				.unwrap_or_else(|| "panic".into());
			session.record_panic(format!("panic: {message}"));
			None
		}
	};
	let last = session
		.last
		.clone()
		.ok_or_else(|| "run produced no frames".to_owned())?;
	Ok(session.finish(last, "headless", error))
}

/// Process exit code: 0 passed, 1 failed checkpoint or bot finding, 2 usage or setup error.
pub fn run_headless(run: &QaRun) -> u8 {
	match run_headless_session(run).and_then(|f| write_report(&f).map(|()| f)) {
		Ok(finished) => {
			println!(
				"{} {} (headless)",
				if finished.passed { "PASS" } else { "FAIL" },
				finished.file.display()
			);
			u8::from(!finished.passed)
		}
		Err(e) => {
			eprintln!("{e}");
			2
		}
	}
}
