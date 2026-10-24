# PLAN — Rust 2D Engine + Arena-Survival Roguelite

**Current Stage:** 1 (Research)  
**Last Updated:** 2026-10-23

---

## Overview

Execution plan across 6 stages as defined in `PROMPT.md`. Each stage has approval gates. Do not skip stages or start implementation before Stage 4 approval.

---

## Stage 1 — Research

**Objective:** Complete research tracks A–G in `docs/research/`.  
**Gate:** Findings documented, unknowns identified, provisional strategy documented.

### Tracks (Priority Order)

- [ ] **Track C — Bevy fork strategy** (highest priority: blocks all architecture decisions)
  - Study current stable Bevy release, module layout, hot reload, testing, platform support
  - Evaluate fork vs plugins vs hybrid approach
  - Compare on maintenance, sync cost per release, API stability, build times
  - Recommend fork strategy with evidence

- [ ] **Track A — Brotato gameplay model**
  - Analyze core loop, wave structure, spawning, movement, combat, stats, progression
  - Compare with similar games (Vampire Survivors, 20 Minutes Till Dawn, Halls of Torment)
  - Extract design principles and black-box gameplay experiments

- [ ] **Track D — Rust practices and tooling**
  - Toolchain, MSRV policy, workspace layout, lints, testing, benchmarking, CI
  - Recommend minimal, justified toolset

- [ ] **Track E — Architecture for ECS roguelite**
  - Simulation vs presentation, fixed-step simulation, RNG determinism
  - Performance risks (enemy/projectile counts, spatial partitioning, draw calls)

- [ ] **Track B — REA (reverse engineering assistance)**
  - Capabilities, supported targets, OS requirements, MCP setup
  - Define permitted role under IP policy and evidence workflow

- [ ] **Track F — SVG asset pipeline**
  - Compare runtime rendering, offline rasterization, hybrid approaches
  - Requirement: art style must be replaceable without touching gameplay

- [ ] **Track G — Spec-Driven Development for AI agents**
  - ADRs, spec templates, acceptance tests, vertical slices, task sizing
  - Parallel work via worktrees with file ownership

### Deliverables

- [ ] `docs/research/00-summary.md` — executive findings, provisional strategy
- [ ] `docs/research/01-gameplay-reference.md` — Track A
- [ ] `docs/research/02-rea.md` — Track B
- [ ] `docs/research/03-bevy-and-fork-strategy.md` — Track C
- [ ] `docs/research/04-rust-architecture-and-tooling.md` — Tracks D and E
- [ ] `docs/research/05-asset-pipeline.md` — Track F
- [ ] `docs/research/06-sdd-and-agent-workflow.md` — Track G
- [ ] `docs/research/sources.md` — source ledger with URLs and access dates
- [ ] `docs/research/open-questions.md` — unknowns and blockers

**Next Step:** Present findings and ask owner for Stage 2 approval.

---

## Stage 2 — Brainstorming

**Objective:** Ask high-impact questions, record decisions.  
**Gate:** Owner approves decision summary.

### Decision Areas (Priority Order)

- [ ] Fork strategy (from Track C research)
- [ ] Target platforms and dev OS
- [ ] Owner's Rust/Bevy experience and available time
- [ ] 1.0 scope (content counts)
- [ ] Original name, identity, and differentiators from Brotato
- [ ] Core mechanics (aiming, weapon slots, stats, shop, meta-progression)
- [ ] Art direction (SVG style)
- [ ] Audio and music approach
- [ ] Controller and accessibility
- [ ] Repository structure and visibility
- [ ] Commercial vs personal release
- [ ] Budget and paid tools

### Deliverables

- [ ] `docs/decisions/decision-register.md` — all decisions with ID, rationale, status
- [ ] `docs/decisions/scope-and-non-goals.md` — required 1.0 / desired / post-1.0 / non-goal lists
- [ ] `docs/decisions/approval-summary.md` — owner-facing summary of all decisions

---

## Stage 3 — GDD and Art Bible

**Objective:** Design implementable game specification.  
**Gate:** Owner approves both documents.

### Deliverables

- [ ] `docs/gdd.md` — concept, pillars, gameplay loop, combat, stats with formulas, content schemas, state machine, UI/HUD, onboarding, scope split (Prototype → Alpha → Beta → RC → 1.0 → post-launch)
- [ ] `docs/art-bible.md` — visual pillars, palette, shape language, silhouettes, animation, VFX, SVG pipeline, future pixel-art path, placeholder audio strategy

