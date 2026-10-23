# Track C — Bevy and fork strategy

Status: Stage 1 recommendation, 2026-10-23; not an approved architecture. Sources: [ledger](sources.md#track-c). No Bevy build, test, benchmark or fork modification was executed.

## Release verification and conflicting documentation

**Verified:** GitHub's latest non-prerelease resolved to **v0.20.0**, published 2026-10-08; the tagged manifest declares package 0.20.0, Rust edition 2024, `rust-version = "1.97.1"`, and `MIT OR Apache-2.0` [C01, C02]. The official announcement corroborates the release [C25]. This is the research baseline; dependency selection still requires a build on the approved platform.

**Verified disagreement:** the quick-start search result still shows 0.19, and the feature documentation served for 0.20.0 contains a 0.18 example [C20, C26]. The migration guide also describes changed sprite/UI internals [C15]. Prefer the exact release's manifest, source and examples over stale tutorial snippets. Do not interpret a search snippet or `latest` URL as a version pin.

**Verified:** Bevy documents roughly three-to-four-month releases, without fixed dates [C16]. Actual announcements include 0.18 on January 13, 0.19 on June 19 and 0.20 on October 8, 2026 [C25]. **Inferred:** cadence is a planning expectation, not a guaranteed quarterly deadline; both game code and ecosystem plugins incur migration work.

## Capability assessment

The first column is **Verified from source/documentation**. The second is **Recommendation / Inferred suitability**, not proof that our game already runs.

| Capability and evidence | Proposed use and practical boundary |
| --- | --- |
| ECS: typed components, queries, resources, systems and configurable schedules [C05, C21]. | Organize simulation around data ownership. No extra ECS wrapper or homegrown scheduler is justified. Implementation details can change; public APIs are the integration boundary. |
| Scheduling: dependencies, sets, ambiguity diagnostics, deferred commands; 0.20 has weak ordering and a schedule shuffle option [C05, C15]. | Explicitly order combat stages and command visibility. Safety from borrow checking does not establish game-semantic order or determinism. |
| States: standard states, substates, computed states, transition schedules [C06]. | Express boot/menu/run/shop/result transitions. Transition guards and simultaneous victory/death priority remain game rules. |
| Fixed time: `Time<Fixed>` follows virtual time; `FixedUpdate` can run zero or multiple times per rendered update. Default is 64 Hz [C04]. | Choose the simulation rate explicitly later. Sample input outside fixed ticks; define consumption, pause, catch-up and interpolation. Default frequency is not a measured project requirement. |
| 2D: tagged example creates `Camera2d` and an image-backed `Sprite`; sprite data supports images, atlases and custom size [C07, C23]. | Sprites and meshes are sufficient starting points. A custom renderer needs a demonstrated requirement. Test the exact 0.20 pipeline when implementing. |
| Assets: typed handles, asynchronous loading, dependency readiness, custom loaders, optional file watching [C08]. | Loading/error states and validated content swaps are project responsibilities. Asset hot reload is not automatic safe live gameplay-rule mutation or Rust-code reload. |
| Input: keyboard, mouse, gamepad, touch [C09]. | Define action-level game commands above device input. Rebinding, focus loss, dead zones and controller navigation need acceptance tests. |
| Audio: source handles, audio entities, playback controls, volume and spatial support [C10]. | Begin with native audio for basic SFX/music. Mixing policy, voice limits, device loss and accessibility remain to validate. |
| UI: nodes, text, flex/grid layout, focus/navigation-related modules and widget APIs [C11]. | Suitable starting point for HUD and shop. A web background helps layout concepts but does not make browser behavior or accessibility automatic. 0.20 deprecations require version-specific examples. |
| Reflection: type metadata, introspection and serialization support [C12]. | Useful for development tools and selected assets; not a promise of stable save compatibility. Avoid serializing the entire ECS as the save contract. |
| Plugins: public lifecycle hooks configure an `App` [C03]. | Reusable 2D services fit this extension mechanism. Plugins are code organization, not stable binary ABI or runtime DLL modules. |
| Features: default enables 2D, 3D, UI and audio; smaller collections and `dynamic_linking`/`file_watcher` exist [C02, C20]. | Disable defaults and select needed collections after a build check. Do not hand-maintain a huge feature matrix. Feature removal is not a measured speedup yet. |
| Headless: tagged examples demonstrate no-window execution and manually updating a minimal app [C13, C24]. | Run simulation tests without GPU/audio; initialize the required plugins, resources and time deliberately. A headless pass does not validate rendering or packaging. |
| Diagnostics: frame-time and entity-count diagnostics [C14]. | Add raw samples for percentiles and scenario metadata. FPS averages are insufficient for latency or stability budgets. |
| Platforms: site lists Windows, macOS, Linux, Web, iOS and Android [C17]. Linux has native build dependencies [C19]. | Each shipping platform needs its own test/packaging lane. Mobile/web support is not parity, certification, or a reason to include them in 1.0. No console support promise is made. |

## A / B / C comparison

All cost judgments below are **Inferred**, based on the extension APIs, migration surface and release process. There is no measured project migration cost; no hours-per-release estimate is defensible yet.

| Dimension | A — Full fork | B — Upstream + own plugins | C — Minimal patch fork + plugins |
| --- | --- | --- | --- |
| Maintenance | Own all divergent internals and their regressions. | Own project code and dependencies; upstream owns engine internals. | Own a bounded patch stack plus the same plugin obligations. |
| Sync per release | Resolve conflicts, understand changed invariants, rerun engine and game validation. | Migrate public API use and compatible plugins; no engine merge conflicts. | All B costs plus rebase, reproduction tests and removal checks for every patch. |
| API stability | Can freeze a private API, but inherits security/platform upkeep and ecosystem divergence. | Public API still changes before 1.0; pin and migrate deliberately. | Private patched APIs add an extra compatibility surface. |
| Build time | Not inherently faster; source changes can invalidate large engine builds. | Same underlying engine; feature/profile choices can be measured independently. | Patched crates can increase rebuild work; must measure. |
| Velocity | Immediate internal freedom, substantial distraction risk for one owner. | Best initial route for requirements served by existing hooks. | Useful for a specific upstream blocker with a small maintainable fix. |
| Future-game reuse | Reusable only if fork remains healthy and general-purpose. | Plugins can transfer without taking an engine fork with them. | Reuse requires matching patch dependencies until fixes land upstream. |
| Upstreaming | Large mixed redesigns are harder to isolate and review. | Public-API requests and independent fixes remain possible. | Small regression-tested patches are easier to propose and retire. |

## Provisional strategy and reversal conditions

**Recommendation:** choose **B** now. Start from released Bevy and a small reusable plugin layer. Keep **C** as an escalation route, not a pre-created fork. **A** is justified only if an approved differentiator demonstrably requires broad engine changes and the owner accepts that maintenance program. No such requirement has been established.

A future internal patch needs: a reproducible blocker on the pinned release; the affected approved requirement; evidence that composition, public extension hooks and acceptable upstream solutions are insufficient; a minimal patch and regression test; owner approval; a named maintenance/removal condition; and a review against each upgrade. An upstream issue/PR may be pending, but publication is a separate action requiring authorization. Patch scope must never include game rules.

For each future upgrade, record migration person-hours, changed project APIs, failing tests, plugin compatibility, compile-time changes and patch conflict count. Compare those observations before changing strategy. Pin within a milestone; inspect releases for needed fixes rather than automatically chasing every release. Treat retaining an older version as an explicit risk decision, not maintenance-free stability.

## Five layers and ownership

**Recommendation; origin: original design:** game code depends on reusable plugins, which depend on upstream Bevy (or an explicitly patched build). Engine code never depends on game code. Content is consumed through validated schemas, never compiled back into an engine dependency.

| Layer | Owns | Must not own |
| --- | --- | --- |
| 1. Upstream Bevy | ECS, scheduling, asset/render/input/audio/UI primitives. | Our combat, economy or content. |
| 2. Fork patches, if approved | Minimal reproduced upstream defect fixes with removal criteria. Initially absent. | General engine expansion or game-specific exceptions. |
| 3. Reusable 2D plugins | Only demonstrated reusable facilities, such as visual-ID mapping or diagnostic scenario support. | Wave rules, damage formulas, shop distributions, character balance. |
| 4. Game systems | Combat, spawning, progression, economy, run state, rule validation. | Reimplementation of Bevy services without evidence. |
| 5. Content and assets | Original definitions, presentation variants and media. | Hidden executable gameplay logic or artwork-derived collision truth. |

A plugin does not need its own crate. A crate requires a concrete reuse, dependency or testing boundary. The first game should prove usefulness before extracting a broad engine toolkit.

## Risks, alternatives and unknowns

**Inferred risks:** 0.20 is recent; third-party integration lag and stale examples may increase onboarding cost. A full fork amplifies these costs. An older stable release is an alternative only if a required integration is unavailable on 0.20 and that trade-off is documented. No such integration is currently required.

**Unknown:** target OS/hardware, owner Rust/Bevy experience, actual compile times, compatible optional physics/vector plugins, packaged-game behavior and baseline performance. These prevent a supported-platform promise, a final toolchain pin, or quantitative budgets. The next owner decision is the fork strategy, followed by platforms and available capacity.
