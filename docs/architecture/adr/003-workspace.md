# ADR-003 — One repository and two crates

Status: **Approved**, by owner blanket approval on 2026-10-24. Implementation evidence remains required; approval does not claim a tested result.

## Context

The game and reusable layer need clear dependency direction without multi-repository release overhead.

## Decision

**Recommendation / original design:** One resolver-3 workspace: omega_2d and omega_game. Game depends on engine; engine never depends on game. Binaries share the game library. Other domains stay modules. Canonical implementation details are in [architecture](../architecture.md); this ADR records rationale, not duplicate schemas.

## Alternatives

One monolithic crate weakens the reusable boundary; many tiny crates add setup/churn; separate repositories require coordinated revisions before a second consumer.

## Evidence and limits

E4-05; DEC-001/DEC-010; concrete contracts ENG-GEOM-001, ENG-CAM-001, ENG-VIS-001. Sources resolve in the [ledger](../../research/sources.md#stage-4-verification). No project build or benchmark is implied by reading documentation.

## Consequences

Shared manifest/lockfile is serially owned. Separate client feature isolates headless builds. Existing Node tooling is not inherited as a game prerequisite.

## Reversal cost

Moderate if extracting engine later; stable IDs and dependency direction reduce migration work.
