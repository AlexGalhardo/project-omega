# Evidence index

Each task records evidence in `docs/validation/evidence/<TASK-ID>/` or in this index. Results below are actual runs; nothing here is a planned command.

## Phase 1 Foundation — 2026-10-24

Environment: Windows 11 Pro 10.0.26300, Ryzen 7 5700X3D, RTX 4060, 32 GiB (performance-plan reference machine). Rust 1.99.1 (`58cf8f9c3 2026-10-13`) via `rust-toolchain.toml`, cargo-deny 0.20.2, resvg 0.48.1, Bevy 0.20.0 from the vendored fork (`engine/bevy/`, upstream `b3bdd75b`). Commit: the `feat` commits of release 0.3.0 (see CHANGELOG.md). Executor: Foundation integrator (Claude Opus 5.5 session); reviewer outcome: integrator self-review only, no independent review yet.

### Commands run and results

| Command | Result |
| --- | --- |
| `cargo metadata --locked --format-version 1` | exit 0; Bevy crates resolve as path dependencies from `engine/bevy/` |
| `cargo fmt --all -- --check` | exit 0 |
| `cargo clippy --workspace --no-default-features --all-targets --locked -- -D warnings` | exit 0 |
| `cargo clippy -p omega_game -p omega_2d --features client --all-targets --locked -- -D warnings` | exit 0 |
| `cargo test --workspace --no-default-features --locked` | exit 0; 38 tests passed, 0 failed |
| `cargo check -p omega_game --features client --locked` | exit 0 |
| `cargo test -p omega_2d --features client --locked --test visual_contract --test asset_contract` | exit 0; 7 tests |
| `cargo deny check` | `advisories ok, bans ok, licenses ok, sources ok`; MIT-0 (encase) reviewed and allowed; duplicate-version warnings only |
| `omega-content validate crates/omega_game/tests/fixtures/content/full --profile full` | exit 0, revision `37313bba…` (4/12/40/10/2/10/4) |
| `omega-content validate …/prototype --profile full` | exit 1, 7 count errors (expected rejection) |
| `omega-sim --scenario smoke --seed 0…0 --ticks 600` | exit 0, final tick 600 |
| `omega-sim --scenario repeatability --seed 0…0 --ticks 600` | exit 0, 600 identical ticks |
| `omega-sim --scenario smoke --seed abc --ticks 600` | exit 2, rejected before creating a run |
| `scripts/export-assets.ps1 -Manifest assets/presentation/proof-manifest.json` | exit 0, 7 assets |
| `scripts/validate-assets.ps1 -Manifest … -Reproduce` | exit 0, `valid: 7 assets, reproducible` |
| `bun run lint` (markdownlint-cli2 0.24.0) | exit 0 |

### Acceptance mapping

