//! Prototype band evaluation and the Markdown summary of a [`BalanceReport`].

use super::metrics::{BalanceReport, OwnedStat, PolicySummary, ratio};
use super::policy::Policy;
use std::collections::BTreeMap;
use std::fmt::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BandStatus {
	Pass,
	Miss,
	/// The policy the band needs is not in this matrix.
	NotRun,
	/// The prototype content cannot exercise the band (one character, three waves).
	NotApplicable,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BandCheck {
	pub name: &'static str,
	pub target: &'static str,
	pub observed: String,
	pub status: BandStatus,
}

impl BandStatus {
	fn label(self) -> &'static str {
		match self {
			Self::Pass => "pass",
			Self::Miss => "MISS",
			Self::NotRun => "not run",
			Self::NotApplicable => "n/a (prototype)",
		}
	}
}

fn pct(value: f64) -> String {
	format!("{:.1} %", value * 100.0)
}

fn check(name: &'static str, target: &'static str, observed: Option<(String, bool)>) -> BandCheck {
	match observed {
		Some((observed, ok)) => BandCheck {
			name,
			target,
			observed,
			status: if ok {
				BandStatus::Pass
			} else {
				BandStatus::Miss
			},
		},
		None => BandCheck {
			name,
			target,
			observed: "-".into(),
			status: BandStatus::NotRun,
		},
	}
}

fn judged(report: &BalanceReport) -> impl Iterator<Item = &PolicySummary> {
	report
		.summaries
		.iter()
		.filter(|s| !matches!(s.policy, Policy::Idle | Policy::Random))
}

/// Largest drop in cumulative survival from one wave to the next (wave 1 starts at 100 %).
fn max_drop(summary: &PolicySummary) -> f64 {
	let mut previous = 1.0;
	let mut worst: f64 = 0.0;
	for value in summary.survival.values() {
		worst = worst.max(previous - value);
		previous = *value;
	}
	worst
}

pub fn bands(report: &BalanceReport) -> Vec<BandCheck> {
	let mut out = Vec::new();
	out.push(check(
		"idle dies in waves 1-2",
		"all idle runs end in wave 1 or 2",
		report.summary(Policy::Idle).map(|s| {
			let early: u32 = s.deaths_by_wave.range(..=2).map(|(_, n)| n).sum();
			let share = ratio(early, s.runs);
			(pct(share), share >= 0.99)
		}),
	));
	let human = report.summary(Policy::HumanLike);
	out.push(check(
		"human_like clears wave 1",
		">= 90 %",
		human.map(|s| {
			let v = s.survival.get(&1).copied().unwrap_or(0.0);
			(pct(v), v >= 0.90)
		}),
	));
	out.push(check(
		"human_like clears the last wave",
		"50-80 %",
		human.map(|s| (pct(s.win_rate), (0.50..=0.80).contains(&s.win_rate))),
	));
	out.push(check(
		"kiter win rate",
		"<= 75 %",
		report
			.summary(Policy::Kiter)
			.map(|s| (pct(s.win_rate), s.win_rate <= 0.75)),
	));
	let drops: Vec<(Policy, f64)> = judged(report).map(|s| (s.policy, max_drop(s))).collect();
	out.push(check(
		"survival curve drop",
		"<= 25 points per wave (non-boss)",
		(!drops.is_empty()).then(|| {
			let (who, worst) = drops
				.iter()
				.copied()
				.max_by(|a, b| a.1.total_cmp(&b.1))
				.unwrap_or((Policy::Idle, 0.0));
			(
				format!("worst {:.1} points ({who})", worst * 100.0),
				worst <= 0.25,
			)
		}),
	));
	let mut worst_delta: Option<(f64, String)> = None;
	// Random buys independently of skill, so it is the cleanest ownership experiment.
	for s in report.summaries.iter().filter(|s| s.policy != Policy::Idle) {
		for (id, stat) in s.weapons.iter().chain(s.items.iter()) {
			if stat.owned_runs >= 50
				&& stat.other_runs >= 50
				&& let Some(delta) = stat.win_delta()
				&& worst_delta.as_ref().is_none_or(|(d, _)| delta > *d)
			{
				worst_delta = Some((delta, format!("{id} under {}", s.policy)));
			}
		}
	}
	out.push(check(
		"no item or weapon above +15 points when owned",
		"<= 15 points (cells with >= 50 runs on both sides)",
		worst_delta.map(|(d, who)| (format!("worst {:+.1} points ({who})", d * 100.0), d <= 0.15)),
	));
	let mut weapon_rates: BTreeMap<&str, (u32, u32)> = BTreeMap::new();
	for s in judged(report) {
		for (id, stat) in &s.weapons {
			let e = weapon_rates.entry(id).or_default();
			e.0 += stat.bought;
			e.1 += stat.offered;
		}
	}
	out.push(check(
		"every weapon is picked",
		">= 3 % of offers, policies other than random and idle",
		(!weapon_rates.is_empty()).then(|| {
			let (id, (bought, offered)) = weapon_rates
				.iter()
				.map(|(id, v)| (*id, *v))
				.min_by(|a, b| ratio(a.1.0, a.1.1).total_cmp(&ratio(b.1.0, b.1.1)))
				.unwrap_or(("", (0, 0)));
			let rate = ratio(bought, offered);
			(format!("lowest {id} {}", pct(rate)), rate >= 0.03)
		}),
	));
	let (mut single, mut spread) = (0u32, 0u32);
	for s in &report.summaries {
		for (pattern, stat) in &s.energy_patterns {
			let powered = pattern.split('-').filter(|p| *p != "none").count();
			if powered >= 2 {
				spread += stat.wins;
			} else {
				single += stat.wins;
			}
		}
	}
	out.push(check(
		"energy: spread and single both win",
		">= 5 winning runs of each pattern class",
		(!report.summaries.is_empty()).then(|| {
			(
				format!("single {single} wins, spread {spread} wins"),
				single >= 5 && spread >= 5,
			)
		}),
	));
	for (name, target) in [
		(
			"characters within 15 points",
			"human_like win rates within 15 points",
		),
		("full-run length", "12-20 minutes of combat (full game)"),
	] {
		out.push(BandCheck {
			name,
			target,
			observed: "one character, three 60 s waves".into(),
			status: BandStatus::NotApplicable,
		});
	}
	out
}

