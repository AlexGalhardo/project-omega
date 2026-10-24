# ADR-011 — Ordered fixed simulation and explicit RNG ownership

Status: **Approved**, by owner blanket approval on 2026-10-24. Implementation evidence remains required; approval does not claim a tested result.

## Context

The GDD defines 60-tick combat and demands art-independent gameplay with honest determinism limits.

## Decision

**Recommendation / original design:** Use one ordered GameTick schedule in both client fixed updates and headless harness. Named ChaCha8 streams use stable seed/tag separation and unbiased integer draws. Critical math uses checked rationals; presentation has separate state/RNG. Canonical implementation details are in [architecture](../architecture.md); this ADR records rationale, not duplicate schemas.

## Alternatives

Variable-step combat complicates timings; lockstep determinism is unneeded; global RNG and unordered mutation make traces unstable.

## Evidence and limits

E4-04/E4-08/E4-09; GDD-005/006/016/DA-20. Sources resolve in the [ledger](../../research/sources.md#stage-4-verification). No project build or benchmark is implied by reading documentation.

## Consequences

Stable IDs/order and explicit flushes are required. Pause stops simulation/RNG. Supported repeatability is conditional on same build/target/content/input; no cross-platform promise.

## Reversal cost

Moderate; RNG or ordering changes invalidate fixtures/replay-like traces and require documented rebasing.