| Task | Criterion | Test or evidence |
| --- | --- | --- |
| ENG-P1-001 | SP-001-AC1 | Locked core tests and client check above, Rust 1.99.1, Bevy 0.20.0 |
| ENG-P1-001 | SP-001-AC2 | `omega_2d/tests/bootstrap_contract.rs::headless_engine_app_has_no_device_resources`, `omega_game/tests/bootstrap_contract.rs::headless_core_has_expected_plugins_and_no_devices` |
| QA-P1-001 | SP-001-AC3 | `.github/workflows/ci.yml` (windows-2022 Rust job, cargo-deny job, Markdown job), actions pinned by reviewed SHA; [hosted run 40075863365](https://github.com/AlexGalhardo/project-omega/actions/runs/40075863365) green |
| QA-P1-001 | SP-001-AC4 | Stage-grouped commits with explicit paths (git history of release 0.2.0 and 0.3.0) |
| GAME-P1-001 | SP-002-AC1 | `sim_contract.rs::ac1_identical_seed_and_input_give_identical_traces` |
| GAME-P1-001 | SP-002-AC2 | `sim_contract.rs::ac2_six_hundred_manual_ticks_without_a_window` |
| GAME-P1-001 | SP-002-AC3 | `sim_contract.rs::ac3_harness_pause_advances_nothing`, `::ac3_app_frames_while_paused_do_not_build_a_backlog` |
| GAME-P1-001 | SP-002-AC4 | `sim_contract.rs::ac4_records_from_many_ticks_arrive_once_in_order`, `::schedule_sets_run_in_documented_order` |
| GAME-P1-002 | SP-003-AC1 | `content_contract.rs::ac1_*` (duplicate id, unknown field, dangling reference, NaN-like text, unsupported enum) |
| GAME-P1-003 | SP-003-AC2/3/4 | `content_contract.rs::ac2_*`, `::ac3_*`, `::ac4_*`, `::boss_schedule_rules_from_cr_001`; `omega-content` runs above |
| ENG-P1-002 | SP-004-AC1–4 | `geometry_contract.rs::ac1_*`, `::ac2_*`, `::ac3_*`, `::ac4_generated_translation_and_swap_invariants` (5 seeds × 2000 cases) |
| ENG-P1-003 | SP-012-AC1 | `camera_contract.rs` (4 tests, five display sizes) |
| ENG-P1-004 | SP-012-AC2/AC3 | `visual_contract.rs` (5 tests); native preview screenshots in [GAME-P1-004/](GAME-P1-004/) |
| ENG-P1-005 | SP-013-AC1/AC3 | `scripts/lib/Assets.psm1` `Test-SvgSource` rejects script, foreignObject, image, text, DTD, event handlers, external href/url and @import before resvg runs; `validate-assets.ps1 -Reproduce` |
| GAME-P1-004 | SP-013-AC2/AC4, DA-20 | `asset_contract.rs` (2 tests), `skin_swap_contract.rs::da20_catalog_swap_keeps_trace_identical`, lock file `assets/presentation/proof-manifest.lock.json` (hashes, command, resvg version) |
| QA-P1-002 | SP-016-AC1/AC2 subset | `diagnostic_contract.rs` (2 tests); PERF-01 and PERF-08 reports in [QA-P1-002/](QA-P1-002/) |

### Native evidence

- Proof preview, `omega-game --resolution WxH --screenshot PATH`: [1280x720](GAME-P1-004/proof-svg-1280x720.png), [1920x1080](GAME-P1-004/proof-svg-1920x1080.png), raster skin [1280x720](GAME-P1-004/proof-pixel-1280x720.png). Player, friendly and hostile cores, ground warning, the enemy category fallback (shown in place of the missing `visual_enemy_chaser`) and the UI icon render; letterboxing fits 16:9.
- Readability observation (REV-009, early AA-04): at 1280x720 the arena maps to 1.067 px/LU, so the bolt core (8 LU) is about 8.5 px and the smallest hostile core (10 LU circle inside a 14.1 LU diamond) about 15 px. Both are distinguishable by color and shape in the capture, but the friendly core is small. **Pending owner review** of the screenshots; AA-01/05/07 human sign-off keeps GAME-P1-004 IN_REVIEW.
- PERF-01, release build, empty proof scene, no vsync, 2 s warm-up, 20 s capture (shorter than the plan's 120 s; recorded as a deviation), three runs per size. CPU frame wall time p50 0.93–0.96 ms, p95 1.16–1.26 ms, p99 1.31–1.51 ms; max 2.4–6.7 ms except one 77.5 ms spike (1280x720 run 2). GPU time, allocations, working set, draw calls and texture memory: Unsupported. No budget is assigned.
- PERF-08, dev profile (dependencies at opt-level 2), MSVC default linker: no-change build 0.79–0.86 s; single game-module edit 11.6–17.2 s. Cold build not measured (`-Cold` switch available).

### Limits

- Hosted CI on commit `ea2a238`: [run 40075863365](https://github.com/AlexGalhardo/project-omega/actions/runs/40075863365), all three jobs green (Rust on windows-2022, cargo-deny, Markdown).
- Visual fidelity and readability need the owner's native review (GAME-P1-004, QA-P1-002 stay IN_REVIEW).
- SP-016 is not complete: combat stress scenarios (PERF-02–07) arrive with SP-007/SP-008.
