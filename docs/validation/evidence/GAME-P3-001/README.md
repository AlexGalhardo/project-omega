# Evidence: GAME-P3-001

Status: DONE. Base commit: `cef312a`. Environment: Windows 11, Rust 1.99.1, Bevy 0.20.0 vendored fork.

## Commands

| Command | Exit | Result |
| --- | --- | --- |
| `cargo test -p omega_game --no-default-features --locked --test payload_contract` | 0 | 4 passed |
| `cargo test -p omega_game --no-default-features --locked --test combat_contract` | 0 | 21 passed (spread, pierce, explosion, capsule) |
| `powershell -NoProfile -File scripts/check.ps1` | 0 | all gates passed |

## Criterion to test

| Criterion | Tests |
| --- | --- |
| All twelve authored payloads attack and damage a target once per attack ID | `every_authored_weapon_hits_a_target_inside_its_reach_once_per_attack` |
| Spread dedup, piercing, explosions | `spread_volley_damages_a_target_once_per_attack_id`, `piercing_projectile_hits_one_plus_pierce_distinct_targets`, `explosion_hits_every_target_in_radius_once_without_a_direct_hit` |
| Offset circle centered ahead of the aim, misses behind | `offset_circle_is_centered_ahead_of_the_aim_and_misses_behind` |
| Centered circle ignores manual direction, respects its radius | `centered_pulse_hits_in_every_direction_regardless_of_manual_aim`, `pulse_beyond_its_radius_does_not_hit` |

## Limits

The payload code landed with GAME-P2-004/P2-005; this task adds the fixture coverage that was still missing for every authored weapon. Enemy-owned hostile payloads are GAME-P3-016 to GAME-P3-019.
