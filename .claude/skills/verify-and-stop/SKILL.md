---
name: verify-and-stop
description: Prove existing work meets acceptance conditions without expanding scope. Use for validation-only tasks, completion checks, focused gate runs, and last-mile proof.
---

# Verify and stop

Translate acceptance conditions into smallest sufficient proof set.

- Reuse still-current results with matching repository state.
- Run focused checks before wider gates.
- Distinguish pass, fail, unavailable, and blocked exactly.
- Do not edit product code unless verification request includes fixes.
- Do not add polish, cleanup, or unrelated tests after criteria pass.

Stop immediately when acceptance proof is complete. Report commands, results, and unresolved risk only.

<!-- project-omega:start -->

## Project Omega

Project map, session routine and non-negotiable rules: the `project-omega` skill and AGENTS.md. Use `scripts/check.ps1` (or the task's V-commands from its spec) as the verification set.

<!-- project-omega:end -->
