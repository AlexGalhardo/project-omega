# SP-003 — Typed content schemas and validation

Status: **APPROVED**, by owner blanket approval on 2026-10-24. Dependencies: [SP-001](sp-001-workspace.md)

Requirements: [GDD / design acceptance](../gdd.md): GDD-009 through GDD-013, GDD-019. Art references resolve in the [Art Bible](../art-bible.md). ADRs: [ADR-006](../architecture/adr/006-content.md). Technical contracts: [architecture](../architecture/architecture.md). Origin: original design implementing the approved Stage 3 baseline; no implementation evidence yet.

## Problem and outcome

Reject invalid content before it can alter a run or silently produce incomplete catalogs.

## Requirements and exclusions

Implement the referenced rules and the acceptance cases below; do not reinterpret numeric P values as permission to rebalance during implementation. Exclusions: No scripting, runtime mods, UI content editors or production catalog authoring (SP-018).

## Technical design and data flow

Serde JSON records follow GDD-012; closed payload/behavior enums and typed IDs. Load manifest then all referenced catalogs into a candidate snapshot; collect record/field errors; publish only a fully valid snapshot. ContentSnapshot owns revision hash and stable ordered lookup tables. CLI: omega-content validate PATH --profile prototype|full, exits zero only on complete validation. Full profile enforces 4/12/10/2/40/10/4 catalog counts; prototype accepts exactly GDD-017 subset and three ordinary waves. Runtime packaging chooses its profile explicitly; full release cannot use prototype mode. Hot reload is Menu-only and atomically swaps complete valid snapshots.

Failure handling: A new mechanic not expressible by approved enums needs a design change request, not an arbitrary JSON escape hatch.

## Testable acceptance criteria

1. **SP-003-AC1:** Given each invalid fixture for duplicate ID, unknown field, dangling reference, NaN-like text or unsupported enum, then reject with record/field path.
2. **SP-003-AC2:** Given full mode with one missing passive item, then fail the count check; prototype content cannot pass full mode.
3. **SP-003-AC3:** Given an invalid Menu reload, then preserve the previous valid snapshot; an active run never changes revision.
4. **SP-003-AC4:** Given a valid original schema fixture, then all referenced unlocks and starting weapons are reachable and weights are positive.

## Test cases and commands

Each criterion requires a named test or documented native procedure with expected/actual results. Exercise normal, rejected and boundary paths; test scaffolding that only mirrors code is insufficient. Required commands **after implementation** (not runnable or executed during planning):

```powershell
cargo test -p omega_game --no-default-features --locked --test content_contract
```

Also run the applicable shared checks in [quality gates](../validation/quality-gates.md). Native visual/audio/controller evidence is required where named, even if unit tests pass. Benchmarks never become correctness thresholds without an approved measured budget.

## Allowed and forbidden paths

Allowed scope: crates/omega_game/src/content/, crates/omega_game/src/bin/omega-content.rs, crates/omega_game/tests/content_contract.rs, crates/omega_game/tests/fixtures/content/. Task assignment must narrow this to explicit files/globs before READY. Shared plugin registration, root manifests, lockfile and central catalogs may be changed only by the serial integrator under an explicitly assigned task; do not infer shared-file ownership from a module dependency.

Forbidden: unrelated modules, Bevy internals, proprietary assets, other workers' files, user setup/Node files, and approval/status changes that claim unperformed validation. Specs, GDD, ADRs and PLAN.md are serially owned; request a change instead of silently editing contracts while implementing.

## Reviewable handoff slices

Schemas/errors; cross-reference validation; CLI/profile enforcement; immutable snapshot/reload. These are prospective decomposition boundaries, not assigned Stage 5 task IDs. No single task may silently encompass the entire spec when several slices are listed.

## Definition of Done and completion evidence

Apply [Definition of Done](../validation/definition-of-done.md). All criteria need evidence, tests must cover confirmed bugs, and no critical decision may remain open. Dependencies must provide their tested contracts; source existence alone is not dependency completion.

**Completion evidence (2026-10-24):** implemented by GAME-P1-002, GAME-P1-003; commands, criterion-to-test mapping, environment and limits are in the [evidence index](../validation/evidence/README.md#phase-1-foundation--2026-10-24). Reviewer outcome: integrator self-review only.
