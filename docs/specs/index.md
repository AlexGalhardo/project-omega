# Specification index

Stage 4, 2026-10-24. Owner authorized approval of all draft/proposed planning artifacts on this date. Specs are **APPROVED** as design contracts; SP-001 is **READY** only for the initial ENG-P1-001 packet in the approved [roadmap](../../ROADMAP.md). Nothing is implemented. Readiness depends on the assigned task and its verified dependencies.

## Lifecycle and readiness

`DRAFT -> APPROVED -> READY -> IN_PROGRESS -> IMPLEMENTED -> VERIFIED -> DONE`, with `BLOCKED / DEFERRED / SUPERSEDED` side states. DRAFT is an incomplete proposal; APPROVED records owner acceptance; READY requires no critical unknown, tested dependency contracts, exact task/file ownership and validation commands. IN_PROGRESS requires an assigned task; IMPLEMENTED means code exists, VERIFIED means acceptance evidence passed, DONE additionally requires integration/review and the Definition of Done. A failed required check prevents VERIFIED/DONE. Preserve stable IDs and supersession links.

A spec can cover several reviewable tasks. Tasks may consume a verified subset of a dependency contract only when the exact subset is named and independently tested; file existence or an entire spec's approval is not implementation completion. Shared interfaces/manifests/catalogs are serially owned. The roadmap assigns stable ENG/GAME/QA/REL task IDs. TODO tasks must receive narrowed file ownership before promotion; only ENG-P1-001 is initially READY.

## Catalog

| ID | Contract | Status | Implementation dependencies |
| --- | --- | --- | --- |
| SP-001 | [Workspace, toolchain and CI bootstrap](sp-001-workspace.md) | READY | None; ENG-P1-001 packet in ROADMAP.md |
| SP-002 | [Ordered simulation and headless harness](sp-002-simulation.md) | APPROVED | SP-001 |
| SP-003 | [Typed content schemas and validation](sp-003-content.md) | APPROVED | SP-001 |
| SP-004 | [Reusable shape and sweep queries](sp-004-geometry.md) | APPROVED | SP-001 |
| SP-005 | [Input actions and fixed-tick buffering](sp-005-input.md) | APPROVED | SP-002, SP-012 |
| SP-006 | [Stats, energy and numeric formulas](sp-006-stats.md) | APPROVED | SP-003 |
| SP-007 | [Movement, attacks, projectiles and damage](sp-007-combat.md) | APPROVED | SP-002, SP-003, SP-004, SP-006 |
| SP-008 | [Wave schedules, enemy AI and bosses](sp-008-encounters.md) | APPROVED | SP-007 |
| SP-009 | [Shop offers and atomic transactions](sp-009-shop.md) | APPROVED | SP-003, SP-006 |
| SP-010 | [Unlock progression and recoverable local saves](sp-010-progression-save.md) | APPROVED | SP-003 |
| SP-011 | [Run state machine and result settlement](sp-011-run-states.md) | APPROVED | SP-005, SP-008, SP-009, SP-010 |
| SP-012 | [Logical camera and visual presentation contracts](sp-012-camera-visuals.md) | APPROVED | SP-001 |
| SP-013 | [Offline SVG proof and reproducible asset metadata](sp-013-asset-pipeline.md) | APPROVED | SP-003, SP-012 |
| SP-014 | [HUD, menus, shop UI and accessibility](sp-014-ui-accessibility.md) | APPROVED | SP-005, SP-009, SP-010, SP-011, SP-012 |
| SP-015 | [State-driven audio and mix controls](sp-015-audio.md) | APPROVED | SP-011, SP-013 |
| SP-016 | [Baseline scenarios and performance evidence](sp-016-diagnostics.md) | APPROVED | SP-002, SP-007, SP-008, SP-012 |
| SP-017 | [Packaging and release qualification](sp-017-release-validation.md) | APPROVED | SP-001, SP-010, SP-014, SP-015, SP-016, SP-018 |
| SP-018 | [Full original content and asset coverage](sp-018-production-content.md) | APPROVED | SP-003, SP-013 |

## Requirement coverage

| Requirements | Canonical specs | Why |
| --- | --- | --- |
| GDD-001 | SP-001, SP-017, SP-018 | Products/platform/scope and release content. |
| GDD-002, GDD-003 | SP-002, SP-011 | Run lifecycle and guarded transitions. |
| GDD-004 | SP-004, SP-005, SP-007, SP-012 | Coordinates, movement, collision and input. |
| GDD-005 | SP-002, SP-004, SP-007 | Tick/attack semantics. |
| GDD-006, GDD-007 | SP-006, SP-007, SP-009 | Formulas, energy, cooldowns and transactions. |
| GDD-008 | SP-009 | Shop economy and refreshes. |
| GDD-009 | SP-003, SP-006, SP-018 | Player catalogs, schema and modifiers. |
| GDD-010, GDD-011 | SP-008, SP-018 | Enemy/boss behavior and wave composition. |
| GDD-012 | SP-003, SP-018 | Validation and complete content. |
| GDD-013, GDD-014 | SP-010, SP-011 | Grants, local saves and results. |
| GDD-015 | SP-005, SP-014, SP-015 | Controls, UI/accessibility and sound settings. |
| GDD-016 | SP-012, SP-013, SP-015 | Presentation boundary and skin replacement. |
| GDD-017 | SP-001, SP-016, SP-017, SP-018 | Staged delivery, measurement and qualification. |
| GDD-018 | All domain specs | DA-01–DA-20 acceptance cases are inherited. |
| GDD-019 | SP-001, SP-013, SP-016, SP-017 | Compatibility, pipeline, baseline and deferred release choices. |
| GDD-020 | This index and quality gates | Documentary traceability; roadmap task links remain Stage 5 work. |
| ART-001, ART-002, ART-003 | SP-014, SP-018 | Original visual direction and content. |
| ART-004 through ART-010 | SP-012, SP-013, SP-014 | Coordinate/animation/camera/export/catalog/replacement contracts. |
| ART-011 | SP-015, SP-018 | Audio implementation and source production. |
| ART-012 | SP-013, SP-017, SP-018 | Provenance/rights and release audit. |
| ART-013 | SP-013, SP-014, SP-016, SP-017 | AA-01–AA-08 validation evidence. |

SP-017 has explicit future readiness blockers: release/license choices, minimum hardware and budgets after baseline. No gameplay requirement is silently dropped because it is not READY. The [architecture](../architecture/architecture.md), [ADR directory links](../architecture/architecture.md#9-substantial-decisions), [quality gates](../validation/quality-gates.md) and [AGENTS.md](../../AGENTS.md) govern implementation handoffs. The roadmap supplies task links to these specs and their acceptance cases/commands. Documentary traceability can be checked now; actual test execution and completion evidence remain future implementation work.
