# Source ledger and evidence conventions

Status: Stage 1 research record. **All sources below were accessed on 2026-10-23.** A source ID used in another research document resolves to the URL, revision and limitations here. Access dates are not publication dates. Only relevant sections were inspected unless explicitly stated; retrieval is not execution or independent replication.

## Evidence vocabulary

- **Verified:** directly checked repository metadata/source, or a fact explicitly stated by a cited source. A secondary gameplay statement is verified *as a source report*, not as current executable behavior.
- **Inferred:** a conclusion or hypothesis derived from evidence, including cost comparisons and design principles. The cited source supports the premise, not necessarily the conclusion.
- **Recommendation:** a proposed original project choice. It remains provisional until the owner approves the applicable decision.
- **Unknown:** not established, unavailable, contradictory or not tested. Absence of evidence is not a negative finding.

Requirement origin is separate: **observed / inferred / original design**. Source-reported observations must identify the external observer/source; none of this research is a team gameplay observation. Future requirements must retain origin, evidence IDs and approved decision links. The owner brief is a project constraint, not an external technical authority.

## Track A

Steam descriptions are developer/publisher product statements, unversioned live pages; they do not identify a tested installed build. Wiki pages are community reports, unversioned at access unless noted. Most wiki revision IDs were not recorded, so historical reproducibility is limited. No proprietary content catalogs were imported.

