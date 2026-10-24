# SP-008 — Wave schedules, enemy AI and bosses

Status: **APPROVED**, by owner blanket approval on 2026-10-24. Dependencies: [SP-007](sp-007-combat.md)

Requirements: [GDD / design acceptance](../gdd.md): GDD-010, GDD-011; DA-11 through DA-13, DA-17, DA-18. Art references resolve in the [Art Bible](../art-bible.md). ADRs: [ADR-006](../architecture/adr/006-content.md), [ADR-011](../architecture/adr/011-simulation-rng.md). Technical contracts: [architecture](../architecture/architecture.md). Origin: original design implementing the approved Stage 3 baseline; no implementation evidence yet.

## Problem and outcome

Turn authored waves and attack profiles into readable, reproducible survival encounters.

## Requirements and exclusions

Implement the referenced rules and the acceptance cases below; do not reinterpret numeric P values as permission to rebalance during implementation. Exclusions: No procedural maps, pathfinding, elites, extra bosses, implicit difficulty or balancing changes.

## Technical design and data flow

Resources WaveRuntime and schedule cursor; SpawnWarning/EnemyAi/BossAi components. Spawn set chooses eligible anchors/types through dedicated streams and records warnings; Intent set advances authored finite-state attacks; Emit delegates payloads to combat. Wave completion yields an eligibility fact, never grants victory directly. Pending bosses remain tracked even before their entity exists. Fixed phase enums avoid marker churn.

Failure handling: Impossible content profiles fail validation; do not add untelegraphed relocation or cancel a boss to unblock a wave.

## Testable acceptance criteria

1. **SP-008-AC1:** Given the ten-wave data, then exactly the authored requests occur and bosses are scheduled only on 5/10.
2. **SP-008-AC2:** Given a player occupying a common warning, then postponements cancel after the GDD retry limit; boss postponements remain pending.
3. **SP-008-AC3:** Given locked ground strikes and displaced beam owners, then authoritative centers/origins follow their distinct policies and presentation facts match.
4. **SP-008-AC4:** Given timer expiry with a living/pending boss, then completion remains false; ordinary-wave survivors are removed without rewards after successful completion.

## Test cases and commands

Each criterion requires a named test or documented native procedure with expected/actual results. Exercise normal, rejected and boundary paths; test scaffolding that only mirrors code is insufficient. Required commands **after implementation** (not runnable or executed during planning):

```powershell
cargo test -p omega_game --no-default-features --locked --test encounter_contract
```

Also run the applicable shared checks in [quality gates](../validation/quality-gates.md). Native visual/audio/controller evidence is required where named, even if unit tests pass. Benchmarks never become correctness thresholds without an approved measured budget.

## Allowed and forbidden paths

Allowed scope: crates/omega_game/src/encounters/, crates/omega_game/tests/encounter_contract.rs, encounter fixtures. Task assignment must narrow this to explicit files/globs before READY. Shared plugin registration, root manifests, lockfile and central catalogs may be changed only by the serial integrator under an explicitly assigned task; do not infer shared-file ownership from a module dependency.

Forbidden: unrelated modules, Bevy internals, proprietary assets, other workers' files, user setup/Node files, and approval/status changes that claim unperformed validation. Specs, GDD, ADRs and PLAN.md are serially owned; request a change instead of silently editing contracts while implementing.

## Reviewable handoff slices

Common scheduling/warnings; chase/ranged profiles; charge/orbit/strike profiles; Press; Foreman; full schedule integration. These are prospective decomposition boundaries, not assigned Stage 5 task IDs. No single task may silently encompass the entire spec when several slices are listed.

## Definition of Done and completion evidence

Apply [Definition of Done](../validation/definition-of-done.md). All criteria need evidence, tests must cover confirmed bugs, and no critical decision may remain open. Dependencies must provide their tested contracts; source existence alone is not dependency completion.

**Completion evidence: NOT IMPLEMENTED.** No commands above have run, no tests/results/benchmarks are claimed, and no task is READY. The implementation report must record commit, environment, criterion-to-test mapping, exact commands/results, native evidence where needed, limitations and reviewer outcome. This section is filled with actual evidence as the lifecycle advances.
