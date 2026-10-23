# Decision register

Stage 2 decision collection is complete; the decision summary still requires owner approval before Stage 3. This file is the canonical record. The owner instructed the planner on 2026-10-23 to use recommendations for all remaining questions and stop asking further brainstorming questions. Those defaults are recorded below with their evidence and limitations. This does not authorize implementation or approve Stage 3.

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
| Not decided | Final dependency/toolchain pin, target hardware, specific reusable features and performance budgets. Workspace default is recorded in DEC-010. |

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
| Capacity follow-up | Available time is recorded in DEC-004. Scheduling constraints and release horizon remain unspecified. |

## DEC-004 — Weekly owner availability

| Field | Record |
| --- | --- |
| Question | Q03, capacity portion: how much time can the owner regularly dedicate each week? |
| Options | Owner-supplied availability; illustrative ranges were 5–10, 10–20 and 20–40 hours weekly. |
| Choice | Use the owner's reported availability of 20–40 hours per week as the planning capacity range. |
| Status | Approved |
| Approval evidence | Verified as owner report: on 2026-10-23, the owner answered 20–40 hours to the weekly availability question. |
| Rationale | Ground scope and future estimates in declared owner capacity rather than assuming full-time availability or unmeasured AI acceleration. |
| Consequences | The range informs future planning but is not measured throughput or a release-date commitment. Account for review, testing, playtesting and content work within the owner's time. |
| Planning recommendation | Start estimates from the lower end of the range and recalibrate after delivered slices. This conservative estimate policy is a Recommendation, not an additional approved owner commitment. |
| Revisit condition | Availability changes or observed delivery/review effort requires recalibration. |
| Not decided | Total time budget, deadline, sustained output per week and allocation between engine, game and validation. |

## DEC-005 — Compact version 1.0 content envelope

