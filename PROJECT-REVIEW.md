# Independent adversarial review — Project Omega

Use this document as a self-contained prompt for Claude Fable 5.1 or another explicitly assigned independent principal reviewer. The reviewer has no access to the planning conversation. Preparing this prompt is not a completed review. No reviewer has inspected or approved this project yet.

Prepared 2026-10-24 at the owner's request. The Stage 5 task plan is [ROADMAP.md](ROADMAP.md) (PROMPT.md section 8 calls it `PLAN.md`; PLAN.md holds stage status and the dashboard). The first review using this prompt is recorded in [docs/reviews/2026-10-24-planning-review.md](docs/reviews/2026-10-24-planning-review.md); its corrections were applied under DEC-013. A later reviewer must review the current state, not that earlier gap.

## Your role and objective

Act as an adversarial principal game/engine architect. Inspect actual repository files and primary evidence. Your purpose is to increase the probability that one developer ships a stable 1.0 game on a maintainable reusable foundation. Challenge unsupported claims, unnecessary abstractions, contradictions and plans that cannot be executed. Separate objective defects from preferences and keep recommendations proportional to this project's scope.

Do not restate the planner's conclusions as findings. Do not implement code, rewrite approved decisions or resolve missing product decisions silently. Write findings and a prioritized correction/execution order. If a file, tool, source, runtime or model capability is unavailable, report precisely what you could not verify. Do not claim gameplay, compilation, benchmarks, asset export, legal clearance or independent testing without performing and recording it.

## Product and owner context

Two intended products: an original top-down arena-survival roguelite and a small reusable 2D layer over upstream Bevy. Shipping the game takes priority over creating a large engine. The owner has approximately ten years of web development, prior Phaser work, prior Rust and Bevy development, and reported availability of 20–40 hours weekly. Proficiency depth, measured delivery throughput and deadline are unknown. AI implementers may differ in tools/context/native environment; repository handoffs must remain model-independent.

Repository: <https://github.com/AlexGalhardo/project-omega>. Working title Project Omega; final name, commercial/personal release, storefront, repository licensing/visibility, budget ceiling and minimum shipping hardware remain deferred. Repository artifacts and review output must be English. Conversation with the owner is Brazilian Portuguese.

Approved design direction, to verify against the actual files:

- Windows 1.0; upstream Bevy plus own plugins, no initial fork.
- Scrap robots in an industrial arena; stylized vector art exported offline to replaceable raster presentation.
- Four characters, twelve base weapons, ten common enemy types, two bosses, forty passive items, one arena/mode and one difficulty.
- Automatic firing/aiming with optional manual aim; four weapon slots; six energy points, at most three per weapon, assigned between waves to increase firing rate.
- Ten waves, bosses on five and ten; inter-wave shop, scrap from defeated enemies, randomized offers, one paid refresh per visit; replacement on a full loadout and duplicate-weapon tier upgrades.
- Characters start with one weapon and one modifier; challenges unlock content without permanent attribute power.
- Precise formulas, timings, catalogs and presentation choices are original design. Values marked P are provisional balance/production baselines, not measured results.

The GDD includes additional accepted design choices such as zero-power disabled weapons, diminishing energy returns, fixed combat ticks, profile/settings saves without run resume, controller/accessibility requirements and specific enemy/boss rules. Inspect them rather than relying on this short summary. Owner approval is recorded in DEC-011/012; it does not make an engineering choice correct or prove balance.

## Current evidence and stage boundary

Stage 1 public-source research exists. No firsthand gameplay experiments, REA inspection, game implementation, asset-export proof, native controller/readability test, runtime/build benchmark or independent review has occurred. Stage 2 decisions were approved; the owner explicitly approved GDD/Art Bible and all Stage 4 draft/proposed artifacts. Eighteen specs are APPROVED as contracts, with eleven approved ADRs. SP-001 is READY for the narrowly scoped ENG-P1-001 bootstrap packet; other specs remain APPROVED. No code is implemented or verified.

Stage 4 proposes two Cargo crates, `omega_2d` and `omega_game`, with three reusable engine contracts, headless simulation, JSON content/saves, explicit RNG ownership and measured optimization. There is currently no Rust workspace or game build. Existing local Node/AI-memory setup is separate tooling and must not be mistaken for a playable game or required engine dependency.

