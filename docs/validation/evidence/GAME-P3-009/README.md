# Evidence: GAME-P3-009

Status: DONE. Base commit: `8081ef1`. Environment: Windows 11, Rust 1.99.1, Bevy 0.20.0 vendored fork, real NTFS temp directories.

## Commands

| Command | Exit | Result |
| --- | --- | --- |
| `cargo test -p omega_game --no-default-features --locked --test persistence_contract` | 0 | 13 passed |
| `powershell -NoProfile -File scripts/check.ps1` | 0 | all gates passed |

## Criterion to test

| Criterion | Tests |
| --- | --- |
| Two generation slots, checked SHA-256, temp write then same-directory rename, round trip, missing profile is fresh | `first_save_load_round_trip_and_a_missing_profile_is_fresh` |
| Serial generations: stale requests rejected | `stale_generations_are_rejected` |
| Profile and settings are separate files; settings survive a profile reset (SP-010-AC5) | `ac5_resetting_the_profile_keeps_the_settings` |
| Saves live under `%LOCALAPPDATA%\ProjectOmega`, no install-directory fallback | `the_save_directory_is_under_localappdata_without_an_install_fallback` |

## Limits

`persist::SlotStore` is generic and injectable (`Storage`); wiring a store into the client lifecycle (load at Boot, save on `ProfileDirty`, Unsaved/Retry UI) is GAME-P3-012/020. Settings fields cover the GDD baseline; later tasks extend the schema with migrations.
