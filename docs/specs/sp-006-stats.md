# SP-006 — Stats, energy and numeric formulas

Status: **APPROVED**, by owner blanket approval on 2026-10-24. Dependencies: [SP-003](sp-003-content.md)

Requirements: [GDD / design acceptance](../gdd.md): GDD-006, GDD-007, GDD-009; DA-01 through DA-06. Art references resolve in the [Art Bible](../art-bible.md). ADRs: [ADR-004](../architecture/adr/004-collision.md), [ADR-006](../architecture/adr/006-content.md). Technical contracts: [architecture](../architecture/architecture.md). Origin: original design implementing the approved Stage 3 baseline; no implementation evidence yet.

## Problem and outcome

Make every damage/rate/energy rule auditable and independent of art or item iteration order.

## Requirements and exclusions

Implement the referenced rules and the acceptance cases below; do not reinterpret numeric P values as permission to rebalance during implementation. Exclusions: No balance changes, extra stats, permanent upgrades or runtime expression evaluation.

## Technical design and data flow

Pure evaluators consume validated character/item/weapon values and produce EffectiveStats, damage amount, fire interval and validated EnergyAllocation. Use checked integers/rationals for percentages, floor/ceil exactly once as GDD specifies; regen uses integer numerator over 600 ticks for tenths HP/s. ECS combat consumes evaluated values; shop recomputes on successful transactions. Energy updates return either a new four-slot array plus remainder or typed rejection, never partially mutate.

Failure handling: Overflow or domain error is reported; do not wrap, saturate silently or silently adjust P values.

## Testable acceptance criteria

1. **SP-006-AC1:** Given the GDD worked examples, then interval 15/27 ticks, damage 21/26/15 and HP limits match exactly.
2. **SP-006-AC2:** Given any valid item permutation, then summed/clamped stats are identical.
3. **SP-006-AC3:** Given allocations, then each occupied slot is 0–3, empty slots are zero and sum plus remainder is six; invalid increments preserve the old value.
4. **SP-006-AC4:** Given 0.5 regen and 120 eligible damaged ticks, then heal exactly one HP; a dead actor never revives.

## Test cases and commands

Each criterion requires a named test or documented native procedure with expected/actual results. Exercise normal, rejected and boundary paths; test scaffolding that only mirrors code is insufficient. Required commands **after implementation** (not runnable or executed during planning):

```powershell
cargo test -p omega_game --no-default-features --locked --test stats_contract
```

Also run the applicable shared checks in [quality gates](../validation/quality-gates.md). Native visual/audio/controller evidence is required where named, even if unit tests pass. Benchmarks never become correctness thresholds without an approved measured budget.

## Allowed and forbidden paths

Allowed scope: crates/omega_game/src/combat/stats.rs, crates/omega_game/src/combat/energy.rs, crates/omega_game/tests/stats_contract.rs. Task assignment must narrow this to explicit files/globs before READY. Shared plugin registration, root manifests, lockfile and central catalogs may be changed only by the serial integrator under an explicitly assigned task; do not infer shared-file ownership from a module dependency.

Forbidden: unrelated modules, Bevy internals, proprietary assets, other workers' files, user setup/Node files, and approval/status changes that claim unperformed validation. Specs, GDD, ADRs and PLAN.md are serially owned; request a change instead of silently editing contracts while implementing.

## Reviewable handoff slices

Stat accumulation/formulas; reactor invariants; healing/cooldown edge cases. These are prospective decomposition boundaries, not assigned Stage 5 task IDs. No single task may silently encompass the entire spec when several slices are listed.

## Definition of Done and completion evidence

Apply [Definition of Done](../validation/definition-of-done.md). All criteria need evidence, tests must cover confirmed bugs, and no critical decision may remain open. Dependencies must provide their tested contracts; source existence alone is not dependency completion.

**Completion evidence:** see [evidence GAME-P2-001](../validation/evidence/GAME-P2-001/README.md) (status DONE).
