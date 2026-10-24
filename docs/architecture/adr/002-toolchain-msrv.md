# ADR-002 — Pinned toolchain and explicit support floor

Status: **Approved**, by owner blanket approval on 2026-10-24. Implementation evidence remains required; approval does not claim a tested result.

## Context

A solo developer needs reproducible native builds and a clear supported compiler.

## Decision

**Recommendation / original design:** Pin Rust 1.99.1, edition 2024, project MSRV 1.99.1, x86_64-pc-windows-msvc and the dependency pins in architecture section 2, refreshed by [DEC-015](../../decisions/decision-register.md#dec-015--dependency-and-tool-pins-refreshed-to-latest-stable). Commit lockfile; upgrade deliberately after a release-specific check. Canonical implementation details are in [architecture](../architecture.md); this ADR records rationale, not duplicate schemas.

## Alternatives

A floating stable toolchain drifts; supporting Bevy's lower minimum adds a compiler matrix without a required consumer. Default linker first; optimization after measurements.

## Evidence and limits

E4-01, E4-06 through E4-11; no project build yet. Sources resolve in the [ledger](../../research/sources.md#stage-4-verification). No project build or benchmark is implied by reading documentation.

## Consequences

First dependency resolution may reveal a concrete incompatibility. Report it rather than silently changing pins. Release/license requirements do not follow automatically from version selection.

## Reversal cost

Low before content/code grows; moderate after a verified release, requiring migration and regression evidence.
