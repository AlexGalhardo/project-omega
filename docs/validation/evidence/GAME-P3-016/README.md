# Evidence: GAME-P3-016

Status: DONE (Keep-distance gunner and scatter). Base commit: `efb3eb0`. Environment: Windows 11, Rust 1.99.1, Bevy 0.20.0 vendored fork.

## Commands

| Command | Exit | Result |
| --- | --- | --- |
| `cargo test -p omega_game --no-default-features --locked --test enemy_profile_contract` | 0 | 17 passed |
| `cargo test --workspace --no-default-features --locked` | 0 | all passed |
| `cargo clippy --workspace --no-default-features --all-targets --locked -- -D warnings` | 0 | clean |
| `powershell -NoProfile -File scripts/check.ps1` | 0 | all gates passed |

## Criterion to test

| Criterion | Tests |
| --- | --- |
| Keep-distance band 220 +/- 20, 55 LU/s | `gunner_approaches_retreats_and_holds_inside_the_220_band` |
| First windup after one full interval, windup freezes, aimed emission, interval from the last start | `gunner_first_windup_follows_one_full_interval_then_freezes_and_emits_aimed` |
| Projectile damage 8 once, consumed on impact | `gunner_projectile_hits_the_player_for_the_authored_damage_once` |
| Out-of-range stays ready and keeps moving | `out_of_range_attacker_stays_ready_and_keeps_moving` |
| Spread -20/0/+20 under one attack ID, damage 10 once | `scatter_fires_three_projectiles_around_the_locked_aim_under_one_attack_id` |
| Dead attacker cancels the windup | `dead_attacker_cancels_its_windup_without_emitting` |
| Consumed even when invulnerability rejects damage | `projectile_is_consumed_even_when_invulnerability_rejects_its_damage` |
| Wave damage scaling | `hostile_damage_scales_with_the_wave_formula` |

## Limits

Behavior and timing come from the content catalogs (GDD 10.1) read at spawn; hostile damage uses the wave formula. Windup cue facts are emitted as `CueStarted`/`CueEnded`; drawing them with real warning sprites is the Beta art work (GAME-P4-010/011). Bosses reuse this state machine in GAME-P3-004/005.
