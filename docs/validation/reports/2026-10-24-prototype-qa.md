# Prototype phase-end QA report

Task: QA-P2-002 (ROADMAP rule 1.11). Status: DRAFT, harness layers filled in; the balance simulation and the integrator's gate results are still open.

## Identification

| Field | Value |
| --- | --- |
| Version | `0.3.0` (workspace manifest) |
| Base commit | `cef312a`; the QA harness changes were uncommitted when this skeleton was written, the integrator records the release commit here |
| Environment | Windows 11 Pro 10.0.26300, Ryzen 7 5700X3D, RTX 4060, 32 GiB; Rust 1.99.1, Bevy 0.20.0 vendored fork |
| Date | 2026-10-24 |
| Executor | Claude (QA harness session) |

## Results per layer

| Layer | Command | Result |
| --- | --- | --- |
| Unit and integration (workspace) | `cargo test --workspace --no-default-features --locked` | 150 passed, 0 failed |
| Unit and integration (`omega_game`) | `cargo test -p omega_game --no-default-features --locked` | 136 passed, 0 failed |
| Client contracts incl. e2e | `cargo test -p omega_game --features client --locked --test e2e_contract --test e2e_loop_contract` | 10 + 19 passed |
| Lint | `cargo clippy` core and client, `-D warnings` | clean |
| e2e scripts (headless, release) | 11 scripts in `tests/e2e/` | 11/11 passed (`foundation` plus 10 prototype scripts) |
| e2e script (windowed, RTX 4060) | `tests/e2e/prototype-loop.json` | passed, 4 screenshots |
| QA bot (headless, release) | 3 profiles x 20 seeds, `--frames 60000` | 60/60 reached the Summary, 0 findings, 0 panics |
| QA bot extended | 3 profiles x 150 seeds (the 20 above are the first 20) | 450/450 clean after the fixes below |
| QA bot (windowed) | chaotic, seed `9e3779b9...` | completed, Victory, 0 findings |
| Integrator gates | `scripts/check.ps1`, CI | open (integrator) |

### e2e scripts

`foundation`, `prototype-loop`, `shop-double-purchase`, `shop-refresh-spam`, `shop-unaffordable`, `shop-zero-energy-start`, `shop-energy-allocation`, `death-at-wave-completion`, `death-in-shop`, `abandon-cancel`, `abandon-confirm`. The scripts use the fixed default seed `5a` x 32; the reports are under `docs/validation/evidence/QA-P2-002/headless/`.

### QA bot matrix

Seeds: `seed(i)` is the 256-bit value `a b c d` where `a = 0x9e3779b97f4a7c15 * i` and `b`, `c`, `d` each apply `x * 6364136223846793005 + 1442695040888963407` to the previous word (64-bit wrapping), for `i = 1..20`; the exact seeds are listed in `docs/validation/evidence/QA-P2-002/bot-matrix-20x3.txt`.

| Profile | Runs | Completed | Findings | Victory | Defeat | Typed requests | Of which mistaken | Wall time per run |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| cautious | 20 | 20 | 0 | 20 | 0 | 278 | 42 | about 1.5 s (max 1.7 s) |
| aggressive | 20 | 20 | 0 | 20 | 0 | 198 | 45 | about 1.5 s (max 1.7 s) |
| chaotic | 20 | 20 | 0 | 19 | 1 | 905 | 683 | about 1.5 s (max 1.6 s) |

Each run plays about 10 800 ticks (three waves of 3 600 ticks) plus two shop visits. The chaotic profile also paused, lost focus and resized hundreds of times, and opened and cancelled the abandon dialog 42 times.

## Findings

| ID | Priority | Finding | Reproducing test | Status |
| --- | --- | --- | --- | --- |
| QA-P2-002-F1 | medium | The shop's energy `+` button was enabled when the click could only be rejected (reactor full or slot at 3) | `energy_add_button_is_disabled_when_the_reactor_cannot_take_a_point` | fixed in `ui/mod.rs` |
| QA-P2-002-F2 | high | A Confirm pressed during combat stayed queued and started the next wave the moment the shop opened, skipping the shop | `confirm_pressed_during_combat_does_not_start_the_next_wave_from_the_shop` | fixed with `ui::take_screen_edges` (client and harness) |
| QA-P2-002-F3 | low (harness) | Windowed bot runs reported `tick-jump` because a render frame can carry several fixed ticks | `windowed_catch_up_ticks_are_not_a_tick_jump_but_a_regression_still_is` | fixed in the harness (`Auditor::tick_slack`) |

No open high-priority finding from the harness layers.

## Checklist walk

`docs/validation/qa-checklist.md` Prototype items are covered by the scripts and bots above except the screenshot item (owner). Mapping: double purchase, refresh spam, unaffordable buys, zero-energy start, energy limits, death at wave completion, death in the shop, abandon cancel and confirm each have a script; stray requests on result screens and a Confirm during combat are covered by the chaotic bot's result-screen pokes and by F2's test; the bot matrix covers the full loop.

## Balance simulation

Source: [2026-10-24-prototype-balance.md](2026-10-24-prototype-balance.md) (GAME-P2-013), content revision after CR-004, 6 policies x 200 seeds, release build.

| Check | Band | Result |
| --- | --- | --- |
| `idle` dies in waves 1-2 | all runs | pass: 100 % of runs die in wave 2 |
| `human_like` clears wave 1 | at least 90 % | pass: 93.5 % |
| `human_like` clears wave 3 | 50-80 % | pass: 59.5 % |
| `kiter` win rate | at most 75 % | pass: 37.5 % |
| Survival curve drop | at most 25 points | **missed** (one-weapon `kiter` cliff), proposal: re-evaluate in QA-P3-003 |
| Item or weapon win-rate delta | at most 15 points | **missed** for Chain Saw on a 2000-seed `random` run (+20.7), proposal: saw damage 20 to 16 with a wave 3 recalibration, deferred to QA-P3-003 |
| Weapon pick rate | at least 3 % | pass |
| Energy spread and concentration both win | both present | pass |

The original prototype content was trivial (every bot won 200 of 200 runs, `idle` never took damage), so CR-004 raised enemy pressure; the before and after matrices are in `evidence/GAME-P2-013/`. The QA bots (cautious, aggressive, chaotic) are scripted human-like players, not a balance measure: their win rates under CR-004 are listed in the bot section above.

## Open items

- Owner checks (cannot be judged by a bot; to be listed in `questions-to-dev.md`): shop readability and the HUD text overlapping the top of the shop panel (see `evidence/QA-P2-002/windowed/energy-allocated.png`), whether disabled buttons state their reason clearly enough, result screen wording, whether the prototype is too easy (the bots win 99% of runs; for the balance report), pacing of the shop between waves.
- `replace_slot` and the replace dialog are unreachable with two prototype weapons; they only matter once Alpha content gives a fifth weapon.
