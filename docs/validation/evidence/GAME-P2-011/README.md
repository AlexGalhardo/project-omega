# Evidence: GAME-P2-011

Status: IN_REVIEW (code complete; the owner's native playthrough is pending, see the local questions file). Base commit: `23100a9`. Environment: Windows 11, Rust 1.99.1, Bevy 0.20.0 vendored fork, NVIDIA RTX 4060.

## Commands

| Command | Exit | Result |
| --- | --- | --- |
| `cargo test -p omega_game --no-default-features --locked --test ui_contract` | 0 | 6 passed |
| `cargo run -p omega_game --features client --locked --bin omega-game -- --play --screenshot docs/validation/evidence/GAME-P2-011/hud.png` | 0 | HUD with HP, wave, timer, scrap and four slots |
| `cargo run -p omega_game --features client --locked --bin omega-game -- --play --fast-forward-ticks 3700 --screenshot docs/validation/evidence/GAME-P2-011/shop.png` | 0 | Shop with four offers, costs, refresh, power panel and Start |
| `powershell -NoProfile -File scripts/check.ps1` | 0 | all gates passed |

## Criterion to test

| Criterion | Evidence |
| --- | --- |
| HUD values | `hud_shows_hp_wave_timer_scrap_and_slots`, `hud.png` |
| Shop shows offers, costs, refresh, energy and Start together | `shop_screen_lists_offers_costs_refresh_power_and_start`, `shop.png` |
| SP-014-AC3 disabled reasons in text | `disabled_actions_state_their_reason_in_text` |
| Typed actions complete the loop | `actions_buy_refresh_allocate_and_start_through_typed_requests` |
| Replacement prompt, cancel changes nothing | `a_full_loadout_prompts_for_a_replacement_and_cancel_changes_nothing` |
| Pause stats with units, abandon confirmation | `pause_screen_shows_stats_with_units_and_abandon_needs_confirmation` |

## Limits

Controller navigation, UI scale, onboarding hints and Unsaved/Retry are Phase 3 (GAME-P3-013 to P3-015). The default Bevy font is used until GAME-P4-004. `bevy/default_font` was enabled as a feature of the client (no new package). Keyboard focus order and the owner's feel review are not covered by headless tests.
