//! Headless balance simulator (ROADMAP rule 1.12, docs/validation/balancing.md).
//!
//! Complete runs go through the real `GameTick` schedule. Bot policies only read what a
//! player can see (positions, HP, scrap, offers) and answer with `TickInput` and shop
//! commands; metric collection reads the world and the presentation stream and never
//! touches an RNG stream or authoritative state.

mod metrics;
mod observe;
mod policy;
mod report;
mod runner;

pub use metrics::{BalanceReport, PolicySummary, RunRecord, ShopRecord, WaveRecord};
pub use observe::{Observation, Observer};
pub use policy::{Policy, ShopAction};
pub use report::{BandCheck, bands, markdown};
pub use runner::{load_content, run_one};

use crate::content::ContentSnapshot;
use sha2::{Digest, Sha256};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

/// What to simulate: every policy against every seed of the range.
#[derive(Debug, Clone)]
pub struct MatrixSpec {
	pub policies: Vec<Policy>,
	pub seed_start: u64,
	pub seeds: u64,
	pub character: String,
	pub label: String,
	pub threads: usize,
}

impl MatrixSpec {
	pub fn new(policies: Vec<Policy>, seed_start: u64, seeds: u64) -> Self {
		Self {
			policies,
			seed_start,
			seeds,
			character: "char_patch".into(),
			label: "balance".into(),
			threads: std::thread::available_parallelism().map_or(1, usize::from),
		}
	}
}

/// Run seed for matrix index `index`; the same index is shared by every policy so the
/// spawn and shop streams are comparable across policies.
pub fn seed_for(index: u64) -> crate::sim::RunSeed {
	let mut hasher = Sha256::new();
	hasher.update(b"omega-balance-seed");
	hasher.update(index.to_le_bytes());
	crate::sim::RunSeed(hasher.finalize().into())
}

/// Runs the matrix and aggregates it. Results are slotted by job index, so the report
/// does not depend on thread scheduling.
pub fn run_matrix(
	content: &Arc<ContentSnapshot>,
	spec: &MatrixSpec,
) -> Result<BalanceReport, String> {
	let jobs: Vec<(Policy, u64)> = spec
		.policies
		.iter()
		.flat_map(|&p| (0..spec.seeds).map(move |i| (p, spec.seed_start + i)))
		.collect();
	let slots: Mutex<Vec<Option<Result<RunRecord, String>>>> =
		Mutex::new((0..jobs.len()).map(|_| None).collect());
	let next = AtomicUsize::new(0);
	let threads = spec.threads.clamp(1, jobs.len().max(1));
	std::thread::scope(|scope| {
		for _ in 0..threads {
			scope.spawn(|| {
				loop {
					let job = next.fetch_add(1, Ordering::Relaxed);
					let Some(&(policy, index)) = jobs.get(job) else {
						break;
					};
					let record = run_one(content, &spec.character, policy, index);
					if let Ok(mut guard) = slots.lock() {
						guard[job] = Some(record);
					}
				}
			});
		}
	});
	let mut runs = Vec::with_capacity(jobs.len());
	let finished = slots
		.into_inner()
		.map_err(|_| "worker panicked".to_string())?;
	for slot in finished {
		runs.push(slot.ok_or("a worker panicked before finishing its job")??);
	}
	Ok(BalanceReport::new(content, spec, runs))
}
