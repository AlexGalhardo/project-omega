# SP-012 — Logical camera and visual presentation contracts

Status: **APPROVED**, by owner blanket approval on 2026-10-24. Dependencies: [SP-001](sp-001-workspace.md). The SP-004 dependency was removed on 2026-10-24 (REV-015): camera and catalog use no geometry query.

Requirements: [GDD / design acceptance](../gdd.md): GDD-004, GDD-016; ART-004 through ART-007, ART-009, ART-010; DA-20. Art references resolve in the [Art Bible](../art-bible.md). ADRs: [ADR-005](../architecture/adr/005-assets.md), [ADR-008](../architecture/adr/008-input.md). Technical contracts: [architecture](../architecture/architecture.md). Origin: original design implementing the approved Stage 3 baseline; no implementation evidence yet.

## Problem and outcome

Render the logical world and map input without letting art dimensions define gameplay.

## Requirements and exclusions

Implement the referenced rules and the acceptance cases below; do not reinterpret numeric P values as permission to rebalance during implementation. Exclusions: No SVG parser in runtime, sprite-derived collision, dynamic camera gameplay or atlas optimization without evidence.

## Technical design and data flow

Implement ENG-CAM-001 pure viewport mapping plus optional client camera integration, and ENG-VIS-001 typed metadata/lookup/projectors. Client-only visual entities map SimId/VisualId/pose/phase facts to sprites. The engine does not interpret weapon/enemy IDs or game phases; phase progress is generic input. Catalog validation owns extents/pivots/frame bounds and semantic layers; absent essential cue reports a typed presentation error to the game.

Failure handling: Rendering failures cannot silently hide a hostile projectile or mutate simulation state.

## Testable acceptance criteria

1. **SP-012-AC1:** Given center/corner world points at supported display sizes, then screen/world round trips preserve positions and bars reject aim.
2. **SP-012-AC2:** Given a 160-pixel player source replaced by another resolution with the same logical metadata, then world extent/pivot are unchanged.
3. **SP-012-AC3:** Given invalid extent/frame/fallback cycle, then reject catalog; missing cosmetic data renders a visible category fallback.
4. **SP-012-AC4:** Given phase updates/cancellation, then cues match authoritative shape/origin and cannot delay attack emission.

## Test cases and commands

Each criterion requires a named test or documented native procedure with expected/actual results. Exercise normal, rejected and boundary paths; test scaffolding that only mirrors code is insufficient. Required commands **after implementation** (not runnable or executed during planning):

```powershell
cargo test -p omega_2d --no-default-features --locked --test camera_contract
cargo test -p omega_2d --features client --locked --test visual_contract
```

Also run the applicable shared checks in [quality gates](../validation/quality-gates.md). Native visual/audio/controller evidence is required where named, even if unit tests pass. Benchmarks never become correctness thresholds without an approved measured budget.

## Allowed and forbidden paths

Allowed scope: crates/omega_2d/src/camera/, crates/omega_2d/src/visuals/, crates/omega_2d/tests/camera_contract.rs, crates/omega_2d/tests/visual_contract.rs. Task assignment must narrow this to explicit files/globs before READY. Shared plugin registration, root manifests, lockfile and central catalogs may be changed only by the serial integrator under an explicitly assigned task; do not infer shared-file ownership from a module dependency.

Forbidden: unrelated modules, Bevy internals, proprietary assets, other workers' files, user setup/Node files, and approval/status changes that claim unperformed validation. Specs, GDD, ADRs and PLAN.md are serially owned; request a change instead of silently editing contracts while implementing.

## Reviewable handoff slices

Pure viewport transform; typed visual catalog; sprite projection/phase cues; fallback/skin swap. These are prospective decomposition boundaries, not assigned Stage 5 task IDs. No single task may silently encompass the entire spec when several slices are listed.

## Definition of Done and completion evidence

Apply [Definition of Done](../validation/definition-of-done.md). All criteria need evidence, tests must cover confirmed bugs, and no critical decision may remain open. Dependencies must provide their tested contracts; source existence alone is not dependency completion.

**Completion evidence (2026-10-24):** implemented by ENG-P1-003, ENG-P1-004 (AC4 phase cues remain for ENG-P3-001); commands, criterion-to-test mapping, environment and limits are in the [evidence index](../validation/evidence/README.md#phase-1-foundation--2026-10-24). Reviewer outcome: integrator self-review only.
