# ADR-008 — Bevy devices with game-owned action mapping

Status: **Approved**, by owner blanket approval on 2026-10-24. Implementation evidence remains required; approval does not claim a tested result.

## Context

Optional manual aim, fixed ticks and UI focus require consistent edge/held semantics.

## Decision

**Recommendation / original design:** Use upstream device input with game-owned remappable actions; capture frame state before fixed updates, consume edges once, and separate UI from tick intent. ENG-CAM-001 maps cursor positions; active controller loss pauses. Canonical implementation details are in [architecture](../architecture.md); this ADR records rationale, not duplicate schemas.

## Alternatives

An action-mapping plugin may be justified later but adds an unverified compatibility dependency now. Polling raw input inside every fixed tick loses/repeats edges.

## Evidence and limits

E4-01/E4-04; GDD-004/015. Sources resolve in the [ledger](../../research/sources.md#stage-4-verification). No project build or benchmark is implied by reading documentation.

## Consequences

Native controller/keyboard tests remain mandatory. Device glyphs are presentation; action IDs are stable. No gameplay action is inferred from an animation.

## Reversal cost

Moderate; action contract allows replacing backend while keeping tick fixtures and UI behavior.
