# SP-016 — Baseline scenarios and performance evidence

Status: **APPROVED**, by owner blanket approval on 2026-10-24. Dependencies: [SP-002](sp-002-simulation.md), [SP-007](sp-007-combat.md), [SP-008](sp-008-encounters.md), [SP-012](sp-012-camera-visuals.md)

Requirements: [GDD / design acceptance](../gdd.md): GDD-017 through GDD-020; ART-013; PERF-01 through PERF-08. Art references resolve in the [Art Bible](../art-bible.md). ADRs: [ADR-009](../architecture/adr/009-ci.md), [ADR-011](../architecture/adr/011-simulation-rng.md). Technical contracts: [architecture](../architecture/architecture.md). Origin: original design implementing the approved Stage 3 baseline; no implementation evidence yet.

## Problem and outcome

Establish measured costs and repeatable scenarios before assigning budgets or optimizing.

## Requirements and exclusions

Implement the referenced rules and the acceptance cases below; do not reinterpret numeric P values as permission to rebalance during implementation. Exclusions: No arbitrary FPS budget, optimization rewrite, headless GPU claims or timing thresholds in ordinary CI.

## Technical design and data flow

Diagnostics observes simulation/presentation counters without mutating their RNG/state. Add the performance-plan scenarios and CSV/JSON reports; instrument wall/tick times and label unsupported GPU/allocation metrics. CLI extends scenario runner with --measure and --output PATH; native client uses --benchmark PERF-ID --duration-seconds N --output PATH. Metadata records hardware/build/content/seed/settings and nearest-rank percentile method. Keep performance reports separate from correctness assertions.

Failure handling: No supported frame-rate claim until actual hardware results exist; do not use zeros for missing instrumentation.

## Testable acceptance criteria

1. **SP-016-AC1:** Given a scenario report, then metadata/sample count/P50/P95/P99/max and unsupported metrics are explicit.
2. **SP-016-AC2:** Given same seed/input with diagnostics off/on, then authoritative trace remains identical.
3. **SP-016-AC3:** Given long-session teardown, then report entity/asset/audio/memory trends including settled baseline; investigate residual growth.
4. **SP-016-AC4:** Given proposed optimization, then provide like-for-like before/after workload and correctness evidence before accepting its claimed gain.

## Test cases and commands

Each criterion requires a named test or documented native procedure with expected/actual results. Exercise normal, rejected and boundary paths; test scaffolding that only mirrors code is insufficient. Required commands **after implementation** (not runnable or executed during planning):

```powershell
cargo test -p omega_game --no-default-features --locked --test diagnostic_contract
cargo run -p omega_game --release --no-default-features --locked --bin omega-sim -- --scenario PERF-03 --seed 0000000000000000000000000000000000000000000000000000000000000000 --ticks 7200 --measure --output artifacts/PERF-03.json
```

Also run the applicable shared checks in [quality gates](../validation/quality-gates.md). Native visual/audio/controller evidence is required where named, even if unit tests pass. Benchmarks never become correctness thresholds without an approved measured budget.

## Allowed and forbidden paths

Allowed scope: crates/omega_game/src/diagnostics/, crates/omega_game/src/bin/omega-sim.rs (serially assigned), crates/omega_game/tests/diagnostic_contract.rs, scripts/measure-build.ps1, docs/validation/evidence/performance/. Task assignment must narrow this to explicit files/globs before READY. Shared plugin registration, root manifests, lockfile and central catalogs may be changed only by the serial integrator under an explicitly assigned task; do not infer shared-file ownership from a module dependency.

Forbidden: unrelated modules, Bevy internals, proprietary assets, other workers' files, user setup/Node files, and approval/status changes that claim unperformed validation. Specs, GDD, ADRs and PLAN.md are serially owned; request a change instead of silently editing contracts while implementing.

## Reviewable handoff slices

Report/counter plumbing; simulation stress scenarios; native frame/build baseline; long-session study. These are prospective decomposition boundaries, not assigned Stage 5 task IDs. No single task may silently encompass the entire spec when several slices are listed.

## Definition of Done and completion evidence

Apply [Definition of Done](../validation/definition-of-done.md). All criteria need evidence, tests must cover confirmed bugs, and no critical decision may remain open. Dependencies must provide their tested contracts; source existence alone is not dependency completion.

**Completion evidence (2026-10-24):** implemented by QA-P1-002 reporting subset only (stress scenarios pending); commands, criterion-to-test mapping, environment and limits are in the [evidence index](../validation/evidence/README.md#phase-1-foundation--2026-10-24). Reviewer outcome: integrator self-review only.
