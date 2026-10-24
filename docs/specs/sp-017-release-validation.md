# SP-017 — Packaging and release qualification

Status: **APPROVED**, by owner blanket approval on 2026-10-24. Dependencies: [SP-001](sp-001-workspace.md), [SP-010](sp-010-progression-save.md), [SP-014](sp-014-ui-accessibility.md), [SP-015](sp-015-audio.md), [SP-016](sp-016-diagnostics.md), [SP-018](sp-018-production-content.md)

Requirements: [GDD / design acceptance](../gdd.md): GDD-001, GDD-014, GDD-017, GDD-019; ART-012, AA-06. Art references resolve in the [Art Bible](../art-bible.md). ADRs: [ADR-009](../architecture/adr/009-ci.md), [ADR-007](../architecture/adr/007-saves.md). Technical contracts: [architecture](../architecture/architecture.md). Origin: original design implementing the approved Stage 3 baseline; no implementation evidence yet.

## Problem and outcome

Qualify a distributable Windows game with evidence for behavior, rights and recovery.

## Requirements and exclusions

Implement the referenced rules and the acceptance cases below; do not reinterpret numeric P values as permission to rebalance during implementation. Exclusions: No storefront SDK, installer service, signing purchase or publication without the separately approved release model.

## Technical design and data flow

Package a locked release build plus validated full content, generated assets and notices; exclude source secrets, development tools/caches and private save data. Install/read from a clean directory while saves use per-user storage. A manifest records hashes/version/build provenance. Release validation consumes prior native/regression/performance evidence and verifies final gates. No new ECS components or runtime subsystem is required.

Failure handling: Critical readiness blockers: owner-approved release/license decisions, hardware floor and post-baseline performance budgets. No automatic publish.

## Testable acceptance criteria

1. **SP-017-AC1:** Given a clean supported Windows target, then package boots and completes victory/defeat/shop/save/restart workflows without developer tool installations.
2. **SP-017-AC2:** Given package manifest, then every shipped dependency/asset has its required notices and recorded provenance; unresolved terms block release.
3. **SP-017-AC3:** Given RC approval, then final 1.0 remains a separate gate with release notes, known limitations and regression evidence.
4. **SP-017-AC4:** Given unsupported minimum hardware/license/store decisions, then this spec cannot become READY for final qualification.

## Test cases and commands

Each criterion requires a named test or documented native procedure with expected/actual results. Exercise normal, rejected and boundary paths; test scaffolding that only mirrors code is insufficient. Required commands **after implementation** (not runnable or executed during planning):

```powershell
powershell -NoProfile -File scripts/package-windows.ps1 -Output artifacts/package
powershell -NoProfile -File scripts/validate-package.ps1 -Package artifacts/package
```

Also run the applicable shared checks in [quality gates](../validation/quality-gates.md). Native visual/audio/controller evidence is required where named, even if unit tests pass. Benchmarks never become correctness thresholds without an approved measured budget.

## Allowed and forbidden paths

Allowed scope: scripts/package-windows.ps1, scripts/validate-package.ps1, docs/validation/evidence/release/, release notes and third-party notices explicitly assigned by task. Task assignment must narrow this to explicit files/globs before READY. Shared plugin registration, root manifests, lockfile and central catalogs may be changed only by the serial integrator under an explicitly assigned task; do not infer shared-file ownership from a module dependency.

Forbidden: unrelated modules, Bevy internals, proprietary assets, other workers' files, user setup/Node files, and approval/status changes that claim unperformed validation. Specs, GDD, ADRs and PLAN.md are serially owned; request a change instead of silently editing contracts while implementing.

## Reviewable handoff slices

Offline package manifest/smoke; clean-machine regression; RC evidence; final release gate. These are prospective decomposition boundaries, not assigned Stage 5 task IDs. No single task may silently encompass the entire spec when several slices are listed.

## Definition of Done and completion evidence

Apply [Definition of Done](../validation/definition-of-done.md). All criteria need evidence, tests must cover confirmed bugs, and no critical decision may remain open. Dependencies must provide their tested contracts; source existence alone is not dependency completion.

**Completion evidence: NOT IMPLEMENTED.** No commands above have run, no tests/results/benchmarks are claimed, and no task is READY. The implementation report must record commit, environment, criterion-to-test mapping, exact commands/results, native evidence where needed, limitations and reviewer outcome. This section is filled with actual evidence as the lifecycle advances.
