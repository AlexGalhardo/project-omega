# Evidence: GAME-P3-004

Status: DONE (The Press). Base commit: `4d27b0a`. Environment: Windows 11, Rust 1.99.1, Bevy 0.20.0 vendored fork.

## Commands

| Command | Exit | Result |
| --- | --- | --- |
| `cargo test -p omega_game --no-default-features --locked --test boss_contract` | 0 | 8 passed |
| `cargo test --workspace --no-default-features --locked` | 0 | all passed |
| `cargo clippy --workspace --no-default-features --all-targets --locked -- -D warnings` | 0 | clean |
| `powershell -NoProfile -File scripts/check.ps1` | 0 | all gates passed |

## Criterion to test

| Criterion | Tests |
| --- | --- |
| 120-tick warning at the fixed position, boss neither entity nor target while pending | `boss_is_absent_and_untargetable_until_its_120_tick_warning_ends` |
| DA-18 boss warning postpones without a cap and blocks completion | `da18_boss_warning_keeps_postponing_without_a_cap_and_blocks_completion` |
| Stamp: 60 recovery ticks, locked 90 LU circle, 72 windup, raw damage 32 | `press_stamp_follows_a_60_tick_recovery_and_locks_on_the_player` |
| Ram: alternation, 60 windup, charge damage 30 replaces contact 18, restored | `press_alternates_to_the_ram_and_charge_damage_replaces_contact` |
| DA-12 live boss at timer expiry keeps the wave open; death releases it | `a_live_boss_at_timer_expiry_keeps_the_wave_open`, `killing_the_boss_releases_wave_completion_only_after_the_timer` |
| Knockback multiplier zero | `boss_takes_no_knockback_from_player_attacks` |

## Limits

Boss stats and damage are authored finals (wave factor 1). Boss art and the boss HP bar are Beta/HUD work (GAME-P4-012, GAME-P3-015). DA-13 (player and boss lethal on one tick) relies on the death-before-completion rule covered in `run_contract`.
