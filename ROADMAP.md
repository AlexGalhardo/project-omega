# ROADMAP.md: development plan for Project Omega

> **Current phase:** PROTOTYPE (Phase 2) · **Next task:** GAME-P2-010 (promote to READY, then implement). Foundation (Phase 1) closed on 2026-10-24 with release `v0.3.0`; GAME-P1-004 and QA-P1-002 wait only for the owner's native visual review (section 3).
>
> This is the onboarding file. It says where the project is, what the next task is and how to prove it is done. Every work session, human or AI agent, starts here.
>
> Game: Project Omega (working title), original top-down arena-survival roguelite with scrap robots, Rust 2024, Bevy 0.20 as a vendored fork in `engine/bevy/`, Windows 1.0. Didactic project with two primary objectives (DEC-017): re-create Brotato from REA study of the owner's purchased copy, and build the project's own 2D engine as a Bevy fork. One developer with AI agents, 20–40 h/week (capacity, not measured throughput).

## How to start a session

1. Run the session routine in [AGENTS.md](AGENTS.md#session-start-claude-code): caveman, `karpathy-guidelines`, `graphify` (`graphify update .` if the graph is stale), `project-omega`.
2. Read this header and section 1 (plan rules). They apply to every task.
3. Find the next task: the first unchecked box after the task named in **Next task**. Command: `rg -n -m 5 "^ *- \[ \]" ROADMAP.md`.
4. Read the task's **Read first** line and open only those sections. The [GDD](docs/gdd.md) owns mechanics, numbers and content; the [Art Bible](docs/art-bible.md) owns visuals; the spec owns allowed paths, tests and commands.
5. Work one microtask at a time, in the written order. Mark `- [x]` when it is proven, not when it is written.
6. When the task is done, run its **Asserts**, close it by rule 1.5 and update this header (**Next task**, and **Current phase** when it changes).

Rules of this file:

- **The plan is updated in the same commit as the work.** Checked boxes, status changes and plan corrections go in with the code that justifies them. An outdated plan is a defect.
- **This plan does not design mechanics.** Rules, numbers and content live in the GDD; a task that seems to contradict the GDD loses, and the plan is corrected in the same commit. A GDD flaw becomes a change request (`docs/decisions/change-requests/`).
- **Allowed paths come from the spec.** If a microtask names a different path than the spec or `docs/architecture/architecture.md`, the spec wins and the task is corrected.
- **Sibling documents:** `docs/architecture/` (how to build), `docs/specs/` (SDD contracts), `docs/validation/` (gates, Definition of Done, evidence), `docs/decisions/` (decisions, change requests), `engine/README.md` (fork patch log), `docs/release.md` (releases), `docs/ai-memory.md` (session memory), `docs/research/brotato-notes/` (REA notes), [PLAN.md](PLAN.md) (history of planning stages 1–6).

## 1. Plan rules

### 1.1 How to read the checklists

- Phases 1–5 contain **tasks** with stable IDs `ENG-Pn-NNN`, `GAME-Pn-NNN`, `QA-Pn-NNN`, `REL-Pn-NNN`, `RES-Pn-NNN`. IDs never change and are never reused; a new task gets the next free number; a split cancels the old ID.
- Inside a task, each box is a **microtask**: one action with an obvious file or command. Boxes run in the written order. In logic work the first microtask is the test.
- Statuses: TODO, READY, IN_PROGRESS, BLOCKED, IN_REVIEW, DONE, DEFERRED, CANCELLED. Size S is one narrow concern; M is one bounded integration or content batch. Nothing is marked "partly".
- Phase 3–5 tasks are compact: when one is promoted to READY, expand it into microtasks in the same commit (rule 1.2).

### 1.2 Promotion to READY and Definition of Done

A task is **READY** when the READY audit in [quality gates](docs/validation/quality-gates.md#ready-audit) passes: dependencies DONE, exact files/globs assigned, one named test per acceptance criterion, commands concrete, no open decision. Record the promotion (status change plus expanded microtasks) in the same commit as the first work.

A task is **DONE** ([Definition of Done](docs/validation/definition-of-done.md)) when:

1. Every microtask box is checked.
2. Every **Asserts** item ran in this session and its output was read, not assumed.
3. `powershell -NoProfile -File scripts/check.ps1` exits 0.
4. Evidence is recorded in `docs/validation/evidence/<TASK-ID>/` or the evidence index: commit, environment, commands, exit codes, criterion-to-test mapping, limits.
5. GDD, Art Bible, specs, `AGENTS.md` and sibling documents are updated if the task changed something they describe.
6. `CHANGELOG.md` `[Unreleased]`, this header and the task status are updated.

Native checks (visuals, audio, controller, feel) need native evidence; a headless test never substitutes. If a native check needs the owner, set IN_REVIEW and say exactly what to look at.

### 1.3 Work cycle for logic

1. Write the test named after the spec criterion (`ac2_...`) with GDD numbers read from content fixtures, not retyped constants.
2. Run it and watch it fail for the right reason.
3. Implement the minimum inside the spec's allowed paths.
4. Run the test, `cargo fmt`, Clippy; then wire the rule into the `GameTick` schedule (`TickSet` order) and add the integration test.
5. A bug found at any time first gets a test that reproduces it (seeded `SimHarness` or `omega-sim` trace).

Simulation code never reads wall time, renderer state or ECS iteration order; it sorts by `SimId`/slot and draws only from `sim::RngStreams`. Damage and economy use checked integers (CR-002).

### 1.4 Versioning by phase

| Phase | Version | Rule |
| --- | --- | --- |
| Planning (stages 1–6) | `0.1.0`, `0.1.1`, `0.2.0` | Retroactive releases, already published |
| Foundation | `0.3.0` | Published |
| Prototype | `0.4.0` (P2-M1), `0.5.0` (P2-M2, phase exit) | One minor per milestone; urgent fixes bump the patch |
| Alpha | `0.6.0`, `0.7.0` | One minor per milestone |
| Beta | `0.8.0`, `0.9.0` | One minor per milestone |
| RC → 1.0 | `1.0.0-rc.N`, then `1.0.0` | Release candidates are GitHub prereleases; only REL-P5-004 publishes `1.0.0` |

Tags are `vX.Y.Z`. The workspace version in the root `Cargo.toml` must equal the tag (the release workflow checks it). Commit types do not decide the version; the phase does.

### 1.5 Commit, push and release

1. Small commits during a task, Conventional Commits in English (`feat(sim): ...`, `fix(content): ...`, `docs`, `test`, `refactor`, `perf`, `chore`, `build`, `ci`, `security`). Code, test and document of one change in one commit. Stage exact paths, never `git add -A` blindly.
2. Never `--no-verify`. A failing hook is a defect to fix.
3. Never commit `.env`, `target/`, `graphify-out/`, raw REA output or any extracted Brotato material (the repository is public).
4. Closing a task: asserts, `scripts/check.ps1`, `CHANGELOG.md` `[Unreleased]` entry `- **scope:** Message ([sha](url))`, this file, commit, push to `main`.
5. Closing a milestone: release by [docs/release.md](docs/release.md): bump the workspace version, move `[Unreleased]` into the new version section, commit `chore(release): X.Y.Z`, tag, push the tag alone. The release workflow builds the Windows archive, SBOM, checksums, cosign bundle and provenance, and publishes notes from CHANGELOG.md.
6. Remote `https://github.com/AlexGalhardo/project-omega.git`, branch `main`. `git reset --hard`, force push, deleting branches or tags: only on the owner's explicit request. `run-overnight.ps1` runs commit but never push, tag or publish. Autonomous sessions under [DEC-020](docs/decisions/decision-register.md#dec-020--autonomous-implementation-sessions) may push after a green gate and publish milestone releases, never `1.0.0`.

### 1.6 Local gate and CI

| Command | When | What it does |
| --- | --- | --- |
| `powershell -NoProfile -File scripts/check.ps1` | Before every push | fmt, Clippy (core and client), headless tests, content validation, repeatability scenario, asset validation, client contract tests, cargo-deny, markdownlint |
| `cargo test -p omega_game --no-default-features --locked --test <name>` | During a task | One contract test file |
| `powershell -NoProfile -File scripts/validate-assets.ps1 -Manifest <m> -Reproduce` | Asset changes | Hashes, sizes, RGBA, byte-identical re-export |
| `cargo run -p omega_game --locked --bin omega-game -- --screenshot PATH` | Native visual checks | Native capture for evidence |

GitHub Actions (`.github/workflows/ci.yml`) runs the same gates on every push to `main` and every PR (windows-2022 Rust job, cargo-deny, Markdown). A red CI run blocks the next release. Debug with `gh run view <id> --log-failed`.

### 1.7 Parallel work and worktrees

One active integration task at a time. Parallel agents or worktrees need the owner's explicit yes for that task (global rule), or the DEC-020 pre-authorization in an autonomous session (ETA of 30 minutes or more), and always disjoint file ownership (AGENTS.md). Root manifests, `Cargo.lock`, content catalogs, this file and contracts are edited serially.

Never wait idle: after a push, start the next task and check CI at the next checkpoint (a red run is fixed before the next release). Jobs with an ETA over 10 minutes (balance simulations, QA bot runs, e2e suites, long benchmarks, release builds, REA analyses) run in a background agent or a separate worktree while the main loop continues with the next task.

### 1.8 When stuck

1. A contract flaw (spec, GDD, ADR): write a change request, set the task BLOCKED with the CR ID, continue the next independent task.
2. An engine limitation: fix it in `engine/bevy/` with a patch-log entry and a regression test (DEC-018), or add it to `omega_2d` if it is a reusable 2D facility.
3. A missing owner decision or native check: set BLOCKED or IN_REVIEW with the exact question, never guess. In autonomous sessions also append the question to the local `questions-to-dev.md` and continue with the next independent task (DEC-020).
4. A flaky test is never retried until green; find the nondeterminism.

### 1.9 Minimal vocabulary

- **LU:** logical unit, world distance; the arena is 1200 × 675 LU. **Tick:** one 1/60 s simulation step.
- **SimHarness:** headless driver of the exact `GameTick` schedule. **Trace:** sorted authoritative state per tick, compared for repeatability.
- **Profile:** content validation mode, `prototype` or `full`. **Skin:** a presentation catalog; swapping skins never changes traces (DA-20).
- **RES task:** a REA study of one Brotato system (rule 1.10). **QA bot:** the seeded simulated-human player of rule 1.11.
- **V01–V18:** the validation commands of SP-001–SP-018. **DA-NN / AA-NN:** design and art acceptance examples in GDD-018 / Art Bible 13.

### 1.10 REA study loop: learn the mechanics, then re-implement them

Objective 1 (DEC-017) is to understand how Brotato's game design and mechanics work by reverse engineering the owner's copy with REA (DEC-019), and to re-implement them in this game with original code, names and art. Every gameplay system follows this loop:

1. **Study (RES task).** Before the first GAME task of a system, its RES task inspects the owner's copy with REA (static analysis of `Brotato.pck`/`Brotato.exe` where the tool supports it) and with ordinary play observation, and answers concrete questions: rules, formulas, timings, state flow, data shapes. Raw output stays in `%LOCALAPPDATA%\ProjectOmega\rea\`.
2. **Describe (prose note).** Write `docs/research/brotato-notes/NN-<system>.md` in your own words with confidence labels (Verified / Inferred / Unknown) and the method used. Never paste code, scripts, data tables, strings or extracted assets; the repository is public.
3. **Compare.** List each difference between the note and the current GDD rule for that system.
4. **Decide.** Adopting a Brotato behavior that differs from the GDD needs a change request (`docs/decisions/change-requests/`) that cites the note; DEC-020 lets an autonomous session approve it provisionally, listing it in `questions-to-dev.md`. Keeping the GDD rule is also a valid outcome; record why.
5. **Implement.** The GAME tasks then implement the GDD as amended, and their tests use the amended numbers from content.

If REA cannot answer a question (for example a package format it does not support), record that in the note, fall back to play observation and public sources, and do not block the GAME task: it implements the current GDD rule and the open question goes to `questions-to-dev.md`. Never bypass DRM or protections, and never add another extraction tool without the owner's decision.

### 1.11 Test layers and phase-end QA

Every task tests at the layers it touches; every phase ends with all four layers green on the release candidate commit:

| Layer | What it proves | Where and how |
| --- | --- | --- |
| Unit | One rule or function in isolation: formulas, clamps, parsers, geometry | `#[cfg(test)]` modules and `crates/*/tests/*_contract.rs`; exact GDD numbers from content fixtures |
| Integration | Several systems through the real `GameTick` schedule | `SimHarness` scenarios with seeded input; authoritative trace assertions |
| End-to-end (e2e) | The real native client binary from launch to exit | `omega-game --e2e-script <script.json>`: scripted input timeline, state checkpoints exported as JSON, screenshots, exit code; runs in CI where a GPU is not required (headless path) and natively on the reference machine |
| Simulated human QA | The game behaves for a person, not only for a script | The QA bot (`omega-game --qa-bot <profile>`): seeded human-like play (reaction delay, imperfect aim, hesitation, menu exploration, mistaken inputs, pause/focus/resize abuse) following `docs/validation/qa-checklist.md`, producing a report with findings, screenshots and the seed to reproduce each one |

Phase-end QA task rules (QA-P2-002, QA-P3-002, QA-P4-004, QA-P5-003):

1. Run the full unit and integration suites, every e2e script and the QA bot across all its profiles and at least 20 seeds per profile; record counts, durations and seeds. Include the balance simulation report of rule 1.12 from GAME-P2-013 onward.
2. Walk `docs/validation/qa-checklist.md` with the QA bot and with screenshot review; every failed item becomes a bug task with a reproducing test before it is fixed.
3. Write `docs/validation/reports/<YYYY-MM-DD>-<phase>-qa.md`: version, commit, environment, results per layer, findings, open items. The phase closes only with no open high-priority finding.
4. Items a bot cannot judge (fun, feel, readability, audio mix) go to `questions-to-dev.md` as owner checks; they do not block the phase in autonomous sessions (DEC-020) but stay listed in the report.

### 1.12 Balance simulation: make it fun, then prove it

Numbers in the GDD are provisional (P). Fun is judged by people, but obviously wrong numbers are caught earlier by simulation. [docs/validation/balancing.md](docs/validation/balancing.md) defines the simulator, bot policies, metrics and provisional target bands.

1. From the Prototype on, `omega-sim --scenario balance` runs full runs headlessly with bot policies (`idle`, `kiter`, `greedy`, `defensive`, `random`, `human_like`) across characters and seeds.
2. Every content or formula change that affects combat or economy re-runs the affected matrix and attaches before/after reports to its evidence.
3. Each phase-end QA task (rule 1.11) includes a balance report; a missed target band is a finding with a proposed GDD/content change, re-run with the same seeds.
4. Simulation never replaces playtests: feel, readability and fun questions go to `questions-to-dev.md`, and the owner's verdict can move a target band.

## 2. Overview

### 2.1 The five phases

| Phase | Goal | Versions | Exit criterion |
| --- | --- | --- | --- |
| 1. FOUNDATION | Reproducible build, headless simulation core, content validation, the three engine contracts, the SVG asset pipeline and a first measured baseline. | `0.3.0` | BUILD-001 verified, headless tick/content foundations, tested geometry/camera/catalog boundaries, SVG/raster replacement proof and an actual initial runtime/build baseline with no invented budgets. |
| 2. PROTOTYPE | The GDD-017 prototype is playable: move, auto/manual aim, fight, collect scrap, shop, allocate energy, end in defeat with a summary. Seeded headless tests and one native playtest prove the loop. | `0.4.0`, `0.5.0` | GDD's three-wave/two-weapon prototype can move, auto/manual aim, fight, collect scrap, shop, allocate energy and end in defeat/summary; targeted headless tests and a native playtest demonstrate the loop. No final-content feasibility claim yet. |
| 3. ALPHA | All approved systems and the full 4/12/10/2/40 content: ten waves, two bosses, saves, menus, controller and accessibility, full-run tests. Placeholder art and audio allowed. | `0.6.0`, `0.7.0` | all approved core systems and full 4/12/10/2/40 content, ten waves/two bosses, saves, menus, controller/accessibility functionality and full-run tests are present. Production polish may still use cleared placeholders. |
| 4. BETA | Production art and audio, balance and readability passes, native controller/accessibility checks, measured budgets, offline package, two-hour stability. | `0.8.0`, `0.9.0` | original/cleared production art/audio, recorded balance/readability passes, native controller/accessibility checks, measured hardware budgets, clean offline package and two-hour stability evidence. Performance work follows observed bottlenecks. |
| 5. RC → 1.0 | Feature freeze, full regression, license/asset audit, release candidate and a separate final 1.0 decision. | `1.0.0-rc.N`, `1.0.0` | RC passes, remaining release blockers are resolved, and a separate final 1.0 gate approves the qualified artifact. Store publication or external spending is not implied by planning approval. |

Engine work is limited to BUILD-001, the three 2D contracts (ENG-GEOM-001, ENG-CAM-001, ENG-VIS-001), fork patches the game needs, and measured fixes. No speculative engine task may displace game delivery.

### 2.2 All tasks

| ID | Task | Status | Depends on |
| --- | --- | --- | --- |
| ENG-P1-001 | Two-crate pinned bootstrap | DONE | Approved planning gates |
| QA-P1-001 | Windows CI and dependency policy | DONE | ENG-P1-001 |
| GAME-P1-001 | Tick schedule, IDs, RNG and headless harness | DONE | ENG-P1-001 [BUILD-001 local verification, REV-014] |
| GAME-P1-002 | Typed content family schemas | DONE | ENG-P1-001 [BUILD-001 local verification, REV-014] |
| GAME-P1-003 | Cross-reference validator and content CLI | DONE | GAME-P1-002 |
| ENG-P1-002 | Circle/capsule/sweep geometry | DONE | ENG-P1-001, QA-P1-001 |
| ENG-P1-003 | Logical viewport and inverse camera map | DONE | ENG-P1-001 (REV-015) |
| ENG-P1-004 | Visual catalog and static preview contract | DONE | ENG-P1-003 |
| ENG-P1-005 | Offline exporter/manifest validation | DONE | ENG-P1-004; GAME-P1-003 validated fixture contract |
| GAME-P1-004 | Original proof assets and raster substitution | IN_REVIEW | ENG-P1-005 [ENG-VIS-001]; GAME-P1-001 |
| QA-P1-002 | Empty-scene and build baseline | IN_REVIEW | ENG-P1-004, GAME-P1-001, GAME-P1-004 |
| ENG-P1-006 | Vendored Bevy fork wiring (DEC-018) | DONE | ENG-P1-001 |
| RES-P1-001 | REA setup and Brotato inventory under DEC-019 | TODO | REA and Ghidra installed 2026-10-24 |
| QA-P1-003 | End-to-end and simulated-QA harness | TODO | ENG-P1-004, GAME-P1-001 |
| GAME-P2-001 | Stats, energy and exact numeric rules | DONE | GAME-P1-003 |
| GAME-P2-002 | Keyboard/mouse action buffer and aim | DONE | GAME-P1-001; ENG-P1-003 [ENG-CAM-001] |
| GAME-P2-003 | Player movement and actor state | DONE | GAME-P2-001, GAME-P2-002; ENG-P1-002 [ENG-GEOM-001] |
| GAME-P2-004 | Bolt and saw attack emission | DONE | GAME-P2-003 |
| GAME-P2-005 | Damage, death, healing and scrap | TODO | GAME-P2-004 |
| GAME-P2-006 | Chaser/runner and three-wave scheduling | TODO | GAME-P2-005 |
| ENG-P2-001 | Live pose/projectile presentation | TODO | ENG-P1-004 |
| GAME-P2-007 | Shop eligibility, prices and one refresh | TODO | GAME-P1-003, GAME-P2-001 |
| GAME-P2-008 | Purchase, upgrade, replacement and allocation transactions | TODO | GAME-P2-007 |
| GAME-P2-009 | Exact prototype content subset | TODO | GAME-P1-003 |
| GAME-P2-010 | Prototype Wave/Shop/Defeat lifecycle | TODO | GAME-P2-006, GAME-P2-008, GAME-P2-009 |
| GAME-P2-011 | Minimal HUD, shop controls and pause | TODO | GAME-P2-010; ENG-P2-001 [ENG-VIS-001] |
| GAME-P2-012 | Cleared placeholder event audio | TODO | GAME-P2-011; ENG-P1-005 [ENG-VIS-001 asset manifest] |
| GAME-P2-013 | Balance simulator, bot policies and first prototype tuning | TODO | GAME-P2-010, QA-P1-003 |
| QA-P2-001 | Prototype regressions and first playtest | TODO | GAME-P2-011, GAME-P2-012 |
| QA-P2-002 | Prototype phase-end QA: unit, integration, e2e and simulated human QA | TODO | QA-P2-001, QA-P1-003, GAME-P2-013 |
| RES-P2-001 | REA study: player stats, weapons and combat | TODO | RES-P1-001 |
| RES-P2-002 | REA study: shop, economy and run flow | TODO | RES-P1-001 |
| ENG-P3-001 | Phase-driven warning/clip presentation | TODO | ENG-P2-001 |
| GAME-P3-001 | Remaining projectile/area payloads | TODO | GAME-P2-005 |
| GAME-P3-002 | Ranged, stationary and charge enemy profiles | CANCELLED | Split into GAME-P3-016 to GAME-P3-019 (REV-010) |
| GAME-P3-016 | Keep-distance gunner and scatter profiles | TODO | GAME-P2-006, GAME-P3-001 |
| GAME-P3-017 | Stationary sentry and orbit skimmer profiles | TODO | GAME-P3-016 |
| GAME-P3-018 | Charge rammer and ground-strike crusher profiles | TODO | GAME-P3-017 |
| GAME-P3-019 | Beam marksman and heavy bulwark profiles | TODO | GAME-P3-018 |
| GAME-P3-003 | Beam, strike, orbit and heavy profiles | CANCELLED | Split into GAME-P3-016 to GAME-P3-019 (REV-010) |
| GAME-P3-004 | The Press encounter | TODO | GAME-P3-019; ENG-P3-001 [ENG-VIS-001] |
| GAME-P3-005 | The Foreman encounter | TODO | GAME-P3-004 |
| GAME-P3-006 | Full character, weapon and item data | TODO | GAME-P2-009, GAME-P3-001 |
| GAME-P3-007 | Full encounter, wave and challenge data | TODO | GAME-P3-005, GAME-P3-006 |
| GAME-P3-008 | Idempotent challenge grants | TODO | GAME-P3-007 |
| GAME-P3-009 | Profile/settings save protocol | TODO | GAME-P3-008 |
| GAME-P3-010 | Interrupted-write and recovery behavior | TODO | GAME-P3-009 |
| GAME-P3-020 | Boot, Menu and character selection flow | TODO | GAME-P3-007, GAME-P3-010, GAME-P2-010 |
| GAME-P3-011 | Full guarded run lifecycle | TODO | GAME-P3-020, GAME-P3-005, GAME-P3-008 |
| GAME-P3-012 | Menus, selection, unlocks and results | TODO | GAME-P3-011; ENG-P3-001 [ENG-VIS-001] |
| GAME-P3-013 | Controller actions, remapping and disconnection | TODO | GAME-P3-012, GAME-P2-002 |
| GAME-P3-014 | Accessible layouts, effects and onboarding | TODO | GAME-P3-013 |
| GAME-P3-015 | Complete shop comparisons and confirmation UI | TODO | GAME-P3-012, GAME-P2-008 |
| QA-P3-001 | Full-run and catalog integration qualification | TODO | GAME-P3-014, GAME-P3-015, GAME-P3-010, GAME-P3-007 |
| QA-P3-003 | Full-content balance simulation and tuning | TODO | QA-P3-001, GAME-P2-013 |
| QA-P3-002 | Alpha phase-end QA: unit, integration, e2e and simulated human QA | TODO | QA-P3-001, QA-P2-002, QA-P3-003 |
| RES-P3-001 | REA study: enemies, bosses, waves and spawning | TODO | RES-P2-001 |
| RES-P3-002 | REA study: characters, unlocks, progression and saves | TODO | RES-P2-002 |
| GAME-P4-001 | Four production character visuals | TODO | QA-P3-001; ENG-P3-001 [ENG-VIS-001] |
| GAME-P4-002 | Twelve weapon visuals and shared payload cues | CANCELLED | Split into GAME-P4-007 to GAME-P4-009 (REV-010) |
| GAME-P4-007 | Weapon visuals batch 1 (bolt, slug, needle, saw) and shared payload cues | TODO | GAME-P4-001 |
| GAME-P4-008 | Weapon visuals batch 2 (hammer, fan, arc, nail) | TODO | GAME-P4-007 |
| GAME-P4-009 | Weapon visuals batch 3 (rocket, pulse, mortar, disk) | TODO | GAME-P4-008 |
| GAME-P4-003 | Enemy/boss/arena production art | CANCELLED | Split into GAME-P4-010 to GAME-P4-013 (REV-010) |
| GAME-P4-010 | Common enemy art 1–5 (chaser, runner, bulwark, gunner, rammer) | TODO | GAME-P4-009 |
| GAME-P4-011 | Common enemy art 6–10 (sentry, marksman, scatter, crusher, skimmer) | TODO | GAME-P4-010 |
| GAME-P4-012 | Boss art (The Press, The Foreman) | TODO | GAME-P4-011 |
| GAME-P4-013 | Arena art | TODO | GAME-P4-012 |
| GAME-P4-004 | Item icons, UI theme and bundled font | TODO | GAME-P4-013 |
| GAME-P4-005 | Cleared production sounds/music and mix | TODO | GAME-P4-004, GAME-P2-012 |
| GAME-P4-006 | Documented balance/readability pass | TODO | GAME-P4-005, QA-P3-001 |
| ENG-P4-001 | Measured engine/render bottleneck assessment | TODO | GAME-P4-005, QA-P1-002 |
| QA-P4-001 | Native controller/accessibility matrix | TODO | GAME-P4-006, GAME-P3-014, GAME-P3-015 |
| QA-P4-002 | Two-hour stability and resource trends | TODO | ENG-P4-001, QA-P4-001 |
| QA-P4-003 | Select shipping hardware and measured budgets | BLOCKED | ENG-P4-001; owner hardware/budget decision |
| REL-P4-001 | Offline Windows package and clean-launch workflow | TODO | GAME-P4-005, QA-P4-002 |
| QA-P4-004 | Beta phase-end QA: unit, integration, e2e and simulated human QA | TODO | QA-P4-002, QA-P3-002 |
| RES-P4-001 | REA study: balance numbers and pacing | TODO | RES-P3-001, RES-P3-002 |
| ENG-P5-001 | Freeze reusable contracts and verify feature graph | TODO | ENG-P4-001, QA-P4-002 |
| GAME-P5-001 | Freeze content and candidate behavior | TODO | GAME-P4-006, QA-P4-001 |
| REL-P5-001 | Resolve release/license/distribution decisions | BLOCKED | Owner deferred DEC-010 choices |
| QA-P5-001 | Full frozen-candidate regression | TODO | ENG-P5-001, GAME-P5-001, QA-P4-003, REL-P4-001 |
| REL-P5-002 | Dependency and asset provenance audit | TODO | REL-P5-001, GAME-P5-001 |
| REL-P5-003 | RC gate and clean target-hardware qualification | TODO | QA-P5-001, REL-P5-002, QA-P4-003 |
| QA-P5-002 | Validate any post-RC fixes on final artifact | TODO | REL-P5-003 |
| REL-P5-004 | Final 1.0 gate and release notes | TODO | QA-P5-002, REL-P5-003, QA-P5-003 |
| QA-P5-003 | RC phase-end QA: unit, integration, e2e and simulated human QA on the frozen candidate | TODO | QA-P5-002, QA-P4-004 |

Critical path: ENG-P1-001 → GAME-P1-001 / GAME-P1-003 / ENG-P1-004 → GAME-P2-001 → … → GAME-P2-010 → QA-P2-001 (owner playtest) → QA-P2-002 (phase-end QA) → Alpha encounters/profile/UI → Beta production and native qualification → RC → separate 1.0 gate.

## 3. Phase 1: FOUNDATION

**Goal:** Reproducible build, headless simulation core, content validation, the three engine contracts, the SVG asset pipeline and a first measured baseline. Entry: owner-approved Stages 3/4 and this dependency-checked plan; exact initial task scope; clean understanding of existing user changes. Exit: BUILD-001 verified, headless tick/content foundations, tested geometry/camera/catalog boundaries, SVG/raster replacement proof and an actual initial runtime/build baseline with no invented budgets.

### P1-M1: Reproducible build and authoritative core

#### ENG-P1-001. Two-crate pinned bootstrap (M) — DONE

**Depends on:** Approved planning gates. **Read first:** [SP-001](docs/specs/sp-001-workspace.md) (architecture §1–2; ADR-002, ADR-003, ADR-009, ADR-012).

**Done when:** Native client compiles; headless smoke has no window/audio initialization.

**Microtasks:**

- [x] Root `Cargo.toml` workspace (resolver 3, exact pins per DEC-015), `rust-toolchain.toml` 1.99.1, `rustfmt.toml` (tabs)
- [x] `crates/omega_2d` and `crates/omega_game` with `client` feature gates and three binaries
- [x] Headless bootstrap tests for both crates (no window/render/audio resources)
- [x] `cargo metadata --locked`, locked core tests and client check pass

**Asserts:** V01 AC1/2; READY packet above

#### QA-P1-001. Windows CI and dependency policy (S) — DONE

**Depends on:** ENG-P1-001. **Read first:** [SP-001](docs/specs/sp-001-workspace.md) (architecture §1–2; ADR-002, ADR-003, ADR-009, ADR-012).

**Done when:** Locked CI reproduces fmt/lint/core/client checks; reviewed action SHAs and advisory/license policy recorded. Owner pushes the branch and attaches the run URL (agents cannot push, REV-014).

**Microtasks:**

- [x] `deny.toml` with the license allowlist (MIT-0 reviewed) and `cargo deny check` green
- [x] `.github/workflows/ci.yml` with SHA-pinned actions: windows-2022 Rust job, cargo-deny job, Markdown job
- [x] `scripts/check.ps1` runs the same gates locally
- [x] Owner-authorized push; hosted run URL recorded in the evidence index

**Asserts:** V01 AC3/4; real CI log

#### GAME-P1-001. Tick schedule, IDs, RNG and headless harness (M) — DONE

**Depends on:** ENG-P1-001 [BUILD-001 local verification, REV-014]. **Read first:** [SP-002](docs/specs/sp-002-simulation.md) (GDD-005 (§6); architecture §5–6; ADR-011).

**Done when:** Explicit input/seed drives one ordered schedule; pause and queue retention pass.

**Microtasks:**

- [x] `sim` module: `GameTick` schedule with ordered `TickSet` and explicit flushes, `SimTick`, `SimPaused`, `IdAllocator`, `TickInput`, `PresentationQueue`
- [x] Named RNG streams (`RngStreams`, tag XOR, rejection sampling) and `RunSeed` hex parsing
- [x] `SimHarness` and `omega-sim` (`smoke`, `repeatability`, input validation)
- [x] `sim_contract.rs` AC1–AC4 (7 tests), including the real fixed-step pause path

**Asserts:** V02 AC1–4

#### GAME-P1-002. Typed content family schemas (M) — DONE

**Depends on:** ENG-P1-001 [BUILD-001 local verification, REV-014]. **Read first:** [SP-003](docs/specs/sp-003-content.md) (GDD-012 (§12) and the catalogs in §9–11, §13; ADR-006).

**Done when:** Valid fixtures deserialize; unknown fields/types/units produce field errors.

**Microtasks:**

- [x] Strict JSON reader with field paths and unknown-field rejection (`content/reader.rs`)
- [x] Typed records for every GDD-012 family, revision 2 units (`base_rate_centi`, `regen_tenths`, `boss_request_tick`)
- [x] Full and prototype fixtures generated from the GDD tables
- [x] `content_contract.rs` AC1 cases

**Asserts:** V03 AC1

#### GAME-P1-003. Cross-reference validator and content CLI (M) — DONE

**Depends on:** GAME-P1-002. **Read first:** [SP-003](docs/specs/sp-003-content.md) (GDD-012 (§12) and the catalogs in §9–11, §13; ADR-006).

**Done when:** Profile counts, IDs, unlock reachability and immutable reload validated.

**Microtasks:**

- [x] Cross-reference validator: global IDs, profile counts, boss rules, unlock awards and reachability, enemy coverage
- [x] `omega-content validate PATH --profile prototype|full` CLI
- [x] Immutable `ContentSnapshot` with SHA-256 revision and Menu-only `ContentStore::reload`
- [x] `content_contract.rs` AC2–AC4 and CR-001 rules

**Asserts:** V03 AC2–4

### P1-M2: Concrete 2D contracts and measured foundation

#### ENG-P1-002. Circle/capsule/sweep geometry (M) — DONE

**Depends on:** ENG-P1-001, QA-P1-001. **Read first:** [SP-004](docs/specs/sp-004-geometry.md) (GDD-004 (§5), GDD-005 §6.2; ADR-004).

**Done when:** Tangency, zero motion, finite TOI and bounds pass with no game-rule dependency.

**Microtasks:**

- [x] `geometry` module: `Circle`, `Capsule`, `Bounds2`, overlap, point-segment distance, swept-circle TOI, radius-aware clamp
- [x] `geometry_contract.rs` AC1–AC4 (8 tests, 10 000 generated cases)

**Asserts:** V04 AC1–4

#### ENG-P1-003. Logical viewport and inverse camera map (S) — DONE

**Depends on:** ENG-P1-001 (REV-015). **Read first:** [SP-012](docs/specs/sp-012-camera-visuals.md) (Art Bible §4–7, §9–10; architecture §3 (ENG-CAM-001, ENG-VIS-001)).

**Done when:** Center/corner round trips and letterbox rejection pass; delivers ENG-CAM-001.

**Microtasks:**

- [x] `camera::ViewportMapping` with letterbox rejection and typed errors
- [x] Client `LogicalCameraPlugin` (viewport plus fixed orthographic scaling)
- [x] `camera_contract.rs` AC1 (4 tests, five display sizes)

**Asserts:** V12 camera test / AC1

#### ENG-P1-004. Visual catalog and static preview contract (M) — DONE

**Depends on:** ENG-P1-003. **Read first:** [SP-012](docs/specs/sp-012-camera-visuals.md) (Art Bible §4–7, §9–10; architecture §3 (ENG-CAM-001, ENG-VIS-001)).

**Done when:** IDs, extent/pivot, frame validation and fallback work in a minimal static preview; first ENG-VIS-001 subset.

**Microtasks:**

- [x] `visuals` module: `VisualId`, `VisualCatalog` validation (extent, pivot, frames, fallback cycles, categories), `resolve` with fallback and typed essential errors
- [x] Client `sprite_parts`/`pose_transform`; static preview in `omega-game`
- [x] `visual_contract.rs` AC2/AC3 (5 tests); native screenshots

**Asserts:** V12 AC2/3; native preview

#### ENG-P1-005. Offline exporter/manifest validation (M) — DONE

**Depends on:** ENG-P1-004; GAME-P1-003 validated fixture contract. **Read first:** [SP-013](docs/specs/sp-013-asset-pipeline.md) (Art Bible §8–10, §12; `assets/presentation/proof-manifest.json`).

**Done when:** Pinned exporter rejects unsafe input and publishes validated outputs with hashes.

**Microtasks:**

- [x] `scripts/export-assets.ps1`, `scripts/validate-assets.ps1`, `scripts/lib/Assets.psm1` with SVG safety checks before resvg 0.48.1
- [x] Lock file with source/output hashes, command and exporter version
- [x] `validate-assets.ps1 -Reproduce` byte-identical

**Asserts:** V13 AC1/2

#### GAME-P1-004. Original proof assets and raster substitution (S) — IN_REVIEW

**Depends on:** ENG-P1-005 [ENG-VIS-001]; GAME-P1-001. **Read first:** [SP-013](docs/specs/sp-013-asset-pipeline.md) (Art Bible §8–10, §12; `assets/presentation/proof-manifest.json`).

**Done when:** Player/projectile/warning/icon proof renders; raster swap preserves logical pose/trace; screenshots at 1280x720 and 1920x1080 compare the smallest friendly and hostile projectile cores (early AA-04 check, REV-009).

**Microtasks:**

- [x] Original proof SVGs (player, friendly and hostile cores, ground warning, enemy fallback, item icon) and provenance id `prov_proof_2026_10_24`
- [x] Exported PNGs, `proof-catalog.json` and the raster substitute `proof-catalog-pixel.json`
- [x] `asset_contract.rs` and `skin_swap_contract.rs` (DA-20 trace equality)
- [x] Native screenshots at 1280x720 and 1920x1080 in `docs/validation/evidence/GAME-P1-004/`
- [ ] Owner native visual review of the screenshots (AA-01/05/07, early AA-04 readability: friendly core about 8.5 px at 720p)

**Asserts:** V13 AC3/4; AA-01/05/07; readability screenshots — plus `scripts/check.ps1` exit 0.

#### QA-P1-002. Empty-scene and build baseline (S) — IN_REVIEW

**Depends on:** ENG-P1-004, GAME-P1-001, GAME-P1-004. **Read first:** [SP-016](docs/specs/sp-016-diagnostics.md) (performance plan; architecture §8).

**Done when:** PERF-01/08 measured with hardware/config metadata; no budgets assigned yet.

**Microtasks:**

- [x] PERF-01 empty-scene benchmark (`omega-game --benchmark PERF-01`), three runs at 1280x720 and 1920x1080
- [x] PERF-08 build baseline (`scripts/measure-build.ps1`)
- [x] `diagnostic_contract.rs` (report fields, trace unchanged by measurement)
- [ ] Owner confirms the hardware/settings metadata; optional cold-build run with `-Cold`

**Asserts:** V16 reporting subset; native/build reports — plus `scripts/check.ps1` exit 0.

#### ENG-P1-006. Vendored Bevy fork wiring (DEC-018) (S) — DONE

**Depends on:** ENG-P1-001. **Read first:** [SP-001](docs/specs/sp-001-workspace.md) (architecture §1–2; ADR-002, ADR-003, ADR-009, ADR-012).

**Done when:** `engine/bevy/` holds the published 0.20.0 crates, `[patch.crates-io]` resolves every Bevy crate to them, patch log exists.

**Microtasks:**

- [x] Published Bevy 0.20.0 crates copied to `engine/bevy/`; `[patch.crates-io]` entries; workspace `exclude = ["engine"]`
- [x] Patch log and fork rules in `engine/README.md`; ADR-012

**Asserts:** V01 with path-resolved lockfile; engine/README.md

#### RES-P1-001. REA setup and Brotato inventory under DEC-019 (S) — TODO

**Depends on:** REA and Ghidra installed 2026-10-24. **Read first:** docs/research/02-rea.md; DEC-019.

**Done when:** REA readiness check on a synthetic fixture, then a read-only inventory of the owner copy; only prose notes in docs/research/brotato-notes/, raw output outside the repository.

**Microtasks:**

- [x] Install REA: `rea-agents@6.7.0` registered as a Claude Code MCP server at local scope for this project only (`claude mcp add --scope local rea -e GHIDRA_INSTALL_DIR=... -e JAVA_HOME=... -- cmd.exe /d /c npx -y rea-agents@6.7.0 mcp`), status Connected (2026-10-24)
- [x] Install the Ghidra backend: Ghidra 12.1.4 (SHA-256 `ddac49f9...d4db` matched the release) in `%LOCALAPPDATA%\Programs\ghidra_12.1.4_PUBLIC`, Temurin JDK 21.0.12; user variables `GHIDRA_INSTALL_DIR` and `JAVA_HOME`
- [ ] Readiness check on a synthetic target: `rea analyze target/release/omega-content.exe` started Ghidra analysis but hit `provider_timeout`; raise the timeout or use a small synthetic PE, and record tool versions
- [ ] Brotato is a Godot game: game logic lives in `Brotato.pck`, which REA 6.7.0 does not list as a supported format. Record which questions REA can answer (native `Brotato.exe` is the Godot engine) and ask the owner in `questions-to-dev.md` before adding any other PCK tool
- [ ] First prose note in `docs/research/brotato-notes/` following its README rules; raw output stays in `%LOCALAPPDATA%\ProjectOmega\rea\`

**Asserts:** Notes file plus recorded tool versions; no extracted material committed — plus `scripts/check.ps1` exit 0.

**Phase closing:** release `v0.3.0` ([CHANGELOG.md](CHANGELOG.md), [evidence index](docs/validation/evidence/README.md)). Remaining owner actions: visual review of the GAME-P1-004 screenshots; REA install for RES-P1-001.

#### QA-P1-003. End-to-end and simulated-QA harness (M) — TODO

**Depends on:** ENG-P1-004, GAME-P1-001. **Read first:** rule 1.11; SP-016; SP-017; `crates/omega_game/src/client/`.

**Done when:** `omega-game --e2e-script` runs a JSON input timeline against the real client and exits non-zero on a failed checkpoint; `omega-game --qa-bot <profile> --seed <hex>` drives seeded human-like input; `docs/validation/qa-checklist.md` exists with the Foundation items; CI runs the headless e2e path.

**Microtasks:**

- [ ] Test first: an e2e script that launches the preview, waits for assets, checks the camera/letterbox checkpoint and exits 0; a broken checkpoint exits 1
- [ ] Implement the e2e runner in the client: timeline of actions (key, mouse, wait ticks, resize, focus loss), checkpoints that export the authoritative snapshot and a screenshot to `artifacts/e2e/`
- [ ] Implement the QA bot input source behind the same action buffer the player uses (seeded reaction delay, aim error, hesitation, random menu exploration, mistaken inputs); profiles `cautious`, `aggressive`, `chaotic`
- [ ] Write `docs/validation/qa-checklist.md` (launch, resize, focus loss, pause spam, invalid input, catalog fallback); later phases append their items
- [ ] Add the headless-capable e2e step to `.github/workflows/ci.yml` and `scripts/check.ps1`; record evidence

**Asserts:** `cargo test -p omega_game --features client --locked --test e2e_contract`; `omega-game --e2e-script tests/e2e/foundation.json` exit 0; QA bot report for 3 profiles x 5 seeds — plus `scripts/check.ps1` exit 0.

## 4. Phase 2: PROTOTYPE

**Goal:** The GDD-017 prototype is playable: move, auto/manual aim, fight, collect scrap, shop, allocate energy, end in defeat with a summary. Seeded headless tests and one native playtest prove the loop. Entry: Foundation exit evidence, initial content/schema/geometry contracts and original proof assets. Exit: GDD's three-wave/two-weapon prototype can move, auto/manual aim, fight, collect scrap, shop, allocate energy and end in defeat/summary; targeted headless tests and a native playtest demonstrate the loop. No final-content feasibility claim yet.

### P2-M1: Playable combat slice

#### RES-P2-001. REA study: player stats, weapons and combat (S) — TODO

**Depends on:** RES-P1-001. **Read first:** rule 1.10; DEC-019; GDD §5–§7 and §9.2; docs/research/brotato-notes/README.md.

**Done when:** `docs/research/brotato-notes/` has an original prose note for player stats, weapons and combat answering the questions below, a GDD comparison, and a change request or a recorded "keep GDD" decision for each difference. Informs GAME-P2-001 to GAME-P2-005.

**Microtasks:**

- [ ] Questions: stat list, caps and stacking; weapon tiers and fire-rate rules; damage, armor and crit formulas; projectile, melee and area hit rules; invulnerability after a hit; knockback
- [ ] Inspect with REA (scope DEC-019) and ordinary play; keep raw output outside the repository
- [ ] Write the prose note with confidence labels and method; no extracted material
- [ ] Compare with GDD §5–§7 and §9.2; file change requests or record "keep GDD" per difference; list provisional approvals in `questions-to-dev.md`
- [ ] Run `bun run lint`; commit the note and any change request

**Asserts:** note exists and passes markdownlint; `git grep` finds no pasted code or data blocks from the game; every difference has a CR or a recorded decision.

#### GAME-P2-001. Stats, energy and exact numeric rules (M) — DONE

**Depends on:** GAME-P1-003. **Read first:** [SP-006](docs/specs/sp-006-stats.md) (GDD-006/007 (§7, §7.1 exact rule from CR-002); DA-04).

**Done when:** GDD examples, clamps, item order and six-point invariants pass.

**Microtasks:**

- [x] Promote: files `crates/omega_game/src/combat/{stats,energy}.rs`, `tests/stats_contract.rs`; name one test per SP-006 criterion
- [x] Test first: GDD §7.1 worked examples (15, 15, 27 ticks; 24-tick hammer floor) from the full fixture, integer expectations
- [x] Implement `EffectiveStats` (sum base + character + items, clamp once, checked arithmetic) and the CR-002 interval rule
- [x] Test item-order independence over generated orders and the six-point energy invariant
- [x] Damage formula (§7.2) with tier percentages and forced crit: 21 and 26 examples, armor clamp at 80
- [x] Run the verification commands and `scripts/check.ps1`; record evidence; close by rule 1.5

**Asserts:** V06 AC1–4 — plus `scripts/check.ps1` exit 0.

#### GAME-P2-002. Keyboard/mouse action buffer and aim (M) — DONE

**Depends on:** GAME-P1-001; ENG-P1-003 [ENG-CAM-001]. **Read first:** [SP-005](docs/specs/sp-005-input.md) (GDD-004 (§5) input table; ADR-008; `omega_2d::camera`).

**Done when:** Zero/multiple tick frames preserve input; auto/manual aim and focus pause pass.

**Microtasks:**

- [x] Promote: files `crates/omega_game/src/input/`, `tests/input_contract.rs`
- [x] Test first: a frame with zero fixed ticks and one with several ticks keep held state and consume digital edges once
- [x] Implement action buffer and `TickInput` sampling before the fixed loop (client system behind `client`)
- [x] Manual aim through `CurrentViewport::screen_to_world`; letterbox points ignored; vector under 1 LU keeps the last direction (default +X)
- [x] Focus loss sets `SimPaused` before another tick; resume discards stale edges
- [x] Run the verification commands and `scripts/check.ps1`; record evidence; close by rule 1.5

**Asserts:** V05 AC1–4 (controller device evidence later) — plus `scripts/check.ps1` exit 0.

#### GAME-P2-003. Player movement and actor state (S) — DONE

**Depends on:** GAME-P2-001, GAME-P2-002; ENG-P1-002 [ENG-GEOM-001]. **Read first:** [SP-007](docs/specs/sp-007-combat.md) (GDD-005 (§6), §7.2–7.3, §9.2; DA-05/06/07/13).

**Done when:** Diagonal speed, arena clamping and initial Patch loadout/HP match GDD.

**Microtasks:**

- [x] Promote: files `crates/omega_game/src/combat/{actor,movement}.rs`, `tests/combat_contract.rs`
- [x] Test first: DA-01 initial Patch state (slot 1 tier-1 Bolt at 3 points, HP 120, scrap 0) and DA-07 diagonal speed
- [x] Spawn the player entity with `SimId`, `Body`, `Health`, `Loadout` from the content snapshot
- [x] Movement in `TickSet::Movement`: normalized input × speed/60, clamp with `Bounds2::clamp_circle_center`
- [x] Run the verification commands and `scripts/check.ps1`; record evidence; close by rule 1.5

**Asserts:** V07 movement fixtures; DA-01/07 — plus `scripts/check.ps1` exit 0.

#### GAME-P2-004. Bolt and saw attack emission (M) — DONE

**Depends on:** GAME-P2-003. **Read first:** [SP-007](docs/specs/sp-007-combat.md) (GDD-005 (§6), §7.2–7.3, §9.2; DA-05/06/07/13).

**Done when:** Target tie-break, cooldown, projectile sweep and capsule timing follow one attack pipeline.

**Microtasks:**

- [x] Test first: DA-04 attack starts 15 ticks apart; no burst after a no-target period; target tie-break by lower `SimId`
- [x] Weapon timers and auto/manual target selection in `TickSet::Emit`, attack IDs from `IdAllocator`
- [x] Bolt projectile sweep with `sweep_circle` (earliest TOI, then lower target ID); saw capsule with windup/recovery
- [x] Presentation records for emission (new `PresentationKind` variants)
- [x] Run the verification commands and `scripts/check.ps1`; record evidence; close by rule 1.5

**Asserts:** V07; DA-04/05 — plus `scripts/check.ps1` exit 0.

#### GAME-P2-005. Damage, death, healing and scrap (M) — TODO

**Depends on:** GAME-P2-004. **Read first:** [SP-007](docs/specs/sp-007-combat.md) (GDD-005 (§6), §7.2–7.3, §9.2; DA-05/06/07/13).

**Done when:** Global invulnerability, one reward per death, regen and pickup settlement pass.

**Microtasks:**

- [x] Test first: DA-05 (26 damage, one hit per attack ID), DA-06 invulnerability 100–135, DA-13 simultaneous lethal
- [x] Damage resolution in `TickSet::Resolve` in attack-ID then target-ID order; crit draw from `RngStreams::crit` once per eligible hit
- [x] Death marks immediately; one scrap pickup per death; pickup collection and regen accumulator in `TickSet::CollectHeal`
- [x] Run the verification commands and `scripts/check.ps1`; record evidence; close by rule 1.5

**Asserts:** V07 AC3–5; DA-06/13 — plus `scripts/check.ps1` exit 0.

#### GAME-P2-006. Chaser/runner and three-wave scheduling (M) — DONE

**Depends on:** GAME-P2-005. **Read first:** [SP-008](docs/specs/sp-008-encounters.md) (GDD-010/011 (§10–11); CR-001; DA-11/12/17/18).

**Done when:** Authored warnings, spawn safety, common scaling and timer completion pass.

**Microtasks:**

- [x] Test first: DA-11 (live enemy at tick 3600 removed without scrap), DA-18 spawn postponement and cancel cap
- [x] Chaser/runner chase behavior; common scaling formulas (§10.1) with `ceil`
- [x] Batch scheduling at authored ticks, 45-tick warnings, eight boundary anchors with `RngStreams::spawn_anchors`, type draw with `spawn_types`
- [x] Run the verification commands and `scripts/check.ps1`; record evidence; close by rule 1.5

**Asserts:** V08 common-profile subset; DA-11/18 — plus `scripts/check.ps1` exit 0.

#### ENG-P2-001. Live pose/projectile presentation (M) — IN_REVIEW

**Depends on:** ENG-P1-004. **Read first:** [SP-012](docs/specs/sp-012-camera-visuals.md) (Art Bible §4–7, §9–10; architecture §3 (ENG-CAM-001, ENG-VIS-001)).

**Done when:** ENG-VIS-001 projects logical actors/projectiles without returning geometry/timing to simulation.

**Microtasks:**

- [x] Promote: files `crates/omega_2d/src/visuals/` (projection), `crates/omega_game/src/client/` presentation systems
- [x] Map `SimId` → presentation entity; interpolate `PreviousPose2` → `Pose2` for rendering only
- [x] Drain `PresentationQueue` once per rendered frame; spawn projectile cores with exact collision diameter
- [ ] Native check (owner, Q-001): catalog swap at Menu keeps the trace (DA-20) and alignment (AA-05)
- [x] Run the verification commands and `scripts/check.ps1`; record evidence; close by rule 1.5

**Asserts:** V12 AC2–4; catalog swap trace — plus `scripts/check.ps1` exit 0.

### P2-M2: Complete preparation loop

#### RES-P2-002. REA study: shop, economy and run flow (S) — TODO

**Depends on:** RES-P1-001. **Read first:** rule 1.10; DEC-019; GDD §3–§4 and §8; docs/research/brotato-notes/README.md.

**Done when:** `docs/research/brotato-notes/` has an original prose note for shop, economy and run flow answering the questions below, a GDD comparison, and a change request or a recorded "keep GDD" decision for each difference. Informs GAME-P2-007 to GAME-P2-010.

**Microtasks:**

- [ ] Questions: currency drops and collection; shop offer count, rarity and weighting; prices and inflation per wave; reroll cost; item stacking and limits; wave-to-shop flow
- [ ] Inspect with REA (scope DEC-019) and ordinary play; keep raw output outside the repository
- [ ] Write the prose note with confidence labels and method; no extracted material
- [ ] Compare with GDD §3–§4 and §8; file change requests or record "keep GDD" per difference; list provisional approvals in `questions-to-dev.md`
- [ ] Run `bun run lint`; commit the note and any change request

**Asserts:** note exists and passes markdownlint; `git grep` finds no pasted code or data blocks from the game; every difference has a CR or a recorded decision.

#### GAME-P2-007. Shop eligibility, prices and one refresh (M) — DONE

**Depends on:** GAME-P1-003, GAME-P2-001. **Read first:** [SP-009](docs/specs/sp-009-shop.md) (GDD-008 (§8); DA-08/09/10).

**Done when:** Two/two offers, exhaustion fallback and one paid refresh preserve RNG on rejection.

**Microtasks:**

- [x] Promote: files `crates/omega_game/src/shop/`, `tests/shop_contract.rs`
- [x] Test first: two weapons + two items, exhaustion fill from the other category, Empty slots
- [x] Prices with one final ceiling (28/42/56 example), refresh cost `8 + 2(w-1)`, one refresh per visit, RNG untouched on rejection (DA-10)
- [x] Run the verification commands and `scripts/check.ps1`; record evidence; close by rule 1.5

**Asserts:** V09 AC1/3/5 — plus `scripts/check.ps1` exit 0.

#### GAME-P2-008. Purchase, upgrade, replacement and allocation transactions (M) — DONE

**Depends on:** GAME-P2-007. **Read first:** [SP-009](docs/specs/sp-009-shop.md) (GDD-008 (§8); DA-08/09/10).

**Done when:** Atomic accepted/rejected actions; correct slot/tier/energy and no negative scrap.

**Microtasks:**

- [x] Test first: DA-08 (42 scrap charge, 41 rejected with no state/RNG change), DA-09 replacement cancel/confirm, DA-02 energy limits
- [x] Atomic `ShopCommand` → `ShopOutcome` or typed rejection; lowest empty slot; replacement keeps slot energy
- [x] Energy allocation commands with the six-point invariant and 0–3 per slot
- [x] Run the verification commands and `scripts/check.ps1`; record evidence; close by rule 1.5

**Asserts:** V09 AC2/4; DA-02/03/08/09 — plus `scripts/check.ps1` exit 0.

#### GAME-P2-009. Exact prototype content subset (S) — DONE

**Depends on:** GAME-P1-003. **Read first:** [SP-018](docs/specs/sp-018-production-content.md) (GDD §9–11, §13, §17; Art Bible §12).

**Done when:** GDD-017 subset validates in prototype mode and fails full-count mode.

**Microtasks:**

- [x] Promote: create `assets/content/` from the prototype fixture (SP-018 owns production data)
- [x] `omega-content validate assets/content --profile prototype` passes and `--profile full` fails
- [x] Client and harness load `assets/content/` explicitly in prototype profile
- [x] Run the verification commands and `scripts/check.ps1`; record evidence; close by rule 1.5

**Asserts:** V03 profile tests; V18 subset fixtures — plus `scripts/check.ps1` exit 0.

#### GAME-P2-010. Prototype Wave/Shop/Defeat lifecycle (M) — TODO

**Depends on:** GAME-P2-006, GAME-P2-008, GAME-P2-009. **Read first:** [SP-011](docs/specs/sp-011-run-states.md) (GDD-003 (§4); DA-03/11/13/16).

**Done when:** Three-wave fixture transitions with death precedence, transient cleanup and valid Start guards.

**Microtasks:**

- [ ] Promote: files `crates/omega_game/src/sim/{run,transitions}.rs` (SP-011 ownership), `tests/run_contract.rs`
- [ ] Test first: three-wave fixture Wave → Shop → Wave → … → Defeat with death precedence (DA-13) and Start guard (DA-03)
- [ ] Transient cleanup on wave exit; HP restore and cooldown reset on wave entry
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence; close by rule 1.5

**Asserts:** V11 prototype subset; DA-03/11/13/16 — plus `scripts/check.ps1` exit 0.

#### GAME-P2-011. Minimal HUD, shop controls and pause (M) — TODO

**Depends on:** GAME-P2-010; ENG-P2-001 [ENG-VIS-001]. **Read first:** [SP-014](docs/specs/sp-014-ui-accessibility.md) (GDD-015 (§15); Art Bible §7).

**Done when:** Keyboard/mouse can complete the loop; costs/HP/energy/reasons visible.

**Microtasks:**

- [ ] Promote: files `crates/omega_game/src/ui/`
- [ ] HUD: HP, wave n/3, timer, scrap, four slots with tier and energy pips
- [ ] Shop screen: four offers, costs, refresh, energy controls, Start with disabled reasons; keyboard and mouse
- [ ] Pause overlay; native run through the whole loop with screenshots
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence; close by rule 1.5

**Asserts:** V14 prototype subset; native run — plus `scripts/check.ps1` exit 0.

#### GAME-P2-012. Cleared placeholder event audio (S) — TODO

**Depends on:** GAME-P2-011; ENG-P1-005 [ENG-VIS-001 asset manifest]. **Read first:** [SP-015](docs/specs/sp-015-audio.md) (Art Bible §11; GDD-016).

**Done when:** Accepted hit/shop/wave cues and mute/state teardown operate without simulation effects.

**Microtasks:**

- [ ] Promote: files `crates/omega_game/src/audio/`, cleared placeholder files with provenance records
- [ ] Cue mapping from presentation records (hit, shop, wave start/end); mute and state teardown
- [ ] Native audition note; simulation trace unchanged with audio on/off
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence; close by rule 1.5

**Asserts:** V15 AC1/2; native audition — plus `scripts/check.ps1` exit 0.

#### GAME-P2-013. Balance simulator, bot policies and first prototype tuning (M) — TODO

**Depends on:** GAME-P2-010, QA-P1-003. **Read first:** rule 1.12; docs/validation/balancing.md; SP-016; GDD §7–§8.

**Done when:** `omega-sim --scenario balance` runs the prototype loop with all six bot policies, writes JSON plus a Markdown report, and the prototype numbers meet the Prototype bands of balancing.md or each miss has a recorded proposal.

**Microtasks:**

- [ ] Test first: balance scenario with a fixed seed is repeatable and the `idle` policy dies in waves 1–2
- [ ] Implement bot policies as `TickInput` and shop/energy command producers (no access to hidden state the player cannot see)
- [ ] Implement metric collection without touching RNG or authoritative state (diagnostics only observe)
- [ ] Run 2 policies x 200 seeds quick matrix, then the full prototype matrix; write `docs/validation/reports/<date>-prototype-balance.md`
- [ ] Propose and apply content changes for missed bands through the GDD (change request), re-run, keep before/after

**Asserts:** `cargo test -p omega_game --no-default-features --locked --test balance_contract`; balance report committed; `scripts/check.ps1` exit 0.

#### QA-P2-001. Prototype regressions and first playtest (M) — TODO

**Depends on:** GAME-P2-011, GAME-P2-012. **Read first:** [SP-007](docs/specs/sp-007-combat.md) (GDD-005 (§6), §7.2–7.3, §9.2; DA-05/06/07/13); [SP-009](docs/specs/sp-009-shop.md) (GDD-008 (§8); DA-08/09/10); [SP-011](docs/specs/sp-011-run-states.md) (GDD-003 (§4); DA-03/11/13/16); [SP-016](docs/specs/sp-016-diagnostics.md) (performance plan; architecture §8).

**Done when:** Seeded loop cases pass; native pacing/readability/economy observations and workload baseline recorded.

**Microtasks:**

- [ ] Seeded loop regression cases for V07/V09/V11 pass
- [ ] PERF-02 prototype workload report
- [ ] **Owner native playtest** (pacing, readability, economy) recorded; this is the Prototype exit and needs the owner
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence; close by rule 1.5

**Asserts:** V07/V09/V11; PERF-02 prototype; observed playtest — plus `scripts/check.ps1` exit 0.

#### QA-P2-002. Prototype phase-end QA: unit, integration, e2e and simulated human QA (M) — TODO

**Depends on:** QA-P2-001, QA-P1-003, GAME-P2-013. **Read first:** rule 1.11; docs/validation/qa-checklist.md.

**Done when:** All four test layers are green on the Prototype release commit; e2e scripts cover move, fight, shop, energy allocation and defeat; the QA bot completes 20 seeds per profile through three waves; the phase QA report exists with no open high-priority finding.

**Microtasks:**

- [ ] Add Prototype items to `docs/validation/qa-checklist.md` (shop exploits, double purchase, refresh spam, zero-energy start, death during shop transition)
- [ ] Write e2e scripts for the full prototype loop and for each checklist item that a script can check
- [ ] Run the QA bot: 3 profiles x 20 seeds; convert each failure into a bug task with a reproducing test, fix, re-run
- [ ] Write `docs/validation/reports/<date>-prototype-qa.md`; owner-only items go to `questions-to-dev.md`

**Asserts:** unit + integration + e2e + QA bot results in the report; `scripts/check.ps1` exit 0; CI green.

## 5. Phase 3: ALPHA

**Goal:** All approved systems and the full 4/12/10/2/40 content: ten waves, two bosses, saves, menus, controller and accessibility, full-run tests. Placeholder art and audio allowed. Entry: playable prototype evidence and recorded correction priorities; approved content envelope remains in force. Exit: all approved core systems and full 4/12/10/2/40 content, ten waves/two bosses, saves, menus, controller/accessibility functionality and full-run tests are present. Production polish may still use cleared placeholders.

### P3-M1: Full combat/content and durable progression

#### RES-P3-001. REA study: enemies, bosses, waves and spawning (S) — TODO

**Depends on:** RES-P2-001. **Read first:** rule 1.10; DEC-019; GDD §10–§11; docs/research/brotato-notes/README.md.

**Done when:** `docs/research/brotato-notes/` has an original prose note for enemies, bosses, waves and spawning answering the questions below, a GDD comparison, and a change request or a recorded "keep GDD" decision for each difference. Informs GAME-P3-016 to GAME-P3-007.

**Microtasks:**

- [ ] Questions: enemy behavior families and their timings; telegraphs; spawn placement and safety; wave scaling of HP and damage; boss phases and attack schedules; elite or special enemies
- [ ] Inspect with REA (scope DEC-019) and ordinary play; keep raw output outside the repository
- [ ] Write the prose note with confidence labels and method; no extracted material
- [ ] Compare with GDD §10–§11; file change requests or record "keep GDD" per difference; list provisional approvals in `questions-to-dev.md`
- [ ] Run `bun run lint`; commit the note and any change request

**Asserts:** note exists and passes markdownlint; `git grep` finds no pasted code or data blocks from the game; every difference has a CR or a recorded decision.

#### ENG-P3-001. Phase-driven warning/clip presentation (M) — TODO

**Depends on:** ENG-P2-001. **Read first:** [SP-012](docs/specs/sp-012-camera-visuals.md) (Art Bible §4–7, §9–10; architecture §3 (ENG-CAM-001, ENG-VIS-001)).

**Done when:** ENG-VIS-001 handles locked ground versus moving-origin cues, cancellation and reduced effects.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** V12 AC4; AA-03/04 — plus `scripts/check.ps1` exit 0.

#### GAME-P3-001. Remaining projectile/area payloads (M) — TODO

**Depends on:** GAME-P2-005. **Read first:** [SP-007](docs/specs/sp-007-combat.md) (GDD-005 (§6), §7.2–7.3, §9.2; DA-05/06/07/13).

**Done when:** Spread dedup, piercing, explosions and centered/offset circles pass all payload fixtures.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** V07 AC1/2/4 — plus `scripts/check.ps1` exit 0.

#### GAME-P3-002. Ranged, stationary and charge enemy profiles (—) — CANCELLED

Cancelled: Split into GAME-P3-016 to GAME-P3-019 (REV-010).

#### GAME-P3-016. Keep-distance gunner and scatter profiles (M) — TODO

**Depends on:** GAME-P2-006, GAME-P3-001. **Read first:** [SP-008](docs/specs/sp-008-encounters.md) (GDD-010/011 (§10–11); CR-001; DA-11/12/17/18).

**Done when:** Rivet Guard aimed shot and Shard Hopper spread match GDD timings and cue facts.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** V08 profile fixtures — plus `scripts/check.ps1` exit 0.

#### GAME-P3-017. Stationary sentry and orbit skimmer profiles (M) — TODO

**Depends on:** GAME-P3-016. **Read first:** [SP-008](docs/specs/sp-008-encounters.md) (GDD-010/011 (§10–11); CR-001; DA-11/12/17/18).

**Done when:** Spark Hub radial volley and Coil Skater orbit and shot match GDD timings.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** V08 profile fixtures — plus `scripts/check.ps1` exit 0.

#### GAME-P3-018. Charge rammer and ground-strike crusher profiles (M) — TODO

**Depends on:** GAME-P3-017. **Read first:** [SP-008](docs/specs/sp-008-encounters.md) (GDD-010/011 (§10–11); CR-001; DA-11/12/17/18).

**Done when:** Ram Cart swept charge and Clamp Walker locked strike match GDD.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** V08 profile fixtures; DA-17 — plus `scripts/check.ps1` exit 0.

#### GAME-P3-019. Beam marksman and heavy bulwark profiles (M) — TODO

**Depends on:** GAME-P3-018. **Read first:** [SP-008](docs/specs/sp-008-encounters.md) (GDD-010/011 (§10–11); CR-001; DA-11/12/17/18).

**Done when:** Survey Eye following beam and Plate Hauler knockback rules pass.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** V08 AC3; DA-17 — plus `scripts/check.ps1` exit 0.

#### GAME-P3-003. Beam, strike, orbit and heavy profiles (—) — CANCELLED

Cancelled: Split into GAME-P3-016 to GAME-P3-019 (REV-010).

#### GAME-P3-004. The Press encounter (M) — TODO

**Depends on:** GAME-P3-019; ENG-P3-001 [ENG-VIS-001]. **Read first:** [SP-008](docs/specs/sp-008-encounters.md) (GDD-010/011 (§10–11); CR-001; DA-11/12/17/18).

**Done when:** Wave-five pending/spawn/stamp/ram/recovery and completion guards pass.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** V08 boss-Press fixtures — plus `scripts/check.ps1` exit 0.

#### GAME-P3-005. The Foreman encounter (M) — TODO

**Depends on:** GAME-P3-004. **Read first:** [SP-008](docs/specs/sp-008-encounters.md) (GDD-010/011 (§10–11); CR-001; DA-11/12/17/18).

**Done when:** Wave-ten volley/discharge and simultaneous death/kill behavior pass.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** V08 boss-Foreman fixtures; DA-12/13 — plus `scripts/check.ps1` exit 0.

#### GAME-P3-006. Full character, weapon and item data (M) — TODO

**Depends on:** GAME-P2-009, GAME-P3-001. **Read first:** [SP-018](docs/specs/sp-018-production-content.md) (GDD §9–11, §13, §17; Art Bible §12).

**Done when:** Exact 4/12/40 records preserve IDs, formulas, prices and groups.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** V18 player-catalog subset — plus `scripts/check.ps1` exit 0.

#### GAME-P3-007. Full encounter, wave and challenge data (M) — TODO

**Depends on:** GAME-P3-005, GAME-P3-006. **Read first:** [SP-018](docs/specs/sp-018-production-content.md) (GDD §9–11, §13, §17; Art Bible §12).

**Done when:** All ten enemy types/two bosses/ten waves/four challenges resolve and are reachable.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** V18 full catalog — plus `scripts/check.ps1` exit 0.

#### GAME-P3-008. Idempotent challenge grants (S) — TODO

**Depends on:** GAME-P3-007. **Read first:** [SP-010](docs/specs/sp-010-progression-save.md) (GDD-013/014 (§13–14); architecture §7; ADR-007).

**Done when:** Grants occur on successful wave completion once; current pool stays frozen.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** V10 AC1/2; DA-14 — plus `scripts/check.ps1` exit 0.

#### GAME-P3-009. Profile/settings save protocol (M) — TODO

**Depends on:** GAME-P3-008. **Read first:** [SP-010](docs/specs/sp-010-progression-save.md) (GDD-013/014 (§13–14); architecture §7; ADR-007).

**Done when:** Separate generation slots, hashes and serial writes preserve readable state.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** V10 save fixtures — plus `scripts/check.ps1` exit 0.

#### GAME-P3-010. Interrupted-write and recovery behavior (M) — TODO

**Depends on:** GAME-P3-009. **Read first:** [SP-010](docs/specs/sp-010-progression-save.md) (GDD-013/014 (§13–14); architecture §7; ADR-007).

**Done when:** Windows faults, unsupported schema and reset confirmation preserve required data.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** V10 AC3–5; native persistence — plus `scripts/check.ps1` exit 0.

### P3-M2: Complete player-facing systems

#### RES-P3-002. REA study: characters, unlocks, progression and saves (S) — TODO

**Depends on:** RES-P2-002. **Read first:** rule 1.10; DEC-019; GDD §9.1, §13–§15; docs/research/brotato-notes/README.md.

**Done when:** `docs/research/brotato-notes/` has an original prose note for characters, unlocks, progression and saves answering the questions below, a GDD comparison, and a change request or a recorded "keep GDD" decision for each difference. Informs GAME-P3-008 to GAME-P3-015.

**Microtasks:**

- [ ] Questions: character differences and starting kits; unlock conditions; what persists between runs; menu and result flow; accessibility and input options
- [ ] Inspect with REA (scope DEC-019) and ordinary play; keep raw output outside the repository
- [ ] Write the prose note with confidence labels and method; no extracted material
- [ ] Compare with GDD §9.1, §13–§15; file change requests or record "keep GDD" per difference; list provisional approvals in `questions-to-dev.md`
- [ ] Run `bun run lint`; commit the note and any change request

**Asserts:** note exists and passes markdownlint; `git grep` finds no pasted code or data blocks from the game; every difference has a CR or a recorded decision.

#### GAME-P3-020. Boot, Menu and character selection flow (M) — TODO

**Depends on:** GAME-P3-007, GAME-P3-010, GAME-P2-010. **Read first:** [SP-011](docs/specs/sp-011-run-states.md) (GDD-003 (§4); DA-03/11/13/16).

**Done when:** Boot validation/error, Menu and unlocked-character selection follow GDD-003 guards (REV-010).

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** V11 Boot/Menu cases — plus `scripts/check.ps1` exit 0.

#### GAME-P3-011. Full guarded run lifecycle (M) — TODO

**Depends on:** GAME-P3-020, GAME-P3-005, GAME-P3-008. **Read first:** [SP-011](docs/specs/sp-011-run-states.md) (GDD-003 (§4); DA-03/11/13/16).

**Done when:** Nine shops, victory/defeat/abandon and summary settle correctly.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** V11 AC1–5 — plus `scripts/check.ps1` exit 0.

#### GAME-P3-012. Menus, selection, unlocks and results (M) — TODO

**Depends on:** GAME-P3-011; ENG-P3-001 [ENG-VIS-001]. **Read first:** [SP-014](docs/specs/sp-014-ui-accessibility.md) (GDD-015 (§15); Art Bible §7).

**Done when:** Every state/action is reachable; locked rewards and Unsaved/Retry are explicit.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** V14 menu/state cases — plus `scripts/check.ps1` exit 0.

#### GAME-P3-013. Controller actions, remapping and disconnection (M) — TODO

**Depends on:** GAME-P3-012, GAME-P2-002. **Read first:** [SP-005](docs/specs/sp-005-input.md) (GDD-004 (§5) input table; ADR-008; `omega_2d::camera`).

**Done when:** Native controller navigates/aims; remapping conflicts and loss pause correctly.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** V05 plus actual controller evidence — plus `scripts/check.ps1` exit 0.

#### GAME-P3-014. Accessible layouts, effects and onboarding (M) — TODO

**Depends on:** GAME-P3-013. **Read first:** [SP-014](docs/specs/sp-014-ui-accessibility.md) (GDD-015 (§15); Art Bible §7).

**Done when:** Scale/focus/reduced-effects/hint settings match GDD and survive settings reload.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** V14 AC2/4/5 — plus `scripts/check.ps1` exit 0.

#### GAME-P3-015. Complete shop comparisons and confirmation UI (M) — TODO

**Depends on:** GAME-P3-012, GAME-P2-008. **Read first:** [SP-014](docs/specs/sp-014-ui-accessibility.md) (GDD-015 (§15); Art Bible §7).

**Done when:** Four offers, actual rates, replacement details and energy controls work with both input modes.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** V14 AC1/3; DA-08/09/10 — plus `scripts/check.ps1` exit 0.

#### QA-P3-001. Full-run and catalog integration qualification (M) — TODO

**Depends on:** GAME-P3-014, GAME-P3-015, GAME-P3-010, GAME-P3-007. **Read first:** [SP-010](docs/specs/sp-010-progression-save.md) (GDD-013/014 (§13–14); architecture §7; ADR-007); [SP-011](docs/specs/sp-011-run-states.md) (GDD-003 (§4); DA-03/11/13/16); [SP-018](docs/specs/sp-018-production-content.md) (GDD §9–11, §13, §17; Art Bible §12).

**Done when:** Initial/full profile runs cover all states/grants; all reference/count checks pass with recorded evidence.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** V10/V11/V18; seeded and native runs — plus `scripts/check.ps1` exit 0.

#### QA-P3-003. Full-content balance simulation and tuning (M) — TODO

**Depends on:** QA-P3-001, GAME-P2-013. **Read first:** rule 1.12; docs/validation/balancing.md; RES-P3-001 and RES-P4-001 notes when available.

**Done when:** Full ten-wave matrix (4 characters x 6 policies x 200 seeds) meets the full-game bands of balancing.md or each miss has an applied, re-simulated proposal; report committed.

**Microtasks:**

- [ ] Run the full matrix on the Alpha content; report per wave, character, weapon, item and energy pattern
- [ ] Compare with the REA balance notes when they exist (rule 1.10); differences become change requests
- [ ] Tune through the GDD and content with before/after reports; regression tests for any formula fix
- [ ] Owner-only judgments (difficulty feel) to `questions-to-dev.md`

**Asserts:** balance report with before/after; `scripts/check.ps1` exit 0.

#### QA-P3-002. Alpha phase-end QA: unit, integration, e2e and simulated human QA (M) — TODO

**Depends on:** QA-P3-001, QA-P2-002, QA-P3-003. **Read first:** rule 1.11; docs/validation/qa-checklist.md.

**Done when:** All four layers green on the Alpha release commit; e2e covers every state of GDD-003, saves and recovery, controller navigation via injected input; the QA bot finishes full ten-wave runs for every character, 20 seeds per profile; report with no open high-priority finding.

**Microtasks:**

- [ ] Add Alpha items to the checklist (boss stalls, unlock idempotence, save interruption, abandon flows, remapping conflicts, UI scale 150%)
- [ ] Write e2e scripts for victory, defeat, abandon, unlock grant and save recovery
- [ ] Run the QA bot across characters and profiles; bug tasks with reproducing tests for every failure
- [ ] Write `docs/validation/reports/<date>-alpha-qa.md`; owner-only items to `questions-to-dev.md`

**Asserts:** four layers recorded in the report; `scripts/check.ps1` exit 0; CI green.

## 6. Phase 4: BETA

**Goal:** Production art and audio, balance and readability passes, native controller/accessibility checks, measured budgets, offline package, two-hour stability. Entry: Alpha complete and core features stable. Exit: original/cleared production art/audio, recorded balance/readability passes, native controller/accessibility checks, measured hardware budgets, clean offline package and two-hour stability evidence. Performance work follows observed bottlenecks.

### P4-M1: Production presentation and balance

#### RES-P4-001. REA study: balance numbers and pacing (S) — TODO

**Depends on:** RES-P3-001, RES-P3-002. **Read first:** rule 1.10; DEC-019; GDD P values in §7–§11; docs/research/brotato-notes/README.md.

**Done when:** `docs/research/brotato-notes/` has an original prose note for balance numbers and pacing answering the questions below, a GDD comparison, and a change request or a recorded "keep GDD" decision for each difference. Informs GAME-P4-006.

**Microtasks:**

- [ ] Questions: typical run length; damage and HP curves per wave; economy pacing; item and weapon power bands; difficulty spikes
- [ ] Inspect with REA (scope DEC-019) and ordinary play; keep raw output outside the repository
- [ ] Write the prose note with confidence labels and method; no extracted material
- [ ] Compare with GDD P values in §7–§11; file change requests or record "keep GDD" per difference; list provisional approvals in `questions-to-dev.md`
- [ ] Run `bun run lint`; commit the note and any change request

**Asserts:** note exists and passes markdownlint; `git grep` finds no pasted code or data blocks from the game; every difference has a CR or a recorded decision.

#### GAME-P4-001. Four production character visuals (M) — TODO

**Depends on:** QA-P3-001; ENG-P3-001 [ENG-VIS-001]. **Read first:** [SP-018](docs/specs/sp-018-production-content.md) (GDD §9–11, §13, §17; Art Bible §12).

**Done when:** Four distinct silhouettes/pivots pass hitbox and scale review with provenance.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** V18; AA-03/05/06 — plus `scripts/check.ps1` exit 0.

#### GAME-P4-002. Twelve weapon visuals and shared payload cues (—) — CANCELLED

Cancelled: Split into GAME-P4-007 to GAME-P4-009 (REV-010).

#### GAME-P4-007. Weapon visuals batch 1 (bolt, slug, needle, saw) and shared payload cues (M) — TODO

**Depends on:** GAME-P4-001. **Read first:** [SP-018](docs/specs/sp-018-production-content.md) (GDD §9–11, §13, §17; Art Bible §12).

**Done when:** Identities and actual collision cores distinguishable and aligned.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** V18; AA-03/04/06 — plus `scripts/check.ps1` exit 0.

#### GAME-P4-008. Weapon visuals batch 2 (hammer, fan, arc, nail) (M) — TODO

**Depends on:** GAME-P4-007. **Read first:** [SP-018](docs/specs/sp-018-production-content.md) (GDD §9–11, §13, §17; Art Bible §12).

**Done when:** Same as GAME-P4-007 for its four weapons.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** V18; AA-03/04/06 — plus `scripts/check.ps1` exit 0.

#### GAME-P4-009. Weapon visuals batch 3 (rocket, pulse, mortar, disk) (M) — TODO

**Depends on:** GAME-P4-008. **Read first:** [SP-018](docs/specs/sp-018-production-content.md) (GDD §9–11, §13, §17; Art Bible §12).

**Done when:** Same as GAME-P4-007 for its four weapons.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** V18; AA-03/04/06 — plus `scripts/check.ps1` exit 0.

#### GAME-P4-003. Enemy/boss/arena production art (—) — CANCELLED

Cancelled: Split into GAME-P4-010 to GAME-P4-013 (REV-010).

#### GAME-P4-010. Common enemy art 1–5 (chaser, runner, bulwark, gunner, rammer) (M) — TODO

**Depends on:** GAME-P4-009. **Read first:** [SP-018](docs/specs/sp-018-production-content.md) (GDD §9–11, §13, §17; Art Bible §12).

**Done when:** Silhouettes match radii and mechanics, with provenance.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** V18; native encounter visual matrix — plus `scripts/check.ps1` exit 0.

#### GAME-P4-011. Common enemy art 6–10 (sentry, marksman, scatter, crusher, skimmer) (M) — TODO

**Depends on:** GAME-P4-010. **Read first:** [SP-018](docs/specs/sp-018-production-content.md) (GDD §9–11, §13, §17; Art Bible §12).

**Done when:** Same as GAME-P4-010.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** V18; native encounter visual matrix — plus `scripts/check.ps1` exit 0.

#### GAME-P4-012. Boss art (The Press, The Foreman) (M) — TODO

**Depends on:** GAME-P4-011. **Read first:** [SP-018](docs/specs/sp-018-production-content.md) (GDD §9–11, §13, §17; Art Bible §12).

**Done when:** Damaging body recognizable; pipework decorative.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** V18; native encounter visual matrix — plus `scripts/check.ps1` exit 0.

#### GAME-P4-013. Arena art (S) — TODO

**Depends on:** GAME-P4-012. **Read first:** [SP-018](docs/specs/sp-018-production-content.md) (GDD §9–11, §13, §17; Art Bible §12).

**Done when:** One non-obstructive arena with no apparent false cover.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** V18; native visual review — plus `scripts/check.ps1` exit 0.

#### GAME-P4-004. Item icons, UI theme and bundled font (M) — TODO

**Depends on:** GAME-P4-013. **Read first:** [SP-018](docs/specs/sp-018-production-content.md) (GDD §9–11, §13, §17; Art Bible §12); [SP-014](docs/specs/sp-014-ui-accessibility.md) (GDD-015 (§15); Art Bible §7).

**Done when:** Forty unique icons and cleared font pass readable shop/UI scale review.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** V18/V14; AA-02/06 — plus `scripts/check.ps1` exit 0.

#### GAME-P4-005. Cleared production sounds/music and mix (M) — TODO

**Depends on:** GAME-P4-004, GAME-P2-012. **Read first:** [SP-015](docs/specs/sp-015-audio.md) (Art Bible §11; GDD-016).

**Done when:** Cue coverage, channel gains, no clipping and state cleanup verified with rights records.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** V15 AC1–4; native mix — plus `scripts/check.ps1` exit 0.

#### GAME-P4-006. Documented balance/readability pass (M) — TODO

**Depends on:** GAME-P4-005, QA-P3-001. **Read first:** [SP-016](docs/specs/sp-016-diagnostics.md) (performance plan; architecture §8); [SP-018](docs/specs/sp-018-production-content.md) (GDD §9–11, §13, §17; Art Bible §12).

**Done when:** At least two observed runs per character across initial/full pools; log economy, survival, power allocations and targeted changes with canonical rule updates. Uses the balance simulator (rule 1.12) with before/after reports for every change. Checklist includes Ram Cart trigger 220 LU versus 168 LU charge reach (REV-017; alternatives: trigger 180 LU or 30 charge ticks), Spark Hub boundary placement and 2/2/1/1 energy dominance (REV-021).

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** Reports, affected V06/V07/V09/V18 regressions — plus `scripts/check.ps1` exit 0.

### P4-M2: Native qualification and measurement

#### ENG-P4-001. Measured engine/render bottleneck assessment (M) — TODO

**Depends on:** GAME-P4-005, QA-P1-002. **Read first:** [SP-016](docs/specs/sp-016-diagnostics.md) (performance plan; architecture §8).

**Done when:** PERF-01/03/04/05/07 results identify actual hot paths or justify no optimization; no speculative engine feature.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** V16; before/after only if a scoped fix is approved — plus `scripts/check.ps1` exit 0.

#### QA-P4-001. Native controller/accessibility matrix (M) — TODO

**Depends on:** GAME-P4-006, GAME-P3-014, GAME-P3-015. **Read first:** [SP-014](docs/specs/sp-014-ui-accessibility.md) (GDD-015 (§15); Art Bible §7).

**Done when:** Keyboard/controller, focus loss, UI scales, muted audio and reduced effects pass with evidence.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** V14; AA-02/04/08 — plus `scripts/check.ps1` exit 0.

#### QA-P4-002. Two-hour stability and resource trends (M) — TODO

**Depends on:** ENG-P4-001, QA-P4-001. **Read first:** [SP-016](docs/specs/sp-016-diagnostics.md) (performance plan; architecture §8).

**Done when:** PERF-06 teardown trends/handles/queues recorded; confirmed failures have regressions.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** V16; long-session logs — plus `scripts/check.ps1` exit 0.

#### QA-P4-003. Select shipping hardware and measured budgets (S) — BLOCKED

**Depends on:** ENG-P4-001; owner hardware/budget decision. **Read first:** [SP-016](docs/specs/sp-016-diagnostics.md) (performance plan; architecture §8); [SP-017](docs/specs/sp-017-release-validation.md) (quality gates; DEC-010; docs/release.md).

**Done when:** Record tested shipping floor and measured P95/P99/resource budgets, with evidence and approval.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** Hardware results and decision record; no arbitrary threshold — plus `scripts/check.ps1` exit 0.

#### REL-P4-001. Offline Windows package and clean-launch workflow (M) — TODO

**Depends on:** GAME-P4-005, QA-P4-002. **Read first:** [SP-017](docs/specs/sp-017-release-validation.md) (quality gates; DEC-010; docs/release.md).

**Done when:** Script packages executable/content/notices without tools/secrets; clean launch and per-user saves work.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** V17 packaging subset; clean directory test — plus `scripts/check.ps1` exit 0.

#### QA-P4-004. Beta phase-end QA: unit, integration, e2e and simulated human QA (M) — TODO

**Depends on:** QA-P4-002, QA-P3-002. **Read first:** rule 1.11; docs/validation/qa-checklist.md.

**Done when:** All four layers green on the Beta release commit with production assets; e2e runs on the packaged build; the QA bot plays two-hour sessions and every character; report with no open high-priority finding.

**Microtasks:**

- [ ] Add Beta items to the checklist (production art readability cues, reduced effects, mute, long sessions, clean-machine launch)
- [ ] Run e2e scripts against the packaged build from REL-P4-001
- [ ] Run the QA bot: all profiles, 20 seeds, plus two long sessions; bug tasks for failures
- [ ] Write `docs/validation/reports/<date>-beta-qa.md`; owner-only items (feel, audio mix, readability) to `questions-to-dev.md`

**Asserts:** four layers recorded in the report; `scripts/check.ps1` exit 0; CI green.

## 7. Phase 5: RC → 1.0

**Goal:** Feature freeze, full regression, license/asset audit, release candidate and a separate final 1.0 decision. Entry: Beta evidence, fixed scope, complete original content/provenance and resolved shipping-floor budgets. Exit: RC passes, remaining release blockers are resolved, and a separate final 1.0 gate approves the qualified artifact. Store publication or external spending is not implied by planning approval.

### P5-M1: Freeze and release candidate

#### ENG-P5-001. Freeze reusable contracts and verify feature graph (S) — TODO

**Depends on:** ENG-P4-001, QA-P4-002. **Read first:** [SP-001](docs/specs/sp-001-workspace.md) (architecture §1–2; ADR-002, ADR-003, ADR-009, ADR-012); [SP-012](docs/specs/sp-012-camera-visuals.md) (Art Bible §4–7, §9–10; architecture §3 (ENG-CAM-001, ENG-VIS-001)).

**Done when:** Headless/client graphs and geometry/camera/visual contracts pass at the frozen candidate commit.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** V01/V04/V12 — plus `scripts/check.ps1` exit 0.

#### GAME-P5-001. Freeze content and candidate behavior (S) — TODO

**Depends on:** GAME-P4-006, QA-P4-001. **Read first:** [SP-018](docs/specs/sp-018-production-content.md) (GDD §9–11, §13, §17; Art Bible §12); [SP-011](docs/specs/sp-011-run-states.md) (GDD-003 (§4); DA-03/11/13/16).

**Done when:** Catalog/rule revision and known issues recorded; no untracked balance changes enter RC.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** V18/V11 — plus `scripts/check.ps1` exit 0.

#### REL-P5-001. Resolve release/license/distribution decisions (S) — BLOCKED

**Depends on:** Owner deferred DEC-010 choices. **Read first:** [SP-017](docs/specs/sp-017-release-validation.md) (quality gates; DEC-010; docs/release.md).

**Done when:** Final title/use/notice/distribution obligations recorded before release qualification; no unapproved storefront integration.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** Decision/provenance review — plus `scripts/check.ps1` exit 0.

#### QA-P5-001. Full frozen-candidate regression (M) — TODO

**Depends on:** ENG-P5-001, GAME-P5-001, QA-P4-003, REL-P4-001. **Read first:** [SP-017](docs/specs/sp-017-release-validation.md) (quality gates; DEC-010; docs/release.md).

**Done when:** All required automated/native cases pass on one identified candidate, including save recovery and confirmed bugs.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** Full quality gates + V17 native matrix — plus `scripts/check.ps1` exit 0.

#### REL-P5-002. Dependency and asset provenance audit (M) — TODO

**Depends on:** REL-P5-001, GAME-P5-001. **Read first:** [SP-017](docs/specs/sp-017-release-validation.md) (quality gates; DEC-010; docs/release.md).

**Done when:** Fresh advisory/dependency review and every shipped asset/font/audio notice resolved; exceptions explicit.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** cargo-deny evidence; AA-06 — plus `scripts/check.ps1` exit 0.

#### REL-P5-003. RC gate and clean target-hardware qualification (M) — TODO

**Depends on:** QA-P5-001, REL-P5-002, QA-P4-003. **Read first:** [SP-017](docs/specs/sp-017-release-validation.md) (quality gates; DEC-010; docs/release.md).

**Done when:** Candidate package passes clean target machine, approved budgets and known-issue gate; RC verdict recorded.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** V17; PERF/native evidence tied to artifact hash — plus `scripts/check.ps1` exit 0.

### P5-M2: Separate final 1.0 decision

#### QA-P5-002. Validate any post-RC fixes on final artifact (S) — TODO

**Depends on:** REL-P5-003. **Read first:** [SP-017](docs/specs/sp-017-release-validation.md) (quality gates; DEC-010; docs/release.md).

**Done when:** Every change after RC has targeted and affected regression evidence; unchanged artifact identity documented if no fix.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** Relevant checks + artifact hash comparison — plus `scripts/check.ps1` exit 0.

#### REL-P5-004. Final 1.0 gate and release notes (S) — TODO

**Depends on:** QA-P5-002, REL-P5-003, QA-P5-003. **Read first:** [SP-017](docs/specs/sp-017-release-validation.md) (quality gates; DEC-010; docs/release.md).

**Done when:** Separate final owner release verdict, exact artifact/notices/notes and no unresolved release blocker.

**Microtasks:**

- [ ] Promote to READY: assign exact files/globs and name one test per acceptance criterion (rule 1.2); expand these microtasks
- [ ] Write the failing acceptance tests first
- [ ] Implement within the spec allowed paths
- [ ] Run the verification commands and `scripts/check.ps1`; record evidence

**Asserts:** Final release evidence checklist; no automatic publication — plus `scripts/check.ps1` exit 0.

#### QA-P5-003. RC phase-end QA: unit, integration, e2e and simulated human QA on the frozen candidate (M) — TODO

**Depends on:** QA-P5-002, QA-P4-004. **Read first:** rule 1.11; docs/validation/qa-checklist.md; SP-017.

**Done when:** All four layers green on the exact release-candidate artifact; full checklist passed; report attached to the RC verdict.

**Microtasks:**

- [ ] Run every suite, e2e script and QA bot profile against the candidate artifact hash
- [ ] Re-run every regression test created by earlier phase-end QA
- [ ] Write `docs/validation/reports/<date>-rc-qa.md`; owner-only items to `questions-to-dev.md`

**Asserts:** four layers recorded in the report tied to the artifact hash; CI green.

## 8. Coverage and maintenance

Every spec maps to at least one task; `docs/specs/index.md` maps GDD-001–020 and ART-001–013 to specs. Use 20–40 h/week as capacity, not throughput; measure task cycle time after the Prototype before dating anything. Alpha behavior families and Beta art batches are the main scope risks; changing the approved content envelope needs a recorded decision. New bugs get new task IDs with a linked reproduction.
