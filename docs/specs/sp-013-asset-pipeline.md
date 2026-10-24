# SP-013 — Offline SVG proof and reproducible asset metadata

Status: **APPROVED**, by owner blanket approval on 2026-10-24. Dependencies: [SP-003](sp-003-content.md), [SP-012](sp-012-camera-visuals.md)

Requirements: [GDD / design acceptance](../gdd.md): GDD-016; ART-004, ART-008 through ART-013; AA-01, AA-05, AA-07. Art references resolve in the [Art Bible](../art-bible.md). ADRs: [ADR-005](../architecture/adr/005-assets.md). Technical contracts: [architecture](../architecture/architecture.md). Origin: original design implementing the approved Stage 3 baseline; no implementation evidence yet.

## Problem and outcome

Prove the authored vector workflow and art-replacement boundary before scaling production.

## Requirements and exclusions

Implement the referenced rules and the acceptance cases below; do not reinterpret numeric P values as permission to rebalance during implementation. Exclusions: No runtime vector backend, paid art tool, full production catalog or automatic atlas packing.

## Technical design and data flow

Offline process validates SVG inputs/resources and catalog manifest, invokes verified resvg 0.45.1 with explicit sizes, validates temporary PNG outputs and publishes only on success. Manifest records tool/source/output hashes and geometry metadata from the Art Bible. Create original proof assets: player, projectile, warning and icon; no proprietary input. Runtime consumes PNG/catalog only. Script CLI takes -Manifest PATH and fails nonzero on unsafe resources or invalid output. Exact resvg CLI flags must be checked with the pinned binary's help during implementation and recorded, not guessed now.

Failure handling: Do not mark export proven from tool exit alone; native visual review and provenance are required.

## Testable acceptance criteria

1. **SP-013-AC1:** Given an SVG with external URL/script/undeclared resource, then reject before invoking exporter.
2. **SP-013-AC2:** Given original proof assets, then record correct dimensions, alpha and pivots with actual exporter command/version.
3. **SP-013-AC3:** Given an unchanged input set on the supported environment, then repeated outputs/manifests match or a documented unresolved reproducibility failure blocks completion.
4. **SP-013-AC4:** Given a raster replacement through catalog alone, then DA-20 authoritative traces match and AA-05 visual alignment passes.

## Test cases and commands

Each criterion requires a named test or documented native procedure with expected/actual results. Exercise normal, rejected and boundary paths; test scaffolding that only mirrors code is insufficient. Required commands **after implementation** (not runnable or executed during planning):

```powershell
powershell -NoProfile -File scripts/export-assets.ps1 -Manifest assets/presentation/proof-manifest.json
powershell -NoProfile -File scripts/validate-assets.ps1 -Manifest assets/presentation/proof-manifest.json
cargo test -p omega_2d --features client --locked --test asset_contract
```

Also run the applicable shared checks in [quality gates](../validation/quality-gates.md). Native visual/audio/controller evidence is required where named, even if unit tests pass. Benchmarks never become correctness thresholds without an approved measured budget.

## Allowed and forbidden paths

Allowed scope: scripts/export-assets.ps1, scripts/validate-assets.ps1, art/source/svg/proof/, assets/generated/sprites/proof/, assets/presentation/, docs/legal/asset-provenance/, crates/omega_2d/tests/asset_contract.rs. Task assignment must narrow this to explicit files/globs before READY. Shared plugin registration, root manifests, lockfile and central catalogs may be changed only by the serial integrator under an explicitly assigned task; do not infer shared-file ownership from a module dependency.

Forbidden: unrelated modules, Bevy internals, proprietary assets, other workers' files, user setup/Node files, and approval/status changes that claim unperformed validation. Specs, GDD, ADRs and PLAN.md are serially owned; request a change instead of silently editing contracts while implementing.

## Reviewable handoff slices

Exporter/resource validation; proof art and metadata; skin substitution evidence. These are prospective decomposition boundaries, not assigned Stage 5 task IDs. No single task may silently encompass the entire spec when several slices are listed.

## Definition of Done and completion evidence

Apply [Definition of Done](../validation/definition-of-done.md). All criteria need evidence, tests must cover confirmed bugs, and no critical decision may remain open. Dependencies must provide their tested contracts; source existence alone is not dependency completion.

**Completion evidence (2026-10-24):** implemented by ENG-P1-005, GAME-P1-004 (native visual sign-off pending); commands, criterion-to-test mapping, environment and limits are in the [evidence index](../validation/evidence/README.md#phase-1-foundation--2026-10-24). Reviewer outcome: integrator self-review only.
