# SP-001 — Workspace, toolchain and CI bootstrap

Status: **READY** for ENG-P1-001 under the approved ROADMAP.md packet, 2026-10-24; later bootstrap slices retain their task dependencies. Dependencies: None; approval gates still apply.

Requirements: [GDD / design acceptance](../gdd.md): GDD-001, GDD-017, GDD-019. Art references resolve in the [Art Bible](../art-bible.md). ADRs: [ADR-002](../architecture/adr/002-toolchain-msrv.md), [ADR-003](../architecture/adr/003-workspace.md), [ADR-009](../architecture/adr/009-ci.md). Technical contracts: [architecture](../architecture/architecture.md). Origin: original design implementing the approved Stage 3 baseline; no implementation evidence yet.

## Problem and outcome

Create the smallest reproducible native/headless build boundary for the two products.

## Requirements and exclusions

Implement the referenced rules and the acceptance cases below; do not reinterpret numeric P values as permission to rebalance during implementation. Exclusions: No actors, gameplay assets, telemetry, external accounts, Bevy patches or benchmark claims. Do not modify unrelated local Node files.

## Technical design and data flow

No gameplay components yet. Register empty project plugin entry points in both packages and a minimal client window only. Headless bootstrap constructs an App without renderer/window/audio plugins. Root manifests own exact dependency candidates, package feature gates and lockfile. CI invokes the same commands as local validation and records tool/runner versions; actions use reviewed full SHAs. Existing Node setup is not imported into Cargo or CI.

Failure handling: Compile or license incompatibility requires a narrow change request with exact diagnostic; never float versions or install a different framework silently.

## Testable acceptance criteria

1. **SP-001-AC1:** Given a clean supported Windows checkout with declared prerequisites, when locked core tests and client check run, then both succeed with Rust 1.99.1 and the resolved Bevy 0.20.0 graph.
2. **SP-001-AC2:** Given headless construction, then no Window/render/audio resources or OS devices are created.
3. **SP-001-AC3:** Given CI, then read-only PR validation runs formatting, linting, headless tests and client checking; advisory/license exceptions are reported explicitly.
4. **SP-001-AC4:** Given git status before staging, then unrelated user changes are excluded from the task commit.

## Test cases and commands

Each criterion requires a named test or documented native procedure with expected/actual results. Exercise normal, rejected and boundary paths; test scaffolding that only mirrors code is insufficient. Required commands **after implementation** (not runnable or executed during planning):

```powershell
cargo metadata --locked --format-version 1
cargo fmt --all -- --check
cargo test --workspace --no-default-features --locked
cargo check -p omega_game --features client --locked
cargo deny check
```

Also run the applicable shared checks in [quality gates](../validation/quality-gates.md). Native visual/audio/controller evidence is required where named, even if unit tests pass. Benchmarks never become correctness thresholds without an approved measured budget.

## Allowed and forbidden paths

Allowed scope: Cargo.toml, Cargo.lock, rust-toolchain.toml, deny.toml, .gitignore, .github/workflows/, crates/omega_2d/Cargo.toml, crates/omega_game/Cargo.toml, initial src/lib.rs and bin entry points, scripts/check.ps1, bootstrap tests. Task assignment must narrow this to explicit files/globs before READY. Shared plugin registration, root manifests, lockfile and central catalogs may be changed only by the serial integrator under an explicitly assigned task; do not infer shared-file ownership from a module dependency.

Forbidden: unrelated modules, Bevy internals, proprietary assets, other workers' files, user setup/Node files, and approval/status changes that claim unperformed validation. Specs, GDD, ADRs and PLAN.md are serially owned; request a change instead of silently editing contracts while implementing.

## Reviewable handoff slices

Workspace/plugin bootstrap; client/headless feature boundary; pinned CI and policy. Each is a separate reviewable task in Stage 5. These are prospective decomposition boundaries, not assigned Stage 5 task IDs. No single task may silently encompass the entire spec when several slices are listed.

## Definition of Done and completion evidence

Apply [Definition of Done](../validation/definition-of-done.md). All criteria need evidence, tests must cover confirmed bugs, and no critical decision may remain open. Dependencies must provide their tested contracts; source existence alone is not dependency completion.

**Completion evidence (2026-10-24):** implemented by ENG-P1-001, QA-P1-001; commands, criterion-to-test mapping, environment and limits are in the [evidence index](../validation/evidence/README.md#phase-1-foundation--2026-10-24). Reviewer outcome: integrator self-review only.
