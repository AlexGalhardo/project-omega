# Evidence: GAME-P3-017

Status: DONE (Stationary sentry and orbit skimmer). Base commit: `efb3eb0`. Environment: Windows 11, Rust 1.99.1, Bevy 0.20.0 vendored fork.

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
| Spark Hub eight-way world-frame volley, no movement, interval 180, windup 45 | `sentry_stays_put_and_fires_an_eight_way_world_frame_volley` |
| Coil Skater clockwise orbit at 180, interval 120, windup 24 | `skimmer_orbits_clockwise_at_180_and_shoots` |

## Limits

Behavior and timing come from the content catalogs (GDD 10.1) read at spawn; hostile damage uses the wave formula. Windup cue facts are emitted as `CueStarted`/`CueEnded`; drawing them with real warning sprites is the Beta art work (GAME-P4-010/011). Bosses reuse this state machine in GAME-P3-004/005.
