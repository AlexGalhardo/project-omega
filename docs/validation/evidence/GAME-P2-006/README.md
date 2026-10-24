# Evidence: GAME-P2-006

Status: DONE. Base commit: `a46db74`. Environment: Windows 11, Rust 1.99.1, Bevy 0.20.0 vendored fork.

## Commands

| Command | Exit | Result |
| --- | --- | --- |
| `cargo test -p omega_game --no-default-features --locked --test encounter_contract` | 0 | 11 passed |
| `cargo clippy --workspace --no-default-features --all-targets --locked -- -D warnings` | 0 | clean |
| `cargo clippy -p omega_game --features client --all-targets --locked -- -D warnings` | 0 | clean |
| `powershell -NoProfile -File scripts/check.ps1` | 0 | all gates passed |

## Criterion to test

| Criterion | Tests |
| --- | --- |
| Common scaling (GDD 10.1 example) | `common_scaling_matches_the_gdd_example` |
| SP-008-AC1 authored ticks, bosses only on 5/10 | `ac1_batches_are_requested_at_authored_ticks_and_bosses_only_on_5_and_10` |
| Anchor boundary, 180 LU exclusion, no reuse before exhaustion | `anchors_are_on_the_boundary_and_far_from_the_player`, `anchors_near_the_player_are_excluded` |
| Seeded repeatability | `same_seed_gives_the_same_warnings` |
| Chase profile | `chasers_close_distance_at_their_speed_and_stop_on_equal_centers` |
| SP-008-AC2 / DA-18 postpone, cancel after 10 | `da18_occupied_common_warning_postpones_then_cancels_without_reward`, `da18_warning_materializes_once_the_player_moves_away` |
| SP-008-AC4 / DA-11 timer completion | `da11_timer_completion_removes_survivors_without_scrap_and_keeps_pickups`, `a_pending_boss_keeps_the_wave_open_at_timer_expiry`, `a_dead_player_never_completes_the_wave` |

## Limits

Boss warnings and settlement belong to GAME-P3-004: a wave that authors a boss stays open (`boss_blocking`). Top edge of the arena is +Y. Only the Chase profile is implemented; other profiles are GAME-P3-016 to GAME-P3-019.
