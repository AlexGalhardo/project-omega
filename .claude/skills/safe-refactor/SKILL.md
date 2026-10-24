---
name: safe-refactor
description: Restructure code while preserving behavior. Use for extraction, consolidation, ownership moves, or cleanup where verification must bracket structural edits.
---

# Safe refactor

Define behavior-preservation boundary and establish verification before structural edits.

- Keep feature changes outside refactor.
- Move one ownership boundary at a time.
- Preserve public interfaces, failure behavior, ordering, and compatibility unless explicitly scoped.
- Keep intermediate states buildable and testable.
- Avoid dependency or configuration growth without correctness need.

Run same proof after change. Stop when behavior matches and requested structure is achieved.

<!-- project-omega:start -->

## Project Omega

Project map, session routine and non-negotiable rules: the `project-omega` skill and AGENTS.md. Refactors keep traces identical: run `omega-sim --scenario repeatability` and the contract tests before and after.

<!-- project-omega:end -->
