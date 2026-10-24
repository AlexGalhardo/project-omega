# SP-014 — HUD, menus, shop UI and accessibility

Status: **APPROVED**, by owner blanket approval on 2026-10-24. Dependencies: [SP-005](sp-005-input.md), [SP-009](sp-009-shop.md), [SP-010](sp-010-progression-save.md), [SP-011](sp-011-run-states.md), [SP-012](sp-012-camera-visuals.md)

Requirements: [GDD / design acceptance](../gdd.md): GDD-015; ART-002, ART-007; DA-19, AA-02, AA-04. Art references resolve in the [Art Bible](../art-bible.md). ADRs: [ADR-008](../architecture/adr/008-input.md), [ADR-005](../architecture/adr/005-assets.md). Technical contracts: [architecture](../architecture/architecture.md). Origin: original design implementing the approved Stage 3 baseline; no implementation evidence yet.

## Problem and outcome

Expose the game's state and choices with readable, reachable controls across keyboard/mouse and controller.

## Requirements and exclusions

Implement the referenced rules and the acceptance cases below; do not reinterpret numeric P values as permission to rebalance during implementation. Exclusions: No screen reader, localization, alternate difficulty or direct ECS debug controls in player flows.

## Technical design and data flow

Client-only UI plugin renders immutable HUD/shop/profile/settings view models. FocusState and ModalState own navigation; commands are typed requests to shop/state/settings modules. Never mutate RunState/HP/wallet from a widget. Build the actual GDD screens, tooltips, energy pips, effective-stat display, onboarding and Unsaved/Retry. Rebuild reactive text only on changes. Reduced effects and volume settings produce presentation configuration, not changed damage/timing.

Failure handling: Synthetic focus tests do not replace actual controller/window validation; report unavailable hardware explicitly.

## Testable acceptance criteria

1. **SP-014-AC1:** Given any shop action, then keyboard/controller focus can reach it and confirmation/cancel semantics match the state/transaction contracts.
2. **SP-014-AC2:** Given 1280x720 or 1920x1080 at 100/125/150 percent UI scale, then no required text/control is clipped or unreachable; capture evidence.
3. **SP-014-AC3:** Given locked content, disabled purchase or zero-powered loadout, then the reason is visible in text and not only color.
4. **SP-014-AC4:** Given muted audio and reduced effects, then hostile cores/telegraph progress and HP changes remain perceivable in human review.
5. **SP-014-AC5:** Given first-run/shop hints, then they pause combat and persist dismissal; Reset Tutorials restores them.

## Test cases and commands

Each criterion requires a named test or documented native procedure with expected/actual results. Exercise normal, rejected and boundary paths; test scaffolding that only mirrors code is insufficient. Required commands **after implementation** (not runnable or executed during planning):

```powershell
cargo test -p omega_game --features client --locked --test ui_contract
cargo run -p omega_game --features client --locked --bin omega-game
```

Also run the applicable shared checks in [quality gates](../validation/quality-gates.md). Native visual/audio/controller evidence is required where named, even if unit tests pass. Benchmarks never become correctness thresholds without an approved measured budget.

## Allowed and forbidden paths

Allowed scope: crates/omega_game/src/ui/, crates/omega_game/tests/ui_contract.rs, UI fixtures, docs/validation/evidence/ui/. Task assignment must narrow this to explicit files/globs before READY. Shared plugin registration, root manifests, lockfile and central catalogs may be changed only by the serial integrator under an explicitly assigned task; do not infer shared-file ownership from a module dependency.

Forbidden: unrelated modules, Bevy internals, proprietary assets, other workers' files, user setup/Node files, and approval/status changes that claim unperformed validation. Specs, GDD, ADRs and PLAN.md are serially owned; request a change instead of silently editing contracts while implementing.

## Reviewable handoff slices

HUD and stats; main/selection/unlocks; shop/confirmation/energy; settings/focus/onboarding; scale/native accessibility pass. These are prospective decomposition boundaries, not assigned Stage 5 task IDs. No single task may silently encompass the entire spec when several slices are listed.

## Definition of Done and completion evidence

Apply [Definition of Done](../validation/definition-of-done.md). All criteria need evidence, tests must cover confirmed bugs, and no critical decision may remain open. Dependencies must provide their tested contracts; source existence alone is not dependency completion.

**Completion evidence: NOT IMPLEMENTED.** No commands above have run, no tests/results/benchmarks are claimed, and no task is READY. The implementation report must record commit, environment, criterion-to-test mapping, exact commands/results, native evidence where needed, limitations and reviewer outcome. This section is filled with actual evidence as the lifecycle advances.