| Field | Record |
| --- | --- |
| Question | Q04: select the initial version 1.0 content envelope. |
| Options | Lean, compact or expanded content scope, with explicit counts presented to the owner. |
| Choice | Compact. Canonical approved counts and counting rules are in [scope and non-goals](scope-and-non-goals.md#approved-content-envelope). |
| Status | Approved |
| Approval evidence | Verified: on 2026-10-23, the owner selected compact in response to the concrete content comparison. |
| Origin | Original design proposal approved by the owner; not derived reference-game content. |
| Rationale | Balance build variety with production and combination-testing work. Feasibility and delivery cost remain unmeasured. |
| Consequences | Use these counts for the GDD and subsequent task coverage. They describe the initial 1.0 baseline, not prototype deliverables or permission to expand the engine. |
| Revisit condition | Review measured prototype production and playtest evidence; any count changes require an explicit owner decision and synchronized scope/design/task updates. |
| Not decided | Individual content identities and exact balance values; see DEC-007–DEC-009. Release date remains unspecified. |

## DEC-006 — Original thematic identity

| Field | Record |
| --- | --- |
| Question | Q05, identity portion: which central fantasy should guide the game? |
| Options | Scrap robots in an industrial arena; alchemists against laboratory creatures; small guardians in a hostile garden; another owner-proposed direction. |
| Choice | Scrap robots in an industrial arena. |
| Status | Approved |
| Approval evidence | Verified: on 2026-10-23, the owner explicitly selected scrap robots in an industrial arena. |
| Origin | Original design direction approved by the owner; not copied reference-game identity or content. |
| Rationale | Owner selected the proposed theme. Inferred production opportunity: geometric silhouettes and modular mechanical forms may suit SVG authoring; no production savings or visual quality have been measured. |
| Consequences | Use this identity to guide later character, weapon, environment and visual proposals within the approved content envelope. This choice does not add mechanics or content counts. The visual direction is recorded in DEC-009. |
| Revisit condition | Explicit owner change of direction, with affected design and asset-production impact reviewed. |
| Not decided | Final name, detailed world and tone, exact palette, art pipeline details and animation scope. Thematic modularity does not approve a crafting or construction system. |

## DEC-007 — Combat agency and energy distribution

| Field | Record |
| --- | --- |
| Question | How should aiming, weapon slots and the original energy mechanic work? |
| Choice | Automatic firing and aiming by default, with optional manual aim. Four simultaneous weapon slots. During each between-wave interval, distribute six reactor points across the slots, with zero to three points per weapon. Each point increases that weapon's firing rate. |
| Status | Approved |
| Approval evidence | Verified owner answers on 2026-10-23: automatic plus optional manual aiming; four slots; points per slot; approved six total and three-per-slot maximum; chose firing rate as the effect. |
| Origin | Original design proposed in brainstorming; not asserted as a reference-game mechanic. |
| Rationale | Gives the industrial robot theme a build-allocation decision between waves while keeping attack input accessible. |
| Consequences | The GDD must specify targeting priority, manual aim input, point-to-fire-rate formula, clamping/rounding, zero-power behavior, and whether unallocated points are allowed. These values are not yet defined. |
| Revisit condition | Prototype playtests show that allocation or manual aim is unclear, unbalanced, or difficult to operate. |

## DEC-008 — Run, shop and weapon progression

| Field | Record |
| --- | --- |
| Question | What run cadence, shop, economy and equipped-weapon progression define the core loop? |
| Choice | Ten waves; boss encounter on waves five and ten. Open the shop between every wave. Scrap comes from defeated enemies. Each visit presents random offers and permits one paid refresh. If all four weapon slots are occupied, buying a weapon may replace an equipped weapon with confirmation. Buying a copy of an equipped weapon improves that weapon instead of consuming another slot. |
| Status | Approved |
| Approval evidence | Verified owner answers on 2026-10-23 to each listed proposal. |
| Origin | Original design decisions; the 10-wave structure, shop and economy are not claimed as copied reference behavior. |
| Rationale | A compact run structure supports the approved content envelope and makes inter-wave preparation a predictable recurring decision. |
| Consequences | The GDD must define wave duration/spawning, exact boss rules, scrap/drop rates, shop offer counts and weights, refresh cost, weapon upgrade tiers, replacement UX, victory and defeat guards. Those numerical rules remain open for design specification. |
| Revisit condition | Prototype simulations and playtests show that run length, economy or offer quality prevents viable builds or produces excessive downtime. |

## DEC-009 — Characters, unlocks and visual/audio direction

| Field | Record |
| --- | --- |
| Question | How should characters differ, what persists, and what production direction should guide the game? |
| Choice | Four characters; each starts with one weapon and one unique modifier. Between-run progression unlocks characters, weapons or items through objective challenges, with no permanent attribute bonuses. Visual direction: stylized vector art with readable scrap-robot silhouettes and contrasting industrial colors. Audio production default: original or properly licensed placeholder sounds and music; no paid generation service is required. |
| Status | Approved (visual/audio defaults accepted under the owner's blanket instruction; detailed implementation remains for Stage 3) |
| Approval evidence | Verified owner answer on 2026-10-23 accepted the visual recommendation and instructed the planner to follow recommendations for all remaining questions. The owner explicitly approved the character and unlock choices. Audio is a conservative planning default under that instruction. |
| Origin | Original character, progression and art direction. Audio placeholders must have documented provenance and license. |
| Rationale | Character starts and challenge unlocks create variety without persistent power growth; clear silhouettes suit crowded arena combat. |
| Consequences | The Art Bible must define palette, silhouette rules, animation, export pipeline, audio placeholders and asset provenance. Exact character content and challenge criteria belong in the GDD. PixelLab and ElevenLabs remain optional future services, not requirements. |
| Revisit condition | Prototype readability/playtests or measured asset workflow show that the proposed direction is unsuitable. |

## DEC-010 — Remaining planning defaults and deferred choices

| Field | Record |
| --- | --- |
| Question | How should remaining workflow, accessibility, distribution and budget choices be handled without further brainstorming questions? |
| Choice | Use one repository with a Cargo workspace as the provisional lowest-maintenance layout; no paid tools or external services are prerequisites. Use one difficulty level for the initial 1.0 release (separate from the single game mode count). Keyboard/mouse is the primary input. Controller support and a baseline of readable contrast, scalable UI, remappable controls and reduced screen effects are provisional GDD recommendations, not approved 1.0 requirements. Defer final title, repository visibility and license, commercial/personal release, storefront, budget ceiling, minimum Windows version, hardware target, detailed controller mapping and release date. |
| Status | Provisional defaults / Deferred choices |
| Approval evidence | Verified owner instruction on 2026-10-23: follow recommendations for remaining questions and do not ask more questions. The listed defaults are planning recommendations, not claims of measured cost or feasibility. |
| Rationale | Keep the project economical and accessible while avoiding irreversible choices about branding, legal terms, platform floor or distribution before the GDD and release plan. |
| Consequences | Stage 3 may propose controls/accessibility and art/audio workflow for review; controller support and accessibility details are not 1.0 commitments until approved with the GDD. Stage 4 must validate workspace and dependency choices. Visibility and licensing must be settled before release or public engine API commitments. No spending or third-party account is authorized by this default. |
| Revisit condition | Owner review of the Stage 2 summary, GDD/art-bible approval, or implementation evidence identifies a material trade-off. |

## Supplied repository destination

**Verified owner instruction, 2026-10-23:** save the project in [AlexGalhardo/project-omega](https://github.com/AlexGalhardo/project-omega). Repository visibility and licensing remain undecided. The provisional workspace recommendation is in DEC-010.

## Stage 2 boundary

The owner directed that no further brainstorming questions be asked and that remaining recommendations be used as defaults. Final title, commercial/personal release, visibility/license, budget ceiling, hardware floor and release date remain explicitly deferred, with consequences recorded in DEC-010. See the [research queue](../research/open-questions.md) for technical unknowns; it is not a competing product-decision register. The complete Stage 2 summary still requires owner approval before Stage 3 begins.
