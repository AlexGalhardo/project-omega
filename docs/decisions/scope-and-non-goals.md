# Scope and non-goals

Stage 2 in progress, 2026-10-23. Only owner-mandated constraints and approved decisions are binding. Unresolved content counts and product choices are tracked in [open questions](../research/open-questions.md); this document does not silently approve research defaults.

## Required for 1.0

- An original, stable arena-survival roguelite and a maintainable reusable 2D layer over Bevy, as required by the owner brief.
- Upstream Bevy with own plugins under [DEC-001](decision-register.md#dec-001--bevy-integration-strategy). Game rules belong to game systems, not reusable engine internals.
- Windows release support under [DEC-002](decision-register.md#dec-002--version-10-platform-scope), including native validation and packaging. Minimum OS version and hardware remain undecided.
- Original code/content and compliance with the [IP policy](../legal/ip-policy.md).
- Verified acceptance evidence before completion claims. Exact content counts and release criteria await the remaining decisions and later approved specifications.

## Desired

Reuse in future games is an owner objective, but additional engine features need a concrete game requirement or justified reuse case. No speculative framework feature is approved merely by placing it in this category.

## Post-1.0

No specific feature has yet been assigned to post-1.0 by the owner. Platform expansion, paid asset services and extra content remain undecided rather than implicitly promised.

## Non-goals

- An initial full or minimal Bevy fork under DEC-001. A future scoped patch requires the recorded escalation process.
- Mandatory Linux, macOS, web or mobile releases in version 1.0 under DEC-002. Later ports require a separate scope decision.
- Maximizing engine size or reproducing Brotato's proprietary content/code.
- Implementing the project during planning. Stages 4 and 5 require approval; earlier feasibility experiments need explicit authorization.
