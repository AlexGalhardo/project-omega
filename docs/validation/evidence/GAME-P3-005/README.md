# Evidence: GAME-P3-005

Status: DONE (The Foreman). Base commit: `4d27b0a`. Environment: Windows 11, Rust 1.99.1, Bevy 0.20.0 vendored fork.

## Commands

| Command | Exit | Result |
| --- | --- | --- |
| `cargo test -p omega_game --no-default-features --locked --test boss_contract` | 0 | 8 passed |
| `powershell -NoProfile -File scripts/check.ps1` | 0 | all gates passed |

## Criterion to test

| Criterion | Tests |
| --- | --- |
| 90 recovery ticks, keep-distance 260 movement, sorting volley of five at 60-tick windup, then 12-way discharge after 60 recovery ticks | `foreman_holds_260_and_alternates_volley_then_discharge` |
| Shared boss warning, completion block and knockback rules | the Press tests in `boss_contract` use the same code path |

## Limits

Projectile angles and damage come from the content record; the exact 12-lane world-frame angles are covered for common enemies by `sentry_stays_put_and_fires_an_eight_way_world_frame_volley` (same emitter). The Foreman fight at wave 10 gates the `unlock_shutdown` challenge (GAME-P3-008) and Victory.
