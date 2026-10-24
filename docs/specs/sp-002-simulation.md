# SP-002 — Ordered simulation and headless harness

Status: **APPROVED**, by owner blanket approval on 2026-10-24. Dependencies: [SP-001](sp-001-workspace.md)

Requirements: [GDD / design acceptance](../gdd.md): GDD-002, GDD-005, GDD-016; DA-04, DA-16, DA-20. Art references resolve in the [Art Bible](../art-bible.md). ADRs: [ADR-011](../architecture/adr/011-simulation-rng.md). Technical contracts: [architecture](../architecture/architecture.md). Origin: original design implementing the approved Stage 3 baseline; no implementation evidence yet.

## Problem and outcome

Make one authoritative tick path usable by both the client and deterministic scenario tests.

## Requirements and exclusions

Implement the referenced rules and the acceptance cases below; do not reinterpret numeric P values as permission to rebalance during implementation. Exclusions: No final enemy/shop algorithms or cross-platform replay guarantee.

## Technical design and data flow

Resources SimTick, RunState, RunSeed, RNG streams, monotonic IDs, TickInput and presentation queue. Implement GameTick ordered sets and explicit deferred-command flushes from architecture section 5. FixedUpdate delegates to that schedule; harness steps it directly. Define omega-sim CLI: --scenario NAME --seed HEX64 --ticks N --trace PATH, rejecting invalid input before creating a run. Initial smoke scenario steps empty simulation; repeatability scenario compares two fresh identical fixtures. Events/queue entries carry tick and sequence; no renderer data flows back.

Failure handling: Unstable ECS/hash iteration is a defect to fix; do not rewrite expected traces without an explained semantics change.

## Testable acceptance criteria

1. **SP-002-AC1:** Given identical explicit seed/input/content in one executable, when stepping the same fixture twice, then authoritative traces match.
2. **SP-002-AC2:** Given 600 manual ticks, then SimTick advances exactly 600 while the harness never opens a window.
3. **SP-002-AC3:** Given pause, then ten attempted application frames advance no tick/RNG; resume discards paused wall-time backlog.
4. **SP-002-AC4:** Given multiple fixed ticks before presentation, then each emitted record is delivered once in tick/sequence order.

## Test cases and commands

Each criterion requires a named test or documented native procedure with expected/actual results. Exercise normal, rejected and boundary paths; test scaffolding that only mirrors code is insufficient. Required commands **after implementation** (not runnable or executed during planning):

```powershell
cargo test -p omega_game --no-default-features --locked --test sim_contract
cargo run -p omega_game --no-default-features --locked --bin omega-sim -- --scenario smoke --seed 0000000000000000000000000000000000000000000000000000000000000000 --ticks 600
```

Also run the applicable shared checks in [quality gates](../validation/quality-gates.md). Native visual/audio/controller evidence is required where named, even if unit tests pass. Benchmarks never become correctness thresholds without an approved measured budget.

## Allowed and forbidden paths

Allowed scope: crates/omega_game/src/sim/ excluding transitions.rs and run.rs (owned by SP-011), crates/omega_game/src/bin/omega-sim.rs, crates/omega_game/tests/sim_contract.rs, simulation fixtures. Task assignment must narrow this to explicit files/globs before READY. Shared plugin registration, root manifests, lockfile and central catalogs may be changed only by the serial integrator under an explicitly assigned task; do not infer shared-file ownership from a module dependency.

Forbidden: unrelated modules, Bevy internals, proprietary assets, other workers' files, user setup/Node files, and approval/status changes that claim unperformed validation. Specs, GDD, ADRs and PLAN.md are serially owned; request a change instead of silently editing contracts while implementing.

## Reviewable handoff slices

Tick schedule/IDs; input fixture stepping; RNG streams and traces; presentation queue lifetime. These are prospective decomposition boundaries, not assigned Stage 5 task IDs. No single task may silently encompass the entire spec when several slices are listed.

## Definition of Done and completion evidence

Apply [Definition of Done](../validation/definition-of-done.md). All criteria need evidence, tests must cover confirmed bugs, and no critical decision may remain open. Dependencies must provide their tested contracts; source existence alone is not dependency completion.

**Completion evidence (2026-10-24):** implemented by GAME-P1-001; commands, criterion-to-test mapping, environment and limits are in the [evidence index](../validation/evidence/README.md#phase-1-foundation--2026-10-24). Reviewer outcome: integrator self-review only.
