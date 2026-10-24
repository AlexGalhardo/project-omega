# Evidence: QA-P2-002 (harness part)

Status: DONE (integrated into `scripts/check.ps1` and CI; owner-only checks listed in the local questions file). Base commit: `cef312a` (changes uncommitted at the time of writing). Environment: Windows 11, Rust 1.99.1, Bevy 0.20.0 vendored fork, NVIDIA GeForce RTX 4060 for the windowed runs.

## Commands

| Command | Exit | Result |
| --- | --- | --- |
| `cargo fmt --all -- --check` | 0 | clean |
| `cargo clippy --workspace --no-default-features --all-targets --locked -- -D warnings` | 0 | clean |
| `cargo clippy -p omega_game --features client --all-targets --locked -- -D warnings` | 0 | clean |
| `cargo test -p omega_game --features client --locked --test e2e_contract --test e2e_loop_contract` | 0 | 10 + 19 passed |
| `cargo test -p omega_game --no-default-features --locked` | 0 | 136 passed |
| `cargo test --workspace --no-default-features --locked` | 0 | 150 passed |
| `omega-game --headless --e2e-script tests/e2e/<each>.json` (11 scripts, release build) | 0 | all checkpoints passed, reports in `headless/` |
| `omega-game --headless --qa-bot <profile> --seed <hex> --frames 60000` x 3 profiles x 20 seeds (release build) | 0 x 60 | 60/60 completed, no finding, no panic (`bot-matrix-20x3.txt`) |
| same, 150 seeds per profile (450 runs) while hunting bugs | 0 x 450 | no finding after the fixes below (`bot-matrix-extended-150x3-summary.txt`) |
| `omega-game --e2e-script tests/e2e/prototype-loop.json` (windowed) | 0 | passed, 4 screenshots in `windowed/` |
| `omega-game --qa-bot chaotic --seed 9e3779b9...` (windowed) | 0 | completed, victory, no finding (`windowed/qa-bot-chaotic-9e3779b9.json`) |

## Criterion to test

| Criterion | Tests |
| --- | --- |
| Full prototype loop by script: move, aim, fight, wave 1 to the shop, buy, energy, waves 2 and 3, Victory or Defeat, Summary | `the_full_prototype_loop_script_plays_three_waves_to_the_summary` (`tests/e2e/prototype-loop.json`) |
| Double purchase rejected, nothing changes | `double_purchase_is_rejected_and_changes_nothing` |
| Refresh spam: second refresh rejected, scrap unchanged | `refresh_spam_is_rejected_after_the_first_refresh` |
| Unaffordable, sold and out-of-range buys rejected with a reason | `unaffordable_and_invalid_purchases_are_rejected_with_reasons` |
| Zero-energy start rejected with the reason, valid start later works | `zero_energy_start_is_rejected_with_the_reason` |
| Energy limits (6 total, 3 per slot, none on empty slots) | `energy_allocation_respects_the_reactor_limits` |
| Death in the completion tick settles Defeat once, no shop | `death_in_the_completion_tick_settles_defeat_and_opens_no_shop` |
| Death inside the shop does not end the run | `death_inside_the_shop_does_not_end_the_run` |
| Abandon cancel and confirm | `abandon_cancel_changes_nothing`, `abandon_confirm_settles_once` |
| Bots play the whole loop with shop decisions, seeded, no finding | `bots_play_the_prototype_loop_to_the_summary_without_findings`, `the_binary_plays_the_prototype_loop_headless_for_a_script_and_a_bot` |
| Auditor detects each documented invariant violation | `the_auditor_flags_every_documented_invariant_violation`, `a_wave_scrap_loss_and_a_state_change_by_a_rejected_command_are_flagged` |
| Script validation of the new actions | `script_errors_in_the_new_actions_are_rejected_at_parse_time` |
| Snapshot exposes run, shop and loadout facts | `the_prototype_host_exposes_run_shop_and_loadout_facts` |

## Harness design

