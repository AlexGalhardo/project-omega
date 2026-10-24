# ADR-010 — Repository contracts, bounded work and serial integration

Status: **Approved**, by owner blanket approval on 2026-10-24. Implementation evidence remains required; approval does not claim a tested result.

## Context

Agents cannot see this conversation and may have different tools. Existing adapters duplicate rules and reference an unavailable skill.

## Decision

**Recommendation / original design:** AGENTS.md becomes canonical; thin adapters point to it. Tasks carry exact paths, dependencies, acceptance and commands. Specs follow the required lifecycle; central contracts and roadmap have serial ownership. Record change requests for blocking flaws. Canonical implementation details are in [architecture](../architecture.md); this ADR records rationale, not duplicate schemas.

## Alternatives

Large agent frameworks add a new dependency; blanket multi-agent parallelism causes shared-file conflicts; model-specific policies do not belong in the project contract.

## Evidence and limits

E4-14 (actual Spec Kit template read); Track G and owner brief. Adapt scenarios/requirements/outcomes only, without installing a framework. Sources resolve in the [ledger](../../research/sources.md#stage-4-verification). No project build or benchmark is implied by reading documentation.

## Consequences

No hidden chat authority or automatic proof of test success. Parallel work is optional, separately authorized and tool-dependent. Existing setup scripts/config are preserved, not used to bypass approvals.

## Reversal cost

Low; handoff formats can evolve with adapters, but evidence/history and stable IDs must remain.
