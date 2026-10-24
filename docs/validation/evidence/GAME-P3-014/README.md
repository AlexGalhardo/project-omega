# Evidence: GAME-P3-014

Status: IN_REVIEW (SP-014-AC4 needs a human review under muted audio and reduced effects; see the local questions file). Base commit: `6e7040c`. Environment: Windows 11, Rust 1.99.1, Bevy 0.20.0 vendored fork, NVIDIA RTX 4060.

## Commands

| Command | Exit | Result |
| --- | --- | --- |
| `cargo test -p omega_game --no-default-features --locked --test accessibility_contract` | 0 | 7 passed |
| `cargo test -p omega_game --no-default-features --locked --test ui_contract --test controller_contract` | 0 | still pass with the two-column layout model |
| `omega-game --play --skip-hints --fast-forward-ticks 3700 --resolution WxH --ui-scale N --screenshot ...` | 0 | `shop-1280x720-100.png`, `shop-1280x720-150.png`, `shop-1920x1080-125.png`, `shop-1920x1080-150.png` |
| `omega-game --play --screenshot hint.png` | 0 | first-run note over the paused wave |
| `powershell -NoProfile -File scripts/check.ps1` | 0 | all gates passed |

## Criterion to test

| Criterion | Evidence |
| --- | --- |
| Scales 100/125/150 only | `ui_scale_accepts_only_the_three_supported_values`; applied through Bevy `UiScale` from settings |
| SP-014-AC2 no required text or control clipped at 1280x720 and 1920x1080 | the four `shop-*.png` captures; the first capture at 150% clipped the buttons, which led to the two-column panel and hiding the HUD while a panel is open |
| Reduced effects drop decorations only; shake defaults off | `reduced_effects_map_to_the_decoration_free_level` |
| SP-014-AC5 hints pause combat, persist and reset | `hints_show_once_on_their_screens_and_never_after_acknowledgment`, `dismissing_a_hint_records_it_resumes_combat_and_owes_a_save`, `skip_all_and_reset_tutorials_through_actions`, `hint_texts_cover_the_gdd_topics`, `hint.png` |
| Settings survive a reload | `settings_and_acknowledgments_survive_a_reload` |
| SP-014-AC4 hostile cores and HP changes perceivable muted and reduced | human review pending |

## Limits

The remapping and settings screens are part of GAME-P3-012. The client loads and saves the profile and settings (`client::persistence`) when a profile directory exists; evidence flags `--ui-scale` and `--skip-hints` override the saved values for one run only.
