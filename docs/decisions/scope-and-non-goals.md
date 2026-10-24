# Scope and non-goals

Stage 2 approved on 2026-10-23; GDD, Art Bible and Stage 4 planning artifacts approved by the owner on 2026-10-24. Decisions and evidence are in the [decision register](decision-register.md). The [GDD](../gdd.md) and [Art Bible](../art-bible.md) own detailed rules/production scope; P values remain untested balance/production baselines. Explicit release deferrals remain uncommitted.

## Primary objectives (DEC-017)

Both are didactic learning objectives set by the owner on 2026-10-24:

1. Study the owner's purchased Steam copy of Brotato with REA and progressively re-create the game as an independent implementation ([DEC-019](decision-register.md#dec-019--scoped-rea-inspection-of-the-owners-brotato-copy), [IP policy](../legal/ip-policy.md)).
2. Develop the project's own 2D-focused engine as a direct fork of Bevy, vendored in `engine/bevy/` ([DEC-018](decision-register.md#dec-018--engine-from-a-vendored-bevy-fork)).

## Required for 1.0

- An original, stable arena-survival roguelite and a maintainable reusable 2D layer over Bevy, as required by the owner brief.
- The vendored Bevy fork plus the project's `omega_2d` layer under [DEC-018](decision-register.md#dec-018--engine-from-a-vendored-bevy-fork). Game rules belong to game systems, not engine code.
- Windows release support under [DEC-002](decision-register.md#dec-002--version-10-platform-scope), including native validation and packaging. Minimum OS version and hardware remain undecided.
- Original code/content and compliance with the [IP policy](../legal/ip-policy.md).
- The approved compact content envelope below, under [DEC-005](decision-register.md#dec-005--compact-version-10-content-envelope).
- The thematic identity approved in [DEC-006](decision-register.md#dec-006--original-thematic-identity), combat identity in [DEC-007](decision-register.md#dec-007--combat-agency-and-energy-distribution), run/shop loop in [DEC-008](decision-register.md#dec-008--run-shop-and-weapon-progression), and character/progression direction in [DEC-009](decision-register.md#dec-009--characters-unlocks-and-visualaudio-direction).
- Controller support and the accessibility baseline in [GDD-015](../gdd.md#15-hud-menus-onboarding-and-accessibility--gdd-015). DEC-010 made them conditional on GDD approval; the GDD was approved on 2026-10-24, so they are 1.0 requirements (planned in GAME-P3-013/014 and QA-P4-001).
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

Weapon tiers and visual variations do not count as additional base weapons. Individual content definitions and the proposal to omit separate elite variants are in the GDD for Stage 3 review. The initial 1.0 uses one difficulty level under DEC-010; this is separate from the one game mode count. These are owner-approved original design counts, not a measured feasibility claim. Prototype scope is proposed in the GDD. Reassessment after the prototype requires an explicit decision rather than an automatic reduction or increase.

Approved core loop: ten waves, bosses in waves five and ten, shop between waves, enemy-drop scrap, randomized shop offers with one paid refresh per visit, and four weapon slots. Six energy points are allocated between slots each inter-wave interval, capped at three per weapon; each allocated point raises firing rate. Automatic aim/fire is the default with optional manual aim. Detailed formulas, content identities and balance values belong in the GDD.

## Desired

Reuse in future games is an owner objective, but additional engine features need a concrete game requirement or justified reuse case. No speculative framework feature is approved merely by placing it in this category.

## Post-1.0

No specific feature has yet been assigned to post-1.0 by the owner. Platform expansion, paid asset services and extra content are not promised. Final title, release model, storefront, visibility/license, budget ceiling, hardware floor and release date are deferred in [DEC-010](decision-register.md#dec-010--remaining-planning-defaults-and-deferred-choices).

## Non-goals

- Mandatory Linux, macOS, web or mobile releases in version 1.0 under DEC-002. Later ports require a separate scope decision.
- Maximizing engine size or reproducing Brotato's proprietary content/code.
- Implementing the project during planning. Stages 4 and 5 require approval; earlier feasibility experiments need explicit authorization.
