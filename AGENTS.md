# Agent Operating Contract

Canonical instructions for all AI agents working on this project. Model-specific adapters (e.g., `CLAUDE.md`) extend but do not override these rules.

## Quick Onboarding

1. **This is a Rust game engine + roguelite.** See `PROMPT.md` for full context.
2. **Current stage:** Read `PLAN.md` for status and next tasks.
3. **Traceability is non-negotiable:** Evidence → Decision → Spec → Task → Test. Every requirement has a task; every task has a test.
4. **Repository is the source of truth.** Read files, not conversation history.

## Tasks and Specs

- Every task has an ID (e.g., `ENG-P1-001`), linked spec, acceptance criteria, and verification command.
- A spec is READY only when all critical decisions are approved.
- Status progression: `DRAFT → APPROVED → READY → IN_PROGRESS → IMPLEMENTED → VERIFIED → DONE`.
- Mark a task BLOCKED if you hit a decision blocker; file a change request with evidence and recommendation.

## What You Can Do

- **Implement:** Write code, tests, specs, architecture documents.
- **Refactor:** Improve clarity and maintainability within approved scope.
- **Test and verify:** Run acceptance criteria, report pass/fail with commands and output.
- **Escalate:** If a spec is unclear, a decision is blocking you, or scope must change, stop and report.

## What You Must Not Do

- **Silently redesign.** If an approved decision is wrong, file a change request; do not ignore it.
- **Skip stages.** Research → Brainstorming → Design → Architecture → Roadmap → Review. No shortcuts.
- **Orphan tasks.** Every task must link to a spec; every spec must link to the GDD and an ADR.
- **Invent facts.** Verify current Bevy release, don't assume. Cite sources and versions.
- **Bypass IP policy.** Use Brotato only for behavioral reference; never copy code or assets.

## Handoff Format

When submitting completed work:

```
**Task:** [ID] [Title]
**Status:** IMPLEMENTED / VERIFIED / DONE

**What changed:**
- [File]: [summary]

**Test results:**
```
$ [command]
[output excerpt]
```

**Blockers:** None / [list with change request links]

**Next task:** [ID] [Title] or "Ready for owner review"
```

## File Ownership

- **Engine specs + code:** `src/engine/` — must pass `cargo test`, `cargo clippy`, `cargo fmt`.
- **Game specs + code:** `src/game/` — must pass game-specific acceptance criteria.
- **Docs:** `docs/research/`, `docs/architecture/`, `docs/specs/` — follow spec-kit conventions.
- **Configuration:** `.claude/settings.json`, `CLAUDE.md`, `AGENTS.md` — only for rule clarifications, never to skip approvals.

## Communication

- **With owner:** Portuguese, direct and conclusive. Evidence first, then details.
- **In repository:** English. Code comments are minimal; ADRs and specs explain decisions.
- **In console output:** Errors exact; long logs abbreviated to relevant lines.

## Escalation Triggers

- Owner-level decision needed (1.0 scope, name, budget, timeline, fork strategy, visibility)
- Breaking change to approved decision
- Security or compliance concern
- Requirement cannot be implemented with current approved scope/architecture
- A decision is missing (spec cannot become READY without it)

File a **change request** with: evidence, affected decision, alternatives, impact, and recommendation.

---

**Effective:** 2026-10-23  
**Stage:** Ready for Stage 1 research  
**Owner:** solo developer, full-stack web background, prior Phaser game work

For model-specific rules, see `CLAUDE.md`.
