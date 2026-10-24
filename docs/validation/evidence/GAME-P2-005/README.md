# Evidence: GAME-P2-005

Status: DONE. Base commit: `13dd97a`. Environment: Windows 11, Rust 1.99.1, Bevy 0.20.0 vendored fork.

## Commands

| Command | Exit | Result |
| --- | --- | --- |
| `cargo test -p omega_game --no-default-features --locked --test combat_contract` | 0 | 21 passed |
| `cargo clippy --workspace --no-default-features --all-targets --locked -- -D warnings` | 0 | clean |
| `cargo clippy -p omega_game --features client --all-targets --locked -- -D warnings` | 0 | clean |
| `powershell -NoProfile -File scripts/check.ps1` | 0 | all gates passed |

## Criterion to test

| Criterion | Tests |
| --- | --- |
| DA-05 forced crit formula, one hit per attack ID | `da05_forced_crit_damage_matches_the_formula_once_per_attack` |
| SP-007-AC3 / DA-06 accepted hits at ticks 1, 37, 73 | `da06_contact_damage_is_accepted_every_36_ticks` |
| SP-007-AC5 / DA-13 simultaneous lethal, one reward | `da13_simultaneous_lethal_hits_kill_both_and_pay_once` |
| Whole-value pickup, dead player collects nothing | `pickup_in_radius_is_collected_whole_and_a_dead_player_collects_nothing` |
| Regeneration, 120 ticks per HP at 0.5 HP/s | `regeneration_heals_one_hp_after_120_ticks_at_half_hp_per_second` |

## Limits

Contact candidates are fixed before any death in the tick, so mutual kills are possible. Knockback applies after each hit in attack/target order and affects later ticks' collisions. Victory/defeat settlement is the lifecycle task (GAME-P2-010); here a zero-HP player is the observable fact.
