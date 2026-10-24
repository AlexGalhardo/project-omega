# Evidence: GAME-P2-001

Status: DONE. Base commit: `74e89ad` (work committed on top of it). Environment: Windows 11, Rust 1.99.1, Bevy 0.20.0 vendored fork.

## Commands

| Command | Exit | Result |
| --- | --- | --- |
| `cargo test -p omega_game --no-default-features --locked --test stats_contract` | 0 | 7 passed |
| `cargo clippy -p omega_game --no-default-features --all-targets --locked -- -D warnings` | 0 | clean |
| `powershell -NoProfile -File scripts/check.ps1` | 0 | all gates passed |

## Criterion to test

| Criterion | Tests |
| --- | --- |
| SP-006-AC1 | `ac1_fire_interval_worked_examples`, `ac1_hammer_animation_floor_and_rate_cap`, `ac1_damage_worked_examples`, `ac1_hp_limits_from_character_and_clamps` |
| SP-006-AC2 | `ac2_item_order_independence` (200 seeded permutations of the full item catalog) |
| SP-006-AC3 | `ac3_energy_allocation_invariants` (DA-02 limits, empty slots, cap 3, six-point invariant) |
| SP-006-AC4 | `ac4_regen_accumulator_and_death` (0.5 HP/s over 120 ticks heals exactly 1; dead never revives) |

## Limits

Pure evaluators only. ECS wiring (`TickSet`) belongs to GAME-P2-003 and later. Paths follow SP-006 (`combat/stats.rs`, `combat/energy.rs`), which win over the shorter `stats/` path in the ROADMAP microtask.
