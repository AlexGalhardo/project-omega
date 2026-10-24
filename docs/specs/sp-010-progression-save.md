# SP-010 — Unlock progression and recoverable local saves

Status: **APPROVED**, by owner blanket approval on 2026-10-24. Dependencies: [SP-003](sp-003-content.md)

Requirements: [GDD / design acceptance](../gdd.md): GDD-013, GDD-014; DA-14, DA-15. Art references resolve in the [Art Bible](../art-bible.md). ADRs: [ADR-007](../architecture/adr/007-saves.md), [ADR-006](../architecture/adr/006-content.md). Technical contracts: [architecture](../architecture/architecture.md). Origin: original design implementing the approved Stage 3 baseline; no implementation evidence yet.

## Problem and outcome

Persist earned horizontal unlocks without losing the previous readable profile on failed writes.

## Requirements and exclusions

Implement the referenced rules and the acceptance cases below; do not reinterpret numeric P values as permission to rebalance during implementation. Exclusions: No active-run resume, cloud, encryption, accounts or lifetime power.

## Technical design and data flow

Profile grant IDs are authority; availability derives from initial pool plus grants. Completion facts feed an idempotent evaluator; SaveRequest carries generation and snapshot. Persistence implements two-slot JSON envelopes and checked SHA-256, temp writes, sync/reopen validation and same-directory rename publication. A serial writer rejects stale generations. Load chooses highest valid generation and exposes recovery status. Store paths and filesystem faults are injectable. Settings persist separately from profile; no ECS serialization.

Failure handling: Unwritable LOCALAPPDATA produces Unsaved/Retry; do not fall back to installation directory or claim durability from serialization success.

## Testable acceptance criteria

1. **SP-010-AC1:** Given successful first wave-3 completion, then grant exactly the GDD bundle once and preserve current run pool.
2. **SP-010-AC2:** Given duplicate grant/result requests, then no duplicate reward/counter/write ordering occurs.
3. **SP-010-AC3:** Given interrupted writes at each protocol boundary, then load a valid current/previous generation or present recovery; never silently create a fresh profile over corrupt bytes.
4. **SP-010-AC4:** Given future schema or invalid hash, then preserve source files and require explicit recovery/reset.
5. **SP-010-AC5:** Given Reset Profile, then settings survive unless separately reset.

## Test cases and commands

Each criterion requires a named test or documented native procedure with expected/actual results. Exercise normal, rejected and boundary paths; test scaffolding that only mirrors code is insufficient. Required commands **after implementation** (not runnable or executed during planning):

```powershell
cargo test -p omega_game --no-default-features --locked --test persistence_contract
```

Also run the applicable shared checks in [quality gates](../validation/quality-gates.md). Native visual/audio/controller evidence is required where named, even if unit tests pass. Benchmarks never become correctness thresholds without an approved measured budget.

## Allowed and forbidden paths

Allowed scope: crates/omega_game/src/progression/, crates/omega_game/src/persistence/, crates/omega_game/tests/persistence_contract.rs, profile/settings fixtures. Task assignment must narrow this to explicit files/globs before READY. Shared plugin registration, root manifests, lockfile and central catalogs may be changed only by the serial integrator under an explicitly assigned task; do not infer shared-file ownership from a module dependency.

Forbidden: unrelated modules, Bevy internals, proprietary assets, other workers' files, user setup/Node files, and approval/status changes that claim unperformed validation. Specs, GDD, ADRs and PLAN.md are serially owned; request a change instead of silently editing contracts while implementing.

## Reviewable handoff slices

Grant evaluator; typed profile/settings; save protocol/fault injection; native Windows interruption/recovery checks. These are prospective decomposition boundaries, not assigned Stage 5 task IDs. No single task may silently encompass the entire spec when several slices are listed.

## Definition of Done and completion evidence

Apply [Definition of Done](../validation/definition-of-done.md). All criteria need evidence, tests must cover confirmed bugs, and no critical decision may remain open. Dependencies must provide their tested contracts; source existence alone is not dependency completion.

**Completion evidence: NOT IMPLEMENTED.** No commands above have run, no tests/results/benchmarks are claimed, and no task is READY. The implementation report must record commit, environment, criterion-to-test mapping, exact commands/results, native evidence where needed, limitations and reviewer outcome. This section is filled with actual evidence as the lifecycle advances.
