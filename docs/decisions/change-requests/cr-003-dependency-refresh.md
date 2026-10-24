# CR-003 — Refresh dependency and tool pins before the first lockfile

Date: 2026-10-24. Author: Foundation integrator. Status: **Approved** under [DEC-015](../decision-register.md#dec-015--dependency-and-tool-pins-refreshed-to-latest-stable). Blocking scope: ENG-P1-001.

## Evidence

`cargo info` on 2026-10-24 (Windows 11, cargo 1.99.0) reported: bevy 0.20.0, serde 1.0.229, serde_json 1.0.154, rand_chacha 0.10.0, getrandom 0.4.3, sha2 0.11.1, resvg 0.48.1, cargo-deny 0.20.2. `rustup check` reported Rust 1.99.1 as current stable. Architecture section 2 named serde 1.0.228, serde_json 1.0.145, rand_chacha 0.9.0, getrandom 0.3.4, sha2 0.10.9, resvg 0.45.1 and cargo-deny 0.18.4.

## Alternatives

1. Keep the Stage 4 candidates. Starts the project already behind and conflicts with the owner's latest-stable rule.
2. Refresh every pin to the latest stable release before the first lockfile exists.

## Recommendation and resolution

Alternative 2. Bevy and Rust are unchanged. rand_chacha 0.10 changes the trait re-export path; the simulation module adapts to it. getrandom is not added until native run creation (GAME-P3-011) needs it, then at its latest stable version. Architecture section 2 now references DEC-015.

## Validation

ENG-P1-001 evidence records `cargo metadata --locked`, the resolved feature graph and `cargo deny check` output.
