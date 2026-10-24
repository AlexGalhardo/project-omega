//! Measurement reports (SP-016, performance plan). Diagnostics only observe: they
//! never touch RNG streams or authoritative state. Unobserved metrics are listed as
//! unsupported, never reported as zero.

use serde::Serialize;

/// Nearest-rank percentiles over raw samples (`ceil(p*N)-1` index).
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Percentiles {
	pub p50: f64,
	pub p95: f64,
	pub p99: f64,
	pub max: f64,
}

impl Percentiles {
	/// `None` for an empty sample set.
	pub fn nearest_rank(samples: &[f64]) -> Option<Self> {
		if samples.is_empty() {
			return None;
		}
		let mut sorted = samples.to_vec();
		sorted.sort_by(f64::total_cmp);
		let at = |p: f64| sorted[((p * sorted.len() as f64).ceil() as usize).saturating_sub(1)];
		Some(Self {
			p50: at(0.50),
			p95: at(0.95),
			p99: at(0.99),
			max: sorted[sorted.len() - 1],
		})
	}
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ReportMetadata {
	pub scenario: String,
	pub package_version: &'static str,
	pub build_profile: &'static str,
	pub target_os: &'static str,
	pub target_arch: &'static str,
	/// From `OMEGA_COMMIT`, set by the measurement scripts; `unknown` otherwise.
	pub commit: String,
	/// From `PROCESSOR_IDENTIFIER` on Windows; `unknown` otherwise.
	pub cpu: String,
	pub seed_hex: String,
	pub content_revision: Option<String>,
	pub settings: Vec<(String, String)>,
	pub percentile_method: &'static str,
}

impl ReportMetadata {
	pub fn new(scenario: &str, seed_hex: String) -> Self {
		let env = |k: &str| std::env::var(k).unwrap_or_else(|_| "unknown".into());
		Self {
			scenario: scenario.to_string(),
			package_version: env!("CARGO_PKG_VERSION"),
			build_profile: if cfg!(debug_assertions) {
				"debug"
			} else {
				"release"
			},
			target_os: std::env::consts::OS,
			target_arch: std::env::consts::ARCH,
			commit: env("OMEGA_COMMIT"),
			cpu: env("PROCESSOR_IDENTIFIER"),
			seed_hex,
			content_revision: None,
			settings: Vec::new(),
			percentile_method: "nearest-rank, index ceil(p*N)-1",
		}
	}
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Metric {
	pub name: String,
	pub unit: &'static str,
	pub samples: usize,
	pub percentiles: Option<Percentiles>,
}

impl Metric {
	pub fn from_samples(name: &str, unit: &'static str, samples: &[f64]) -> Self {
		Self {
			name: name.to_string(),
			unit,
			samples: samples.len(),
			percentiles: Percentiles::nearest_rank(samples),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Report {
	pub metadata: ReportMetadata,
	pub metrics: Vec<Metric>,
	/// Metrics the performance plan names that this run could not observe.
	pub unsupported: Vec<String>,
}

impl Report {
	pub fn to_json(&self) -> String {
		serde_json::to_string_pretty(self).expect("report serializes")
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn nearest_rank_matches_definition() {
		let samples: Vec<f64> = (1..=100).map(f64::from).collect();
		let p = Percentiles::nearest_rank(&samples).unwrap();
		assert_eq!((p.p50, p.p95, p.p99, p.max), (50.0, 95.0, 99.0, 100.0));
		let one = Percentiles::nearest_rank(&[7.0]).unwrap();
		assert_eq!((one.p50, one.p99, one.max), (7.0, 7.0, 7.0));
		assert!(Percentiles::nearest_rank(&[]).is_none());
	}
}