| ID | Source | Version / evidence used |
| --- | --- | --- |
| A01 | [Brotato — Steam](https://store.steampowered.com/app/1942280/Brotato/) | Live product description: automatic fire/manual aim, short runs/waves, materials/shop, accessibility. Build unspecified. |
| A02 | [Stats — Brotato Wiki](https://brotato.wiki.spellsandguns.com/Stats) | Live community page; primary stat groups, harvesting and DLC-stat distinction. |
| A03 | [Waves — Brotato Wiki](https://brotato.wiki.spellsandguns.com/Waves) | Live community page; normal wave cycle and post-wave processing. |
| A04 | [Weapons — Brotato Wiki](https://brotato.wiki.spellsandguns.com/Weapons) | Page labels last update **1.1.6.3**; attack types, tiers, slots, scaling. Not evidence that every listed value matches the current build. |
| A05 | [Shop — Brotato Wiki](https://brotato.wiki.spellsandguns.com/Shop) | Live community page; offers, locking, rerolls, selection factors. |
| A06 | [Enemies — Brotato Wiki](https://brotato.wiki.spellsandguns.com/Enemies) | Live community page; roles, scaling introduction and population-cap report. No cap adopted by our game. |
| A07 | [Danger Levels — Brotato Wiki](https://brotato.wiki.spellsandguns.com/Danger_Levels) | Live community page; cumulative danger settings and Nightmare. |
| A08 | [Progress — Brotato Wiki](https://brotato.wiki.spellsandguns.com/Progress) | Live community page; challenge unlocks; scope wording can lag newer content. |
| A09 | [Damage — Brotato Wiki](https://brotato.wiki.spellsandguns.com/Damage) | Search/source excerpt and public stat descriptions; no complete interaction-order verification. |
| A10 | [Armor — Brotato Wiki](https://brotato.wiki.spellsandguns.com/Armor) | Positive-armor examples/table; formula fit in Track A is explicitly inferred. |
| A11 | [Attack Speed — Brotato Wiki](https://brotato.wiki.spellsandguns.com/Attack_Speed) | Mechanics introduction, negative-speed and melee/range caveats; no exact formula reproduced. |
| A12 | [Materials — Brotato Wiki](https://brotato.wiki.spellsandguns.com/Materials) | Public pickup/XP/currency and bagged-material model; retrieved page identifies oldid 8136. Read with Waves. |
| A13 | [HP Regeneration — Brotato Wiki](https://brotato.wiki.spellsandguns.com/HP_Regeneration) | Public regeneration examples; not timing measurements. |
| A14 | [Life Steal — Brotato Wiki](https://brotato.wiki.spellsandguns.com/Life_Steal) | Public chance/HP/lockout description. |
| A15 | [Crit Chance — Brotato Wiki](https://brotato.wiki.spellsandguns.com/Crit_Chance) | Public chance bounds and base-chance interaction. |
| A16 | [Knockback — Brotato Wiki](https://brotato.wiki.spellsandguns.com/Knockback) | Public direction description; no distance/time curve verified. |
| A17 | [Items — Brotato Wiki](https://brotato.wiki.spellsandguns.com/Items) | Public item-effect excerpts; evidence of effect categories, not a reused item database. |
| A18 | [Vampire Survivors — Steam](https://store.steampowered.com/app/1794680/Vampire_Survivors/) | Live developer description; minimal-input survival and persistent upgrades. |
| A19 | [20 Minutes Till Dawn — Steam](https://store.steampowered.com/app/1966900/20_Minutes_Till_Dawn/) | Live developer description; aiming/firing, runes and builds. |
| A20 | [Halls of Torment — Steam](https://store.steampowered.com/app/2218750/Halls_of_Torment/) | Live developer description; exploration, quests, equipment and bosses. |
| A21 | [Death Must Die — Steam](https://store.steampowered.com/app/2334730/Death_Must_Die/) | Live developer description; blessings, equipment, risk/reward. No current content count adopted. |
| A22 | [Soulstone Survivors — Steam](https://store.steampowered.com/app/2066020/Soulstone_Survivors/) | Live developer description; skills, crafting and progression. Marketing scale is not a project target. |
| A23 | [Dodge — Brotato Wiki](https://brotato.wiki.spellsandguns.com/Dodge) | Live community page; chance cap and character/item exceptions. |

## Track B

REA main resolved to `33d00a06010e0d890bb66fc68b91e2653f3c0390` during consultation. Files were read through the GitHub connector; links below preserve that revision. All capability statements are documentation/source findings, not local execution results.

| ID | Source | Version / evidence used |
| --- | --- | --- |
| B01 | [REA README](https://github.com/morluto/rea/blob/33d00a06010e0d890bb66fc68b91e2653f3c0390/README.md) | Targets, CLI/MCP, local analysis/model data path and limitations. |
| B02 | [REA installation](https://github.com/morluto/rea/blob/33d00a06010e0d890bb66fc68b91e2653f3c0390/docs/installation.md) | Setup/registration, main vs npm release boundary, Hopper/Ghidra prerequisites. |
| B03 | [Windows Ghidra P0](https://github.com/morluto/rea/blob/33d00a06010e0d890bb66fc68b91e2653f3c0390/docs/windows-ghidra-p0.md) | Experimental host/format/operation boundary; not parity. |
| B04 | [IDA provider](https://github.com/morluto/rea/blob/33d00a06010e0d890bb66fc68b91e2653f3c0390/docs/ida-provider.md) | Attached/headless routes, capability checks, native bridge security/cloud limitations. |
| B05 | [REA package manifest](https://github.com/morluto/rea/blob/33d00a06010e0d890bb66fc68b91e2653f3c0390/package.json) | Declared 6.7.0 and Node engine ranges; not npm registry verification. |
| B06 | [REA revision](https://github.com/morluto/rea/commit/33d00a06010e0d890bb66fc68b91e2653f3c0390) | Main revision identity, commit date 2026-10-23. |

## Track C

Source-code records use **v0.20.0**, except the explicitly dated/live web pages. A tag is more reproducible than main, but the GitHub release metadata reports the release is not immutable. Verify dependency identity/checksum when implementation begins.

| ID | Source | Evidence used |
| --- | --- | --- |
| C01 | [Bevy v0.20.0 release](https://github.com/bevyengine/bevy/releases/tag/v0.20.0) | Latest-release endpoint resolved here; published 2026-10-08; draft=false, prerelease=false. |
| C02 | [Bevy manifest](https://github.com/bevyengine/bevy/blob/v0.20.0/Cargo.toml) | Version, edition, MSRV, license, feature collections and flags. |
| C03 | [Plugin trait](https://github.com/bevyengine/bevy/blob/v0.20.0/crates/bevy_app/src/plugin.rs) | Public app extension lifecycle. |
| C04 | [Fixed time](https://github.com/bevyengine/bevy/blob/v0.20.0/crates/bevy_time/src/fixed.rs) | Default 64 Hz, virtual time, zero/multiple fixed ticks. |
| C05 | [Schedule implementation](https://github.com/bevyengine/bevy/blob/v0.20.0/crates/bevy_ecs/src/schedule/schedule.rs) and [tests/module](https://github.com/bevyengine/bevy/blob/v0.20.0/crates/bevy_ecs/src/schedule/mod.rs) | Ordering, weak edges, ambiguity detection and shuffle option. |
| C06 | [States](https://github.com/bevyengine/bevy/blob/v0.20.0/crates/bevy_state/src/lib.rs) | Standard, child and computed states. |
| C07 | [2D sprite example](https://github.com/bevyengine/bevy/blob/v0.20.0/examples/2d/sprite.rs) | Camera/image sprite usage. Read, not compiled. |
| C08 | [Asset crate](https://github.com/bevyengine/bevy/blob/v0.20.0/crates/bevy_asset/src/lib.rs) | Typed handles, async readiness, change/reload behavior. |
| C09 | [Input crate](https://github.com/bevyengine/bevy/blob/v0.20.0/crates/bevy_input/src/lib.rs) | Supported device categories and resources. |
| C10 | [Audio crate](https://github.com/bevyengine/bevy/blob/v0.20.0/crates/bevy_audio/src/lib.rs) | Playback entities and controls. |
| C11 | [UI crate](https://github.com/bevyengine/bevy/blob/v0.20.0/crates/bevy_ui/src/lib.rs) | Layout, widgets and navigation modules; migration caveats in C15. |
| C12 | [Reflection crate](https://github.com/bevyengine/bevy/blob/v0.20.0/crates/bevy_reflect/src/lib.rs) | Runtime metadata/introspection/serialization; not save compatibility. |
| C13 | [Headless example](https://github.com/bevyengine/bevy/blob/v0.20.0/examples/app/headless.rs) | Headless runner/feature selection; not a project test. |
| C14 | [Diagnostics crate](https://github.com/bevyengine/bevy/blob/v0.20.0/crates/bevy_diagnostic/src/lib.rs) | Frame and entity diagnostics. |
| C15 | [0.19 to 0.20 migration guide](https://bevy.org/learn/migration-guides/0-19-to-0-20/) | Live official guide; schedule, sprite and UI API changes. |
| C16 | [Release process](https://bevy.org/learn/contribute/project-information/release-process/) | Live official policy; roughly three-to-four-month cadence, no fixed dates. |
| C17 | [Bevy home](https://bevy.org/) and [example platform policy](https://bevy.org/learn/contribute/helping-out/creating-examples/) | Live platform claims; desktop example expectations. No certified game support inferred. |
| C18 | [Setup guide](https://bevy.org/learn/quick-start/getting-started/setup/) | Live OS prerequisites, optional dynamic linking/linkers. Published speed claims not adopted. |
| C19 | [Linux dependencies](https://github.com/bevyengine/bevy/blob/v0.20.0/docs/linux_dependencies.md) | Native dependencies and WSL caveats. |
| C20 | [Feature documentation](https://github.com/bevyengine/bevy/blob/v0.20.0/docs/cargo_features.md) and [0.20.0 API landing](https://docs.rs/bevy/0.20.0/bevy/) | Collections and development flags; stale 0.18 example recorded. |
| C21 | [ECS README](https://github.com/bevyengine/bevy/blob/v0.20.0/crates/bevy_ecs/README.md) | Conceptual component/system/query model; do not rely on its resource-storage wording as an internal contract. |
| C22 | [Messages](https://github.com/bevyengine/bevy/blob/v0.20.0/crates/bevy_ecs/src/message/mod.rs) | Buffered pull handling vs observer events. |
| C23 | [Sprite data](https://github.com/bevyengine/bevy/blob/v0.20.0/crates/bevy_sprite/src/sprite.rs) | Image, atlas and custom-size fields. |
| C24 | [Time example](https://github.com/bevyengine/bevy/blob/v0.20.0/examples/time/time.rs) | Minimal app, manual update and virtual/fixed clocks. |
| C25 | [0.20 announcement](https://bevy.org/news/bevy-0-20/), [0.19 announcement](https://bevy.org/news/bevy-0-19/), [0.18 announcement](https://bevy.org/news/bevy-0-18/) | Dated 2026-10-08, 2026-06-19, 2026-01-13 respectively. Release dates, not promised future cadence. |
| C26 | [Getting started](https://bevy.org/learn/quick-start/getting-started/) | Live search excerpt still recommends 0.19; lower authority for version selection than C01/C02. |

## Tracks D and E

Cargo/rustup/tool books below are live official documentation without a captured commit; use as policy/concept evidence and recheck exact command compatibility at the chosen pin. No tools were installed or run from these instructions.

| ID | Source | Version / evidence used |
| --- | --- | --- |
| D01 | [Rust releases](https://blog.rust-lang.org/releases/) and [Rust 1.99.1](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/) | Official releases list; latest consulted entry 1.99.1, dated 2026-10-15. |
| D02 | [Cargo rust-version](https://doc.rust-lang.org/cargo/reference/rust-version.html) | Live support declaration and MSRV policy. |
| D03 | [rustup overrides](https://rust-lang.github.io/rustup/overrides.html) | Live toolchain selection and file precedence. |
| D04 | [Cargo workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html) | Shared lockfile/output/configuration boundaries. |
| D05 | [Overriding dependencies](https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html) | Root patch mechanism and version-resolution caveats. |
| D06 | [Clippy usage](https://doc.rust-lang.org/stable/clippy/usage.html) | Standard lint groups and warning handling; current page discusses Cargo 1.97+. |
| D07 | [Rust test organization](https://doc.rust-lang.org/book/ch11-03-test-organization.html) | Unit/integration boundaries. |
| D08 | [cargo-deny](https://embarkstudios.github.io/cargo-deny/) | Maintainer docs: advisories/licenses/bans/sources. Version not selected. |
| D09 | [RustSec](https://rustsec.org/) | Maintainer description of vulnerability database and cargo-audit. |
| D10 | [Proptest](https://proptest-rs.github.io/proptest/intro.html) | Maintainer description of generation/shrinking. Version not selected. |
| D11 | [Bevy profiling](https://github.com/bevyengine/bevy/blob/v0.20.0/docs/profiling.md) | Tagged tracing/Tracy, memory-overhead and profiling workflow. |
| D12 | [Criterion book](https://bheisler.github.io/criterion.rs/book/) | Maintainer microbenchmark purpose; no performance results adopted. |
| D13 | [Cargo features](https://doc.rust-lang.org/cargo/reference/features.html) | Additive feature model and unification. |
| D14 | [GitHub Actions secure use](https://docs.github.com/en/actions/reference/security/secure-use) | Official live workflow hardening guidance; SHA pinning and least privilege. |
| D15 | [cargo build](https://doc.rust-lang.org/cargo/commands/cargo-build.html) | Locked resolution and build/timing options. |
| D16 | [Cargo profiles](https://doc.rust-lang.org/cargo/reference/profiles.html) | Optimization/profile configuration. |
| E01 | [Rand reproducibility](https://rust-random.github.io/book/crate-reprod.html) | Live maintainer policy; value stability differs from API compatibility. RNG not selected. |
| E02 | [Serde overview](https://serde.rs/) | Typed serialization and format separation. Version not selected. |
| E03 | [RON README](https://github.com/ron-rs/ron/blob/cbb3d032f3b490dfb0d9ab43763ebc31575a3238/README.md) | `master` resolved to this SHA; comments/enums and format limitations. |
| E04 | [Bevy message collection](https://github.com/bevyengine/bevy/blob/v0.20.0/crates/bevy_ecs/src/message/messages.rs) | Retention/update and skipped-reader caveats. |
| E05 | [Bevy archetypes](https://github.com/bevyengine/bevy/blob/v0.20.0/crates/bevy_ecs/src/archetype.rs) | Component combinations, tables and lifetime of empty archetypes. |

## Track F

| ID | Source | Version / evidence used |
| --- | --- | --- |
| F01 | [resvg README](https://github.com/linebender/resvg/blob/5192641d4fd67aef5c0c4d667d570e338da15361/README.md) | Main SHA at access; static SVG, CLI/library, limitations and license. Reproducibility is an upstream claim, not our test. |
| F02 | [bevy_svg README](https://github.com/Weasy666/bevy_svg/blob/c1b0e291368562f9aa57c9c9db67cfdf0e205bd8/README.md) | Main SHA at access; tessellation and limited support. |
| F03 | [bevy_vello README](https://github.com/linebender/bevy_vello/blob/659b06fdb422dcd9992446d2c30cda024659b9f4/README.md) | Main SHA at access; compute-centric integration and SVG caveats. Browser-version wording may be stale; no current browser support claim adopted. |
| F04 | [bevy_svg manifest](https://github.com/Weasy666/bevy_svg/blob/c1b0e291368562f9aa57c9c9db67cfdf0e205bd8/Cargo.toml) | Declares plugin 0.19.0, Bevy 0.19. |
| F05 | [bevy_vello manifest](https://github.com/linebender/bevy_vello/blob/659b06fdb422dcd9992446d2c30cda024659b9f4/Cargo.toml) | Declares plugin 0.14.0, Bevy 0.19.0. |

## Track G

| ID | Source | Version / evidence used |
| --- | --- | --- |
| G01 | [kali-tools specs directory](https://github.com/AlexGalhardo/kali-tools/tree/0314f36ed76660de776c98c2f45b4d1745221a92/docs/specs) | Main revision at access; actual file listing. |
| G02 | [kali-tools SDD README](https://github.com/AlexGalhardo/kali-tools/blob/0314f36ed76660de776c98c2f45b4d1745221a92/docs/specs/README.md) | Read instructions/template/evidence/index; studied as data, not governing instructions. |
| G03 | [A08 theme spec](https://github.com/AlexGalhardo/kali-tools/blob/0314f36ed76660de776c98c2f45b4d1745221a92/docs/specs/A08-terminal-theme.md) | Read bounded visual change, files and validation conventions; no implementation reproduced. |
| G04 | [H01 bootstrap spec](https://github.com/AlexGalhardo/kali-tools/blob/0314f36ed76660de776c98c2f45b4d1745221a92/docs/specs/H01-bootstrap.md) | Inspected metadata, goal/current state, headings and delivery assertions. Not a full code review. |
| G05 | [jogo-hades PLAN.md](https://github.com/AlexGalhardo/jogo-hades/blob/311b2fa4f01fe3af3bed9c97372792a19d7596d2/PLAN.md) | Fetched actual file; read session rules, phase/dependency/estimate sections and history tail; not every task. No project-specific authorization imported. |
| G06 | [Documenting Architecture Decisions](https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions) | Michael Nygard's original 2011-11-15 article; small decision records and superseded history. |
| G07 | [Gherkin reference](https://cucumber.io/docs/gherkin/reference/) | Live official Given/When/Then semantics; runtime not required by our proposal. |
| G08 | [Git worktree manual](https://git-scm.com/docs/git-worktree) | Live official manual, page reports 2.56.0 update; separate checkouts/shared repository. |

## Access failures, disagreements and excluded evidence

- All five references supplied by the owner were accessible: Brotato Steam, Bevy website/repository, REA, kali-tools specs and jogo-hades plan.
- The guessed Bevy platform-support URL returned inaccessible through the web tool. The home page and official example policy were used instead; no contents were inferred for the failed URL.
- `agent-browser read` returned no usable body for the Bevy setup URL; the web search tool supplied its content. Tool availability is not proof of successful page extraction.
- An old Rand portability URL returned no content; the maintained reproducibility chapter was found and read instead.
- Search surfaced community comments and scraped/database mirrors; none supplies technical engine facts or copied game data in this research.
- Current release metadata and older tutorial version examples disagree; use C01/C02 for the baseline. Gameplay wiki pages have uneven update scope. Vector plugin manifests currently lag the Bevy baseline.

## Recommendation, alternatives, risks and unknowns

**Recommendation:** prefer pinned upstream source and executable evidence for technical decisions; use public gameplay reports as hypotheses until a versioned observation matters to a design choice. Preserve concise original findings and links rather than mirroring whole source documents. **Alternative:** archive permitted source excerpts with revision IDs if long-term reproducibility requires it, respecting licenses and the IP policy.

**Inferred risks:** link rot, mutable pages, tag movement, stale examples and source reports mistaken for experiments. **Unknown:** gameplay build identity, experiment outcomes, dependency compatibility in a real project and review findings. Revalidate affected sources at decision/implementation time; do not relabel a recommendation Verified simply because it appears in several project documents.
