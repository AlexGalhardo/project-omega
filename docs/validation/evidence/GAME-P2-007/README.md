# Evidence: GAME-P2-007

Status: DONE. Base commit: `d50987e`. Environment: Windows 11, Rust 1.99.1, Bevy 0.20.0 vendored fork.

## Commands

| Command | Exit | Result |
| --- | --- | --- |
| `cargo test -p omega_game --no-default-features --locked --test shop_contract` | 0 | 14 passed |
| `cargo clippy --workspace --no-default-features --all-targets --locked -- -D warnings` | 0 | clean |
| `powershell -NoProfile -File scripts/check.ps1` | 0 | all gates passed |

## Criterion to test

| Criterion | Tests |
| --- | --- |
| SP-009-AC1 prices 28/42/56, refresh cost `8 + 2(w-1)` | `ac1_prices_use_one_final_ceiling` |
| Two weapons plus two items from the unlock snapshot | `offers_are_two_weapons_and_two_items_from_unlocked_content`, `unlocked_content_joins_the_pool_only_via_the_run_snapshot` |
| SP-009-AC3 owned tier 3 / owned item never offered | `owned_weapon_appears_only_as_its_next_tier_and_tier3_is_excluded`, `owned_items_are_never_offered_again` |
| SP-009-AC3 exhaustion fallback and Empty slots | `exhaustion_fills_from_the_other_category_then_shows_empty` |
| SP-009-AC5 / DA-10 one refresh per visit, RNG kept on rejection | `da10_one_refresh_per_visit_and_rejections_keep_rng_and_scrap`, `refresh_without_funds_leaves_rng_and_offers_alone` |

## Limits

Unlock snapshot is supplied through `RunBuild.unlocked`; persistence of unlocks is GAME-P3-008/009.
