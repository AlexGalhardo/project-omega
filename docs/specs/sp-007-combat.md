# SP-007 — Movement, attacks, projectiles and damage

Status: **APPROVED**, by owner blanket approval on 2026-10-24. Dependencies: [SP-002](sp-002-simulation.md), [SP-003](sp-003-content.md), [SP-004](sp-004-geometry.md), [SP-006](sp-006-stats.md)

Requirements: [GDD / design acceptance](../gdd.md): GDD-004 through GDD-007, GDD-009; DA-04 through DA-07, DA-13. Art references resolve in the [Art Bible](../art-bible.md). ADRs: [ADR-004](../architecture/adr/004-collision.md), [ADR-011](../architecture/adr/011-simulation-rng.md). Technical contracts: [architecture](../architecture/architecture.md). Origin: original design implementing the approved Stage 3 baseline; no implementation evidence yet.

## Problem and outcome

Implement the common attack pipeline once so all authored payloads obey the same hit/death rules.

## Requirements and exclusions

Implement the referenced rules and the acceptance cases below; do not reinterpret numeric P values as permission to rebalance during implementation. Exclusions: No independent enemy AI/boss schedules, shop or visual damage authority.

## Technical design and data flow

Components Pose2, Body, Health, Loadout, WeaponTimer, PendingAttack, Projectile, Pickup; transient HitCandidates/DeathFacts. Movement/Emit/Collide/Resolve/CollectHeal sets use content plus ENG-GEOM-001. Targeting picks nearest then SimId; attack snapshots capture stats/aim. Resolve deduplicates per AttackId/target, applies global player i-frames, knockback and one death reward; presentation records describe accepted outcomes. Each emitted attack's candidates are fixed before its knockback.

Failure handling: Any mismatch with GDD damage/geometry is a defect; performance changes require baseline and unchanged trace tests.

## Testable acceptance criteria

1. **SP-007-AC1:** Given a spread volley striking one target with several pellets, then damage occurs once for that attack ID.
2. **SP-007-AC2:** Given an explosion impact, then the direct target receives only explosion damage, not a second direct hit.
3. **SP-007-AC3:** Given a hit at T, then damage is rejected until T+36; duplicates do not create extra drops.
4. **SP-007-AC4:** Given dead attacker before emission, then cancel pending attack; already emitted projectiles persist until their own end/state exit.
5. **SP-007-AC5:** Given simultaneous player/boss lethal hits, then the state layer observes a dead player and cannot award victory.

## Test cases and commands

Each criterion requires a named test or documented native procedure with expected/actual results. Exercise normal, rejected and boundary paths; test scaffolding that only mirrors code is insufficient. Required commands **after implementation** (not runnable or executed during planning):

```powershell
cargo test -p omega_game --no-default-features --locked --test combat_contract
```

Also run the applicable shared checks in [quality gates](../validation/quality-gates.md). Native visual/audio/controller evidence is required where named, even if unit tests pass. Benchmarks never become correctness thresholds without an approved measured budget.

## Allowed and forbidden paths

Allowed scope: crates/omega_game/src/combat/ (excluding stats.rs and energy.rs unless task explicitly assigns them), crates/omega_game/tests/combat_contract.rs, combat fixtures. Task assignment must narrow this to explicit files/globs before READY. Shared plugin registration, root manifests, lockfile and central catalogs may be changed only by the serial integrator under an explicitly assigned task; do not infer shared-file ownership from a module dependency.

Forbidden: unrelated modules, Bevy internals, proprietary assets, other workers' files, user setup/Node files, and approval/status changes that claim unperformed validation. Specs, GDD, ADRs and PLAN.md are serially owned; request a change instead of silently editing contracts while implementing.

## Reviewable handoff slices

Player movement/targeting; timers/emission; swept/piercing payloads; area attacks/damage/deaths; regen/pickup settlement. Separate bounded tasks. These are prospective decomposition boundaries, not assigned Stage 5 task IDs. No single task may silently encompass the entire spec when several slices are listed.

## Definition of Done and completion evidence

Apply [Definition of Done](../validation/definition-of-done.md). All criteria need evidence, tests must cover confirmed bugs, and no critical decision may remain open. Dependencies must provide their tested contracts; source existence alone is not dependency completion.

**Completion evidence: NOT IMPLEMENTED.** No commands above have run, no tests/results/benchmarks are claimed, and no task is READY. The implementation report must record commit, environment, criterion-to-test mapping, exact commands/results, native evidence where needed, limitations and reviewer outcome. This section is filled with actual evidence as the lifecycle advances.
