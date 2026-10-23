# Track G — Specification-driven delivery for agents

Status: Stage 1 research, 2026-10-23. Sources: [ledger](sources.md#track-g). This document recommends a future operating model. It is not the Stage 4 agent contract, spec template, roadmap, or independent review.

## Findings from the supplied references

**Verified:** the actual `kali-tools/docs/specs` listing, its README, `A08-terminal-theme.md`, and selected sections of `H01-bootstrap.md` were read at the recorded revision [G01–G04]. The README defines goals/non-goals, current-state checks, file ownership, Given/When/Then criteria, named tests, exact validation commands, risk/rollback and evidence records. Its index names dependencies. The sample shows how a small change can specify affected files and visual checks.

**Verified:** the actual `jogo-hades/PLAN.md` was fetched; its session-entry instructions, rules, phase/group/dependency tables, estimation caveats, headings and change-log tail were examined [G05]. It provides a current-work dashboard, stable IDs, milestone delivery assertions, measurement checkpoints and explicit scope cuts. This was a conventions study, not a full audit of its roughly 304,000-character plan or the underlying game.

**Verified caveats in these references:** kali-tools records a project-specific exception allowing continuation before native acceptance; jogo-hades records phase closure with unresolved simulation checks and explicitly compressed estimates. Neither source's approval, toolchain, language policy, release commands or exception transfers to this repository. Their claims of completed work were not independently reproduced here. We must not treat a useful document format as proof of delivery quality.

## What to adopt and what to avoid

**Recommendation; origin: original design:** adopt compact task contracts, stable IDs, explicit dependency edges, bounded context loading and evidence attached to criteria. Use the owner's requested lifecycle rather than copying a simpler checklist status model. Keep game design, architecture, execution order and evidence in their respective canonical documents.

Do not inherit Go/Godot tooling, another project's estimates or content counts, automatic releases, duplicated model rules, or hundreds of prescribed microsteps. Preserve a human-reviewable outcome per task; split only when there is an independently verifiable boundary. A change can include code, tests and the necessary spec/evidence updates without becoming three artificial tasks.

## ADRs and specifications

**Verified:** the original ADR proposal uses short records of substantial decisions and preserves superseded history [G06]. **Recommendation:** use ADRs for choices with meaningful alternatives/reversal cost; keep routine implementation detail in the relevant spec. The owner decides product questions. Architecture proposals state evidence and trade-offs before approval.

**Verified:** Gherkin defines Given/When/Then as structured scenario language [G07]. **Recommendation:** use the structure in Markdown and ordinary Rust tests initially; a Cucumber runtime is not required. Criteria must specify inputs, observable outcomes, boundaries and failure handling. Each criterion needs a test or named manual procedure and evidence location.

Illustrative acceptance language only, not an approved economy rule: given a balance of 10 and an eligible offer priced 7, when one purchase succeeds, then the balance is 3 and exactly one item is granted; when the same request is retried, then behavior follows an explicitly defined duplicate-request policy. A spec must decide that policy before READY. Tests should use independent expected outputs and boundary fixtures, not repeat the implementation formula as an oracle.

The future spec template should include metadata/dependencies, linked requirements/ADRs, problem, requirements/exclusions, ECS data and ordering, public contracts, errors, allowed/forbidden paths, acceptance criteria, tests/commands, Definition of Done and completion evidence. Do not freeze uncertain APIs in a READY task.

## Vertical slices and readiness

**Recommendation:** first prove a narrow playable chain with original placeholders: move, encounter a threat, attack, take/deal damage, finish a short encounter, make one purchase, repeat. This is a scope candidate, not approved game content. It exposes gameplay, presentation and integration weaknesses earlier than completing a generalized engine first.

Use the owner-required lifecycle:

`DRAFT -> APPROVED -> READY -> IN_PROGRESS -> IMPLEMENTED -> VERIFIED -> DONE`

`BLOCKED`, `DEFERRED` and `SUPERSEDED` describe explicit exceptions, not completion. Approval belongs to a specific revision. READY requires resolved critical decisions, completed required dependencies, clear file ownership and executable validation instructions. IMPLEMENTED means the change exists; VERIFIED requires recorded passing checks; DONE additionally requires the relevant review/documentation gates. Missing tools or unrun native checks remain visible, never converted to passes. If a requirement changes, revise approval and affected readiness before coding.

## Repository-only handoff

**Recommendation:** create one canonical `AGENTS.md` during Stage 4. Thin model adapters may link to it if needed; no model name is part of a build contract. Minimal loading order should be: project identity and current stage; roadmap dashboard/current task; canonical agent rules; task spec and only its linked GDD/ADR sections; touched files and existing tests. Required rules must remain discoverable without this chat.

A task handoff must identify task/spec IDs, objective, base commit, dependencies, allowed and forbidden paths, acceptance criteria, exact commands and environment, reporting format and escalation conditions. On pickup, the agent rechecks repository state and available tools. A claim that another agent tested something requires accessible evidence for the relevant commit.

Reports should state changes, commit, criterion-to-test results, exact commands/exit status, environment, evidence artifacts, deviations and remaining blockers. Distinguish compile success, simulation correctness, native behavior and subjective feel. A headless agent can prepare a manual protocol; it cannot invent a controller playtest or owner approval.

When a blocking flaw appears, issue a change request containing evidence, affected requirement/decision, alternatives, impact and recommendation. Do not silently change a formula, remove an assertion or implement an unapproved feature to work around it.

## Parallel work and integration

**Verified:** Git worktrees provide separate working trees with shared repository metadata [G08]. They do not prevent two branches from changing the same file or incompatible contracts.

**Recommendation:** parallelize only independent READY work with explicit file ownership and agreed contracts. One branch/worktree per task; integration revalidates the combined result, not just both branches separately. Keep central contracts, lockfile/dependency changes, roadmap and shared schemas under serial ownership. Agents report proposed central changes for integration instead of racing to edit them. A shared checkout is not isolation.

No parallel agents or worktrees were used for this research. Future execution must follow the owner's authorization and actual environment capabilities; this proposal does not assume identical tools or context limits across models.

## Alternatives, risks and recommendation

Chat-only coordination is fast initially but loses rationale and makes handoff unreliable. A large external agent framework adds installation and process overhead without a demonstrated requirement. Exhaustive up-front specs can become stale before implementation. **Recommendation:** repository Markdown, native tests, small vertical slices and revisioned approvals; detail near-term tasks, keep later work tied to approved requirements without speculative APIs.

**Inferred risks:** verbose documents hide contradictions, duplicated criteria drift, tests can mirror flawed code, and a named reviewer can be mistaken for a performed review. The future review prompt must explicitly demand inspection of actual files and state what could not be verified. Preparing that prompt is not a review.

**Unknown:** implementation-agent environments, available native hardware, owner review capacity, Rust learning curve and realistic throughput. Resolve capacity during Stage 2, then calibrate task estimates with delivered work. No reference project's hours or model capability claims establish this project's schedule.
