# Evidence: GAME-P3-018

Status: DONE (Charge rammer and ground-strike crusher). Base commit: `efb3eb0`. Environment: Windows 11, Rust 1.99.1, Bevy 0.20.0 vendored fork.

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
| Ram Cart wait 60, windup 45, swept charge 168 LU, charge damage 24, base contact restored, reset | `rammer_waits_winds_up_charges_with_charge_damage_and_resets` |
| Wall ends the charge, no extra area | `rammer_charge_ends_at_a_wall_without_extra_damage_area` |
| DA-17 locked ground strike stays on the locked center | `da17_crusher_strike_stays_on_the_locked_center_when_either_actor_moves` |
| Strike misses a player who left the circle | `crusher_strike_misses_a_player_who_left_the_locked_circle` |

## Limits

Behavior and timing come from the content catalogs (GDD 10.1) read at spawn; hostile damage uses the wave formula. Windup cue facts are emitted as `CueStarted`/`CueEnded`; drawing them with real warning sprites is the Beta art work (GAME-P4-010/011). Bosses reuse this state machine in GAME-P3-004/005.
