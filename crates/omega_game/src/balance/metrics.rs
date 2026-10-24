//! Per-run records and their aggregation. Everything here is computed from finished
//! records, so collecting and summarizing can never influence a run.

use super::policy::Policy;
use super::{MatrixSpec, seed_for};
use crate::content::ContentSnapshot;
use crate::diagnostics::Percentiles;
use crate::sim::TICKS_PER_SECOND;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WaveRecord {
	pub wave: u32,
	pub cleared: bool,
	pub combat_ticks: u64,
	pub damage_taken: u32,
	pub scrap_earned: u32,
	pub kills: u32,
	pub hp_end: u32,
	/// Build the wave started with.
	pub weapons: Vec<String>,
	/// Tier of each entry of `weapons`.
	pub tiers: Vec<u8>,
	pub items: Vec<String>,
	pub energy: [u8; 4],
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ShopRecord {
	pub visit: u32,
	pub scrap_before: u32,
	pub spent: u32,
	pub unspent: u32,
	pub refreshed: bool,
	/// Offer IDs shown (first draw and refresh), for pick rates.
	pub offered: Vec<String>,
	pub bought: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct KillRecord {
	pub enemy: String,
	pub wave: u32,
	pub ttk_ticks: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RunRecord {
	pub policy: Policy,
	pub seed_index: u64,
	/// Input style inside the policy, when it has several.
	pub style: Option<String>,
	pub victory: bool,
	/// Waves fully cleared before the run ended.
	pub waves_cleared: u32,
	pub combat_ticks: u64,
	pub final_tick: u64,
	pub waves: Vec<WaveRecord>,
	pub shops: Vec<ShopRecord>,
	pub kills: Vec<KillRecord>,
}

impl RunRecord {
	/// Wave number the player died in.
	pub fn death_wave(&self) -> Option<u32> {
		(!self.victory).then(|| self.waves_cleared + 1)
	}
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct OwnedStat {
	pub offered: u32,
	pub bought: u32,
	/// Runs that reached the last wave owning / not owning the entry, and their wins.
	pub owned_runs: u32,
	pub owned_wins: u32,
	pub other_runs: u32,
	pub other_wins: u32,
}

impl OwnedStat {
	pub fn pick_rate(&self) -> f64 {
		ratio(self.bought, self.offered)
	}

	pub fn win_delta(&self) -> Option<f64> {
		(self.owned_runs > 0 && self.other_runs > 0).then(|| {
			ratio(self.owned_wins, self.owned_runs) - ratio(self.other_wins, self.other_runs)
		})
	}
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct EnergyStat {
	pub runs: u32,
	pub wins: u32,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct StyleStat {
	pub runs: u32,
	pub wins: u32,
	pub cleared: BTreeMap<u32, u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PolicySummary {
	pub policy: Policy,
	pub runs: u32,
	pub wins: u32,
	pub win_rate: f64,
	/// Runs that died in wave `n`.
	pub deaths_by_wave: BTreeMap<u32, u32>,
	/// Fraction of all runs that cleared wave `n`.
	pub survival: BTreeMap<u32, f64>,
	pub combat_seconds: Option<Percentiles>,
	pub ttk_seconds: BTreeMap<String, Option<Percentiles>>,
	pub damage_taken_mean: BTreeMap<u32, f64>,
	pub scrap_earned_mean: BTreeMap<u32, f64>,
	pub shop_scrap_before_mean: BTreeMap<u32, f64>,
	pub shop_spent_mean: BTreeMap<u32, f64>,
	pub shop_unspent_mean: BTreeMap<u32, f64>,
	pub weapons: BTreeMap<String, OwnedStat>,
	pub items: BTreeMap<String, OwnedStat>,
	/// Energy pattern at the start of the last wave played, by sorted point vector.
	pub energy_patterns: BTreeMap<String, EnergyStat>,
	pub winning_builds: u32,
	pub distinct_winning_builds: u32,
	/// Per input style: runs, wins and runs that cleared each wave.
	pub styles: BTreeMap<String, StyleStat>,
}

pub(super) fn ratio(a: u32, b: u32) -> f64 {
	if b == 0 {
		0.0
	} else {
		f64::from(a) / f64::from(b)
	}
}

fn mean(values: &[f64]) -> f64 {
	if values.is_empty() {
		0.0
	} else {
		values.iter().sum::<f64>() / values.len() as f64
	}
}

fn per_key_mean(samples: BTreeMap<u32, Vec<f64>>) -> BTreeMap<u32, f64> {
	samples.into_iter().map(|(k, v)| (k, mean(&v))).collect()
}

/// Sorted nonzero point vector, for example `3-3` or `3-2-1`.
pub fn energy_pattern(energy: [u8; 4]) -> String {
	let mut points: Vec<u8> = energy.into_iter().filter(|&p| p > 0).collect();
	points.sort_unstable_by(|a, b| b.cmp(a));
	if points.is_empty() {
		return "none".into();
	}
	points
		.iter()
		.map(u8::to_string)
		.collect::<Vec<_>>()
		.join("-")
}

fn summarize(
	policy: Policy,
	runs: &[&RunRecord],
	weapon_ids: &[String],
	item_ids: &[String],
	total_waves: u32,
) -> PolicySummary {
	let n = runs.len() as u32;
	let wins = runs.iter().filter(|r| r.victory).count() as u32;
	let mut deaths_by_wave = BTreeMap::new();
	for wave in runs.iter().filter_map(|r| r.death_wave()) {
		*deaths_by_wave.entry(wave).or_insert(0u32) += 1;
	}
	let survival = (1..=total_waves)
		.map(|w| {
			let cleared = runs.iter().filter(|r| r.waves_cleared >= w).count() as u32;
			(w, ratio(cleared, n))
		})
		.collect();
	let seconds = |ticks: u64| ticks as f64 / f64::from(TICKS_PER_SECOND);
	let lengths: Vec<f64> = runs.iter().map(|r| seconds(r.combat_ticks)).collect();
	let mut ttk: BTreeMap<String, Vec<f64>> = BTreeMap::new();
	let (mut damage, mut earned) = (BTreeMap::new(), BTreeMap::new());
	let (mut before, mut spent, mut unspent) = (BTreeMap::new(), BTreeMap::new(), BTreeMap::new());
	for run in runs {
		for kill in &run.kills {
			ttk.entry(kill.enemy.clone())
				.or_default()
				.push(seconds(kill.ttk_ticks));
		}
		for wave in &run.waves {
			damage
				.entry(wave.wave)
				.or_insert_with(Vec::new)
				.push(f64::from(wave.damage_taken));
			earned
				.entry(wave.wave)
				.or_insert_with(Vec::new)
				.push(f64::from(wave.scrap_earned));
		}
		for shop in &run.shops {
			before
				.entry(shop.visit)
				.or_insert_with(Vec::new)
				.push(f64::from(shop.scrap_before));
			spent
				.entry(shop.visit)
				.or_insert_with(Vec::new)
				.push(f64::from(shop.spent));
			unspent
				.entry(shop.visit)
				.or_insert_with(Vec::new)
				.push(f64::from(shop.unspent));
		}
	}
	let mut weapons: BTreeMap<String, OwnedStat> = weapon_ids
		.iter()
		.map(|id| (id.clone(), OwnedStat::default()))
		.collect();
	let mut items: BTreeMap<String, OwnedStat> = item_ids
		.iter()
		.map(|id| (id.clone(), OwnedStat::default()))
		.collect();
	let mut energy_patterns: BTreeMap<String, EnergyStat> = BTreeMap::new();
	let mut builds = std::collections::BTreeSet::new();
	for run in runs {
		for shop in &run.shops {
			for id in &shop.offered {
				if let Some(s) = weapons.get_mut(id).or_else(|| items.get_mut(id)) {
					s.offered += 1;
				}
			}
			for id in &shop.bought {
				if let Some(s) = weapons.get_mut(id).or_else(|| items.get_mut(id)) {
					s.bought += 1;
				}
			}
		}
		let Some(last) = run.waves.last().filter(|w| w.wave == total_waves) else {
			continue;
		};
		for (id, stat) in weapons.iter_mut().chain(items.iter_mut()) {
			let owned = last.weapons.contains(id) || last.items.contains(id);
			let (r, w) = if owned {
				(&mut stat.owned_runs, &mut stat.owned_wins)
			} else {
				(&mut stat.other_runs, &mut stat.other_wins)
			};
			*r += 1;
			*w += u32::from(run.victory);
		}
		let e = energy_patterns
			.entry(energy_pattern(last.energy))
			.or_default();
		e.runs += 1;
		e.wins += u32::from(run.victory);
		if run.victory {
			builds.insert((
				last.weapons.clone(),
				last.tiers.clone(),
				last.items.clone(),
				last.energy,
			));
		}
	}
	let mut styles: BTreeMap<String, StyleStat> = BTreeMap::new();
	for run in runs {
		if let Some(name) = &run.style {
			let s = styles.entry(name.clone()).or_default();
			s.runs += 1;
			s.wins += u32::from(run.victory);
			for w in 1..=run.waves_cleared {
				*s.cleared.entry(w).or_insert(0) += 1;
			}
		}
	}
	PolicySummary {
		policy,
		runs: n,
		wins,
		win_rate: ratio(wins, n),
		deaths_by_wave,
		survival,
		combat_seconds: Percentiles::nearest_rank(&lengths),
		ttk_seconds: ttk
			.into_iter()
			.map(|(k, v)| (k, Percentiles::nearest_rank(&v)))
			.collect(),
		damage_taken_mean: per_key_mean(damage),
		scrap_earned_mean: per_key_mean(earned),
		shop_scrap_before_mean: per_key_mean(before),
		shop_spent_mean: per_key_mean(spent),
		shop_unspent_mean: per_key_mean(unspent),
		weapons,
		items,
		energy_patterns,
		winning_builds: wins,
		distinct_winning_builds: builds.len() as u32,
		styles,
	}
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct BalanceReport {
	pub scenario: &'static str,
	pub label: String,
	pub content_revision: String,
	pub content_profile: String,
	pub character: String,
	pub seed_start: u64,
	pub seeds: u64,
	/// Hex of the run seed for `seed_start`, to reproduce the first run by hand.
	pub first_seed_hex: String,
	pub total_waves: u32,
	pub percentile_method: &'static str,
	pub summaries: Vec<PolicySummary>,
	pub runs: Vec<RunRecord>,
}

impl BalanceReport {
	pub(super) fn new(content: &ContentSnapshot, spec: &MatrixSpec, runs: Vec<RunRecord>) -> Self {
		let total_waves = u32::try_from(content.catalogs.waves.len()).unwrap_or(u32::MAX);
		let weapon_ids: Vec<String> = content.catalogs.weapons.keys().cloned().collect();
		let item_ids: Vec<String> = content.catalogs.items.keys().cloned().collect();
		let summaries = spec
			.policies
			.iter()
			.map(|&policy| {
				let subset: Vec<&RunRecord> = runs.iter().filter(|r| r.policy == policy).collect();
				summarize(policy, &subset, &weapon_ids, &item_ids, total_waves)
			})
			.collect();
		Self {
			scenario: "balance",
			label: spec.label.clone(),
			content_revision: content.revision_hex(),
			content_profile: format!("{:?}", content.profile),
			character: spec.character.clone(),
			seed_start: spec.seed_start,
			seeds: spec.seeds,
			first_seed_hex: seed_for(spec.seed_start).to_hex(),
			total_waves,
			percentile_method: "nearest-rank, index ceil(p*N)-1",
			summaries,
			runs,
		}
	}

	pub fn summary(&self, policy: Policy) -> Option<&PolicySummary> {
		self.summaries.iter().find(|s| s.policy == policy)
	}

	pub fn to_json(&self) -> String {
		serde_json::to_string_pretty(self).unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"))
	}
}
