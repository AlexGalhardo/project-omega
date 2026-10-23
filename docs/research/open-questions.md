# Open questions and evidence gaps

Status: technical evidence gaps after Stage 2 decision collection, 2026-10-23. Product decisions are canonical in the [decision register](../decisions/decision-register.md); this file does not repeat them. Confidence and sources: [ledger](sources.md). Detailed trade-offs remain in the relevant track.

## Findings and evidence

**Verified (owner brief and answers):** the intended products are an original arena-survival roguelite and a reusable 2D layer on Bevy, maintained by one developer with agent assistance. Repository artifacts must be English; owner conversation is Brazilian Portuguese. Product answers/defaults are in the decision register. **Unknown:** the technical gaps below. The reference repositories' settings and experience claims do not establish answers for this project.

## Stage 2 result

Brainstorming choices and owner-accepted defaults are recorded in [DEC-001–DEC-010](../decisions/decision-register.md). The owner requested no further brainstorming questions and instructed the planner to use recommendations for remaining questions. Explicitly deferred release and legal choices have consequences and revisit triggers in DEC-010. The complete decision summary must still be approved before Stage 3.

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
