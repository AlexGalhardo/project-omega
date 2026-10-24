# Stage 1 — Research summary

Research date: 2026-10-23. Navigation updated 2026-10-24: **research delivered; Stages 2, 3 and 4 artifacts approved**. The owner decisions are canonical in the [decision register](../decisions/decision-register.md); recommendations below retain their dated research context. Planning is not complete; Stage 5 task mapping remains. No game implementation, feasibility spike, benchmark, REA inspection or independent model review has occurred.

## Project mandate and current state

**Verified (owner-provided constraints):** build two products: a reusable 2D layer over Bevy and an original top-down arena-survival roguelite serving as its reference game. Shipping a stable, maintainable 1.0 takes precedence over expanding the engine. The owner reports roughly ten years of web development and TypeScript/Phaser game work. Subsequent owner clarification confirms prior development with Rust and Bevy in [DEC-003](../decisions/decision-register.md#dec-003--owner-experience-baseline); weekly availability is recorded as 20-40 hours in [DEC-004](../decisions/decision-register.md#dec-004--weekly-owner-availability). Delivery throughput and release date remain **Unknown**.

The planning role researches, proposes and specifies; other agents implement after approvals. Agent identities are not repository dependencies. All repository artifacts, future code/comments/UI/tests/commits and agent instructions must be English. Owner conversation is Brazilian Portuguese. Behavioral inspiration must comply with the canonical [IP policy](../legal/ip-policy.md).

The owner selected [AlexGalhardo/project-omega](https://github.com/AlexGalhardo/project-omega) as the repository. It was empty when cloned into the working directory; no existing implementation or project-specific instructions needed migration. The repository identifier is not the game's final name.

## Executive findings and evidence

Source IDs resolve to dated URLs/revisions in [sources.md](sources.md). Recommendations below have origin **original design**; design lessons have origin **inferred**. Gameplay findings are external reports, not direct observations by this team.

| Finding | Label and evidence | Practical consequence |
| --- | --- | --- |
| R-A01: alternating pressure and build decisions is the central useful reference pattern. | **Inferred** from public loop descriptions [A01, A03, A05]. | Preserve deliberate decisions; do not inherit every reference mechanic or content count. |
| R-A02: exact formulas and current behavior are not uniformly established. | **Verified as source skew / Unknown as runtime behavior** [A04, A07, A08, A11]. | Use proposed black-box experiments or choose transparent original rules. |
| R-B01: REA has relevant inspection capabilities but no authorized project use. | **Verified documentation / owner policy** [B01–B05]. | Defer it; planning has no demonstrated dependency on binary inspection. |
| R-C01: Bevy 0.20.0 is the verified current stable baseline. | **Verified** release/tag metadata; Rust minimum declared 1.97.1 [C01, C02, C25]. | Use release-specific source/examples; validate a concrete toolchain at Foundation. |
| R-C02: no current requirement establishes a need to change engine internals. | **Inferred** from the stated needs and public extension mechanisms [C03–C14]. | **Recommendation:** upstream plus project plugins; retain minimal-patch escalation. |
| R-D01: a small standard toolset covers the immediate engineering needs. | **Recommendation**, grounded in Cargo/Rust/Bevy tooling [D02–D16]. | Avoid large crate/tool matrices; adopt extra tools against specific verification needs. |
| R-E01: repeatable simulation requires controlled inputs, ordering, content and RNG. | **Inferred**, supported by fixed-time/schedule/RNG contracts [C04, C05, E01, E04]. | Promise only demonstrated same-build repeatability initially; no automatic cross-platform determinism. |
| R-F01: SVG authoring does not require vector rendering at runtime. | **Inferred** from exporter and sprite capabilities [F01, C23]. | **Recommendation:** offline rasterization; preserve visual-ID and logical-geometry boundaries. |
| R-F02: the inspected vector-plugin manifests target Bevy 0.19. | **Verified** [F04, F05]. | Runtime vector adoption needs compatibility proof; it is avoidable for the proposed pipeline. |
| R-G01: explicit task contracts and evidence suit repository-only handoffs. | **Recommendation**, informed by actual supplied conventions [G01–G08]. | One canonical rule set, bounded context, criterion-to-test evidence; no external agent framework required. |

## Provisional strategy

**Recommendation:** choose strategy **B — released upstream Bevy plus own plugin code**, initially in one repository/Cargo workspace. Extract only services demonstrated by the game or a concrete reuse requirement. A plugin is not automatically a separate crate. The five-layer boundary and full A/B/C comparison live in [Track C](03-bevy-and-fork-strategy.md).

Keep strategy **C — a minimal patch fork** available only for a reproduced blocker, with evidence that public hooks and an acceptable upstream solution are insufficient, owner approval, a regression test and an exit condition. Strategy **A — a full fork** offers internal control but creates a substantially larger maintenance responsibility. No measured sync cost or requirement currently justifies it.

Use fixed-step gameplay with explicit ordering and a presentation boundary, typed/versioned content, standard testing and a measured baseline before optimization. Prefer SVG source assets exported to textures. Keep visual style replaceable without altering simulation rules. Subsequent product approvals are recorded in the decision register. Detailed mechanics and art direction are now proposed in the [GDD](../gdd.md) and [Art Bible](../art-bible.md); research recommendations do not independently approve those drafts.

## Deliverables and checkpoints

| Track | Canonical file | Research checkpoint |
| --- | --- | --- |
| A | [Gameplay reference](01-gameplay-reference.md) | Public behavioral model, five-game comparison, version caveats and proposed experiments. |
| B | [REA](02-rea.md) | Capabilities, providers/OS limits, security and gated evidence workflow. |
| C | [Bevy and fork strategy](03-bevy-and-fork-strategy.md) | Current release/source assessment, alternatives and five layers. |
| D + E | [Rust architecture and tooling](04-rust-architecture-and-tooling.md) | Minimal tools, simulation/data/test recommendations and baseline plan. |
| F | [Asset pipeline](05-asset-pipeline.md) | Runtime/offline/hybrid comparison and replacement boundary. |
| G | [SDD and agent workflow](06-sdd-and-agent-workflow.md) | Actual reference conventions, bounded handoffs, evidence and integration. |
| Cross-cutting | [Source ledger](sources.md), [open questions](open-questions.md), [IP policy](../legal/ip-policy.md) | Source provenance, unresolved decisions and research restrictions. |

## Risks and unknowns

**Unknown:** delivery throughput and release horizon, minimum Windows/hardware requirements, feasibility of the approved content envelope, actual feel, build/runtime performance, exact reference formulas and independent review findings. The original differentiator was subsequently selected in DEC-007. Evidence gaps are tracked in [open questions](open-questions.md), not silently resolved here.

**Inferred:** the largest delivery risks are excessive content, engine generalization before need, learning/migration overhead, unproven integration, and confusing automated test success with a satisfying playable game. The strongest mitigations are small approved scope, an early complete playable slice, native playtesting and evidence-based cuts. No schedule or performance budget is established.

## Documentary verification

**Verified locally at the Stage 1 checkpoint:** the expected research/legal files existed; all 11 then-current Markdown files decoded as UTF-8; an automated check found no broken local file/heading links, undefined cited source IDs or unbalanced fenced blocks. Later-stage artifacts were absent at that checkpoint; subsequent design work is recorded in PLAN.md and the GDD. A manual consistency pass checked recommendation/approval boundaries and corrected ambiguous dependency-direction wording. This is document validation, not a game test, full legal audit or independent review.

## Planning gates and continuation

The owner requires this order:

1. **Research:** findings and unknowns documented. This checkpoint satisfies the documentary output, not the unexecuted experiments.
2. **Brainstorming:** one high-impact question per turn; record answers in `docs/decisions/` and obtain approval of the decision summary.
3. **Design:** write and obtain approval of `docs/gdd.md` and `docs/art-bible.md`.
4. **Architecture + SDD:** architecture, substantial ADRs, specs, canonical agent contract and validation gates; obtain owner approval.
5. **Roadmap:** exactly five implementation phases (Foundation, Prototype, Alpha, Beta, RC to 1.0), dependency-checked tasks and owner approval before implementation begins.
6. **Review handoff:** prepare the self-contained adversarial prompt; preparation is not a completed review.

No GDD, Art Bible, architecture ADRs, implementation specs, agent contract, roadmap or review prompt is created at Stage 1. No implementation or Bevy-internal edits are permitted before Stages 4 and 5 approvals. Earlier feasibility spikes require explicit experiment authorization. Research recommendations cannot mark implementation tasks READY.

The full traceability chain is the later planning acceptance condition: evidence -> approved decision -> GDD requirement -> ADR/architecture -> spec -> roadmap task -> acceptance test. At this stage only evidence, proposals and question IDs exist; absent later links are **intentionally pending**, not a passed planning audit.

**Continuation:** The owner approved GDD/Art Bible and all Stage 4 draft/proposed artifacts. See [PLAN.md](../../PLAN.md) for current status and Stage 5 continuation; the original brainstorming queue is no longer active. Approval does not imply executed gameplay validation.