fn owned_table(out: &mut String, report: &BalanceReport, title: &str, weapons: bool) {
	let _ = writeln!(out, "\n### {title}\n");
	let _ = write!(out, "| ID |");
	let policies: Vec<&PolicySummary> = report
		.summaries
		.iter()
		.filter(|s| s.policy != Policy::Idle)
		.collect();
	for s in &policies {
		let _ = write!(out, " {} pick | {} win delta |", s.policy, s.policy);
	}
	let _ = writeln!(out);
	let _ = write!(out, "| --- |");
	for _ in &policies {
		let _ = write!(out, " ---: | ---: |");
	}
	let _ = writeln!(out);
	fn pick(s: &PolicySummary, weapons: bool) -> &BTreeMap<String, OwnedStat> {
		if weapons { &s.weapons } else { &s.items }
	}
	let ids: Vec<&String> = policies
		.first()
		.map(|s| pick(s, weapons).keys().collect())
		.unwrap_or_default();
	for id in ids {
		let _ = write!(out, "| `{id}` |");
		for s in &policies {
			let stat: &OwnedStat = &pick(s, weapons)[id];
			let delta = stat.win_delta().map_or("-".to_string(), |d| {
				format!(
					"{:+.0} (n={}/{})",
					d * 100.0,
					stat.owned_runs,
					stat.other_runs
				)
			});
			let _ = write!(out, " {} | {} |", pct(stat.pick_rate()), delta);
		}
		let _ = writeln!(out);
	}
}