- Snapshot: `run` (state, wave, outcome, waves completed, settlements), `shop` (visit, revision, refresh flag and cost, offers), `loadout` (slots, tier, points, occupied, energy used, unallocated, powered, items) and `ui` (modal, last message, request count). The legal-action layer (`ui::screen_view`) rides along unserialized for the bot.
- Script actions: `ui` with a typed action (`buy`, `refresh`, `energy_add`, `energy_remove`, `start`, `replace_slot`, `cancel_replace`, `ask_abandon`, `confirm_abandon`, `cancel_abandon`, `continue`), `wait_state` (bounded, fails when paused or out of ticks), `debug_headless` (`kill_player`, `expire_wave`, `set_scrap`; the windowed host fails the run on it), script `mode` `sandbox` (default, foundation) or `prototype`.
- Expectations: numbers, booleans, text, `{ "contains": ... }`, `null` for absent, array indices in paths, and `"@path"` for equality with the previous checkpoint (for example `"scrap": "@scrap"` means unchanged).
- Both hosts apply `Command::Ui` through `qa::apply_ui` (the same `ui::apply_action` widgets use) and route Confirm/Back through `ui::edge_action` and `ui::take_screen_edges`, the code the client's keyboard handler now shares.
- Bot policies: cautious (items first, even energy, rarely wrong), aggressive (weapons first, concentrated energy, one refresh when nothing is affordable), chaotic (random enabled buttons plus unaffordable and sold buys, out-of-range offers, refresh spam, zero-energy starts, abandon dialogs it always cancels, stray requests on result screens).
- Auditor invariants added: slots at most 4, energy at most 6 and 3 per slot, none on empty slots, duplicate weapon, scrap overflow or loss in combat, death unresolved in Wave, one settlement, result immutable, legal run transitions, wave monotonic, no shop after the final wave, no enemies in the shop, start only when powered, wave start at full HP, a rejected shop command changes nothing, stuck states, and a wave advancing without a shop or a shop left without a Start request. A request from an enabled button that changes nothing is a finding.

## Bugs found and fixed

| Bug | Found by | Regression test | Fix |
| --- | --- | --- | --- |
| The shop's `Slot N energy +` button was enabled when the reactor was full or the slot at 3 points, so a legal-looking click was rejected | chaotic bot, seed 1 (`enabled-action-had-no-effect`) | `energy_add_button_is_disabled_when_the_reactor_cannot_take_a_point` | `ui/mod.rs` `shop_screen` enables `+` only with free points and room in the slot |
| A Confirm pressed during combat stayed queued and started the next wave the instant the shop opened, skipping the shop | the harness routing real edges (`wave-advanced-without-shop`, cautious bot seed 1 with the fix removed) | `confirm_pressed_during_combat_does_not_start_the_next_wave_from_the_shop` | `ui::take_screen_edges` drops edges queued before a screen opened; used by the client and the harness |
| Windowed bot runs flagged `tick-jump` because a render frame can carry several fixed ticks | first windowed bot run | `windowed_catch_up_ticks_are_not_a_tick_jump_but_a_regression_still_is` | `Auditor::tick_slack` (15 ticks) set by the windowed host only |

## Limits

- Windowed runs assert ranges and use the wall clock; focus loss by the operating system pauses the game and a `wait_state` then fails by design (one windowed attempt failed this way after the window lost focus and passed on rerun).
- `replace_slot` is part of the script and bot vocabulary but unreachable with two weapons in the prototype content (a new weapon never needs a replacement); it is covered by the existing `ui_contract` tests.
- The bots win almost every run on the shipped content (59 of 60 runs ended in Victory, 444 of 450 in the extended batch); balance claims belong to the simulation report.
- Screenshots are for the owner's visual review: the HUD text overlaps the top of the shop panel in `windowed/energy-allocated.png`.

## Matrix on the merged content (after CR-004)

`bot-matrix-20x3-after-cr-004.txt` re-ran the same 60 seeds in a release build after GAME-P2-013 raised enemy pressure: 60 of 60 runs exit 0 with no finding and no panic. Outcomes: cautious 19 victories and 1 defeat, aggressive 4 victories and 16 defeats, chaotic 0 victories and 20 defeats (it spends its time on pause, focus and mistaken-input abuse). The earlier 99 percent bot win rate was an artifact of the trivial original content.
