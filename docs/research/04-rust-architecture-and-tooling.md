# Tracks D and E — Rust tooling and ECS architecture

Status: Stage 1 research, 2026-10-23. All proposed policies and architecture choices are provisional. Sources: [ledger](sources.md#tracks-d-and-e). No project toolchain was installed and no Rust build, simulation or performance experiment was run.

## Track D: findings and minimal toolset

**Verified:** Bevy 0.20.0 declares Rust 1.97.1 and edition 2024 [C02]. Official Rust release announcements list 1.99.1, released 2026-10-15, as the latest entry consulted [D01]. **Recommendation:** evaluate stable 1.99.1 at Foundation and pin the exact tested toolchain in `rust-toolchain.toml`; recheck releases when implementation starts. The declared project MSRV must be tested against the resolved dependency graph, not merely copied from Bevy. Initially support the selected compiler only; a wider engine-library MSRV promise needs a concrete consumer. `rust-version` communicates support; rustup's file selects the toolchain [D02, D03].

**Verified:** Cargo workspaces share lockfile/output and support common dependency/lint declarations; root-only patches can override published crates [D04, D05]. **Recommendation:** one repo and one workspace, with a game library/binary and a small reusable plugin library when an actual boundary is known. Keep simulation separate from presentation at module/plugin level first. A separate simulation crate is justified if needed to keep headless dependency builds free of rendering/audio, not merely because it sounds architectural. Do not create one crate per system.

| Tool / practice | Why / evidence | Proposed adoption and alternative |
| --- | --- | --- |
| Cargo + rustup + committed application lockfile | **Verified:** compiler selection and locked dependency resolution are supported [D03, D15]. | **Recommendation:** required; use locked commands in CI. Avoid nightly without a reproduced need. |
| rustfmt + Clippy | **Verified:** standard lint execution and configuration are documented [D06]. | **Recommendation:** required; begin with standard lints, warnings denied in CI on the pinned toolchain, narrow reasoned exceptions. Do not enable every restriction/pedantic lint. |
| Rust unit/integration tests | **Verified:** native test organization supports private-unit and public-interface tests [D07]. | **Recommendation:** required; formulas and economy as pure tests, system interactions through a minimal app. |
| `cargo-deny` | **Verified:** checks advisories, licenses, dependency bans and sources [D08]. | **Recommendation:** one dependency-policy tool initially. RustSec/cargo-audit is an alternative vulnerability-only entry point [D09]; do not duplicate mandatory audit lanes without a need. |
| Property testing | **Verified:** Proptest generates and shrinks counterexamples [D10]. | **Recommendation:** add for meaningful invariants such as transaction conservation, bounded HP and content validation; retain failing seeds/cases. Ordinary example tests remain necessary. |
| Diagnostics / tracing | **Verified:** Bevy provides frame/entity diagnostics and tracing integrations [C14, D11]. | **Recommendation:** baseline raw frame samples first; add one profiler backend when diagnosing a bottleneck. Memory instrumentation adds overhead that must be recorded. |
| Criterion | **Verified:** supports statistical microbenchmarks [D12]. | **Recommendation:** optional for isolated collision/formula/content-loading comparisons; whole-frame benchmarks need their own scenarios. Benchmarks do not become noisy PR pass/fail gates. |
| Editor | **Recommendation:** rust-analyzer; editor choice remains personal [C18]. | No editor-specific build or test prerequisite. |

### Features, CI and build time

**Verified:** Cargo features are additive and can be unified across dependencies [D13]. **Recommendation:** keep few documented modes: shipping, development diagnostics, and headless validation as needed. Test supported combinations explicitly; do not equate `--all-features` with a valid release configuration. Avoid letting a test dependency silently enable the renderer in a supposedly headless lane.

**Recommendation:** initial CI should run formatting, linting, unit/integration/headless tests, content validation once content exists, and a release build on the first supported OS. Add a native lane for each additional shipping OS. Keep GPU/UI/input-device/packaging checks separate from headless proof. Pin third-party Actions to full commits, minimize token permissions, and keep release secrets out of untrusted PR execution, following GitHub guidance [D14]. Cache keys must distinguish toolchain, target, features and lockfile.

Commands such as `cargo fmt --all -- --check`, `cargo test --workspace --locked`, `cargo clippy --workspace --all-targets --locked`, and `cargo deny check` are **candidate validation commands**, not executed checks or a final task contract. Stage 4 must supply exact feature/package selections and required environment. Current Clippy documentation supports `CARGO_BUILD_WARNINGS=deny` on sufficiently recent Cargo [D06]; use shell-appropriate environment syntax.

**Verified:** Bevy documents optional dynamic linking and alternative linkers, including Windows caveats; Cargo provides profile overrides and build timing reports [C18, D15, D16]. **Recommendation:** first measure default clean build, no-change build, single game-file edit and engine-plugin edit. Then compare dependency optimization, dynamic linking, and a platform-appropriate linker one variable at a time. Record CPU, RAM, disk, OS, compiler, dependency graph and cache state. Do not carry published speedup ratios into project estimates. Dynamic linking should remain a development option and be absent from distribution unless deliberately justified. A fork is not a build-time optimization strategy.

### Alternatives, risks and unknowns

Separate engine/game repos enable independent releases but add synchronized changes and dependency publication/pinning overhead. A monorepo makes cross-boundary changes atomic; later extraction remains possible. Git dependencies should use full SHA pins if genuinely needed. An emergency root `[patch]` must actually resolve in `cargo tree`/lockfile; patching only the umbrella crate may not replace all affected Bevy subcrates [D05].

**Unknown:** owner's Rust fluency, available time, first target OS, actual build environment and dependency versions. **Inferred risk:** aggressive linting, many crates, broad platform matrices and several overlapping test tools can consume more maintenance than they save. **Recommendation:** add tools only against a documented failure or verification need.

## Track E: simulation and presentation

**Verified:** Bevy supports fixed schedules and minimal apps, but neither implies deterministic gameplay [C04, C05, C13, C24]. The Rand project explicitly distinguishes reproducible output from API compatibility; general-purpose RNG aliases may not preserve streams across versions/platforms [E01].

**Recommendation; origin: original design:** the authoritative simulation owns logical positions, velocity, health, cooldowns, collision, inventory, currency, wave state and RNG. Presentation consumes state and tick-tagged outcomes to render transforms, animation, VFX, sound and HUD. Art handles, animation frame counts and camera effects must not determine combat outcomes. The same simulation plugin must run without presentation.

Proposed conceptual flow (not a final schedule contract):

`device input -> action buffer -> fixed tick: commands -> spawn/move/target -> attack/contact -> resolve outcomes -> death/rewards/transitions -> presentation extraction`

Stage 4 must define exact ordering, command flush boundaries and priorities. Examples requiring explicit rules include death and pickup on the same tick, the final hit at a wave deadline, two buyers of the last offer, and a projectile whose source has despawned. Do not leave those to incidental system order.

### Time and input

**Recommendation:** sample devices each rendered frame; buffer edges for one-time consumption and hold continuous actions across ticks. Handle zero-tick and multi-tick frames without losing or repeating a press. Decide pause behavior separately for simulation, UI and audio. A catch-up cap can change time progression, so specify its consequence instead of silently skipping game outcomes. Render interpolation may smooth motion but must not feed positions back into collision. Select tick frequency after the first control/collision experiment; no frequency is approved here.

### Realistic determinism contract

| Level | Proposed position |
| --- | --- |
| Formula and transaction tests | **Recommendation:** exact results for defined integer/rounding operations; floating-point comparisons use a rule-specific tolerance where appropriate. |
| Same-build seeded simulation | **Recommendation:** aim for repeated outcomes on the same target/toolchain, content hash, seed and tick-indexed input. Prove this with replayed fixtures and canonical state digests before claiming support. |
| Different frame rates on one build | **Recommendation:** compare equivalent tick inputs and fixed-tick counts; rendering-only randomness must not change gameplay. Device-sampling differences are a separate input problem. |
| Cross-platform / cross-version bit identity | **Unknown / not proposed as an initial guarantee.** Floating point, math implementations, RNG/distribution changes, container order and scheduling can diverge. |
| Lockstep multiplayer / persistent replay compatibility | **Not assumed.** Requires an owner requirement and substantially stronger contracts. |

**Recommendation:** own RNG streams by domain (for example spawning, loot/shop and combat) with an explicitly selected, versioned algorithm; cosmetics get an independent stream. Specify seed derivation and sampling algorithms, not only a seed number. Use stable game IDs for save/replay identity and deterministic tie-breaking; transient ECS entity allocation is not a content ID. Avoid unordered iteration changing draw order. Do not enable extra parallelism in authoritative resolution before its semantics are tested.

### Content and reloads

**Verified:** Serde supports typed serialization across formats; RON provides Rust-like structures/enums/comments with documented limitations [E02, E03]. Bevy asset loading can notify changes [C08].

| Alternative | Trade-off (Inferred) |
| --- | --- |
| RON | Comfortable typed enum authoring and comments; tighter Rust vocabulary and less general editor tooling. |
| JSON | Familiar to a web developer and broad tooling; verbose enum encoding and no standard comments. |
| TOML | Appropriate for shallow settings; complex nested gameplay effects become less convenient. |
| Code-only definitions | Strong compile-time checking but slow content iteration and unnecessary coupling to rebuilds. |

**Recommendation:** provisionally prefer JSON + typed Serde definitions for the owner's web background, unless the eventual effect schemas make RON materially clearer. This is reversible before content grows and remains an explicit Stage 2/4 decision. Keep format choice separate from semantics.

Proposed validation: schema version, unique stable IDs, reference existence, finite numerics, units, ranges, nonempty eligible offer pools, bounded trigger chains, valid tier progression, resolvable visual IDs and deliberate defaults. Produce errors with content ID and field path; reject invalid snapshots atomically. Keep the last valid snapshot in development. Lock gameplay definitions for a run or switch at an approved boundary; cosmetic hot reload can be separate. Save schema and migrations require their own decision rather than serializing reflection internals.

### Events and lifetimes

**Verified:** Bevy 0.20 differentiates buffered `Message`s from observer-triggered `Event`s [C22]. Message collections retain only a bounded number of update cycles by default; paused/skipped readers can miss them, and never updating buffers causes growth [E04].

**Recommendation:** use explicit ordered batches for authoritative combat outcomes; use typed notifications for presentation. Persistent transactions/unlocks must live in authoritative state until processed, not only in transient messages. Record ownership, producer/consumer schedules, deduplication, lifetime and cleanup for every channel. Avoid observer recursion becoming an invisible gameplay pipeline. No custom universal event bus is justified.

### Collision alternatives

**Recommendation:** evaluate simple circle/segment overlap and swept-projectile queries first if approved mechanics only need kinematic arena motion. A physics plugin becomes attractive for genuine rigid-body contacts, complex shapes or solver-dependent movement. Its release compatibility and determinism would require separate verification. Do not select a physics library or write a general solver before the mechanics decision.

## Measurement plan — proposed experiments, not budgets

**Verified:** Bevy archetypes represent component combinations and persist even when empty [E05]. **Inferred risks:** frequent component-set changes can multiply archetypes; naive all-pairs collision can dominate CPU time; spawn/despawn bursts can create allocation spikes; materials, transparency and effects can stress rendering. None has been measured in this game.

| Scenario | Measurement | Decision it enables |
| --- | --- | --- |
| Empty rendered scene and headless baseline | Raw frame/tick samples; CPU/GPU work separately, memory and entity counts. | Establish overhead before attributing cost to gameplay. |
| Enemy/projectile sweep | Suggested experimental ladder: 100/500/1,000 enemies crossed with 0/500/2,000 projectiles, plus approved design loads later. | Find scaling limits; these are test inputs, not promised content counts or capacity targets. |
| Dense collisions | Overlapping crowds, sweeping melee and fast projectiles; candidate pairs, resolved contacts, missed/duplicate hits. | Compare naive queries with a uniform grid only if evidence warrants it; assess tunneling separately. |
| Spawn/death bursts | Same total population with staggered vs burst creation/removal; allocations and spike duration. | Decide whether batching/capacity reservation helps. Pooling requires evidence plus reset correctness. |
| Archetype variation | Add/remove effects vs stable data at equal entity count; archetype/table counts and tick cost. | Decide representation without prematurely homogenizing all entities. |
| Render stress | Repeated vs distinct images/materials, atlas variants, transparency, VFX and dense HUD. | Measure draw calls, batches, GPU time and memory before atlas/custom-renderer work. |
| Long session / repeated runs | Run beyond the proposed maximum session length once decided; repeated scene/shop transitions. | Detect retained entities/assets/messages, memory growth, stale state and save corruption. |
| Build iteration | Clean and representative incremental edits with controlled cache state. | Decide compiler profile, linker and dynamic-linking options. |

**Recommendation:** record warmup, repetitions, sample count, raw distributions and P50/P95/P99, max spikes, CPU/GPU timing, allocations, resident memory, entity/archetype counts and draw calls when available. Document unavailable counters instead of estimating them. Use release-like builds with symbols, fixed content/seed/input and a fixed resolution/VSync policy; separate profiler overhead. Retain hardware/driver/power/thermal conditions. Establish budgets only after the owner selects minimum hardware and measurements show feasible quality settings.

## Recommendation, risks and unknowns

Prefer a small, explicitly ordered simulation with a thin presentation boundary and versioned data. Single-threaded authoritative ordering is an initial correctness option, not a blanket performance claim. Reconsider based on profiling. Prefer native Bevy mechanisms over extra frameworks.

**Unknown:** actual determinism, collision load, content format preference, save requirements, hardware and baseline. **Inferred risk:** a false promise of replay/lockstep determinism can force late redesign; presentation-dependent hitboxes can make art replacement change gameplay. Proposed validation must address these before corresponding guarantees enter the GDD/specs.
