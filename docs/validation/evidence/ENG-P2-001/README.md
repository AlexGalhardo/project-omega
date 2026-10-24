# Evidence: ENG-P2-001

Status: IN_REVIEW (code complete; native alignment and skin-swap review is the owner's, see Q-001 in the local questions file). Base commit: `a697cbd`. Environment: Windows 11, Rust 1.99.1, Bevy 0.20.0 vendored fork, NVIDIA RTX 4060.

## Commands

| Command | Exit | Result |
| --- | --- | --- |
| `cargo test -p omega_game --features client --locked --test presentation_contract` | 0 | 3 passed |
| `cargo test -p omega_game --features client --locked --lib` | 0 | interpolation unit test passed |
| `cargo run -p omega_game --features client --locked --bin omega-game -- --play --screenshot docs/validation/evidence/ENG-P2-001/play.png` | 0 | live player drawn at the arena center (`play.png`) |
| `powershell -NoProfile -File scripts/check.ps1` | 0 | all gates passed |

## Criterion to test

| Criterion | Evidence |
| --- | --- |
| Views follow `SimId` and vanish with it | `views_follow_sim_entities_and_disappear_with_them` |
| Projectile core = exact collision diameter (SP-012-AC4 shape match) | `projectile_core_uses_the_exact_collision_diameter` |
| Queue drained once per rendered frame | `queue_is_drained_once_per_frame` |
| Interpolation for rendering only | `interpolation_is_clamped_and_linear` |
| Catalog swap keeps the trace (DA-20) | existing `skin_swap_contract` (headless); native swap pending Q-001 |

## Limits

`--play` is a development sandbox using the full fixture catalog and a fixed seed until GAME-P2-009/010. Hostile projectile visuals and effects arrive with later tasks. `play.png` shows only the player because the screenshot is taken before the first batch spawns.
