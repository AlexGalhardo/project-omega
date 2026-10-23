# Open questions and evidence gaps

Status: end of Stage 1, 2026-10-23. This is the canonical research question queue, not a list of approved decisions or implementation tasks. Product choices require the owner. Confidence and sources: [ledger](sources.md). Detailed trade-offs remain in the relevant track.

## Findings and evidence

**Verified (owner brief):** the intended products are an original arena-survival roguelite and a reusable 2D layer on Bevy, maintained by one developer with agent assistance. Repository artifacts must be English; owner conversation is Brazilian Portuguese. **Unknown:** the product and capacity choices below. The reference repositories' settings and experience claims do not establish answers for this project.

## Stage 2 owner decisions, in priority order

Q01 and the 1.0 shipping-platform portion of Q02 are resolved in the [decision register](../decisions/decision-register.md); the repository destination was also supplied. Other entries remain **Open**. Suggested defaults are **Recommendations**, not decisions. Ask one high-impact question at a time, with rationale and alternatives; do not present this entire queue as a questionnaire.

| ID | Question / provisional default | Impact and resolution point |
| --- | --- | --- |
| Q01 | Resolved: see [DEC-001](../decisions/decision-register.md#dec-001--bevy-integration-strategy). | [Research trade-offs](03-bevy-and-fork-strategy.md). |
| Q02 | 1.0 shipping platform resolved: see [DEC-002](../decisions/decision-register.md#dec-002--version-10-platform-scope). Current Windows/PowerShell environment is the working development baseline. | Minimum OS/hardware remain open; confirm any additional development OS only if needed. |
| Q03 | Experience resolved in [DEC-003](../decisions/decision-register.md#dec-003--owner-experience-baseline). Available hours and release horizon remain open. | Before content counts and schedule estimates. No proficiency level or productivity multiplier for AI is assumed. |
| Q04 | What is the affordable 1.0 content envelope: characters, weapons, enemies, bosses, items, arenas and modes? Recommend a small complete offering, calibrated by an early slice. | Before Stage 3; counts must be owner-approved, not copied from references. |
| Q05 | What original identity, working name and one principal differentiator should define the game? | Before GDD/art direction. `project-omega` is a repository identifier, not an approved final title. Name clearance depends on release plans. |
| Q06 | What aiming and attack agency is intended? Recommend auto-fire as a candidate consistent with the brief; manual aim is an alternative to discuss. | Input, targeting, controller/accessibility, attention budget. |
| Q07 | What weapon-slot and build constraints should create decisions? | Combat schemas, shop, HUD and balance scope. Six slots are a reference fact, not a requirement. |
| Q08 | What wave/run cadence, arena behavior, boss structure and victory/death rules are intended? | Spawn model, state transitions, camera and total content production. |
| Q09 | Which stat families, shop tools and progression model are essential? Recommend a small transparent stat set and deliberate economy; distinguish unlock variety from permanent power. | Formulas, save data, fairness, balance/test combinations. |
| Q10 | What SVG style and animation ambition are intended? Recommend simple readable silhouettes with offline exports. | Art production, camera scale and presentation pipeline. Logical geometry stays separate. |
| Q11 | What audio direction and production method are affordable? Recommend licensed/original placeholders and simple SFX/music first. | Audio scope, provenance and optional paid tools. |
| Q12 | What controller and accessibility commitments belong in 1.0? | Remapping, focus/navigation, readability, reduced effects, assistance settings and native QA. |
| Q13 | Repository organization and visibility? Destination **supplied by owner:** `https://github.com/AlexGalhardo/project-omega`. Recommend one workspace in this repository. | No need to ask for the URL again. Visibility, public engine API and licensing are still choices; do not change remote visibility implicitly. |
| Q14 | Commercial or personal release, stores and distribution license? | Release QA, branding, legal review, packaging and storefront integration. |
| Q15 | Budget and paid tools? Recommend no paid-service prerequisite. | Asset-production options, recurring costs and approval boundaries. |

Before Stage 3, resolve or explicitly defer every design-affecting entry with a consequence and revisit trigger. The [decision register](../decisions/decision-register.md) is canonical for answers; this queue links to it rather than maintaining competing decisions.

## Technical unknowns and proposed ways to resolve them

These are **Unknown**, not approval requests. Experiments are proposed only; none was executed.

| ID | Gap | Evidence / cheapest next step | Blocks |
| --- | --- | --- | --- |
| U01 | Exact reference build and behavioral formulas | Select an ordinary-play build if needed; use EXP-A01–A08 from [Track A](01-gameplay-reference.md). Prefer an original rule when exact imitation has no design value. | Only requirements that depend on exact reference behavior. |
| U02 | Firsthand gameplay feel / readability | Observe and record normal gameplay; later test our own prototype with people. | Claims about feel; not the public-source research checkpoint. |
| U03 | Rust/Bevy bootstrap and optional plugin compatibility | After approval, build the selected release on the chosen OS and verify only required plugins. | Final architecture pins and Foundation validation. |
| U04 | Baseline runtime/build performance and target hardware | Owner selects hardware; run the proposed [measurement scenarios](04-rust-architecture-and-tooling.md#measurement-plan--proposed-experiments-not-budgets). | Performance budgets and optimization choices. |
| U05 | Collision requirements and physics dependency | Resolve attack/movement mechanics, then compare the simplest correct implementation with alternatives. | Collision ADR/spec readiness. |
| U06 | Supported determinism level | Resolve replay/multiplayer needs; repeated same-build input/seed tests before guarantees. | Replay/lockstep claims and RNG contract. |
| U07 | Data/save format and content-reload boundary | Discuss authoring preference; compare representative original schemas; define save-version/recovery needs. | Content/save ADRs and READY tasks. |
| U08 | SVG exporter fidelity, scale and skin-replacement boundary | Run the proposed [asset experiment](05-asset-pipeline.md#feasibility-experiment-proposed--not-executed) after approval. | Pipeline proof; not a reason to assume a custom renderer. |
| U09 | Toolchain and dependency license/security graph | Pin/build actual dependencies, run advisory/license checks and inspect notices during implementation. | Claims of audited dependencies and release qualification. |
| U10 | EULA/law review and scoped REA authorization | Owner review plus explicit recorded scope, only if a necessary question remains. | All Brotato binary/package inspection. Can remain deferred indefinitely. |
| U11 | Implementation-agent tools/native test availability | Record capabilities per task/environment; prepare manual procedures where necessary. | Claiming native acceptance; does not permit fabricating results. |
| U12 | Real throughput and maintainable engine extraction | Measure early delivered slices, review time and rework. | Reliable schedule estimates; broad reusable framework scope. |
| U13 | Independent adversarial review | Prepare handoff only at Stage 6; reviewer must inspect actual approved documents. | Claiming reviewed planning. No review has occurred. |

## Alternatives, risks and recommendation

**Recommendation:** resolve product/capacity questions first and run only experiments capable of changing an approved decision. An alternative is exhaustive reference reconstruction or broad engine experimentation, but that delays original design and can create sunk costs.

**Inferred risks:** silent defaults become accidental scope commitments; an unresolved design question gets hidden in an implementation task; a proposed experiment is mistaken for a result. Keep Open/Deferred visible, state consequences and revisit conditions, and never mark a dependent task READY with a critical unknown.
