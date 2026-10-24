# Evidence: GAME-P2-010

Status: DONE. Base commit: `6763ef3`. Environment: Windows 11, Rust 1.99.1, Bevy 0.20.0 vendored fork.

## Commands

| Command | Exit | Result |
| --- | --- | --- |
| `cargo test -p omega_game --no-default-features --locked --test run_contract` | 0 | 9 passed |
| `cargo clippy --workspace --no-default-features --all-targets --locked -- -D warnings` | 0 | clean |
| `cargo clippy -p omega_game --features client --all-targets --locked -- -D warnings` | 0 | clean |
| `powershell -NoProfile -File scripts/check.ps1` | 0 | all gates passed |

## Criterion to test

| Criterion | Tests |
| --- | --- |
| SP-011-AC1 shops occur between waves, final victory bypasses a shop (three-wave subset) | `three_wave_run_visits_two_shops_then_victory` |
| SP-011-AC2 / DA-13 death precedence, Defeat once | `da13_death_in_the_completion_tick_settles_defeat_once` |
| SP-011-AC3 wave entry restores HP, clears transients, retains build/currency/energy | `wave_entry_restores_the_player_and_clears_transients_but_keeps_the_build` |
| SP-011-AC4 abandon settles once (cancel is not calling it) | `abandon_settles_once_and_cancel_is_simply_not_calling_it` |
| DA-03 Start guard | `da03_start_needs_a_powered_weapon_and_ignores_unspent_points`, `start_outside_the_shop_is_rejected` |
| DA-11 timer completion | `da11_timer_completion_removes_survivors_collects_pickups_and_enters_shop` |
| DA-16 combat frozen outside Wave (headless part) | `shop_freezes_combat_ticks` |
| Locked character guard | `a_locked_character_cannot_start_a_run` |

## Limits

Boss waves stay open until GAME-P3-004. Boot/Menu/CharacterSelect, Pause/Settings overlays and save/unlock grants belong to Phase 3 tasks (GAME-P3-020, P3-011, P3-008/009). SP-011-AC5 (invalid content at Boot) is GAME-P3-020.
