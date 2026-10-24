# Open questions and evidence gaps

Status: evidence gaps after owner approval of Stages 3/4 artifacts, updated 2026-10-24. Decisions are canonical in the [register](../decisions/decision-register.md); rules in the [GDD](../gdd.md), [Art Bible](../art-bible.md) and [architecture](../architecture/architecture.md). Confidence and sources: [ledger](sources.md). Approval does not close unexecuted validation gaps.

## Findings and evidence

**Verified (owner brief and answers):** the intended products are an original arena-survival roguelite and a reusable 2D layer on Bevy, maintained by one developer with agent assistance. Repository artifacts must be English; owner conversation is Brazilian Portuguese. Product answers/defaults are in the decision register. **Unknown:** the technical gaps below. The reference repositories' settings and experience claims do not establish answers for this project.

## Stage 2 result

Brainstorming choices and defaults are recorded in DEC-001–DEC-010; explicit design approval in DEC-011 and Stage 4 approval in DEC-012. Deferred release/legal choices retain DEC-010 consequences. Document checks do not resolve the unexecuted experiments below.

## Technical unknowns and proposed ways to resolve them

These are **Unknown**, not approval requests. Experiments are proposed only; none was executed.

| ID | Gap | Evidence / cheapest next step | Blocks |
| --- | --- | --- | --- |
| U01 | Exact reference build and behavioral formulas | Select an ordinary-play build if needed; use EXP-A01–A08 from [Track A](01-gameplay-reference.md). Prefer an original rule when exact imitation has no design value. | Only requirements that depend on exact reference behavior. |
| U02 | Firsthand gameplay feel / readability | Observe and record normal gameplay; later test our own prototype with people. | Claims about feel; not the public-source research checkpoint. |
| U03 | Rust/Bevy bootstrap and optional plugin compatibility | After approval, build the selected release on the chosen OS and verify only required plugins. | Final architecture pins and Foundation validation. |
| U04 | Baseline runtime/build performance and minimum shipping hardware | Development reference inventoried; execute [performance plan](../architecture/performance-plan.md), then select shipping floor/budgets. | Performance budgets, release qualification and optimization choices. |
| U05 | Correctness/performance of chosen primitive collision queries | ADR-004 selects no physics dependency; SP-004/SP-007 must prove geometry and hit behavior. | Implementation verification, not renewed product brainstorming. |
| U06 | Actual same-build repeatability | ADR-011 limits the promise; execute SP-002/SP-016 trace checks. Replays/multiplayer excluded. | Any claim of proven determinism. |
| U07 | Actual JSON/save/reload correctness | Formats/boundaries chosen in ADR-006/007; SP-003/SP-010 must validate errors, snapshots and interrupted saves. | Implementation verification and release recovery evidence. |
| U08 | SVG exporter fidelity, scale and skin-replacement boundary | Run the proposed [asset experiment](05-asset-pipeline.md#feasibility-experiment-proposed--not-executed) after approval. | Pipeline proof; not a reason to assume a custom renderer. |
| U09 | Toolchain and dependency license/security graph | Pin/build actual dependencies, run advisory/license checks and inspect notices during implementation. | Claims of audited dependencies and release qualification. |
| U10 | EULA/law review and scoped REA authorization | Owner review plus explicit recorded scope, only if a necessary question remains. | All Brotato binary/package inspection. Can remain deferred indefinitely. 2026-10-24: the owner offered a local installation path during the planning review; the offer was declined pending these prerequisites and the path was not recorded (REV-019). |
| U11 | Implementation-agent tools/native test availability | Record capabilities per task/environment; prepare manual procedures where necessary. | Claiming native acceptance; does not permit fabricating results. |
| U12 | Real throughput and maintainable engine extraction | Measure early delivered slices, review time and rework. | Reliable schedule estimates; broad reusable framework scope. |
| U13 | Independent adversarial review | Prepare handoff only at Stage 6; reviewer must inspect actual approved documents. | Claiming reviewed planning. No review has occurred. |

## Alternatives, risks and recommendation

**Recommendation:** resolve product/capacity questions first and run only experiments capable of changing an approved decision. An alternative is exhaustive reference reconstruction or broad engine experimentation, but that delays original design and can create sunk costs.

**Inferred risks:** silent defaults become accidental scope commitments; an unresolved design question gets hidden in an implementation task; a proposed experiment is mistaken for a result. Keep Open/Deferred visible, state consequences and revisit conditions, and never mark a dependent task READY with a critical unknown.
