# Project Omega

[![CI](https://github.com/AlexGalhardo/project-omega/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexGalhardo/project-omega/actions/workflows/ci.yml)

A didactic Rust project with two goals (DEC-017): re-create an arena-survival roguelite in the style of Brotato from study of a purchased copy, with original code and art, and build a 2D-focused game engine as a fork of [Bevy](https://bevy.org/). `Project Omega` is a working title.

**Start here:** [ROADMAP.md](ROADMAP.md) names the current phase and the next task and holds the plan rules. Agents also read [AGENTS.md](AGENTS.md).

## Status

Phase 1 (Foundation) is done: reproducible workspace, headless fixed-tick simulation, strict content validation for the full GDD catalog, geometry/camera/visual-catalog engine contracts, an offline SVG pipeline with original proof art, a native preview client and a first performance baseline. Phase 2 (Prototype: the first playable loop) is next. Evidence: [docs/validation/evidence/](docs/validation/evidence/README.md).

## Layout

| Path | Contents |
| --- | --- |
| `engine/bevy/` | Vendored Bevy 0.20.0 fork, patched in place ([engine/README.md](engine/README.md)) |
| `crates/omega_2d/` | The project's 2D engine layer: geometry, logical camera, visual catalog |
| `crates/omega_game/` | Game rules, content, simulation, diagnostics, native client; binaries `omega-game`, `omega-content`, `omega-sim` |
| `assets/`, `art/source/` | Runtime assets and their SVG sources |
| `docs/` | GDD, Art Bible, architecture and ADRs, specs, decisions, validation, research |
| `scripts/` | Local gate, asset export/validation, measurement, release notes |

## Requirements

Windows 11 x64 with the MSVC build tools, [rustup](https://rustup.rs/) (the pinned Rust 1.99.1 installs from `rust-toolchain.toml`), `cargo install cargo-deny --version 0.20.2 --locked`, `cargo install resvg --version 0.48.1 --locked` (asset export only) and [Bun](https://bun.sh/) (Markdown lint only).

```powershell
powershell -NoProfile -File scripts/check.ps1           # every local gate
cargo run -p omega_game --locked --bin omega-game        # native preview
cargo test --workspace --no-default-features --locked    # headless tests
```

## Documents

[GDD](docs/gdd.md) · [Art Bible](docs/art-bible.md) · [Architecture](docs/architecture/architecture.md) · [Specs](docs/specs/index.md) · [Decisions](docs/decisions/decision-register.md) · [Quality gates](docs/validation/quality-gates.md) · [Release process](docs/release.md) · [IP policy](docs/legal/ip-policy.md) · [Changelog](CHANGELOG.md)

Repository artifacts are English; the owner's conversation is Brazilian Portuguese. Nothing extracted from Brotato is stored here; see the IP policy.
