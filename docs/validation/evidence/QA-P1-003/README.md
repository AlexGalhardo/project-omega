# Evidence: QA-P1-003

Status: IMPLEMENTED, awaiting integration (CI and `scripts/check.ps1` wiring). Base commit: `d50987e`. Environment: Windows 11, Rust 1.99.1, Bevy 0.20.0 vendored fork, NVIDIA GeForce RTX 4060 for the windowed run.

## Commands

| Command | Exit | Result |
| --- | --- | --- |
| `cargo test -p omega_game --no-default-features --locked --test e2e_contract` | 0 | 9 passed |
| `cargo test -p omega_game --features client --locked --test e2e_contract` | 0 | 10 passed (adds the binary exit-code test) |
| `omega-game --headless --e2e-script tests/e2e/foundation.json --out-dir docs/validation/evidence/QA-P1-003` | 0 | all checkpoints passed |
| `omega-game --e2e-script tests/e2e/foundation.json --out-dir artifacts/e2e-win` (windowed) | 0 | all checkpoints passed, screenshot written |
| `omega-game --headless --qa-bot chaotic --seed 5a...5a --frames 3600 --out-dir docs/validation/evidence/QA-P1-003` | 0 | no findings, no panics |
| `cargo clippy --workspace --no-default-features --all-targets --locked -- -D warnings` | 0 | clean |
| `cargo clippy -p omega_game --features client --all-targets --locked -- -D warnings` | 0 | clean |

Artifacts: `foundation-headless.json`, `foundation-windowed.json`, `foundation-windowed-final.png`, `qa-bot-chaotic-5a5a5a5a.json`.

## Criterion to test

| Criterion | Tests |
| --- | --- |
| Script launches, checks camera and letterbox checkpoints, exits 0 | `foundation_script_passes_and_exports_the_camera_checkpoint` |
| Broken checkpoint exits 1, invalid or missing script exits 2 | `a_broken_checkpoint_exits_one_and_a_bad_script_exits_two`, `the_binary_exits_zero_one_and_two_headless` |
| No ticks while unfocused | `no_ticks_are_simulated_while_unfocused` |
| A wait that cannot finish fails instead of hanging | `waiting_while_paused_fails_instead_of_hanging` |
| Bot deterministic per seed | `bot_is_deterministic_per_seed` |
| Bot report for 3 profiles x 5 seeds, zero panics | `bot_reports_for_three_profiles_and_five_seeds_have_no_panics_or_findings` |
| Pause, focus and resize abuse, mistakes, hesitation | `chaotic_bot_abuses_pause_focus_and_resize` |
| Menu exploration against a stub | `bot_explores_a_stub_menu_without_leaving_its_bounds` |
| Aim error stays inside the arena | `manual_aim_points_stay_inside_the_arena` |

## Limits

- Menus do not exist yet; the bot explores a `MenuView` stub, and runs never present one.
- Windowed checkpoints use ranges; tick-exact values are only reproducible headless because render frames and fixed ticks are decoupled. The windowed snapshot reports the requested window size until the window confirms it.
- Windowed runs do not block real keyboard input; edges a person types in the same frame join the buffer.
- Headless frames are one per tick; screenshots are windowed only (`screenshot: null` headless).
- Bot findings cover the audited invariants only (tick monotonic, none while paused, HP bound, player in arena, camera accepts every size and keeps content inside the screen).
- `omega-game` requires the `client` feature, so CI builds it with the client dependencies but needs no GPU for `--headless`.
