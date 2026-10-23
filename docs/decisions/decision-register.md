# Decision register

Stage 2 in progress. This file is the canonical record of owner choices. Research recommendations remain provisional unless recorded as Approved here. Approval of one decision does not approve the complete brainstorming summary or authorize implementation.

## DEC-001 — Bevy integration strategy

| Field | Record |
| --- | --- |
| Question | Q01: full fork, upstream with own plugins, or a minimal patch fork? |
| Options | A: full Bevy fork; B: released upstream Bevy plus own plugins; C: minimal patch fork plus plugins. |
| Choice | B: released upstream Bevy plus own plugins. |
| Status | Approved |
| Approval evidence | Verified: owner answered yes on 2026-10-23 to the explicit proposal to approve B, retaining a future move to C only for a demonstrated blocker and owner approval. |
| Rationale | Minimize engine maintenance and preserve time for the game while using Bevy's extension mechanisms. No current requirement demonstrates a need for internal changes. Maintenance-cost comparisons are Inferred, not measured. |
| Evidence | [Track C](../research/03-bevy-and-fork-strategy.md), especially C01–C16 in the [source ledger](../research/sources.md#track-c). |
| Consequences | No initial Bevy fork. Reusable facilities use project plugins; game rules stay in game systems. Plugin boundaries do not automatically require separate crates. |
| Revisit condition | A reproduced blocker on the selected release, evidence that public hooks and an acceptable upstream solution are insufficient, a minimal regression-tested patch proposal, maintenance/removal criteria and explicit owner approval. |
| Not decided | Final dependency/toolchain pin, workspace layout, target platforms, specific reusable features, content scope and performance budgets. |

## Supplied repository destination

**Verified owner instruction, 2026-10-23:** save the project in [AlexGalhardo/project-omega](https://github.com/AlexGalhardo/project-omega). This settles the destination portion of Q13 only; repository visibility, workspace organization and licensing remain undecided.

## Next decision

Q02: choose the 1.0 shipping platforms, then confirm the development OS if needed. All other unresolved questions remain in the [research queue](../research/open-questions.md). No complete Stage 2 decision summary has been approved.
