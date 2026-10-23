# Scope and non-goals

Stage 2 decision collection complete, 2026-10-23; complete summary awaits owner approval. Approved choices and accepted defaults are in the [decision register](decision-register.md). Remaining explicit deferrals are not implicit commitments.

## Required for 1.0

- An original, stable arena-survival roguelite and a maintainable reusable 2D layer over Bevy, as required by the owner brief.
- Upstream Bevy with own plugins under [DEC-001](decision-register.md#dec-001--bevy-integration-strategy). Game rules belong to game systems, not reusable engine internals.
- Windows release support under [DEC-002](decision-register.md#dec-002--version-10-platform-scope), including native validation and packaging. Minimum OS version and hardware remain undecided.
- Original code/content and compliance with the [IP policy](../legal/ip-policy.md).
- The approved compact content envelope below, under [DEC-005](decision-register.md#dec-005--compact-version-10-content-envelope).
- The thematic identity approved in [DEC-006](decision-register.md#dec-006--original-thematic-identity), combat identity in [DEC-007](decision-register.md#dec-007--combat-agency-and-energy-distribution), run/shop loop in [DEC-008](decision-register.md#dec-008--run-shop-and-weapon-progression), and character/progression direction in [DEC-009](decision-register.md#dec-009--characters-unlocks-and-visualaudio-direction).
- Verified acceptance evidence before completion claims. Detailed release criteria await the remaining decisions and later approved specifications.

### Approved content envelope

| Content | Initial version 1.0 count |
| --- | ---: |
| Playable characters | 4 |
| Base weapons | 12 |
| Common enemy types | 10 |
| Bosses | 2 |
| Passive items | 40 |
| Arenas | 1 |
| Game modes | 1 |

Weapon tiers and visual variations do not count as additional base weapons. Elite variants and individual content definitions remain undecided. The initial 1.0 uses one difficulty level under DEC-010; this is separate from the one game mode count. These are owner-approved original design counts, not a measured feasibility claim. Prototype scope will be smaller and specified separately. Reassessment after the prototype requires an explicit decision rather than an automatic reduction or increase.

Approved core loop: ten waves, bosses in waves five and ten, shop between waves, enemy-drop scrap, randomized shop offers with one paid refresh per visit, and four weapon slots. Six energy points are allocated between slots each inter-wave interval, capped at three per weapon; each allocated point raises firing rate. Automatic aim/fire is the default with optional manual aim. Detailed formulas, content identities and balance values belong in the GDD.

## Desired

Reuse in future games is an owner objective, but additional engine features need a concrete game requirement or justified reuse case. No speculative framework feature is approved merely by placing it in this category.

## Post-1.0

No specific feature has yet been assigned to post-1.0 by the owner. Platform expansion, paid asset services and extra content are not promised. Final title, release model, storefront, visibility/license, budget ceiling, hardware floor and release date are deferred in [DEC-010](decision-register.md#dec-010--remaining-planning-defaults-and-deferred-choices).

## Non-goals

- An initial full or minimal Bevy fork under DEC-001. A future scoped patch requires the recorded escalation process.
- Mandatory Linux, macOS, web or mobile releases in version 1.0 under DEC-002. Later ports require a separate scope decision.
- Maximizing engine size or reproducing Brotato's proprietary content/code.
- Implementing the project during planning. Stages 4 and 5 require approval; earlier feasibility experiments need explicit authorization.
