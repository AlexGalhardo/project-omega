---
name: project-omega
description: Project Omega context map and session-start routine. Use at the start of every session in this repository and whenever a task needs to know where the architecture, specs (SDD), GDD, Art Bible, roadmap, engine fork, commands or quality gates live. Also use before applying any generic skill here, to adapt it to this project's contracts.
---

# Project Omega

Rust 2024 / Bevy 0.20 arena-survival game plus the project's own 2D engine. Didactic project with two primary objectives (DEC-017): re-create Brotato from REA study of the owner's copy, and build an engine as a vendored Bevy fork. [AGENTS.md](../../../AGENTS.md) is the canonical contract; this skill only routes.

## Session start (every session, in order)

1. `/caveman` (if the SessionStart hook did not already announce it).
2. Load the `karpathy-guidelines` skill.
3. graphify: if `graphify-out/graph.json` is missing or stale, run `graphify update .` (local AST, no LLM); answer codebase questions with `graphify query "<question>"` before opening files.
4. Read the [ROADMAP.md](../../../ROADMAP.md) header (current phase, next task) and section 1 (plan rules), then the task and its spec.

## Where things live

| Need | Path |
| --- | --- |
| Onboarding: phase, next task, rules, checklists | `ROADMAP.md` |
| History of planning stages 1–6 | `PLAN.md` |
| Game rules, formulas, content catalogs (canonical) | `docs/gdd.md` (revision 2) |
| Visual/audio rules, SVG pipeline, catalog fields | `docs/art-bible.md` |
| Architecture, crate boundaries, pins | `docs/architecture/architecture.md`, `docs/architecture/adr/` |
| Specs (SDD) and requirement coverage | `docs/specs/index.md`, `docs/specs/sp-0NN-*.md` |
| Decisions and change requests | `docs/decisions/decision-register.md`, `docs/decisions/change-requests/` |
| Gates and Definition of Done | `docs/validation/quality-gates.md`, `docs/validation/definition-of-done.md` |
| Task evidence | `docs/validation/evidence/<TASK-ID>/` |
| Vendored Bevy fork and patch log | `engine/bevy/`, `engine/README.md` |
| Reusable 2D engine layer | `crates/omega_2d` (geometry, camera, visuals) |
| Game rules, content, sim, diagnostics, client | `crates/omega_game` |
| Content fixtures (full and prototype) | `crates/omega_game/tests/fixtures/content/` |
| Proof art, export manifest, catalogs | `art/source/svg/proof/`, `assets/presentation/`, `assets/generated/` |
| Brotato REA notes (prose only) | `docs/research/brotato-notes/` (DEC-019) |

## Commands

```powershell
powershell -NoProfile -File scripts/check.ps1                   # every local gate, CI order
cargo test --workspace --no-default-features --locked            # headless core
cargo check -p omega_game --features client --locked              # client compile
cargo run -p omega_game --locked --bin omega-game                 # native preview
cargo run -p omega_game --no-default-features --bin omega-content -- validate PATH --profile full
cargo run -p omega_game --no-default-features --bin omega-sim -- --scenario smoke --seed <64 hex> --ticks 600
powershell -NoProfile -File scripts/export-assets.ps1 -Manifest assets/presentation/proof-manifest.json
powershell -NoProfile -File scripts/validate-assets.ps1 -Manifest assets/presentation/proof-manifest.json -Reproduce
```

## Rules generic skills must respect here

- Specs are contracts. Implement the assigned slice only; a contract flaw becomes a change request, never a silent redesign.
- Simulation is authoritative and headless: no wall clock, no renderer reads, no ECS/hash iteration order. Order by `SimId`/slot, use named RNG streams (`sim::RngStreams`), integers for damage/economy (CR-002).
- Presentation never feeds gameplay: no sprite sizes, animation completion or audio completion in rules.
- Content is strict JSON through `content::reader` with field paths; no scripts, no escape hatches.
- Engine changes go into `engine/bevy/` with a patch-log entry and a regression test; reusable 2D code goes into `omega_2d`; rules never go into either.
- Tabs (width 4) in project code; upstream style inside `engine/bevy/`.
- No proprietary Brotato material in the repository (public repo). REA output stays outside it.
- Evidence is real: name the command, exit code and evidence file. Never claim a check that did not run.
- Commits: Conventional Commits; CHANGELOG.md in Keep a Changelog with `- **scope:** Message ([sha](url))` entries; releases per `docs/release.md`.