**Consistency Check:** No contradictions between GDD and art bible (hitboxes, timing, UI match).

---

## Stage 4 — Architecture and SDD

**Objective:** Spec system ready for implementation agents.  
**Gate:** Owner approves architecture, ADRs, and specs.

### Deliverables

- [ ] **Repository strategy** — single repo vs separate engine/game, workspace layout, Git dependency pinning, `[patch]` overrides
- [ ] **Module map** — responsibility, inputs/outputs, owned data, public interface, events, failure modes, test approach, performance notes for each crate
- [ ] **Fixed-step simulation and RNG** — determinism guarantees, seeding strategy
- [ ] **Performance plan** — benchmark scenarios, metrics (P95/P99 frame time, allocations, entity counts, draw calls), target hardware
- [ ] **Testing layers** — unit, integration, headless seeded simulations, property tests, regression tests, benchmarks
- [ ] `docs/architecture/adr/` — ADRs for fork, toolchain/MSRV, workspace layout, collision/physics, asset pipeline, content format, save format, input, CI, agent workflow
- [ ] `docs/specs/_template.md` — standard spec format (ID, status, problem, requirements, technical design, acceptance criteria, tests, DoD)
- [ ] `docs/specs/index.md` — all specs and their dependencies
- [ ] `AGENTS.md` + thin model-specific adapters (`CLAUDE.md`) — agent operating contract, task IDs, spec references, allowed paths, test requirements, report format
- [ ] `docs/validation/definition-of-done.md` — what "done" means
- [ ] `docs/validation/quality-gates.md` — how to verify completion

**Gate:** Implementation agent can pick a READY task and know exactly what to build.

---

## Stage 5 — Roadmap Finalization

**Objective:** Executable, dependency-aware plan in 5 phases.  
**Gate:** Internally consistent; next tasks are READY.

### Five Phases

1. **Foundation** — workspace, toolchain, Bevy setup, CI, engine/game bootstrap, SVG proof of concept, performance baseline
2. **Prototype** — minimal playable loop (player, movement, arena, spawning, enemy, auto-attack, damage, waves, shop, HUD, headless tests)
3. **Alpha** — all approved core systems, save/load, menus, engine extensions
4. **Beta** — balancing, polish, profiling, controller, accessibility, packaging, long-session stability
5. **RC → 1.0** — feature freeze, regression, license/asset audit, dependency review, release notes

### Task Structure

- [ ] Phase hierarchy: Phase → Milestone → Task with stable IDs (`ENG-P1-001`, `GAME-P2-004`)
- [ ] Each task: title, status, dependencies, spec reference, complexity, acceptance criteria, verification
- [ ] Task sizing: fits in one reviewable change
- [ ] Dashboard: current phase/milestone, next READY tasks, blockers, latest verified checkpoint
- [ ] Phase entry/exit criteria measurable and checkable
- [ ] Circular dependencies checked and resolved
- [ ] Realistic for one developer with AI agents

---

## Stage 6 — Review Handoff

**Objective:** Self-contained prompt for adversarial principal reviewer.  
**Gate:** Review completed and reported.

### Deliverable

- [ ] `docs/reviews/` — reviewer prompt and findings from Claude Fable 5.1
  - Research quality, game design, Bevy architecture, fork sustainability, specs, roadmap realism
  - Developer experience, security, licensing, IP
  - Verdict: APPROVED / APPROVED WITH CONDITIONS / CHANGES REQUIRED / REJECTED

---

## Consistency Audit

Before declaring planning complete:

- [ ] Traceability: Evidence → Decision → Requirement (GDD) → ADR/Architecture → Spec → Roadmap task → Acceptance test
- [ ] Flag any requirement with no task
- [ ] Flag any task with no requirement
- [ ] Flag any decision with no rationale
- [ ] Flag any contradiction between documents

---

## Overnight Execution Notes

- **Checkpoint after each stage** and commit with message: `docs(stage-X): checkpoint after [task]`
- **Keep working directory clean** — `git status` before each session
- **Record all decisions** in `docs/decisions/` with links
- **Link specs to GDD** — don't duplicate information
- **Update this file** as progress happens

**Branch:** `main` (can switch to `overnight/research` if long session)

---

**Last Checkpoint:** 2026-10-23 — Setup complete, ready for Stage 1.
