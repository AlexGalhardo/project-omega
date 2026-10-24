# Project Instructions — Rust 2D Engine + Arena-Survival Roguelite

These rules apply to all work in this repository.

## Execution

- **Follow PROMPT.md strictly:** 6 stages in order, no skips. Approval gates are hard stops.
- **Update PLAN.md at each checkpoint:** mark completed items, note blockers, commit after each stage.
- **Commit frequently:** after each milestone, with message format: `docs(stage-X): [description]` or `feat: [feature]`.
- **Traceability:** every GDD requirement → spec → roadmap task → test. No orphaned tasks or requirements.

## Language

- **Repository:** all code, comments, specs, ADRs, tests, commits in **English**.
- **Communication with owner:** **Brazilian Portuguese** (`pt-BR`).
- No Portuguese in code or repo artifacts.

## Specs and Acceptance Criteria

- Use **spec-kit** conventions (https://github.com/github/spec-kit) for SDD structure.
- Acceptance criteria must be **testable**, never vague ("works correctly"). Use Given/When/Then format where applicable.
- Link every spec to relevant GDD sections and ADRs.
- Status lifecycle: `DRAFT → APPROVED → READY → IN_PROGRESS → IMPLEMENTED → VERIFIED → DONE` (or `BLOCKED / DEFERRED / SUPERSEDED`).

## Overnight Execution

When running unattended (via `run-overnight.ps1` or `/loop`):

1. **Do not ask for approval** on decisions already made (check `docs/decisions/` first).
2. **Escalate only for:** owner-level decisions (scope, name, budget, timeline), breaking changes to approved decisions, security/compliance concerns.
3. **Commit small:** one task = one commit. If something fails 3 times, note it in PLAN.md and move to the next task.
4. **Keep context:** after `/compact`, resume with `/loop` and the same pacing.
5. **Mark DONE when:** acceptance criteria passed, test commands succeed, linked spec marked VERIFIED.
6. **Permissions:** restricted to safe subsets in `.claude/settings.json` (no `git push`, `bun -e`, direct shell escapes). If task requires blocked command, escalate.

## Code Style

- Rust: follow official Rust guidelines. Use `cargo fmt` and `cargo clippy` before committing.
- No comments stating the obvious. Comment only: edge cases, "why" decisions (not "what"), workarounds with issue references, non-trivial logic.
- Strong typing: avoid `any`, use `unknown` with narrowing when genuinely unknown. Explicit return types for public functions.
- Fix root causes, never silence errors. No `#[allow(...)]`, `@ts-ignore`, or skipped tests without escalation.

## Dependencies

- Always use **stable, production-ready versions.** No `latest`, `next`, `canary`, `rc`, `beta`, `alpha`.
- Specify exact versions in `Cargo.toml` or `Cargo.lock` — no open ranges like `^` without justification.
- Confirm in lockfile before committing.
- Do not add new dependencies without asking the owner first.

## Architecture Decisions

- Record every substantial decision in `docs/architecture/adr/` with context, alternatives, evidence, consequences, and reversal cost.
- If you hit a blocking flaw in an approved decision, file a **change request:** evidence, affected decision, alternatives, impact, and recommendation. Do not silently redesign.

## Testing and Validation

- Prefer typecheck, lint, and focused tests over full suite runs.
- Before declaring a task complete, run: `cargo test`, `cargo clippy`, `cargo fmt --check`.
- Benchmark results must include hardware, method, and date. No assumptions without baselines.

## File Organization

- Specs in `docs/specs/` with stable IDs and dependencies listed.
- Research and findings in `docs/research/` with source ledger.
- Decisions in `docs/decisions/` with ID, question, options, choice, rationale, status.
- ADRs in `docs/architecture/adr/` for substantial decisions.
- Game content and assets separate from engine (in subfolders, not mixed).

## Documentation

- **One canonical source per topic.** Link instead of duplicating.
- GDD and art bible must not contradict (hitboxes, timing, UI alignment).
- PLAN.md is the source of truth for task status.
- Update `CHANGELOG.md` after each stage or feature set.

## Escalation Checklist

Stop and ask the owner if:

- [ ] 1.0 scope or feature removals are needed
- [ ] Budget or timeline constraints discovered
- [ ] Fork strategy needs reconsideration (after Stage 1 research)
- [ ] Rust/Bevy learning curve impacts schedule
- [ ] Repository visibility or licensing changes
- [ ] Breaking change to an approved decision
- [ ] Security or compliance concerns

---

**Repository State:** Ready for Stage 1 (Research)  
**Created:** 2026-10-23  
**Last Updated:** 2026-10-23
