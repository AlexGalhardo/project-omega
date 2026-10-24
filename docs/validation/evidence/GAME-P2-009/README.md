# Evidence: GAME-P2-009

Status: DONE. Base commit: `517d2bf`. Environment: Windows 11, Rust 1.99.1, Bevy 0.20.0 vendored fork.

## Commands

| Command | Exit | Result |
| --- | --- | --- |
| `cargo run -q -p omega_game --no-default-features --bin omega-content -- validate assets/content --profile prototype` | 0 | 1 character, 2 weapons, 8 items, 2 enemies, 3 waves |
| `cargo run -q -p omega_game --no-default-features --bin omega-content -- validate assets/content --profile full` | 1 | 7 content errors (count rules) |
| `cargo test -p omega_game --no-default-features --locked --test prototype_content_contract` | 0 | 3 passed |
| `powershell -NoProfile -File scripts/check.ps1` | 0 | all gates passed |

## Criterion to test

| Criterion | Tests |
| --- | --- |
| GDD-017 prototype scope | `shipped_subset_matches_the_gdd_prototype_scope` |
| Fails full-count mode | `shipped_subset_is_rejected_by_the_full_profile` |
| Same revision as the test fixture | `shipped_subset_equals_the_test_fixture` |

## Limits

`assets/content/` is a copy of the prototype fixture; later phases replace it with SP-018 production data. The release workflow packages `assets/content`; `scripts/check.ps1` validates it.
