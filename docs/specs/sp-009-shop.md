# SP-009 — Shop offers and atomic transactions

Status: **APPROVED**, by owner blanket approval on 2026-10-24. Dependencies: [SP-003](sp-003-content.md), [SP-006](sp-006-stats.md)

Requirements: [GDD / design acceptance](../gdd.md): GDD-007, GDD-008; DA-02, DA-03, DA-08 through DA-10. Art references resolve in the [Art Bible](../art-bible.md). ADRs: [ADR-006](../architecture/adr/006-content.md), [ADR-011](../architecture/adr/011-simulation-rng.md). Technical contracts: [architecture](../architecture/architecture.md). Origin: original design implementing the approved Stage 3 baseline; no implementation evidence yet.

## Problem and outcome

Make build decisions clear and protect currency/loadout/RNG from failed or repeated UI actions.

## Requirements and exclusions

Implement the referenced rules and the acceptance cases below; do not reinterpret numeric P values as permission to rebalance during implementation. Exclusions: No selling, stash, offer lock, discount items or additional refreshes.

## Technical design and data flow

ShopVisit owns visit ID, offer IDs, revision and refresh-used flag. Typed ShopCommand includes expected revision and selected replacement slot. Validate first, stage candidate changes and RNG clone, then commit once; reject stale command IDs/revisions. Use immutable run unlock pool, ordered eligibility, two/two selection and fallback. ShopOutcome carries presentation and persistence-independent build changes; UI has no direct mutable access.

Failure handling: Never repair a negative wallet by clamping after purchase; invalid transactions must not commit.

## Testable acceptance criteria

1. **SP-009-AC1:** Given base price 24 after wave 3, then new/tier2/tier3 prices are 28/42/56.
2. **SP-009-AC2:** Given insufficient funds, stale offer or cancelled replacement, then no currency/build/offer/RNG changes occur.
3. **SP-009-AC3:** Given owned tier3/owned item, then it cannot appear as an unusable duplicate; depleted pools show Empty after fallback.
4. **SP-009-AC4:** Given successful replacement, then install tier1 and retain slot energy; successful duplicate upgrades one tier without consuming a slot.
5. **SP-009-AC5:** Given one refresh, then later refresh commands fail until a new visit.

## Test cases and commands

Each criterion requires a named test or documented native procedure with expected/actual results. Exercise normal, rejected and boundary paths; test scaffolding that only mirrors code is insufficient. Required commands **after implementation** (not runnable or executed during planning):

```powershell
cargo test -p omega_game --no-default-features --locked --test shop_contract
```

Also run the applicable shared checks in [quality gates](../validation/quality-gates.md). Native visual/audio/controller evidence is required where named, even if unit tests pass. Benchmarks never become correctness thresholds without an approved measured budget.

## Allowed and forbidden paths

Allowed scope: crates/omega_game/src/shop/, crates/omega_game/tests/shop_contract.rs, shop fixtures. Task assignment must narrow this to explicit files/globs before READY. Shared plugin registration, root manifests, lockfile and central catalogs may be changed only by the serial integrator under an explicitly assigned task; do not infer shared-file ownership from a module dependency.

Forbidden: unrelated modules, Bevy internals, proprietary assets, other workers' files, user setup/Node files, and approval/status changes that claim unperformed validation. Specs, GDD, ADRs and PLAN.md are serially owned; request a change instead of silently editing contracts while implementing.

## Reviewable handoff slices

Eligibility/pricing; transactions/replacement; refresh depletion; reactor panel command integration. These are prospective decomposition boundaries, not assigned Stage 5 task IDs. No single task may silently encompass the entire spec when several slices are listed.

## Definition of Done and completion evidence

Apply [Definition of Done](../validation/definition-of-done.md). All criteria need evidence, tests must cover confirmed bugs, and no critical decision may remain open. Dependencies must provide their tested contracts; source existence alone is not dependency completion.

**Completion evidence: NOT IMPLEMENTED.** No commands above have run, no tests/results/benchmarks are claimed, and no task is READY. The implementation report must record commit, environment, criterion-to-test mapping, exact commands/results, native evidence where needed, limitations and reviewer outcome. This section is filled with actual evidence as the lifecycle advances.
