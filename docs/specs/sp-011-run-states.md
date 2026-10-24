# SP-011 — Run state machine and result settlement

Status: **APPROVED**, by owner blanket approval on 2026-10-24. Dependencies: [SP-005](sp-005-input.md), [SP-008](sp-008-encounters.md), [SP-009](sp-009-shop.md), [SP-010](sp-010-progression-save.md)

Requirements: [GDD / design acceptance](../gdd.md): GDD-002, GDD-003, GDD-014; DA-01, DA-03, DA-11 through DA-16. Art references resolve in the [Art Bible](../art-bible.md). ADRs: [ADR-011](../architecture/adr/011-simulation-rng.md), [ADR-007](../architecture/adr/007-saves.md). Technical contracts: [architecture](../architecture/architecture.md). Origin: original design implementing the approved Stage 3 baseline; no implementation evidence yet.

## Problem and outcome

Coordinate combat, shop and progression through guarded transitions instead of scattered side effects.

## Requirements and exclusions

Implement the referenced rules and the acceptance cases below; do not reinterpret numeric P values as permission to rebalance during implementation. Exclusions: No new root state, silent restarts or run autosave.

## Technical design and data flow

RunState and transition intents are resolved in Transition after damage/collection. Process death before wave completion; settlement ID prevents duplicate summary/counters. Entry/exit hooks reset only GDD-specified transient state, collect remaining earned pickups on success and snapshot results. Shop guards validate powered loadout. Pause/settings are overlays preserving previous screen; Boot content/save errors route to actionable error states.

Failure handling: Do not clear profile state when resetting a run; failed persistence stays visible in results.

## Testable acceptance criteria

1. **SP-011-AC1:** Given ten valid completed waves, then nine shops occur and final victory bypasses a tenth shop.
2. **SP-011-AC2:** Given player/boss simultaneous death, then settle Defeat once and grant no wave-completion challenge.
3. **SP-011-AC3:** Given entering a new wave, then HP is full, transient projectiles/timers cleared and build/currency/energy retained.
4. **SP-011-AC4:** Given Abandon cancel, then continue prior state; confirm yields Abandoned and retains already saved grants.
5. **SP-011-AC5:** Given invalid content at Boot, then Start cannot become available.

## Test cases and commands

Each criterion requires a named test or documented native procedure with expected/actual results. Exercise normal, rejected and boundary paths; test scaffolding that only mirrors code is insufficient. Required commands **after implementation** (not runnable or executed during planning):

```powershell
cargo test -p omega_game --no-default-features --locked --test run_contract
```

Also run the applicable shared checks in [quality gates](../validation/quality-gates.md). Native visual/audio/controller evidence is required where named, even if unit tests pass. Benchmarks never become correctness thresholds without an approved measured budget.

## Allowed and forbidden paths

Allowed scope: crates/omega_game/src/sim/transitions.rs, crates/omega_game/src/sim/run.rs, crates/omega_game/tests/run_contract.rs, run fixtures. Task assignment must narrow this to explicit files/globs before READY. Shared plugin registration, root manifests, lockfile and central catalogs may be changed only by the serial integrator under an explicitly assigned task; do not infer shared-file ownership from a module dependency.

Forbidden: unrelated modules, Bevy internals, proprietary assets, other workers' files, user setup/Node files, and approval/status changes that claim unperformed validation. Specs, GDD, ADRs and PLAN.md are serially owned; request a change instead of silently editing contracts while implementing.

## Reviewable handoff slices

Root transitions/entry cleanup; pause/settings overlays; terminal results; progression/save integration. These are prospective decomposition boundaries, not assigned Stage 5 task IDs. No single task may silently encompass the entire spec when several slices are listed.

## Definition of Done and completion evidence

Apply [Definition of Done](../validation/definition-of-done.md). All criteria need evidence, tests must cover confirmed bugs, and no critical decision may remain open. Dependencies must provide their tested contracts; source existence alone is not dependency completion.

**Completion evidence: NOT IMPLEMENTED.** No commands above have run, no tests/results/benchmarks are claimed, and no task is READY. The implementation report must record commit, environment, criterion-to-test mapping, exact commands/results, native evidence where needed, limitations and reviewer outcome. This section is filled with actual evidence as the lifecycle advances.
