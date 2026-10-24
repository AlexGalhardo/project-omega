# SP-018 — Full original content and asset coverage

Status: **APPROVED**, by owner blanket approval on 2026-10-24. Dependencies: [SP-003](sp-003-content.md), [SP-013](sp-013-asset-pipeline.md)

Requirements: [GDD / design acceptance](../gdd.md): GDD-009 through GDD-013, GDD-017; ART-002, ART-003, ART-008, ART-012. Art references resolve in the [Art Bible](../art-bible.md). ADRs: [ADR-006](../architecture/adr/006-content.md), [ADR-005](../architecture/adr/005-assets.md). Technical contracts: [architecture](../architecture/architecture.md). Origin: original design implementing the approved Stage 3 baseline; no implementation evidence yet.

## Problem and outcome

Deliver exactly the approved playable catalog and its original presentation without orphaned or unlicensed records.

## Requirements and exclusions

Implement the referenced rules and the acceptance cases below; do not reinterpret numeric P values as permission to rebalance during implementation. Exclusions: No extra content counts, effect enums, paid services, proprietary art or unreviewed balance changes.

## Technical design and data flow

Author full JSON catalogs from the GDD, preserving IDs/formulas and challenge bundles. Produce original SVG/cleared sound/font assets following Art Bible and export pipeline; gameplay definitions contain visual IDs only. Content validation builds the immutable full snapshot; catalog tests compare counts, wave coverage, initial/unlocked pools and references. No new ECS systems are introduced by content authoring.

Failure handling: Any catalog/rule mismatch is fixed at its canonical source; new behavior requires a change request rather than hidden scripting.

## Testable acceptance criteria

1. **SP-018-AC1:** Given full catalogs, then counts are 4/12/10/2/40 with 10 waves/4 unlocks and all references resolve.
2. **SP-018-AC2:** Given initial profile versus all challenge grants, then availability matches exactly the GDD pools and has no circular unlock.
3. **SP-018-AC3:** Given every shipped asset, then a source/provenance record and valid visual/audio/font reference exists; cleared placeholder status is explicit.
4. **SP-018-AC4:** Given the same data through validation and runtime load, then they share the same schema and snapshot revision.

## Test cases and commands

Each criterion requires a named test or documented native procedure with expected/actual results. Exercise normal, rejected and boundary paths; test scaffolding that only mirrors code is insufficient. Required commands **after implementation** (not runnable or executed during planning):

```powershell
cargo run -p omega_game --no-default-features --locked --bin omega-content -- validate assets/content --profile full
cargo test -p omega_game --no-default-features --locked --test catalog_contract
```

Also run the applicable shared checks in [quality gates](../validation/quality-gates.md). Native visual/audio/controller evidence is required where named, even if unit tests pass. Benchmarks never become correctness thresholds without an approved measured budget.

## Allowed and forbidden paths

Allowed scope: assets/content/, art/source/svg/ (excluding proof/ during concurrent pipeline work), art/source/audio/, assets/generated/, assets/fonts/, assets/presentation/ (serial catalog owner), docs/legal/asset-provenance/, crates/omega_game/tests/catalog_contract.rs. Task assignment must narrow this to explicit files/globs before READY. Shared plugin registration, root manifests, lockfile and central catalogs may be changed only by the serial integrator under an explicitly assigned task; do not infer shared-file ownership from a module dependency.

Forbidden: unrelated modules, Bevy internals, proprietary assets, other workers' files, user setup/Node files, and approval/status changes that claim unperformed validation. Specs, GDD, ADRs and PLAN.md are serially owned; request a change instead of silently editing contracts while implementing.

## Reviewable handoff slices

Characters/weapons; common enemies/bosses/waves; items/challenges; original art by family; audio/font/provenance batches. Each batch is a bounded task, never one all-content implementation change. These are prospective decomposition boundaries, not assigned Stage 5 task IDs. No single task may silently encompass the entire spec when several slices are listed.

## Definition of Done and completion evidence

Apply [Definition of Done](../validation/definition-of-done.md). All criteria need evidence, tests must cover confirmed bugs, and no critical decision may remain open. Dependencies must provide their tested contracts; source existence alone is not dependency completion.

**Completion evidence: NOT IMPLEMENTED.** No commands above have run, no tests/results/benchmarks are claimed, and no task is READY. The implementation report must record commit, environment, criterion-to-test mapping, exact commands/results, native evidence where needed, limitations and reviewer outcome. This section is filled with actual evidence as the lifecycle advances.