pub fn markdown(report: &BalanceReport) -> String {
	let mut out = String::new();
	let _ = writeln!(out, "## Run summary\n");
	let _ = writeln!(
		out,
		"Label `{}`, character `{}`, content profile {}, content revision `{}`, seeds {}..{} ({} per policy), {} waves. Percentiles: {}.\n",
		report.label,
		report.character,
		report.content_profile,
		report.content_revision,
		report.seed_start,
		report.seed_start + report.seeds,
		report.seeds,
		report.total_waves,
		report.percentile_method
	);
	let _ = writeln!(
		out,
		"| Policy | Runs | Win rate | Combat s p50 | Combat s p95 | Deaths by wave | Distinct winning builds |"
	);
	let _ = writeln!(out, "| --- | ---: | ---: | ---: | ---: | --- | ---: |");
	for s in &report.summaries {
		let (p50, p95) = s.combat_seconds.map_or((0.0, 0.0), |p| (p.p50, p.p95));
		let deaths = s
			.deaths_by_wave
			.iter()
			.map(|(w, n)| format!("w{w}: {n}"))
			.collect::<Vec<_>>()
			.join(", ");
		let _ = writeln!(
			out,
			"| {} | {} | {} | {:.1} | {:.1} | {} | {}/{} |",
			s.policy,
			s.runs,
			pct(s.win_rate),
			p50,
			p95,
			if deaths.is_empty() {
				"-".into()
			} else {
				deaths
			},
			s.distinct_winning_builds,
			s.winning_builds
		);
	}
	let _ = writeln!(out, "\n### Input styles inside a policy\n");
	let _ = writeln!(
		out,
		"| Policy | Style | Runs | Win rate | Cleared wave 1 | Cleared wave 2 |"
	);
	let _ = writeln!(out, "| --- | --- | ---: | ---: | ---: | ---: |");
	for s in &report.summaries {
		for (style, stat) in &s.styles {
			let cleared =
				|w: u32| pct(ratio(stat.cleared.get(&w).copied().unwrap_or(0), stat.runs));
			let _ = writeln!(
				out,
				"| {} | {style} | {} | {} | {} | {} |",
				s.policy,
				stat.runs,
				pct(ratio(stat.wins, stat.runs)),
				cleared(1),
				cleared(2)
			);
		}
	}
	let _ = writeln!(out, "\n## Prototype bands\n");
	let _ = writeln!(out, "| Check | Target | Observed | Status |");
	let _ = writeln!(out, "| --- | --- | --- | --- |");
	for b in bands(report) {
		let _ = writeln!(
			out,
			"| {} | {} | {} | {} |",
			b.name,
			b.target,
			b.observed,
			b.status.label()
		);
	}
	let _ = writeln!(out, "\n## Survival (share of runs that cleared the wave)\n");
	let _ = write!(out, "| Policy |");
	for w in 1..=report.total_waves {
		let _ = write!(out, " Wave {w} |");
	}
	let _ = write!(out, "\n| --- |");
	for _ in 1..=report.total_waves {
		let _ = write!(out, " ---: |");
	}
	let _ = writeln!(out);
	for s in &report.summaries {
		let _ = write!(out, "| {} |", s.policy);
		for v in s.survival.values() {
			let _ = write!(out, " {} |", pct(*v));
		}
		let _ = writeln!(out);
	}
	let _ = writeln!(out, "\n## Damage taken and scrap earned per wave (mean)\n");
	let _ = writeln!(out, "| Policy | Wave | Damage taken | Scrap earned |");
	let _ = writeln!(out, "| --- | ---: | ---: | ---: |");
	for s in &report.summaries {
		for (wave, damage) in &s.damage_taken_mean {
			let earned = s.scrap_earned_mean.get(wave).copied().unwrap_or(0.0);
			let _ = writeln!(out, "| {} | {wave} | {damage:.1} | {earned:.1} |", s.policy);
		}
	}
	let _ = writeln!(out, "\n## Shop economy per visit (mean scrap)\n");
	let _ = writeln!(out, "| Policy | Visit | Held on entry | Spent | Unspent |");
	let _ = writeln!(out, "| --- | ---: | ---: | ---: | ---: |");
	for s in &report.summaries {
		for (visit, before) in &s.shop_scrap_before_mean {
			let spent = s.shop_spent_mean.get(visit).copied().unwrap_or(0.0);
			let unspent = s.shop_unspent_mean.get(visit).copied().unwrap_or(0.0);
			let _ = writeln!(
				out,
				"| {} | {visit} | {before:.1} | {spent:.1} | {unspent:.1} |",
				s.policy
			);
		}
	}
	let _ = writeln!(out, "\n## Time to kill (seconds from spawn to death)\n");
	let _ = writeln!(out, "| Policy | Enemy | Kills | p50 | p95 | max |");
	let _ = writeln!(out, "| --- | --- | ---: | ---: | ---: | ---: |");
	for s in &report.summaries {
		for (enemy, p) in &s.ttk_seconds {
			let kills = report
				.runs
				.iter()
				.filter(|r| r.policy == s.policy)
				.flat_map(|r| &r.kills)
				.filter(|k| &k.enemy == enemy)
				.count();
			if let Some(p) = p {
				let _ = writeln!(
					out,
					"| {} | `{enemy}` | {kills} | {:.2} | {:.2} | {:.2} |",
					s.policy, p.p50, p.p95, p.max
				);
			}
		}
	}
	owned_table(
		&mut out,
		report,
		"Weapon pick rate and win delta when owned",
		true,
	);
	owned_table(
		&mut out,
		report,
		"Item pick rate and win delta when owned",
		false,
	);
	let _ = writeln!(
		out,
		"\nPick rate is purchases over offers shown. Win delta is the win rate of runs that owned the entry at the start of the last wave minus the rest, in points (n owned / n other)."
	);
	let _ = writeln!(
		out,
		"\n## Energy allocation at the start of the last wave\n"
	);
	let _ = writeln!(out, "| Policy | Pattern | Runs | Wins | Win rate |");
	let _ = writeln!(out, "| --- | --- | ---: | ---: | ---: |");
	for s in &report.summaries {
		for (pattern, stat) in &s.energy_patterns {
			let _ = writeln!(
				out,
				"| {} | {} | {} | {} | {} |",
				s.policy,
				pattern,
				stat.runs,
				stat.wins,
				pct(ratio(stat.wins, stat.runs))
			);
		}
	}
	out
}
