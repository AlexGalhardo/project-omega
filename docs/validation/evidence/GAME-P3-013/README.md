# Evidence: GAME-P3-013

Status: IN_REVIEW (headless model and client wiring complete; the native controller check is the owner's, see the local questions file; no controller was available to the session, recorded as a missing check per SP-005). Base commit: `b202b33`. Environment: Windows 11, Rust 1.99.1, Bevy 0.20.0 vendored fork.

## Commands

| Command | Exit | Result |
| --- | --- | --- |
| `cargo test -p omega_game --no-default-features --locked --test controller_contract` | 0 | 11 passed |
| `cargo test -p omega_game --no-default-features --locked --test input_contract` | 0 | existing keyboard and mouse cases still pass |
| `cargo check -p omega_game --features client --locked` | 0 | client wiring compiles (no new dependency: `bevy_gilrs` was already in the graph) |
| `powershell -NoProfile -File scripts/check.ps1` | 0 | all gates passed |

## Criterion to test

| Criterion | Tests |
| --- | --- |
| Move dead zone 0.20 rescaled and capped; analog movement; digital wins | `radial_dead_zone_rescales_and_caps`, `analog_movement_scales_with_the_stick_and_digital_wins_when_pressed` |
| Aim stick 0.25, normalized, centered returns to auto | `aim_stick_aims_normalized_and_centering_returns_to_auto` |
| Buttons to edges consumed once; Back pauses | `buttons_become_edges_once_and_back_pauses` |
| SP-005-AC4 loss pauses before the next tick and drops edges | `ac4_controller_loss_pauses_before_the_next_tick_and_drops_edges` (the client calls it on an active-controller disconnect event) |
| Source switches on deliberate actions only | `source_switches_only_on_deliberate_actions` |
| UI navigation repeat 0.35 s then 0.12 s | `navigation_repeats_after_035_then_every_012_seconds` |
| Remapping conflicts, Back stays reachable, restore defaults, settings round trip | `rebinding_detects_conflicts_and_keeps_back_reachable`, `bindings_round_trip_through_settings_and_report_unknown_actions` |
| Focus skips disabled buttons, activates only enabled, Back never buys | `focus_skips_disabled_buttons_wraps_and_activates_only_enabled`, `back_never_buys_or_starts` |

## Limits

The remapping screen itself is part of GAME-P3-012/014; this task delivers the binding model, its persistence format and the controller sampling. Toggle aim mode is GAME-P3-014. Native controller behavior (Windows XInput/DirectInput through gilrs) is unverified.
