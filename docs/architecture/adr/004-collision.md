# ADR-004 — Simple authored shapes without a physics dependency

Status: **Approved**, by owner blanket approval on 2026-10-24. Implementation evidence remains required; approval does not claim a tested result.

## Context

GDD-004/005 require circles, capsules, walls and swept projectiles, with no rigid bodies or terrain simulation.

## Decision

**Recommendation / original design:** Implement pure tested shape queries under ENG-GEOM-001. Begin with stable brute-force candidates in the game; use f64 LU geometry and checked integer/rational damage math. Canonical implementation details are in [architecture](../architecture.md); this ADR records rationale, not duplicate schemas.

## Alternatives

A physics library adds integration/configuration for unused features; spatial indexing may become useful after profiling. Neither is preapproved.

## Evidence and limits

GDD-004/005/006; research Track E. This is original design, not a benchmark result. Sources resolve in the [ledger](../../research/sources.md#stage-4-verification). No project build or benchmark is implied by reading documentation.

## Consequences

Geometry must handle tangency, zero-length sweeps, already-overlapping starts and ordered earliest impacts. No sprites-as-hitboxes or tunneling shortcuts.

## Reversal cost

Moderate if geometry changes; high if damage rules become coupled to a physics event stream. Preserve contract tests.
