# SP-004 — Reusable shape and sweep queries

Status: **APPROVED**, by owner blanket approval on 2026-10-24. Dependencies: [SP-001](sp-001-workspace.md)

Requirements: [GDD / design acceptance](../gdd.md): GDD-004, GDD-005, GDD-006; DA-07. Art references resolve in the [Art Bible](../art-bible.md). ADRs: [ADR-004](../architecture/adr/004-collision.md). Technical contracts: [architecture](../architecture/architecture.md). Origin: original design implementing the approved Stage 3 baseline; no implementation evidence yet.

## Problem and outcome

Supply the exact primitive collision semantics required by the game without coupling geometry to game rules.

## Requirements and exclusions

Implement the referenced rules and the acceptance cases below; do not reinterpret numeric P values as permission to rebalance during implementation. Exclusions: No teams, HP, damage, spatial index, rigid body solver, pooling or sprite-derived geometry.

## Technical design and data flow

Implement ENG-GEOM-001 with pure f64 LU queries and typed validation errors. Circle/capsule overlap includes tangency; swept circle returns first TOI in [0,1], including t=0 overlap and zero motion. Arena clamping accounts for radius. No ECS systems/resources are needed for these pure queries; game systems supply ordered candidates and consume hit results. Expose immutable shapes and result types only.

Failure handling: Use a documented tolerance only for numeric comparisons; do not inflate player hurtboxes to hide numerical errors.

## Testable acceptance criteria

1. **SP-004-AC1:** Given tangent circles/capsules, then overlap is true; separation beyond the boundary is false.
2. **SP-004-AC2:** Given a fast sweep crossing a target between endpoints, then report its first impact rather than missing it.
3. **SP-004-AC3:** Given zero-length motion or equal centers, then return a finite documented result, never NaN.
4. **SP-004-AC4:** Given generated translations/swaps of valid shape pairs, then overlap invariants hold within the declared numeric tolerance.

## Test cases and commands

Each criterion requires a named test or documented native procedure with expected/actual results. Exercise normal, rejected and boundary paths; test scaffolding that only mirrors code is insufficient. Required commands **after implementation** (not runnable or executed during planning):

```powershell
cargo test -p omega_2d --no-default-features --locked --test geometry_contract
```

Also run the applicable shared checks in [quality gates](../validation/quality-gates.md). Native visual/audio/controller evidence is required where named, even if unit tests pass. Benchmarks never become correctness thresholds without an approved measured budget.

## Allowed and forbidden paths

Allowed scope: crates/omega_2d/src/geometry/, crates/omega_2d/tests/geometry_contract.rs. Task assignment must narrow this to explicit files/globs before READY. Shared plugin registration, root manifests, lockfile and central catalogs may be changed only by the serial integrator under an explicitly assigned task; do not infer shared-file ownership from a module dependency.

Forbidden: unrelated modules, Bevy internals, proprietary assets, other workers' files, user setup/Node files, and approval/status changes that claim unperformed validation. Specs, GDD, ADRs and PLAN.md are serially owned; request a change instead of silently editing contracts while implementing.

## Reviewable handoff slices

Shapes/validation and overlap; swept collision; arena bounds and invariant fixtures. These are prospective decomposition boundaries, not assigned Stage 5 task IDs. No single task may silently encompass the entire spec when several slices are listed.

## Definition of Done and completion evidence

Apply [Definition of Done](../validation/definition-of-done.md). All criteria need evidence, tests must cover confirmed bugs, and no critical decision may remain open. Dependencies must provide their tested contracts; source existence alone is not dependency completion.

**Completion evidence (2026-10-24):** implemented by ENG-P1-002; commands, criterion-to-test mapping, environment and limits are in the [evidence index](../validation/evidence/README.md#phase-1-foundation--2026-10-24). Reviewer outcome: integrator self-review only.
