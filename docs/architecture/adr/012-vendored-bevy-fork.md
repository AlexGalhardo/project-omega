# ADR-012 — Vendored Bevy fork as the engine base

Status: **Approved** under [DEC-018](../../decisions/decision-register.md#dec-018--engine-from-a-vendored-bevy-fork), 2026-10-24. Supersedes [ADR-001](001-bevy-strategy.md).

## Context

The owner's second primary objective (DEC-017) is to own a 2D-focused engine built as a direct fork of Bevy, for learning. ADR-001 chose upstream crates without a fork.

## Decision

Vendor the published Bevy 0.20.0 crates into `engine/bevy/` and redirect crates.io to them with `[patch.crates-io]`. Edit the fork in place when the game needs engine code or hits an engine bug. Keep project-specific reusable 2D facilities in `crates/omega_2d`; keep game rules in `crates/omega_game`. Record every engine change in the [patch log](../../../engine/README.md#patch-log).

## Alternatives

A GitHub fork pinned by SHA (rejected by the owner); upstream-only crates (ADR-001, superseded); a git submodule (extra tooling, same upgrade cost).

## Consequences

The lockfile resolves Bevy crates as path dependencies. Workspace `cargo fmt`, Clippy and tests do not cover `engine/` (excluded from the workspace); engine patches are validated by the contract tests that exercise them. Upgrades are manual: copy the new release, re-apply the patch log, run the gates.

## Reversal cost

Low while the patch log is empty: delete the patch section and `engine/`. Grows with each logged patch.
