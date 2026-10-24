# Evidence: GAME-P2-008

Status: DONE. Base commit: `d50987e`. Environment: Windows 11, Rust 1.99.1, Bevy 0.20.0 vendored fork.

## Commands

| Command | Exit | Result |
| --- | --- | --- |
| `cargo test -p omega_game --no-default-features --locked --test shop_contract` | 0 | 14 passed |
| `powershell -NoProfile -File scripts/check.ps1` | 0 | all gates passed |

## Criterion to test

| Criterion | Tests |
| --- | --- |
| SP-009-AC2 / DA-08 charge 42 once, 41 rejected unchanged | `da08_upgrade_charges_once_and_rejection_changes_nothing`, `stale_revision_is_rejected_without_effects` |
| SP-009-AC4 / DA-09 replacement cancel and confirm, slot energy kept | `da09_replacement_cancel_changes_nothing_and_confirm_keeps_slot_energy` |
| Lowest empty slot, zero energy | `new_weapon_fills_the_lowest_empty_slot_with_zero_energy` |
| DA-02 energy limits, empty slots reject points | `da02_energy_limits_and_empty_slots_reject_points` |
| Item modifiers applied, max HP raise does not heal | `buying_an_item_applies_its_modifiers_without_healing` |

## Limits

DA-03 (Start guard) belongs to the lifecycle task GAME-P2-010. The shop is a command API; the UI is GAME-P2-011.
