# ADR-001 — Upstream Bevy with project plugins

Status: **Superseded** by [ADR-012](012-vendored-bevy-fork.md) (DEC-018, 2026-10-24). Originally approved by owner blanket approval on 2026-10-24. Implementation evidence remains required; approval does not claim a tested result.

## Context

DEC-001 approves upstream integration; no reproduced need for internal changes exists.

## Decision

**Recommendation / original design:** Use released Bevy 0.20.0 without a fork. Keep game rules in omega_game; use omega_2d only for the three concrete engine contracts. Canonical implementation details are in [architecture](../architecture.md); this ADR records rationale, not duplicate schemas.

## Alternatives

A full fork gives internal control with ongoing synchronization cost; a minimal patch fork is reserved for a proven public-API blocker and explicit approval.

## Evidence and limits

E4-01/E4-02 and research Track C; DEC-001. Sources resolve in the [ledger](../../research/sources.md#stage-4-verification). No project build or benchmark is implied by reading documentation.

## Consequences

Version migrations still require compatibility checks. No internal patch or private API may bypass the recorded escalation path.

## Reversal cost

Low while using public APIs; a future patch adds regression/upstream/removal obligations.
