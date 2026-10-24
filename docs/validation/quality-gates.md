# Quality gates and validation commands

Status: APPROVED by owner direction, 2026-10-24. Commands below are required **after the relevant implementation exists**. There is currently no Cargo workspace; none of these Rust/game/asset commands was run during planning. Do not interpret this document as a runnable test result.

## Gate ordering

1. Research/decisions/design approval: evidence and explicit owner authorization recorded. Stage 3 baseline accepted for Stage 4.
2. Stage 4 architecture/spec approval: owner supplied blanket approval of draft/proposed artifacts on 2026-10-24; documentary completion still requires actual checks.
3. Stage 5: executable roadmap with exact tasks, acyclic dependencies, file ownership and next READY work. ROADMAP.md now supplies this artifact under owner standing approval; only its exact initial READY packet can be started.
4. Per-task implementation/verification: scoped criteria and checks below, then integration/review and Definition of Done.
5. Native milestone and release qualification: real supported-hardware input/render/audio/save/package evidence. RC and 1.0 are separate gates.

## Shared correctness commands

Run from repository root in PowerShell after bootstrap:

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --no-default-features --all-targets --locked -- -D warnings
cargo test --workspace --no-default-features --locked
cargo check -p omega_game --features client --locked
cargo clippy -p omega_game --features client --all-targets --locked -- -D warnings
cargo deny check
```

Client integration tests named in specs must run separately with client features; core tests do not exercise window/audio/UI plugins. CI uses windows-2022 and Rust 1.99.1, records image version and pins action revisions at implementation time. A client compile proves neither graphics correctness nor controller support. Use read-only CI permissions and no release secrets in untrusted PR jobs. Advisory database revision/date and license exceptions accompany cargo-deny output; a stale scan is not a current release audit.

## Layers and targeted evidence

| Layer | Required checks | Evidence / scope |
| --- | --- | --- |
| Unit | GDD formulas, clamps, energy, shop prices and shape boundaries | Exact examples and edge cases; integer/rational expectations, not approximate rounded UI output. |
| Invariant/property | Item order independence; energy sum; no negative currency; at-most-once hit/grant; geometry symmetry/translation | Fixed reproducible generated cases; record failing seed and minimize/reproduce a counterexample before fixing. No added library required initially. |
| Integration | Tick ordering, deferred spawn/death, shop transactions, wave transitions, persistence faults | Named spec integration tests with assertions on authoritative state and rejected side effects. |
| Headless | Same-build seeded scenarios, pause, catch-up frames, full lifecycle | No renderer/window/audio initialization; record seed/content/commit/input fixture and first divergence. |
| Content/assets | Full/prototype schema profiles, reference/rights coverage, unsafe SVG rejection, export metadata | Content CLI and export scripts from SP-003/SP-013; native visual review for fidelity. |
| Native | Keyboard/controller, focus/disconnect, UI scales, telegraphs, audio/mute, real file interruption and clean package | Hardware/OS, steps, screenshots/logs and actual outcome. Unavailable hardware is an incomplete check. |
| Regression | Every confirmed bug | A failing reproduction before fix where practical, then a persistent targeted test; visual-only bugs require a documented native regression procedure. |
| Benchmark | Performance-plan scenarios | Separate reports and hardware; no ordinary CI time threshold until approved baseline budgets exist. |

## Scope and stopping rule

Run checks appropriate to changed paths plus the relevant integration contract. Shared schedule/dependency/content-schema changes require the affected consumers' checks. Broaden testing only when new failures, code changes or unresolved evidence justify it. Documentation-only work uses link/ID/dependency/arithmetic/status checks and `git diff --check`, not fictitious Cargo success. Balance/readability still needs human playtests even after all automated checks pass.

## READY audit

A task needs an approved spec/ADRs, a named test function or fixture for every acceptance criterion it claims (REV-020), no critical open decisions, named tested dependencies, exact allowed/forbidden paths, required test setup/commands, acceptance mapping, reviewable size, assigned owner and stage-gate authorization. SP-017 remains ineligible for final release readiness until hardware/license/release choices and measured budgets are resolved. Future unexecuted validation is expected for a READY implementation task; unresolved decisions about what to build are not.

## Stage 4 and 5 documentary record

Executed 2026-10-24 with an ephemeral Python script (not committed), after the planning-review corrections:

- 65 Markdown files outside vendored/generated trees; 386 local links and heading anchors checked; all resolve after the files created in the same change set.
- ROADMAP.md: 73 task rows (1 READY at the time of the check, 66 TODO, 4 CANCELLED by the REV-010 split, 2 BLOCKED); no undefined dependency; no cycle; all 18 specs referenced by at least one task.
- Spec dependency graph in `docs/specs/index.md`: unchanged except SP-012 no longer depends on SP-004 (REV-015).
- The independent planning review reproduced the same pass/fail picture with its own script (see [the review](../reviews/2026-10-24-planning-review.md#tools-and-commands-actually-run)).
- `git diff --check` clean for the planning files after removing the trailing whitespace in PLAN.md.

Implementation evidence starts in [the evidence index](evidence/README.md). No gameplay, balance or independent implementation review result is claimed here.
