# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **encounters:** Add the following beam profile (Survey Eye) and verify the Plate Hauler knockback rule (SHA_PLACEHOLDER)
- **encounters:** Add the swept charge and locked ground-strike profiles (Ram Cart, Clamp Walker) (SHA_PLACEHOLDER)
- **encounters:** Add the stationary radial volley and orbit-and-shoot profiles (Spark Hub, Coil Skater) (SHA_PLACEHOLDER)
- **encounters:** Add the enemy attack state machine, keep-distance movement, hostile projectiles and the Rivet Guard and Shard Hopper profiles (SHA_PLACEHOLDER)
- **combat:** Add payload fixture coverage for all twelve authored weapons, offset and centered circles ([9ae6352](https://github.com/AlexGalhardo/project-omega/commit/9ae635242c415c489d0b3f4a6a166f6bf8c74562))
- **engine:** Add phase-driven cue model (locked versus following anchors, cancellation, reduced effects) with contract tests ([9ae6352](https://github.com/AlexGalhardo/project-omega/commit/9ae635242c415c489d0b3f4a6a166f6bf8c74562))
- **research:** Record the RES-P4-001 Brotato study note (original prose, REA limits recorded, all differences kept as GDD) ([5651e70](https://github.com/AlexGalhardo/project-omega/commit/5651e704ef5443360afe7642603de7cda13a3060))
- **research:** Record the RES-P3-002 Brotato study note (original prose, REA limits recorded, all differences kept as GDD) ([5651e70](https://github.com/AlexGalhardo/project-omega/commit/5651e704ef5443360afe7642603de7cda13a3060))
- **research:** Record the RES-P3-001 Brotato study note (original prose, REA limits recorded, all differences kept as GDD) ([5651e70](https://github.com/AlexGalhardo/project-omega/commit/5651e704ef5443360afe7642603de7cda13a3060))
- **qa:** Add the PERF-02 prototype workload scenario (omega-sim --scenario perf02) and record three release-build runs of the three-wave loop ([5651e70](https://github.com/AlexGalhardo/project-omega/commit/5651e704ef5443360afe7642603de7cda13a3060))
- **qa:** Add the e2e script runner (headless and windowed), the seeded QA bot with cautious, aggressive and chaotic profiles, the QA checklist and the CI/check.ps1 e2e steps ([7c323b3](https://github.com/AlexGalhardo/project-omega/commit/7c323b303d3812101f69569999692e7bacc6f55b))
- **audio:** Add 13 original synthesized placeholder cues with provenance, event-to-cue mapping with grouping and once-only result latch, mix settings and the Bevy cue player (M mutes) ([5bd7a18](https://github.com/AlexGalhardo/project-omega/commit/5bd7a18be2a0e8f5cffa84f6ce8992c7e14a61f6))
- **ui:** Add the prototype HUD, shop, replacement prompt, pause, abandon and result screens as view models with keyboard and mouse widgets, plus --fast-forward-ticks for evidence captures ([8477a0a](https://github.com/AlexGalhardo/project-omega/commit/8477a0a77ddf46548e342d165e35c9f70e9885ec))
- **research:** Record the RES-P2-002 Brotato study note (original prose; REA cannot read the Godot package, so claims are Inferred or Unknown) ([0cd3d48](https://github.com/AlexGalhardo/project-omega/commit/0cd3d4800350f0772eced3ad2999671cb820a3ad))
- **research:** Record the RES-P2-001 Brotato study note (original prose; REA cannot read the Godot package, so claims are Inferred or Unknown) ([0cd3d48](https://github.com/AlexGalhardo/project-omega/commit/0cd3d4800350f0772eced3ad2999671cb820a3ad))
- **research:** Record the RES-P1-001 Brotato study note (original prose; REA cannot read the Godot package, so claims are Inferred or Unknown) ([0cd3d48](https://github.com/AlexGalhardo/project-omega/commit/0cd3d4800350f0772eced3ad2999671cb820a3ad))
- **run:** Add the run lifecycle (Wave, Shop, Victory, Defeat, Summary) with death precedence, Start guard, wave-entry reset, transient cleanup and idempotent settlement (DA-03, DA-11, DA-13) ([8fea1ba](https://github.com/AlexGalhardo/project-omega/commit/8fea1ba35db51ad599fc6046d57b4c25504e7252))
- **content:** Ship the prototype content subset in assets/content, load it in the client sandbox and validate it in the local gate and release package ([52836c6](https://github.com/AlexGalhardo/project-omega/commit/52836c68ef59b3d2f2b8d6c53bc5efd4c46ee41d))
- **shop:** Add atomic buy, upgrade, replacement and energy-allocation commands with stale-revision and no-RNG-change rejection (DA-02, DA-08, DA-09) ([2d848a3](https://github.com/AlexGalhardo/project-omega/commit/2d848a3637c54bf5179ea74576a189a0bbf80678))
- **shop:** Add shop eligibility, one-ceiling prices, two/two offer draws with fallback and one paid refresh ([2d848a3](https://github.com/AlexGalhardo/project-omega/commit/2d848a3637c54bf5179ea74576a189a0bbf80678))
- **client:** Add live presentation (SimId view map, pose interpolation, exact projectile core size, per-frame queue drain) and the --play sandbox with contract tests ([8d602b7](https://github.com/AlexGalhardo/project-omega/commit/8d602b7ddf6c9184dc6290b9e7065358c3ea33e5))
- **encounters:** Add chase enemies, authored batch scheduling, spawn warnings with postponement and cancellation, common scaling and timer completion with SP-008 contract tests (DA-11, DA-18) ([4e61fab](https://github.com/AlexGalhardo/project-omega/commit/4e61fab82f9b9726f516875bb3c812bc30263050))
- **combat:** Add damage, crit, knockback, death, scrap pickup, contact damage with 36-tick invulnerability and regeneration resolution with SP-007 contract tests (DA-05, DA-06, DA-13) ([0d879b3](https://github.com/AlexGalhardo/project-omega/commit/0d879b38095657dcce6fa3d22ae22f91db8b78ac))
- **combat:** Add the player actor spawn and tick movement (DA-01, DA-07) with SP-007 contract tests ([75e649a](https://github.com/AlexGalhardo/project-omega/commit/75e649ad8954481b9a6f90e9a914f6bdba1aa256))
- **input:** Add the keyboard/mouse action buffer, aim mapping and focus pause with SP-005 contract tests ([b6d8691](https://github.com/AlexGalhardo/project-omega/commit/b6d86910198fe3b3d074abaffa86ad1ef19f0124))
- **combat:** Add exact stat, fire-rate, damage, energy and regeneration evaluators with SP-006 contract tests ([29badf4](https://github.com/AlexGalhardo/project-omega/commit/29badf496b2ef21eceffe675343d51c38ddd5e62))

## [0.3.0] - 2026-10-24

### Added

- **engine:** Vendor the Bevy 0.20.0 fork in engine/bevy and pin Rust 1.99.1, exact dependency versions and the cargo-deny policy ([909932c](https://github.com/AlexGalhardo/project-omega/commit/909932cba1767973bff695693993fd1da1b94bc6))
- **engine:** Add the geometry, logical camera and visual catalog contracts with their acceptance tests ([be0bef2](https://github.com/AlexGalhardo/project-omega/commit/be0bef278e050c904a2ea5e7eaa8b821d6441cfa))
- **game:** Add the headless fixed-tick simulation, strict content validation for the full GDD catalog, diagnostics reports and the native preview client ([e18c775](https://github.com/AlexGalhardo/project-omega/commit/e18c77551ac899c387d21f4732163e29b84f7bed))
- **assets:** Add the offline SVG pipeline with safety checks, reproducible export and original proof art ([24fd02b](https://github.com/AlexGalhardo/project-omega/commit/24fd02b7c49e63a729e5ed0e83e0c68b1099da3a))
- **ci:** Add GitHub Actions CI, the signed release workflow and the local quality gate ([864a48d](https://github.com/AlexGalhardo/project-omega/commit/864a48d932f9138f8984d05a80faba8b13659983))
- **tooling:** Add editor, Markdown lint, graphify, caveman, Rust and Bevy skills and the session start routine ([0c3a39b](https://github.com/AlexGalhardo/project-omega/commit/0c3a39b29c34d393c5dbf064df6c35c709841d5f))

### Changed

- **roadmap:** Restructure ROADMAP.md as the onboarding file and record the Foundation evidence ([ea2a238](https://github.com/AlexGalhardo/project-omega/commit/ea2a2389bc58e46ea0597cd42443aa3f8539a332))
- **overnight:** Rewrite run-overnight.ps1 with a run cap, Sonnet model, safe stderr handling and no power-setting changes ([0c3a39b](https://github.com/AlexGalhardo/project-omega/commit/0c3a39b29c34d393c5dbf064df6c35c709841d5f))

### Removed

- **tooling:** Remove the empty ai-memory npm placeholder package; ai-memory is a native install ([0c3a39b](https://github.com/AlexGalhardo/project-omega/commit/0c3a39b29c34d393c5dbf064df6c35c709841d5f))

## [0.2.0] - 2026-10-24

### Added

- **planning:** Add the GDD (revision 2), Art Bible, architecture with twelve ADRs, eighteen specs, validation gates and the Stage 5 roadmap ([6892859](https://github.com/AlexGalhardo/project-omega/commit/6892859bb68b8952376a530d119c307a12107acf))
- **planning:** Record the independent planning review and its corrections, decisions DEC-011 to DEC-019 and change requests CR-001 to CR-003 ([6892859](https://github.com/AlexGalhardo/project-omega/commit/6892859bb68b8952376a530d119c307a12107acf))

### Removed

- **planning:** Remove the Portuguese overnight context, the HTTP ai-memory guide and .env.example ([6892859](https://github.com/AlexGalhardo/project-omega/commit/6892859bb68b8952376a530d119c307a12107acf))

## [0.1.1] - 2026-10-23

### Added

- **setup:** Initialize the project structure for overnight execution ([9d29e9c](https://github.com/AlexGalhardo/project-omega/commit/9d29e9cd29a0e2ed2ac6dbbb3724c935b2f8c939))
- **ai-memory:** Add the ai-memory configuration and integration guide ([e8421c8](https://github.com/AlexGalhardo/project-omega/commit/e8421c81708bbab22f37b7b5247e9ccdb334f814))
- **setup:** Add model selection rules for parallel work and the karpathy-guidelines requirement ([a6abc2f](https://github.com/AlexGalhardo/project-omega/commit/a6abc2f8078752b660d00029b85184453048157e))

### Security

- **setup:** Restrict permissions in .claude/settings.json ([c0d9547](https://github.com/AlexGalhardo/project-omega/commit/c0d95475748721bc5bdcf64a959d3c384898a0e8))
- **setup:** Remove the wildcard from bun run permissions ([b553738](https://github.com/AlexGalhardo/project-omega/commit/b553738806c97ccde6e999c83b3b09fca515ed51))

## [0.1.0] - 2026-10-23

### Added

- **research:** Document stage-one engine and game research ([3263ccd](https://github.com/AlexGalhardo/project-omega/commit/3263ccdfa89f1bd38f757bc394f8bbc335ea7d4e))
- **decisions:** Approve the upstream Bevy plugin strategy ([72fa123](https://github.com/AlexGalhardo/project-omega/commit/72fa12305ee186d3487251ae4ba1f70385cc6c04))
- **decisions:** Approve the Windows-only release scope ([10acdae](https://github.com/AlexGalhardo/project-omega/commit/10acdaeecf92b63f6fd9828b32796e1f8c4a7260))
- **decisions:** Record prior Rust and Bevy experience ([e11201c](https://github.com/AlexGalhardo/project-omega/commit/e11201cb98ba1180a00ad124f7bfc36522ccd847))
- **decisions:** Record weekly owner availability ([837ed9d](https://github.com/AlexGalhardo/project-omega/commit/837ed9d5986674babff4327fef50f092637882e9))
- **decisions:** Approve the compact content scope ([4ae1203](https://github.com/AlexGalhardo/project-omega/commit/4ae1203c4952305e0f5532825d93a81ca5ee8217))
- **decisions:** Approve the scrap robot industrial theme ([f3f020b](https://github.com/AlexGalhardo/project-omega/commit/f3f020b43fda6906ef571635f844693ad6b449aa))
- **decisions:** Complete the brainstorming defaults ([6396617](https://github.com/AlexGalhardo/project-omega/commit/6396617954e927fa8d9190043d1d478769d68b99))

[Unreleased]: https://github.com/AlexGalhardo/project-omega/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/AlexGalhardo/project-omega/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/AlexGalhardo/project-omega/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/AlexGalhardo/project-omega/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/AlexGalhardo/project-omega/releases/tag/v0.1.0
