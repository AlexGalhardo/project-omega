# Evidence: ENG-P3-001

Status: DONE. Base commit: `cef312a`. Environment: Windows 11, Rust 1.99.1, Bevy 0.20.0 vendored fork.

## Commands

| Command | Exit | Result |
| --- | --- | --- |
| `cargo test -p omega_2d --no-default-features --locked --test cue_contract` | 0 | 6 passed |
| `cargo clippy -p omega_2d --all-targets --locked -- -D warnings` | 0 | clean |
| `powershell -NoProfile -File scripts/check.ps1` | 0 | all gates passed |

## Criterion to test

| Criterion | Tests |
| --- | --- |
| Locked ground cue keeps its center when the attacker is displaced (DA-17, AA-03) | `locked_ground_cue_stays_put_when_the_attacker_is_displaced` |
| Following cue tracks the displaced origin with the locked direction (AA-04) | `beam_cue_follows_the_displaced_origin_with_the_locked_direction` |
| Progress is elapsed over total ticks | `progress_equals_elapsed_over_total_and_never_overruns`, `zero_length_cue_is_complete` |
| SP-012-AC4 cancellation is immediate and per cue | `cancellation_and_emission_remove_the_cue_at_once_and_only_that_cue` |
| Reduced effects keep shape, anchor and progress | `reduced_effects_drop_decorations_but_keep_shape_anchor_and_progress` |

## Limits

`omega_2d::visuals::cues` is the engine-side model (headless). Drawing real warning sprites per enemy profile and the presentation facts that start and cancel cues arrive with GAME-P3-016 to GAME-P3-019 and the boss tasks; no cue can delay an attack because the model only reads facts.
