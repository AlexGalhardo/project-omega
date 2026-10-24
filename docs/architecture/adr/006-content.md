# ADR-006 — Typed JSON and immutable per-run content

Status: **Approved**, by owner blanket approval on 2026-10-24. Implementation evidence remains required; approval does not claim a tested result.

## Context

Content is numerous enough to validate, and the owner already knows web data tooling.

## Decision

**Recommendation / original design:** UTF-8 JSON, typed Serde enums, versioned manifest, stable IDs and unknown-field rejection. Load/validate as a whole; freeze snapshot at run start. Explicit reload only at Menu. Canonical implementation details are in [architecture](../architecture.md); this ADR records rationale, not duplicate schemas.

## Alternatives

RON could improve Rust-centric authoring but is unnecessary for these schemas; scripts enlarge security/testing scope; runtime mutation undermines reproducibility.

## Evidence and limits

E4-07; GDD-012; research D/E. Sources resolve in the [ledger](../../research/sources.md#stage-4-verification). No project build or benchmark is implied by reading documentation.

## Consequences

Schemas and errors must be maintained; content fingerprints accompany traces. No reflection-only unchecked mutation or data containing executable formulas.

## Reversal cost

Low before shipping; later schema migration requires fixtures and ID preservation.
