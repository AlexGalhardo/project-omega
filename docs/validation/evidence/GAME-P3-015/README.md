# Evidence: GAME-P3-015

Status: DONE. Base commit: `4e8f05d`. Environment: Windows 11, Rust 1.99.1, Bevy 0.20.0 vendored fork.

## Commands

| Command | Exit | Result |
| --- | --- | --- |
| `cargo test -p omega_game --no-default-features --locked --test ui_contract` | 0 | 10 passed |
| `cargo clippy --workspace --no-default-features --all-targets --locked -- -D warnings` | 0 | clean |
| `powershell -NoProfile -File scripts/check.ps1` | 0 | all gates passed |

## Criterion to test

| Criterion | Tests |
| --- | --- |
| Four offers with costs, reasons, refresh and Start visible together | `shop_screen_lists_offers_costs_refresh_power_and_start`, `disabled_actions_state_their_reason_in_text` |
| Weapon comparison: damage by tier, actual rate at the slot's energy, range | `upgrade_offer_shows_old_and_new_damage_and_the_slots_actual_rate` |
| Empty slot shows rates at 1, 2 and 3 points, not a fake powered state | `new_weapon_in_an_empty_slot_shows_rates_at_every_point_count` |
| Item modifiers with units | `item_offers_list_their_modifiers_with_units` |
| Replacement lists what is lost and gained per slot, cancel changes nothing (DA-09) | `replacement_prompt_names_what_is_lost_and_gained_per_slot`, `a_full_loadout_prompts_for_a_replacement_and_cancel_changes_nothing` |
| Energy controls and typed actions (DA-08/10 rules) | `actions_buy_refresh_allocate_and_start_through_typed_requests`, `shop_contract` |
| Both input modes reach every enabled control | keyboard shortcuts in `ui::client::keyboard_actions`; controller focus in `controller_contract` |

## Limits

Comparison text uses the player's actual stats at the time the shop opens. Layout at 1280x720 and 1920x1080 with UI scales (SP-014-AC2) and the owner's feel review belong to GAME-P3-014 and the native matrix (QA-P4-001).
