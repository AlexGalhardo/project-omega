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

## DEC-002 — Version 1.0 platform scope

| Field | Record |
| --- | --- |
| Question | Q02: which shipping platforms are mandatory for version 1.0? |
| Options | Windows only; Windows and Linux; broader desktop, web or mobile support. |
| Choice | Windows only for version 1.0. |
| Status | Approved |
| Approval evidence | Verified: owner answered Windows on 2026-10-23 to the question about mandatory version 1.0 platforms. |
| Rationale | Owner selected the recommended single-platform scope. Inferred benefit: concentrate packaging and native validation on one platform. No quantitative savings have been measured. |
| Evidence | Owner answer; platform trade-offs in [Track C](../research/03-bevy-and-fork-strategy.md). |
| Consequences | Plan native Windows validation and packaging. Other operating systems, web and mobile are outside mandatory 1.0 scope; future ports are not promised. |
| Revisit condition | An explicit owner decision to change release scope, with its testing, distribution and schedule impact reviewed. |
| Not decided | Minimum Windows version, CPU architecture, target hardware, storefront and detailed packaging requirements. The current Windows/PowerShell environment is the working development baseline; exclusive developer-OS support is not implied. |

## DEC-003 — Owner experience baseline

| Field | Record |
| --- | --- |
| Question | Q03, experience portion: what is the owner's current Rust and Bevy experience? |
| Options | New to both; experienced with Rust but new to Bevy; prior development with both. |
| Choice | Plan for an owner who has already developed with both Rust and Bevy. |
| Status | Approved |
| Approval evidence | Verified as owner report: on 2026-10-23, the owner stated prior development experience with both Rust and Bevy. Proficiency depth, project scale and versions used were not assessed. |
| Rationale | Use the owner's stated experience to avoid prescribing a beginner onboarding phase. |
| Consequences | Begin future Foundation work with project-specific compatibility and workflow validation. Do not infer expert proficiency, measured throughput or a larger content budget from this answer. |
| Revisit condition | A concrete implementation or review reveals a learning need; address it within the affected task. |
| Still open | Available hours, scheduling constraints and release horizon. |

## Supplied repository destination

**Verified owner instruction, 2026-10-23:** save the project in [AlexGalhardo/project-omega](https://github.com/AlexGalhardo/project-omega). This settles the destination portion of Q13 only; repository visibility, workspace organization and licensing remain undecided.

## Next decision

Q03, capacity portion: establish available owner time before setting content scope or estimating the roadmap. Experience is recorded in DEC-003. All other unresolved questions remain in the [research queue](../research/open-questions.md). No complete Stage 2 decision summary has been approved.
