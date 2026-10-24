# SP-005 — Input actions and fixed-tick buffering

Status: **APPROVED**, by owner blanket approval on 2026-10-24. Dependencies: [SP-002](sp-002-simulation.md), [SP-012](sp-012-camera-visuals.md)

Requirements: [GDD / design acceptance](../gdd.md): GDD-004, GDD-015; DA-07, DA-16. Art references resolve in the [Art Bible](../art-bible.md). ADRs: [ADR-008](../architecture/adr/008-input.md). Technical contracts: [architecture](../architecture/architecture.md). Origin: original design implementing the approved Stage 3 baseline; no implementation evidence yet.

## Problem and outcome

Preserve automatic firing and optional manual aim across device, focus and render-rate changes.

## Requirements and exclusions

Implement the referenced rules and the acceptance cases below; do not reinterpret numeric P values as permission to rebalance during implementation. Exclusions: No action-mapping dependency, aiming assistance beyond GDD or UI layout work.

## Technical design and data flow

Resources ActionBindings, FrameInput, PendingEdges, InputSource, AimState. Sample before fixed loop; produce TickInput without modifying world entities directly. UI intents have a separate queue and consume confirmation/back once. ENG-CAM-001 maps cursor to world and rejects bars. Apply GDD stick dead zones and hold/toggle policy. Focus/device loss requests Pause before the next GameTick. Public boundary is action IDs and TickInput, not raw hardware handles.

Failure handling: Native device behavior needs Windows evidence in addition to synthetic input; absence of a controller is a recorded missing check.

## Testable acceptance criteria

1. **SP-005-AC1:** Given one press and three fixed ticks in a frame, then its edge is consumed once while held movement applies to all ticks.
2. **SP-005-AC2:** Given a frame with zero fixed ticks, then an unconsumed gameplay edge remains available for the next tick.
3. **SP-005-AC3:** Given full digital diagonal input, then movement magnitude is one; manual aim in letterbox bars does not create a new direction.
4. **SP-005-AC4:** Given an active controller disconnect or focus loss, then pause precedes further combat; confirm/Back cannot leak through a modal.

## Test cases and commands

Each criterion requires a named test or documented native procedure with expected/actual results. Exercise normal, rejected and boundary paths; test scaffolding that only mirrors code is insufficient. Required commands **after implementation** (not runnable or executed during planning):

```powershell
cargo test -p omega_game --no-default-features --locked --test input_contract
```

Also run the applicable shared checks in [quality gates](../validation/quality-gates.md). Native visual/audio/controller evidence is required where named, even if unit tests pass. Benchmarks never become correctness thresholds without an approved measured budget.

## Allowed and forbidden paths

Allowed scope: crates/omega_game/src/input/, crates/omega_game/tests/input_contract.rs, input fixtures. Task assignment must narrow this to explicit files/globs before READY. Shared plugin registration, root manifests, lockfile and central catalogs may be changed only by the serial integrator under an explicitly assigned task; do not infer shared-file ownership from a module dependency.

Forbidden: unrelated modules, Bevy internals, proprietary assets, other workers' files, user setup/Node files, and approval/status changes that claim unperformed validation. Specs, GDD, ADRs and PLAN.md are serially owned; request a change instead of silently editing contracts while implementing.

## Reviewable handoff slices

Action mapping/buffer; aim coordinate conversion; pause/disconnection and remapping errors. These are prospective decomposition boundaries, not assigned Stage 5 task IDs. No single task may silently encompass the entire spec when several slices are listed.

## Definition of Done and completion evidence

Apply [Definition of Done](../validation/definition-of-done.md). All criteria need evidence, tests must cover confirmed bugs, and no critical decision may remain open. Dependencies must provide their tested contracts; source existence alone is not dependency completion.

**Completion evidence:** see [evidence GAME-P2-002](../validation/evidence/GAME-P2-002/README.md) (status DONE).
