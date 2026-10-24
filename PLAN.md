# PLAN — Rust 2D Engine + Arena-Survival Roguelite

**Current Stage:** Planning complete (Stages 1–6). Implementation status lives in ROADMAP.md.

**Last Updated:** 2026-10-24

Planning-stage history. Implementation status and onboarding: [ROADMAP.md](ROADMAP.md).

## Dashboard

Implementation status moved to [ROADMAP.md](ROADMAP.md): its header names the current phase and next task. This file keeps the history of planning stages 1–6.

---

## Overview

Execution plan across 6 stages as defined in `PROMPT.md`. Stage 2 was approved on 2026-10-23. On 2026-10-24 the owner approved Stages 3 and 4 (DEC-011, DEC-012), the Stage 5 roadmap and review corrections (DEC-013), GDD revision 2 (DEC-014), refreshed dependency pins (DEC-015) and engineering/release conventions (DEC-016). The independent planning review exists in [docs/reviews/](docs/reviews/2026-10-24-planning-review.md); its findings were corrected on 2026-10-24.

---

## Stage 1 — Research

**Objective:** Complete research tracks A–G in `docs/research/`.
**Gate:** Findings documented, unknowns identified, provisional strategy documented.

### Tracks (Priority Order)

- [x] **Track C — Bevy fork strategy** (documentary research; experiments remain pending)
  - Study current stable Bevy release, module layout, hot reload, testing, platform support
  - Evaluate fork vs plugins vs hybrid approach
  - Compare on maintenance, sync cost per release, API stability, build times
  - Recommend fork strategy with evidence

- [x] **Track A — Brotato gameplay model**
  - Analyze core loop, wave structure, spawning, movement, combat, stats, progression
  - Compare with similar games (Vampire Survivors, 20 Minutes Till Dawn, Halls of Torment)
  - Extract design principles and black-box gameplay experiments

- [x] **Track D — Rust practices and tooling**
  - Toolchain, MSRV policy, workspace layout, lints, testing, benchmarking, CI
  - Recommend minimal, justified toolset

- [x] **Track E — Architecture for ECS roguelite**
  - Simulation vs presentation, fixed-step simulation, RNG determinism
  - Performance risks (enemy/projectile counts, spatial partitioning, draw calls)

- [x] **Track B — REA (reverse engineering assistance)**
  - Capabilities, supported targets, OS requirements, MCP setup
  - Define permitted role under IP policy and evidence workflow

- [x] **Track F — SVG asset pipeline**
  - Compare runtime rendering, offline rasterization, hybrid approaches
  - Requirement: art style must be replaceable without touching gameplay

- [x] **Track G — Spec-Driven Development for AI agents**
  - ADRs, spec templates, acceptance tests, vertical slices, task sizing
  - Parallel work via worktrees with file ownership

### Deliverables

- [x] `docs/research/00-summary.md` — executive findings, provisional strategy
- [x] `docs/research/01-gameplay-reference.md` — Track A
- [x] `docs/research/02-rea.md` — Track B
- [x] `docs/research/03-bevy-and-fork-strategy.md` — Track C
- [x] `docs/research/04-rust-architecture-and-tooling.md` — Tracks D and E
- [x] `docs/research/05-asset-pipeline.md` — Track F
- [x] `docs/research/06-sdd-and-agent-workflow.md` — Track G
- [x] `docs/research/sources.md` — source ledger with URLs and access dates
- [x] `docs/research/open-questions.md` — unknowns and blockers

**Checkpoint:** Documentary research delivered. See the source ledger for versions and limits; no gameplay observation, REA execution or benchmark is claimed.

---

## Stage 2 — Brainstorming

**Objective:** Ask high-impact questions, record decisions.
**Gate:** Owner approves decision summary.

### Decision Areas (Priority Order)

- [x] Fork strategy (from Track C research)
- [x] Target platforms and dev OS (minimum OS/hardware explicitly deferred)
- [x] Owner's Rust/Bevy experience and available time
- [x] 1.0 scope (content counts)
- [x] Original identity and differentiator; final name explicitly deferred
- [x] Core direction (detailed stat/rule proposals belong to Stage 3)
- [x] Art direction (SVG style)
- [x] Audio/music production default
- [x] Controller/accessibility detailed commitment deferred to GDD review
- [x] Repository default; visibility/license explicitly deferred
- [x] Commercial/personal release explicitly deferred
- [x] No paid-service prerequisite; budget ceiling explicitly deferred

### Deliverables

- [x] `docs/decisions/decision-register.md` — decisions and owner approval recorded
- [x] `docs/decisions/scope-and-non-goals.md` — approved scope and explicit deferrals
- [x] Owner-facing summary approved on 2026-10-23; canonical approval evidence is in the decision register's Stage 2 boundary section, avoiding a duplicate summary file

---

## Stage 3 — GDD and Art Bible

**Objective:** Design implementable game specification.
**Gate:** Owner approves both documents.

### Deliverables

