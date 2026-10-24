# Decision register

Stage 2 was approved by the owner on 2026-10-23. On 2026-10-24 the owner directed Stage 4, approved all draft/proposed artifacts, and explicitly approved the GDD and Art Bible. This is the canonical approval record. Stage 3/4 documents and specs are approved design contracts; implementation readiness still requires the Stage 5 roadmap, satisfied dependencies and actual validation.

## DEC-001 — Bevy integration strategy

| Field | Record |
| --- | --- |
| Question | Q01: full fork, upstream with own plugins, or a minimal patch fork? |
| Options | A: full Bevy fork; B: released upstream Bevy plus own plugins; C: minimal patch fork plus plugins. |
| Choice | B: released upstream Bevy plus own plugins. |
| Status | Superseded by DEC-018 (vendored Bevy fork), 2026-10-24 |
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
| Status | Approved |
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
| Status | Approved |
| Approval evidence | Verified owner instruction on 2026-10-23: follow recommendations for remaining questions and do not ask more questions; subsequently approved the consolidated Stage 2 summary. Approval accepts the listed defaults and deferrals, including one difficulty level. Detailed controller/accessibility proposals still await GDD approval as explicitly stated in the choice. No measured cost or feasibility is claimed. |
| Rationale | Keep the project economical and accessible while avoiding irreversible choices about branding, legal terms, platform floor or distribution before the GDD and release plan. |
| Consequences | Stage 3 may propose controls/accessibility and art/audio workflow for review; controller support and accessibility details are not 1.0 commitments until approved with the GDD. Stage 4 must validate workspace and dependency choices. Visibility and licensing must be settled before release or public engine API commitments. No spending or third-party account is authorized by this default. |
| Revisit condition | Owner review of the Stage 2 summary, GDD/art-bible approval, or implementation evidence identifies a material trade-off. |

## Supplied repository destination

