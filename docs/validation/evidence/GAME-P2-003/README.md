# Evidence: GAME-P2-003

Status: DONE. Base commit: `b6d8691` (work committed on top of it). Environment: Windows 11, Rust 1.99.1, Bevy 0.20.0 vendored fork.

## Commands

| Command | Exit | Result |
| --- | --- | --- |
| `cargo test -p omega_game --no-default-features --locked --test combat_contract` | 0 | 6 passed |
| `powershell -NoProfile -File scripts/check.ps1` | 0 | all gates passed |

## Criterion to test

| Criterion | Tests |
| --- | --- |
| DA-01 initial Patch state | `da01_initial_patch_state` (HP 120, tier-1 Bolt in slot 1 at three points, scrap 0) |
| DA-07 diagonal speed | `da07_diagonal_speed_equals_single_axis`, `move_speed_stat_scales_distance` (+20% is 3.6 LU/tick) |
| Arena clamp | `movement_clamps_to_the_arena_inset_by_the_radius` |
| Interpolation support | `previous_pose_follows_the_last_tick` |
| Spawn guard | `unknown_character_is_rejected` |

## Limits

`SimId` and `AttackId` became Bevy components. Movement reads `ArenaRes`/`ContentRes` resources inserted by the run setup (GAME-P2-010 owns the run lifecycle). Enemy bodies are added by GAME-P2-006.
