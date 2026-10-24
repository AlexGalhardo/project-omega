# Evidence: GAME-P3-008

Status: DONE. Base commit: `8081ef1`. Environment: Windows 11, Rust 1.99.1, Bevy 0.20.0 vendored fork.

## Commands

| Command | Exit | Result |
| --- | --- | --- |
| `cargo test -p omega_game --no-default-features --locked --test persistence_contract` | 0 | 13 passed |
| `cargo clippy --workspace --no-default-features --all-targets --locked -- -D warnings` | 0 | clean |
| `powershell -NoProfile -File scripts/check.ps1` | 0 | all gates passed |

## Criterion to test

| Criterion | Tests |
| --- | --- |
| SP-010-AC1 / DA-14 first wave-3 completion grants the bundle once; pools match the GDD (1/6/20 initial, 4/12/40 all) | `ac1_wave_three_grants_the_first_shift_bundle_once`, `full_availability_matches_the_gdd_pools` |
| SP-010-AC2 duplicate requests grant nothing more | `ac2_duplicate_evaluation_grants_nothing_more` |
| The current run pool stays frozen; the grant reaches the next run; a save is owed | `the_current_runs_pool_stays_frozen_while_the_next_run_gets_the_grant` |

## Limits

Grants fire from the run lifecycle on successful wave completion after death precedence. Wave 5 and 10 grants need the boss encounters (GAME-P3-004/005). The profile persists through GAME-P3-009.
