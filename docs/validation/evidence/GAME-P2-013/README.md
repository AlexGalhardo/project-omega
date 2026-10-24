# Evidence: GAME-P2-013

Status: DONE pending integration (work left uncommitted in worktree `gameplay-p2-013`). Base commit: `0fffd87`. Environment: Windows 11, Rust 1.99.1, Bevy 0.20.0 vendored fork.

## Commands

| Command | Exit | Result |
| --- | --- | --- |
| `cargo test -p omega_game --no-default-features --locked --test balance_contract` | 0 | 4 passed |
| `cargo test -p omega_game --no-default-features --locked` | 0 | all suites passed (balance_contract 4, run_contract 9, shop_contract 14, encounter_contract 11, prototype_content_contract 3, others unchanged) |
| `cargo clippy --workspace --no-default-features --all-targets --locked -- -D warnings` | 0 | clean |
| `cargo fmt --all -- --check` | 0 | clean |
| `markdownlint-cli2` on the new Markdown files | 0 | 0 errors |
| `omega-sim --scenario balance --policies idle,human_like --seeds 200 --label quick` (release, original content) | 0 | both policies 100 % wins; [matrix-quick-original-content.md](matrix-quick-original-content.md) |
| `omega-sim --scenario balance --policies all --seeds 200 --label 2026-10-24-before` (release, original content) | 0 | all six policies 100 % wins; [matrix-before-original-content.md](matrix-before-original-content.md) |
| `omega-sim --scenario balance --policies all --seeds 200 --label 2026-10-24-after` (release, CR-004 content) | 0 | `idle` 0 %, `kiter` 37.5 %, `human_like` 59.5 %; [matrix-after-cr-004.md](matrix-after-cr-004.md) |
| `omega-sim --scenario balance --policies random --seeds 2000 --seed-start 1000 --label 2026-10-24-random-2000-after` (release) | 0 | `random` 5.2 % wins, `wpn_saw` ownership +20.7 points; [matrix-random-2000-after-cr-004.md](matrix-random-2000-after-cr-004.md) |
| `cargo run -p omega_game --no-default-features --locked --bin omega-sim -- --scenario balance --policies idle,human_like --seeds 2 --label smoke --out artifacts/balance/smoke.json` (debug) | 0 | 5 s, proposed check/CI smoke step |
| `omega-sim --scenario balance --policy nonsense` | 2 | usage error before any run |

## Criterion to test

| Criterion | Tests |
| --- | --- |
| A fixed-seed balance scenario is repeatable (identical JSON twice, different seeds differ) | `a_fixed_seed_balance_scenario_is_repeatable` |
| `idle` dies in waves 1-2 | `the_idle_policy_dies_in_waves_one_or_two` |
| All six policies finish runs and produce records, Markdown and band checks | `every_policy_finishes_a_run_and_records_its_metrics` |
| Metric collection and observation never touch RNG or authoritative state | `observing_never_changes_the_simulation` |
| Prototype bands met or each miss has a proposal | [report](../../reports/2026-10-24-prototype-balance.md), [CR-004](../../../decisions/change-requests/cr-004-prototype-pressure-calibration.md) |

## Limits

JSON artifacts live in the git-ignored `artifacts/balance/`; the Markdown outputs above are the committed evidence. Two bands remain recorded misses with proposals (survival curve of `kiter`, Chain Saw ownership delta) and two cannot be measured with one character and three waves. Bots are heuristics; the per-source damage breakdown is a single bucket until a non-contact enemy exists. CR-004 numbers are proposed to the GDD; the integrator edits the GDD and the roadmap status.
