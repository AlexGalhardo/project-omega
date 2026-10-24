# Evidence: GAME-P2-002

Status: DONE. Base commit: `29badf4` (work committed on top of it). Environment: Windows 11, Rust 1.99.1, Bevy 0.20.0 vendored fork.

## Commands

| Command | Exit | Result |
| --- | --- | --- |
| `cargo test -p omega_game --no-default-features --locked --test input_contract` | 0 | 6 passed |
| `cargo clippy -p omega_game -p omega_2d --features client --all-targets --locked -- -D warnings` | 0 | clean |
| `powershell -NoProfile -File scripts/check.ps1` | 0 | all gates passed |

## Criterion to test

| Criterion | Tests |
| --- | --- |
| SP-005-AC1 | `ac1_held_repeats_and_edge_is_consumed_once_across_three_ticks` |
| SP-005-AC2 | `ac2_zero_tick_frame_keeps_the_edge_for_the_next_tick` |
| SP-005-AC3 | `ac3_diagonal_has_unit_length_and_opposites_cancel`, `ac3_manual_aim_maps_cursor_and_ignores_letterbox` (uses `omega_2d::camera::ViewportMapping`) |
| SP-005-AC4 | `ac4_focus_loss_pauses_and_resume_discards_stale_edges`, `ac4_confirm_and_back_do_not_leak_through_a_modal` |

## Limits

The Bevy sampling systems (`InputClientPlugin`: keyboard, right mouse, cursor mapping through `CurrentViewport`, `WindowFocused`) compile and pass Clippy but are exercised only by the native run in GAME-P2-011. `AimOrigin` is set to the player center by GAME-P2-003. Controller input, remapping and toggle aim are Phase 3 (GAME-P3-012/013).
