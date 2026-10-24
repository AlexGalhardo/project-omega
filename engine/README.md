# Vendored Bevy fork

This directory is the project's own fork of [Bevy](https://bevy.org/), kept inside this repository under [DEC-018](../docs/decisions/decision-register.md#dec-018--engine-from-a-vendored-bevy-fork). There is no separate GitHub fork.

## Baseline

| Item | Value |
| --- | --- |
| Upstream release | Bevy 0.20.0 |
| Upstream commit | `b3bdd75becaffe9a16677cf5bae5bd6d198f052b` (from `bevy/.cargo_vcs_info.json`) |
| Source | The published crates.io packages of every `bevy*` 0.20.0 crate in `Cargo.lock`, copied verbatim on 2026-10-24 |
| Wiring | Root `Cargo.toml` `[patch.crates-io]` points each crate to `engine/bevy/<crate>`; `engine/` is excluded from the workspace |
| License | Bevy is MIT OR Apache-2.0; each crate keeps its own `LICENSE-*` files |

The published packages are used instead of the git repository because they contain only the crate sources (no multi-hundred-megabyte assets) and match the released version byte for byte.

## Layering

- `engine/bevy/` — the forked engine. Change it only when the game needs something Bevy cannot do through its public API, or to fix an engine bug.
- `crates/omega_2d/` — the project's own 2D engine layer (geometry, logical camera, visual catalog) built on the fork. Prefer adding reusable 2D features here first.
- `crates/omega_game/` — game rules. Never put game rules in either engine layer.

## Rules for changing the fork

1. Add an entry to the patch log below before or with the change: ID, crate, files, reason, upstream issue/PR if one exists, and the test that covers it.
2. Keep the upstream code style (4-space indentation, upstream rustfmt). The project's tab rule does not apply here.
3. Every patch has a regression test in `crates/omega_2d` or `crates/omega_game`, or a documented native check.
4. Upgrading to a new Bevy release: copy the new published crates into a clean `engine/bevy/`, re-apply each logged patch, drop patches upstream already fixed, and run the full quality gates.

## Patch log

| ID | Crate | Files | Reason | Covered by |
| --- | --- | --- | --- | --- |
| — | — | — | No patch yet; the fork equals upstream 0.20.0. | — |
