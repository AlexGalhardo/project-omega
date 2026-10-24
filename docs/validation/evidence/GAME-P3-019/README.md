# Evidence: GAME-P3-019

Status: DONE (Beam marksman and heavy bulwark). Base commit: `efb3eb0`. Environment: Windows 11, Rust 1.99.1, Bevy 0.20.0 vendored fork.

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
| Survey Eye beam follows the displaced origin (DA-17, V08 AC3) | `marksman_beam_follows_the_displaced_origin` |
| Beam damage 18 on the emission tick | `marksman_beam_hits_the_player_for_18_when_not_displaced` |
| Plate Hauler takes 0.25 of the weapon knockback | `plate_hauler_takes_a_quarter_of_the_weapon_knockback` |

## Limits

Behavior and timing come from the content catalogs (GDD 10.1) read at spawn; hostile damage uses the wave formula. Windup cue facts are emitted as `CueStarted`/`CueEnded`; drawing them with real warning sprites is the Beta art work (GAME-P4-010/011). Bosses reuse this state machine in GAME-P3-004/005.