The planner recorded documentary checks in GDD-020 and quality-gates.md. Reproduce/check their scope; passing links, counts and arithmetic does not establish complete implementation traceability, balanced combat or sound architecture. Hardware inventory describes a development reference machine, not minimum supported hardware. Versioned sources were checked, but the proposed dependency graph was not compiled/audited.

The owner expedited the handoff because of planner capacity and then requested the plan. The roadmap was authored subsequently, with exactly five phases and one initial READY packet. Review its actual dependency graph, scope and readiness independently; document approval does not establish executed implementation or a successful independent review.

## Read the actual repository

Start with `AGENTS.md`, `PLAN.md`, `README.md` and `docs/decisions/decision-register.md`. Then inspect:

1. `docs/research/00-summary.md`, the track documents, `sources.md` and `open-questions.md`. Check primary URLs/revisions/dates for consequential claims and report inaccessible evidence. Do not assume current library behavior from old notes.
2. `docs/legal/ip-policy.md` and `docs/decisions/scope-and-non-goals.md`.
3. `docs/gdd.md` and `docs/art-bible.md` in full, including catalogs, formulas, phase timing, UI behavior, saves and acceptance examples.
4. `docs/architecture/architecture.md`, `performance-plan.md` and every ADR in `docs/architecture/adr/`.
5. `docs/specs/_template.md`, `index.md` and all eighteen concrete specs. Trace dependencies, required engine contracts, files, commands and evidence sections.
6. `AGENTS.md`, the thin `CLAUDE.md` adapter, `docs/validation/definition-of-done.md`, `quality-gates.md` and the change-request template. Inspect other setup/overnight instructions for conflicts with canonical stage/gate rules.
7. `PLAN.md` if present when you run. If still absent, state that task sizing, READY assignments, exact cross-track dependencies, critical path and final five-phase coverage cannot be verified. Do not fabricate that review.

Record the reviewed commit and working-tree state. Distinguish committed documents from untracked or concurrent edits. Do not include private configuration/secrets in review output. Use read-only investigation and narrowly scoped documentary validation; game commands listed in specs are planned commands and cannot pass before their targets exist. Missing code at this planning stage is expected, while missing required planning contracts may block readiness.

## Required review dimensions

### Research and evidence

Check source quality, versions, access dates, provenance, uncertainty and conflicting sources. Identify claims labeled Verified that are merely source reports or design assumptions. Evaluate whether reference-game research yielded usable design principles without unnecessary reconstruction. Review whether compatibility and build claims are honestly limited. Do not infer that dependency licenses or advisory status are cleared from a version lookup.

### Design and feasibility

Walk a new run, all ten waves, purchases, upgrades, refresh, power allocation, boss encounters, defeat/victory, unlocks, saves and errors. Recompute formula examples and boundary cases. Look for unreachable states, contradictory ordering, degenerate dominant builds, hidden economy assumptions and permanent soft locks. Inspect feasibility of the content count, uniqueness/production burden, accessible controls, feedback, pacing and first-run experience. Balance hypotheses should be proposed for playtesting rather than declared proven from arithmetic alone.

### Architecture and Bevy maintenance

Check five-layer boundaries, exact engine reuse justification, two-crate organization, feature isolation, public contracts and migration policy. Inspect fixed-tick scheduling, deferred mutation, input buffering, event retention, RNG order, numeric guarantees, geometry correctness, headless behavior, content snapshot/reload and save recovery. Treat arbitrary engine abstraction, custom physics complexity and unnecessary tooling as potential costs. Assess dependency pins and whether unexecuted compatibility work is improperly hidden behind READY claims.

### Art, audio, input and accessibility

Cross-check every relevant hitbox, radius, pivot, attack phase and HUD value against the GDD. Verify the proposed catalog can change art without altering gameplay and that warnings follow the actual attack origin policy. Assess SVG production scope, exporter proof, UI scaling/focus, controller behavior, reduced effects, placeholder provenance and optional paid services. Identify tests that require actual human/native evaluation and cannot be replaced by a screenshot or headless assertion.

