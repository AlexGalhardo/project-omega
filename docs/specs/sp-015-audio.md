# SP-015 — State-driven audio and mix controls

Status: **APPROVED**, by owner blanket approval on 2026-10-24. Dependencies: [SP-011](sp-011-run-states.md), [SP-013](sp-013-asset-pipeline.md)

Requirements: [GDD / design acceptance](../gdd.md): GDD-015, GDD-016; ART-011, ART-012. Art references resolve in the [Art Bible](../art-bible.md). ADRs: [ADR-005](../architecture/adr/005-assets.md). Technical contracts: [architecture](../architecture/architecture.md). Origin: original design implementing the approved Stage 3 baseline; no implementation evidence yet.

## Problem and outcome

Provide clear optional feedback without coupling sound timing or device availability to gameplay.

## Requirements and exclusions

Implement the referenced rules and the acceptance cases below; do not reinterpret numeric P values as permission to rebalance during implementation. Exclusions: No voice acting, paid generation, adaptive music engine or mandatory audio feedback.

## Technical design and data flow

Client AudioCuePlugin consumes accepted PresentationRecords and state transitions. AudioCatalog maps stable cue IDs to cleared assets; AudioSettings applies master times channel gain. Track looping handles by state and cancel un-emitted charge cues. Group cosmetic pickup sounds without changing logical pickup events. Headless tests consume a recording audio sink instead of a device.

Failure handling: Missing optional clips may be silent with diagnostic, but no audio failure may halt or alter simulation.

## Testable acceptance criteria

1. **SP-015-AC1:** Given duplicate result input, then only one result cue emits; wave exit stops its loops.
2. **SP-015-AC2:** Given master zero or output-device failure, then combat/trace/unlocks are unchanged.
3. **SP-015-AC3:** Given cancelled enemy windup, then its charge cue stops while already emitted projectile behavior remains authoritative.
4. **SP-015-AC4:** Given voice/mix stress, then no clipping or masked critical cue is reported as passing without native audition; all clips have provenance.

## Test cases and commands

Each criterion requires a named test or documented native procedure with expected/actual results. Exercise normal, rejected and boundary paths; test scaffolding that only mirrors code is insufficient. Required commands **after implementation** (not runnable or executed during planning):

```powershell
cargo test -p omega_game --features client --locked --test audio_contract
```

Also run the applicable shared checks in [quality gates](../validation/quality-gates.md). Native visual/audio/controller evidence is required where named, even if unit tests pass. Benchmarks never become correctness thresholds without an approved measured budget.

## Allowed and forbidden paths

Allowed scope: crates/omega_game/src/audio/, crates/omega_game/tests/audio_contract.rs, assets/presentation/audio-catalog.json, audio fixtures. Task assignment must narrow this to explicit files/globs before READY. Shared plugin registration, root manifests, lockfile and central catalogs may be changed only by the serial integrator under an explicitly assigned task; do not infer shared-file ownership from a module dependency.

Forbidden: unrelated modules, Bevy internals, proprietary assets, other workers' files, user setup/Node files, and approval/status changes that claim unperformed validation. Specs, GDD, ADRs and PLAN.md are serially owned; request a change instead of silently editing contracts while implementing.

## Reviewable handoff slices

Cue mapping/recording sink; channel/state lifecycle; native mix validation. These are prospective decomposition boundaries, not assigned Stage 5 task IDs. No single task may silently encompass the entire spec when several slices are listed.

## Definition of Done and completion evidence

Apply [Definition of Done](../validation/definition-of-done.md). All criteria need evidence, tests must cover confirmed bugs, and no critical decision may remain open. Dependencies must provide their tested contracts; source existence alone is not dependency completion.

**Completion evidence: NOT IMPLEMENTED.** No commands above have run, no tests/results/benchmarks are claimed, and no task is READY. The implementation report must record commit, environment, criterion-to-test mapping, exact commands/results, native evidence where needed, limitations and reviewer outcome. This section is filled with actual evidence as the lifecycle advances.
