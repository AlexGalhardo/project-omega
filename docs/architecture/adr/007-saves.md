# ADR-007 — Versioned local profile/settings with two-generation recovery

Status: **Approved**, by owner blanket approval on 2026-10-24. Implementation evidence remains required; approval does not claim a tested result.

## Context

GDD-014 requires persistence, visible errors and preservation on interruption, without active-run resume.

## Decision

**Recommendation / original design:** Separate JSON profile/settings envelopes, two generation slots, SHA-256 payload hashes, verified writes and a replaceable manifest. Use LOCALAPPDATA/ProjectOmega; inject paths for tests. No cloud/account or active-run serialization. Canonical implementation details are in [architecture](../architecture.md); this ADR records rationale, not duplicate schemas.

## Alternatives

One overwrite is simpler but loses recovery; a database adds machinery for tiny documents; encryption is not needed for offline integrity.

## Evidence and limits

E4-10/E4-12; GDD-014. Filesystem crash behavior remains to test. Sources resolve in the [ledger](../../research/sources.md#stage-4-verification). No project build or benchmark is implied by reading documentation.

## Consequences

Generation ordering prevents stale async writes. Missing/corrupt/newer versions require explicit recovery decisions. Hashes do not prevent intentional edits.

## Reversal cost

Moderate after release: migrations must preserve bytes, use backups and prove rollback; run-resume would be a separate scope decision.
