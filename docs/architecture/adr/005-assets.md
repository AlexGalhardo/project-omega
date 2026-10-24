# ADR-005 — Offline SVG export and independent visual IDs

Status: **Approved**, by owner blanket approval on 2026-10-24. Implementation evidence remains required; approval does not claim a tested result.

## Context

The approved vector style must be replaceable with raster/pixel art without simulation changes.

## Decision

**Recommendation / original design:** Use pinned resvg 0.45.1 as the candidate offline exporter, committed generated PNGs initially, and ENG-VIS-001 catalogs. Use four texels/LU as the Art Bible's provisional density; validate on Windows before declaring the pipeline proven. Canonical implementation details are in [architecture](../architecture.md); this ADR records rationale, not duplicate schemas.

## Alternatives

Runtime vectors or runtime raster caching add loading/rendering complexity; atlases only after measured need.

## Evidence and limits

E4-13; Track F; ART-004/008/009/010. Sources resolve in the [ledger](../../research/sources.md#stage-4-verification). No project build or benchmark is implied by reading documentation.

## Consequences

Exporter features/command and fidelity are still implementation validation work. Reject external resources. Clip duration and image dimensions never govern gameplay.

## Reversal cost

Low for replacing textures/catalogs; moderate for changing the exporter; content rules remain unchanged.
