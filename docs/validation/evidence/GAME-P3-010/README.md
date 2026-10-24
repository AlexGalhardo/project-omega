# Evidence: GAME-P3-010

Status: DONE. Base commit: `8081ef1`. Environment: Windows 11, Rust 1.99.1, Bevy 0.20.0 vendored fork, real NTFS temp directories (native filesystem behavior, no mocks except the fault injector).

## Commands

| Command | Exit | Result |
| --- | --- | --- |
| `cargo test -p omega_game --no-default-features --locked --test persistence_contract` | 0 | 13 passed |
| `powershell -NoProfile -File scripts/check.ps1` | 0 | all gates passed |

## Criterion to test

| Criterion | Tests |
| --- | --- |
| SP-010-AC3 a fault (error or half-written file) at every protocol operation leaves a valid current or previous generation, and a retry succeeds | `ac3_a_fault_at_every_protocol_boundary_keeps_a_readable_profile`, `a_failed_directory_creation_reports_io_and_changes_nothing` |
| Damaged newest slot falls back to the previous generation | `a_damaged_newest_slot_falls_back_to_the_previous_generation` |
| Corrupt bytes are never replaced silently; quarantine keeps the files | `ac3_unreadable_slots_are_preserved_and_never_replaced_silently` |
| SP-010-AC4 future schema or invalid hash preserves the files and requires explicit recovery | `ac4_future_schema_and_bad_hash_block_saving_and_keep_the_files` |
| SP-010-AC5 reset keeps settings | `ac5_resetting_the_profile_keeps_the_settings` |

## Limits

Windows-specific faults beyond injected I/O errors (antivirus sharing violations, disk full) are modeled by the same error path; a native run with an unwritable `LOCALAPPDATA` waits for the lifecycle wiring (GAME-P3-020). The Reset Profile confirmation dialog listing lost unlocks is GAME-P3-012; the data operation (`quarantine`) never deletes.