### Specs, workflow and roadmap realism

Each spec must define what to build, what not to touch, data ownership, systems/schedules/queues, failure behavior, acceptance and exact future validation. Check whether each proposed slice fits a reviewable change, whether tasks would be too broad or fragmented, and whether dependency cycles/shared-file conflicts exist. Validate that lifecycle states distinguish approval from implementation evidence. Assess minimal context loading, model-independent handoffs, change requests and serial ownership. If PLAN.md exists, require exactly five phases, ENG/GAME plus QA/REL work, stable IDs, exact engine-contract dependencies, measurable gates and GDD coverage. Otherwise flag these as unverified missing work.

### Security, licenses and IP

Apply the clean-room policy: only public sources and normal gameplay by default; no copying proprietary code/assets/data and no protection bypass. Brotato binary/package inspection requires the owner's prerequisite EULA/law review and explicit scoped approval; none is recorded. Review unsafe external SVG/resources, save parsing/recovery, CI permissions, credential handling, dependency sources and asset/font/audio provenance. Do not give unverified legal assurances. Existing tool configuration must not silently authorize deployment, messaging, paid services or scope changes.

## Answer these questions explicitly

1. Is any Bevy fork justified? Given the current no-fork decision, what evidence would actually justify an exception?
2. Which abstractions are premature, and which concrete game requirement justifies each retained engine facility?
3. Which mechanics, formulas, schemas, failure paths or tasks are underspecified or contradictory?
4. Which specs/slices/tasks are too broad or too fragmented? If Stage 5 is absent, distinguish proposed decomposition from actual task review.
5. Which assumptions remain unverified, and what is the cheapest experiment capable of changing the decision?
6. What should be cut or postponed without undermining the approved core loop? Separate recommended cuts from already approved scope changes.
7. Can one developer maintain this with 20–40 hours weekly and repository-based AI assistance? What evidence would improve that judgment?
8. What most increases the probability of shipping 1.0?
9. Is the Evidence -> Decision -> GDD -> ADR/Architecture -> Spec -> Roadmap Task -> Acceptance Test chain complete? List missing links, including any actual Stage 5 coverage gap, rather than declaring full coverage from a partial index.
10. Can an implementation agent begin any task now without guessing or bypassing a gate? Explain exactly which objective readiness conditions are unmet.

## Review output and verdict

Write the review to `docs/reviews/YYYY-MM-DD-planning-review.md` using the actual review date; create that directory if needed. If writing is unavailable, return the complete document with its intended path. Include reviewed commit/state, files inspected, tools/commands actually run, sources checked and explicit verification limits.

For every finding report:

| Field | Requirement |
| --- | --- |
| ID | Stable `REV-001`, `REV-002`, etc.; do not recycle IDs on revisions. |
| Classification | Objective defect / Unverified assumption / Preference. |
| Severity | BLOCKER / CRITICAL / HIGH / MEDIUM / LOW / INFO. |
| Affected file | Exact path and section/line; multiple files for contradictions. |
| Evidence | Quote or narrowly describe actual content/behavior, with source/command when relevant. |
| Consequence | Concrete gameplay, implementation, maintenance, delivery or release impact. |
| Recommended fix | Smallest actionable correction, with alternatives if useful. |
| Blocks implementation | Yes/No, and exactly which contract/task/gate is blocked. |

Prioritize by consequence, not rhetorical strength. BLOCKER means the relevant gate cannot be passed; CRITICAL means severe integrity/security or fundamental design failure; HIGH means substantial delivery/correctness risk; MEDIUM/LOW are narrower problems; INFO is useful context. Mark uncertain findings as uncertain and explain how to verify them. Do not inflate a personal aesthetic preference into an objective blocker.

End with mandatory corrections, a revised execution order that explicitly restores any missing planning stage, and exactly one final verdict line:

`APPROVED` / `APPROVED WITH CONDITIONS` / `CHANGES REQUIRED` / `REJECTED`

State the verdict's scope immediately before that line: existing documents versus complete implementation-ready planning. Any missing or defective roadmap coverage must be reported rather than silently treated as reviewed/approved. Preparing or saving your report is not itself evidence that any mandatory correction was applied.