- [x] `docs/gdd.md` — draft rules, formulas/examples, complete content catalogs/schemas, state machine, UI/accessibility, persistence and release scope split
- [x] `docs/art-bible.md` — draft visuals, SVG pipeline/catalog, art replacement path, original/licensed audio policy and production acceptance
- [x] Owner explicitly approved both Stage 3 documents on 2026-10-24

**Consistency Check:** GDD-020 records actual catalog/reference/arithmetic/link checks and manual geometry/timing/UI review. Passed documentary checks do not establish runtime readability or balance.

---

## Stage 4 — Architecture and SDD

**Objective:** Spec system ready for implementation agents.
**Gate:** Owner approves architecture, ADRs, and specs.

### Deliverables

- [x] **Repository strategy** — two-crate workspace, upstream dependency and scoped patch policy in architecture.md
- [x] **Module map** — responsibilities, state/interfaces/events, failures, tests and performance notes
- [x] **Fixed-step simulation and RNG** — ordered tick, separate streams and limited repeatability claims
- [x] **Performance plan** — scenarios/metrics and verified development-hardware inventory; no measured budgets
- [x] **Testing layers** — scoped unit/integration/invariant/headless/native/regression checks; benchmarks separate
- [x] `docs/architecture/adr/` — eleven approved ADRs, including simulation/RNG
- [x] `docs/specs/_template.md` — full authoring and evidence requirements
- [x] `docs/specs/index.md` — eighteen approved specs, dependency graph and GDD/Art requirement coverage
- [x] `AGENTS.md` + thin `CLAUDE.md` — canonical model-independent workflow and explicit file ownership
- [x] `docs/validation/definition-of-done.md` — evidence-based completion
- [x] `docs/validation/quality-gates.md` — commands/acceptance layers and approval boundaries
- [x] Owner blanket approval recorded in DEC-012; no repeated approval request needed

**Gate:** Contracts define what an implementation task must build, exclude and validate. Specs are APPROVED; actual READY tasks require Stage 5 assignment, exact ownership and satisfied dependencies. The documentary audit result is recorded in quality-gates.md (Stage 4 and 5 documentary record).

---

## Stage 5 — Roadmap Finalization

**Objective:** Executable, dependency-aware plan in 5 phases. **Deliverable:** [ROADMAP.md](ROADMAP.md) (named `PLAN.md` in PROMPT.md section 8; the repository keeps stages here and tasks there). Approved under DEC-013.
**Gate:** Internally consistent; next tasks are READY.

### Five Phases

1. **Foundation** — workspace, toolchain, Bevy setup, CI, engine/game bootstrap, SVG proof of concept, performance baseline
2. **Prototype** — minimal playable loop (player, movement, arena, spawning, enemy, auto-attack, damage, waves, shop, HUD, headless tests)
3. **Alpha** — all approved core systems, save/load, menus, engine extensions
4. **Beta** — balancing, polish, profiling, controller, accessibility, packaging, long-session stability
5. **RC → 1.0** — feature freeze, regression, license/asset audit, dependency review, release notes

### Task Structure

- [x] Phase hierarchy: Phase → Milestone → Task with stable IDs (`ENG-P1-001`, `GAME-P2-004`)
- [x] Each task: title, status, dependencies, spec reference, complexity, acceptance criteria, verification
- [x] Task sizing: fits in one reviewable change
- [x] Dashboard: current phase/milestone, next READY tasks, blockers, latest verified checkpoint
- [x] Phase entry/exit criteria measurable and checkable
- [x] Circular dependencies checked and resolved
- [x] Realistic for one developer with AI agents

---

## Stage 6 — Review Handoff

**Objective:** Self-contained prompt for adversarial principal reviewer.
**Gate:** Self-contained review handoff prepared; actual independent review is a separate event and must not be claimed before it occurs.

### Deliverable

- [x] `PROJECT-REVIEW.md` — self-contained prompt for Claude Fable 5.1; the actual review was performed on 2026-10-24 and recorded in `docs/reviews/2026-10-24-planning-review.md` (verdict CHANGES REQUIRED; all findings addressed under DEC-013)
  - Research quality, game design, Bevy architecture, fork sustainability, specs, roadmap realism
  - Developer experience, security, licensing, IP
  - Verdict: APPROVED / APPROVED WITH CONDITIONS / CHANGES REQUIRED / REJECTED

---

## Consistency Audit

Before declaring planning complete:

- [x] Traceability: Evidence → Decision → Requirement (GDD) → ADR/Architecture → Spec → Roadmap task → Acceptance test
- [x] Flag any requirement with no task
- [x] Flag any task with no requirement
- [x] Flag any decision with no rationale
- [x] Flag any contradiction between documents

---

## Execution notes

- Commit per reviewable task with Conventional Commits; stage exact paths, never blanket-add.
- Keep `git status` clean before each session; record decisions in `docs/decisions/`.
- Update this dashboard when a task changes status.

**Branch:** `main`.

---

**Last Checkpoint:** 2026-10-24 — planning review corrections applied; Phase 1 Foundation closed (release 0.3.0). Continue in ROADMAP.md.