**Verified owner instruction, 2026-10-23:** save the project in [AlexGalhardo/project-omega](https://github.com/AlexGalhardo/project-omega). Repository visibility and licensing remain undecided. The provisional workspace recommendation is in DEC-010.

## Stage 2 boundary

**Verified owner approval, 2026-10-23:** the owner approved Stage 2 and requested Stage 3. The approval covers the consolidated DEC-001–DEC-010 summary and its explicit deferrals. Final title, commercial/personal release, visibility/license, budget ceiling, hardware floor and release date remain deferred under DEC-010. See the [research queue](../research/open-questions.md) for technical unknowns. This approval authorizes writing the GDD and Art Bible, not implementation.

## DEC-011 — Stage 3 original design baseline proposal

| Field | Record |
| --- | --- |
| Question | Which precise rules and production contracts make the approved direction implementable? |
| Options | Specify a compact original baseline with explicit provisional values; defer mechanics until implementation; reproduce reference formulas. |
| Choice | Use [GDD revision 1](../gdd.md) and [Art Bible revision 1](../art-bible.md) as the Stage 4 baseline. Values marked P remain provisional for balance/production validation. |
| Status | Approved |
| Evidence | Owner authorized Stage 3, then directed Stage 4 on 2026-10-24 and subsequently explicitly approved both the GDD and Art Bible. This is owner approval, not an independent detailed review or tested balance. Research evidence limits remain linked from both documents. |
| Rationale | Define testable mechanics, content, failure behavior and replaceable presentation before implementation; retain measured balance/feasibility as future evidence. |
| Consequences | Includes a proposed 60-tick simulation, formulas, tier/economy rules, full content catalogs, profile/settings persistence, controller/accessibility proposal, offline SVG pipeline and original placeholder audio policy. These are not tested results or architecture API choices. |
| Revisit condition | Owner Stage 3 review, later architecture findings or playtest evidence; change approved rules through a recorded change request rather than silent redesign. |

## DEC-012 — Stage 4 architecture and spec approval

| Field | Record |
| --- | --- |
| Question | May the drafted architecture, ADRs, specs and operating contracts advance out of draft/proposed status? |
| Options | Approve the planning contracts; retain drafts for owner questions; reject a specific proposal with evidence. |
| Choice | Approve all authored Stage 4 planning contracts under the owner's explicit blanket approval. Canonical scope is [architecture](../architecture/architecture.md), its eleven ADRs, [18 specs](../specs/index.md), [agent contract](../../AGENTS.md) and validation documents. |
| Status | Approved |
| Approval evidence | Verified owner instruction on 2026-10-24: approve all files/items requiring owner acceptance to leave draft. The owner also explicitly approved the GDD and Art Bible. |
| Rationale | Owner chose to accept recommended planning defaults and continue without repeated approval questions. |
| Consequences | Mark documents/specs APPROVED and ADRs Approved; do not mark READY, IMPLEMENTED, VERIFIED or DONE without their separate objective conditions. Stage 5 task/dependency mapping remains to create. No build/test/review result is inferred from approval. |
| Revisit condition | A concrete implementation incompatibility, test failure or scope conflict requires a recorded change request and targeted verification. |

## DEC-013 — Stage 5 roadmap approval and review corrections

| Field | Record |
| --- | --- |
| Question | Is ROADMAP.md approved as the Stage 5 task plan, and may Phase 1 Foundation start? |
| Options | Approve ROADMAP.md with the review corrections; keep it PROPOSED; reject it. |
| Choice | Approve [ROADMAP.md](../../ROADMAP.md) as corrected for findings REV-001 through REV-021 of the [2026-10-24 planning review](../reviews/2026-10-24-planning-review.md), and authorize all Phase 1 Foundation tasks in roadmap order. Later phases still require Foundation exit evidence. |
| Status | Approved |
| Approval evidence | Verified owner instruction, 2026-10-24 (owner conversation, Brazilian Portuguese): correct every point of the planning review, then start Phase 1 Foundation and leave the project ready for Phase 2 onwards. Approved revision: the first commit that contains this entry; its hash is the `docs(stage-5)` commit recorded in git history. |
| Rationale | The review found governance and record-keeping gaps, not design defects; the owner chose to fix them and proceed. |
| Consequences | PLAN.md is the single status file and carries the dashboard. CR-001, CR-002 and CR-003 are filed and approved under DEC-014 and DEC-015. Phase 2 tasks remain TODO until their dependencies are DONE. |
| Revisit condition | A Foundation result that invalidates a roadmap dependency or estimate. |

## DEC-014 — GDD revision 2: exact rate units and boss request tick

| Field | Record |
| --- | --- |
| Question | How are REV-007 (unspecified boss warning start) and REV-008 (`base_rate` without exact unit) resolved? |
| Options | See [CR-001](change-requests/cr-001-boss-request-tick.md) and [CR-002](change-requests/cr-002-exact-rate-units.md). |
| Choice | Approve both change requests: `base_rate_centi` integer hundredths of attacks per second, energy multiplier as integer percent, integer interval formula; Wave records carry `boss_request_tick`, authored as 0 for waves 5 and 10. GDD moves to revision 2. No balance value changes. |
| Status | Approved |
| Approval evidence | Same owner instruction as DEC-013 ("correct every point of the review"). |
| Rationale | Removes floating-point boundary ambiguity before SP-006, and fixes boss pacing before SP-008. |
| Consequences | SP-003 content schemas implement the revised fields in Phase 1. |
| Revisit condition | Playtest evidence that boss arrival at wave start is poor pacing; change the authored tick, not the schema. |

## DEC-015 — Dependency and tool pins refreshed to latest stable

| Field | Record |
| --- | --- |
| Question | Which dependency/tool versions does the Foundation bootstrap pin? |
| Options | Keep the Stage 4 candidates; refresh to the latest stable releases at bootstrap time. See [CR-003](change-requests/cr-003-dependency-refresh.md). |
| Choice | Refresh to the latest stable releases verified with `cargo info` on 2026-10-24, pinned exactly: Rust 1.99.1, Bevy 0.20.0, serde 1.0.229, serde_json 1.0.154, rand_chacha 0.10.0, sha2 0.11.1; cargo-deny 0.20.2 and resvg 0.48.1 as tools. getrandom is deferred until native run creation needs it. |
| Status | Approved |
| Approval evidence | Owner standing rule (global instructions): always install the latest stable, production-ready version, never pre-releases. |
| Rationale | Stage 4 candidates were already one or more releases behind; refreshing before the first lockfile costs nothing. |
| Consequences | Architecture section 2 and ADR-002 reference this decision. The bootstrap evidence records the resolved graph. |
| Revisit condition | A compile, license or advisory incompatibility; record it through a change request. |

## DEC-016 — Engineering conventions and release process

| Field | Record |
| --- | --- |
| Question | Which formatting, versioning and release conventions does the repository use? |
| Options | Tool defaults; owner conventions from global instructions and the owner's existing repositories. |
| Choice | Tabs of width 4 everywhere except YAML/Markdown/JSON formats that need spaces (`.editorconfig`, `rustfmt.toml hard_tabs`), markdownlint-cli2, Conventional Commits, SemVer starting at 0.1.0, Keep a Changelog. Releases follow the owner's [vukem](https://github.com/AlexGalhardo/vukem/releases) pattern: tag `vX.Y.Z`, notes extracted from the matching CHANGELOG.md section, archive per platform (Windows x86_64 only), SBOM per archive, `checksums.txt` signed keyless with cosign and a build provenance attestation. |
| Status | Approved |
| Approval evidence | Owner instruction, 2026-10-24, naming CI/CD with GitHub Actions, Keep a Changelog, SemVer, `.editorconfig` with tabs, formatter, linter and the vukem release pattern. |
| Rationale | One consistent convention across the owner's repositories. |
| Consequences | Release workflow and process are documented in [docs/release.md](../release.md). Pushing tags remains an owner action. |
| Revisit condition | A storefront or distribution decision under DEC-010. |

## DEC-017 — The project's two primary objectives

| Field | Record |
| --- | --- |
| Question | What is this project for? |
| Choice | Two primary objectives, both for didactic learning: **(1)** study the owner's legitimately purchased Steam copy of Brotato with REA (reverse engineering assistance) and progressively re-create the game; **(2)** develop the project's own 2D-focused game engine as a direct fork of [Bevy](https://bevy.org/). |
| Status | Approved |
| Approval evidence | Verified owner instruction, 2026-10-24 (owner conversation): two primary objectives "para fins didáticos de aprendizado", with the Brotato installation at `C:\Program Files (x86)\Steam\steamapps\common\Brotato` purchased on Steam. |
| Rationale | The owner's learning goals: how a shipped arena-survival game works internally, and how a production engine works by owning and extending one. |
| Consequences | DEC-018 replaces DEC-001's upstream-only strategy with a vendored fork. DEC-019 authorizes scoped REA inspection. The re-creation stays an independent implementation: code, art, audio, text and names in this repository remain original (IP policy). Inspection findings can calibrate the GDD only through change requests. |
| Revisit condition | Any plan to distribute builds publicly, change repository visibility or sell the game requires a new owner decision and a legal review first. |

## DEC-018 — Engine from a vendored Bevy fork

| Field | Record |
| --- | --- |
| Question | How is "own engine as a direct Bevy fork" implemented? |
| Options | A: GitHub fork referenced by git SHA; B: Bevy source vendored inside this repository and patched in place; C: keep upstream crates (DEC-001 B). |
| Choice | B. The published Bevy 0.20.0 crates (upstream commit `b3bdd75becaffe9a16677cf5bae5bd6d198f052b`) live in [`engine/bevy/`](../../engine/README.md) and the root manifest patches crates.io to them. No GitHub fork. The fork evolves in parallel with the game: new engine code when the game needs it, and fixes for engine bugs. `crates/omega_2d` remains the project's own 2D layer on top. |
| Status | Approved; supersedes DEC-001 choice B |
| Approval evidence | Verified owner instruction, 2026-10-24: download the code into this project and develop it in parallel with the game; no explicit GitHub fork. |
| Rationale | Full ownership of engine code for learning, with the smallest operational cost: one repository, one lockfile, no submodule or git dependency. |
| Consequences | [ADR-012](../architecture/adr/012-vendored-bevy-fork.md) supersedes ADR-001. Every engine change is logged in the engine patch log with a regression test. Upgrades re-apply logged patches onto a new release. About 37 MB of engine source is committed. |
| Revisit condition | Repository size or upgrade effort becomes a measured problem. |

## DEC-019 — Scoped REA inspection of the owner's Brotato copy

| Field | Record |
| --- | --- |
| Question | Under which scope may REA inspect Brotato (IP policy "Restricted inspection")? |
| Choice | **Target:** the owner's Steam Windows build at `C:\Program Files (x86)\Steam\steamapps\common\Brotato` (`Brotato.exe`, `Brotato.pck`). **Question:** how the game's rules, formulas, timings, wave/shop/stat systems and data model work, for didactic re-creation. **Permitted operations:** local, read-only REA inventory and static analysis (including Godot PCK inspection), plus ordinary play. **Excluded:** DRM or license bypass; modifying the installation; committing, or pasting into implementation context, any extracted asset, script, decompiled code, raw data table or screenshot of extracted assets. **Retention:** raw REA output stays outside the repository (for example `%LOCALAPPDATA%\ProjectOmega\rea\`); only original prose notes describing behavior, with uncertainty, enter `docs/research/brotato-notes/`. |
| Status | Approved, owner-asserted legal basis |
| Approval evidence | Verified owner instruction, 2026-10-24 (DEC-017). The owner states the copy was purchased on Steam. EULA and law review are the owner's responsibility; no agent verified them and this is not a legal opinion. |
| Rationale | Fulfils objective (1) while keeping the repository free of proprietary material. |
| Consequences | REA is not yet installed (verified 2026-10-24: no `rea` on PATH). Setup follows [Track B](../research/02-rea.md) as task RES-P1-001. REA output may travel to a remote model provider; the inspecting session must know that before running it. |
| Revisit condition | Any need to copy material instead of describing it; any distribution plan. |

## DEC-020 — Autonomous implementation sessions

| Field | Record |
| --- | --- |
| Question | What may an unattended agent session do while the owner is away for many hours? |
| Choice | A session started from the owner's local `start-work-for-sleep.md` prompt works through ROADMAP.md Phase 2 to Phase 5 without stopping, under these rules: **(1)** parallel agents and git worktrees are pre-authorized for any task or batch with an ETA of 30 minutes or more, with disjoint file ownership and serial edits of shared files (AGENTS.md); **(2)** after `scripts/check.ps1` passes, commits may be pushed to `main`, and milestone releases follow ROADMAP.md 1.4/1.5; **(3)** a check that needs the owner (native playtest, visual/audio/controller review, hardware or business decision) is recorded IN_REVIEW or BLOCKED with the exact question in the local `questions-to-dev.md`, and dependents may proceed on the code-complete result as a provisional acceptance the owner revisits; **(4)** BLOCKED owner decisions (QA-P4-003, REL-P5-001) are skipped, not guessed; **(5)** the final `1.0.0` release (REL-P5-004) and any store publication are never performed autonomously. |
| Status | Approved |
| Approval evidence | Verified owner instruction, 2026-10-24: implement Phase 2 through Phase 5 without interruptions while away, use worktrees/parallel agents when the ETA is at least 30 minutes, and collect questions in `questions-to-dev.md`. |
| Rationale | Lets work continue for many hours without manual steps, while keeping owner-only judgments explicit and reversible. |
| Consequences | Provisional acceptances are listed in `questions-to-dev.md` and in the evidence of each affected task; the owner's later review can reopen tasks. A red CI run or a failing gate stops pushing and releasing until fixed. |
| Revisit condition | Any provisional acceptance the owner rejects; any destructive or outward action outside these rules. |
