# Evidence: GAME-P2-004

Status: DONE. Base commit: `75e649a` (work committed on top of it). Environment: Windows 11, Rust 1.99.1, Bevy 0.20.0 vendored fork.

## Commands

| Command | Exit | Result |
| --- | --- | --- |
| `cargo test -p omega_game --no-default-features --locked --test combat_contract` | 0 | 16 passed |
| `cargo clippy --workspace --no-default-features --all-targets --locked -- -D warnings` | 0 | clean |
| `powershell -NoProfile -File scripts/check.ps1` | 0 | all gates passed |

## Criterion to test

| Criterion | Tests |
| --- | --- |
| DA-04 attack starts 15 ticks apart | `da04_attack_starts_are_15_ticks_apart` |
| No burst after a no-target period | `no_burst_after_a_period_without_targets` |
| Target tie-break by lower `SimId`, range, manual aim | `target_tie_breaks_on_the_lower_sim_id`, `out_of_range_enemies_are_not_targeted_but_manual_aim_fires_anyway` |
| Projectile sweep, earliest impact | `bolt_projectile_hits_the_target_once_and_ends` |
| Pierce `1 + pierce` distinct targets | `piercing_projectile_hits_one_plus_pierce_distinct_targets` |
| SP-007-AC1 one hit per attack ID | `spread_volley_damages_a_target_once_per_attack_id` |
| SP-007-AC2 explosion, no direct hit | `explosion_hits_every_target_in_radius_once_without_a_direct_hit` |
| Capsule windup timing | `saw_capsule_hits_on_the_windup_completion_tick` |
| SP-007-AC4 dead attacker cancels pending | `a_dead_attacker_cancels_pending_attacks` |

## Limits

Collide produces `HitCandidates`; damage, crit draws, knockback, deaths, scrap and incoming damage are GAME-P2-005 (the "V07 AC3-5" asserts belong there). Hit candidates for different attacks in one tick are all computed from tick-start positions; knockback (applied in Resolve by P2-005) therefore affects later ticks, a slight simplification of GDD 6.2 that matters only when two attacks hit the same target in the same tick. Presentation variants for emission, projectile start/end and explosion were added to `PresentationKind`; hit/death variants are defined for P2-005.
