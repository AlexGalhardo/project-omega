# Independent planning review — 2026-10-24

Reviewer role: adversarial principal game/engine architect, executed by Claude Fable 5.1 (claude-fable-5-1) under [PROJECT-REVIEW.md](../../PROJECT-REVIEW.md). The reviewer had no access to the planning conversation. This document records findings only; writing it applies no correction, and no finding below is resolved until its fix is made and re-verified.

## Reviewed state

| Item | Value |
| --- | --- |
| Repository | AlexGalhardo/project-omega, local checkout `C:\dev\project-omega`, branch `main` |
| Last commit | `b553738806c97ccde6e999c83b3b09fca515ed51` (2026-10-23, "security: remove wildcard from bun run permissions") |
| Tracked files at HEAD | 22, all Stage 1 research, decisions, IP policy, setup files and the Portuguese `overnight-context.md` |
| Working tree | Dirty. Modified: `.claude/settings.json`, `AGENTS.md`, `CLAUDE.md`, `PLAN.md`, `README.md`, `docs/decisions/*`, `docs/research/00-summary.md`, `open-questions.md`, `sources.md`. Deleted: `overnight-context.md`. Untracked: `CHANGELOG.md`, `PROJECT-REVIEW.md`, `ROADMAP.md`, `docs/gdd.md`, `docs/art-bible.md`, `docs/architecture/`, `docs/specs/`, `docs/validation/`, `docs/decisions/change-requests/`, `package.json`, `bun.lock`, `node_modules/`, `.claude/skills/` |
| Consequence | Every Stage 3, 4 and 5 artifact reviewed here exists only in the uncommitted working tree. No reviewable revision carries them. |

Edits made by the reviewer during this session, at the owner's explicit request and outside the review scope: project-scoped copy of the caveman helper skills/agents under `.claude/skills/` and `.claude/agents/`, `.caveman.json`, `.gitignore`, a corrected hook block plus plugin registration in `.claude/settings.json`, and a helper section appended to `CLAUDE.md`. None of these touches a planning contract. They are disclosed so the owner can separate them from the planner's work.

### Files inspected in full

`AGENTS.md`, `CLAUDE.md`, `PLAN.md`, `ROADMAP.md`, `README.md`, `CHANGELOG.md`, `PROMPT.md`, `PROJECT-REVIEW.md`, `run-overnight.ps1`, `.env.example`, `.claude/settings.json`, `docs/AI_MEMORY_SETUP.md`, `docs/gdd.md`, `docs/art-bible.md`, `docs/legal/ip-policy.md`, `docs/decisions/decision-register.md`, `docs/decisions/scope-and-non-goals.md`, `docs/decisions/change-requests/_template.md`, `docs/architecture/architecture.md`, `docs/architecture/performance-plan.md`, all eleven ADRs, `docs/specs/_template.md`, `docs/specs/index.md`, SP-001 through SP-018, `docs/validation/quality-gates.md`, `docs/validation/definition-of-done.md`, `docs/research/00-summary.md` through `06-sdd-and-agent-workflow.md`, `sources.md`, `open-questions.md`, and the committed `HEAD:overnight-context.md` and `HEAD:CLAUDE.md` for comparison.

### Tools and commands actually run

- `git log -1`, `git ls-files`, `git status --short`, `git diff --stat`, `git diff --check` (exit 2: trailing whitespace at `PLAN.md:3`), `git show HEAD:overnight-context.md`, `git show HEAD:CLAUDE.md`, `wc -lc` over all Markdown.
- An ephemeral Python script (`python -I`, not committed) that checked: every local Markdown link and heading anchor across the repository (0 broken after path normalization), the spec dependency graph from `docs/specs/index.md` (18 specs, no cycle, all files present), the ROADMAP.md task table (61 tasks: 11/14/17/11/8 per phase; ENG 9, GAME 38, QA 9, REL 5; 1 READY, 2 BLOCKED, 58 TODO; 47 M, 14 S; no undefined dependency, no cycle, no dependency on a later phase; all 18 specs mapped to at least one task), the GDD catalogs (4/12/40/10/2/10/4 rows, item groups 20+5+5+5+5, no duplicate IDs, 408 spawn requests) and the GDD worked examples (intervals 15/15/27/24 ticks, damage 21/26/15, prices 28/42/56, wave-4 Scrapper 37 HP / 10 damage, DA-01 120 HP, 120 ticks at 0.5 regen = 1 HP). All arithmetic matched the GDD text.

### Explicit verification limits

- No Cargo workspace exists; no Rust, Bevy, resvg, cargo-deny or CI command was run or could be run. Every "V01–V18" command remains a planned command.
- No network verification of external sources was performed by this review. Versions and dates in `sources.md` (Bevy 0.20.0, Rust 1.99.1, resvg 0.45.1, cargo-deny 0.18.4, rand_chacha 0.9.0, getrandom 0.3.4, sha2 0.10.9, serde 1.0.228, serde_json 1.0.145, windows-2022 image) and the Bevy 0.20 feature names used in `architecture.md` are accepted as the planner's dated reports, not re-verified facts. The cheapest verification is ENG-P1-001 itself.
- No gameplay, playtest, benchmark, asset export, native controller/readability test, legal review or dependency audit was performed. Balance statements below are arithmetic observations and hypotheses for playtesting.
- Private configuration values were not recorded. During this session the owner offered the local Brotato installation path for REA inspection; it was neither used nor recorded (see REV-019).

## Findings

Classification: **Objective defect** (document contradicts itself, another document or a stated rule), **Unverified assumption** (plan depends on something nobody has checked), **Preference** (reviewer judgment; not a gate). Severity follows PROJECT-REVIEW.md. "Blocks" names the gate, task or contract that cannot honestly pass until fixed.

### REV-001 — Stage status is contradictory across canonical files

| Field | Record |
| --- | --- |
| Classification | Objective defect |
| Severity | BLOCKER |
| Affected files | `PLAN.md` lines 3, 10, 150–157, 167, 198; `README.md` line 5; `CHANGELOG.md` line 9; `PROJECT-REVIEW.md` line 5; versus `ROADMAP.md` lines 3–8, `AGENTS.md` line 16, `docs/specs/index.md` line 33, `docs/specs/sp-001-workspace.md` line 96, `docs/validation/quality-gates.md` line 9; `PROMPT.md` §3 and §8 |
| Evidence | `PLAN.md`: "Stage 5 roadmap is not yet written; no implementation task is READY", Stage 5 checklist entirely unchecked, Stage 6 deliverable unchecked, "Next planning artifact: Stage 5 PLAN.md". `README.md`: "Stage 5 roadmap is next … there are no READY tasks yet." `PROJECT-REVIEW.md`: "**Stage 5 PLAN.md is absent.**" `ROADMAP.md`: "Status: **APPROVED planning baseline** … This is the canonical implementation task plan … Next READY task: **ENG-P1-001**." `AGENTS.md`: "Current stage/status lives only in PLAN.md" and step 1 "Read PLAN.md dashboard", but PLAN.md has no dashboard; the dashboard is in ROADMAP.md. `PROMPT.md` §8 names the Stage 5 artifact `PLAN.md`; the planner wrote it as `ROADMAP.md` without updating the mandate text or PLAN.md. |
| Consequence | An implementation agent following the AGENTS.md loading order reads PLAN.md first and is told nothing is READY; an agent that reads ROADMAP.md is told ENG-P1-001 is READY. The Stage 5 gate ("internally consistent; next tasks are READY") cannot be declared passed while the status file of record denies the roadmap exists. |
| Recommended fix | Make PLAN.md the single status file it claims to be: add a dashboard (current stage 5/6, link to ROADMAP.md as the task plan, next READY task, blockers, latest verified checkpoint), tick the Stage 5 and Stage 6 deliverables that actually exist, fix "Stage 5 PLAN.md" to "ROADMAP.md", and align README.md, CHANGELOG.md and PROJECT-REVIEW.md line 5. Alternatively merge ROADMAP.md into PLAN.md to match PROMPT.md literally; either is acceptable if applied everywhere. Remove the trailing whitespace at `PLAN.md:3`. |
| Blocks implementation | Yes. ENG-P1-001 cannot be honestly promoted or started while the canonical status file denies the roadmap. |

### REV-002 — Stage 5 approval is not recorded in the decision register

| Field | Record |
| --- | --- |
| Classification | Objective defect |
| Severity | HIGH |
| Affected files | `ROADMAP.md` line 3; `docs/decisions/decision-register.md` DEC-012 (Consequences: "Stage 5 task/dependency mapping remains to create"); `docs/research/00-summary.md` line 68 ("dependency-checked tasks and owner approval before implementation begins"); `AGENTS.md` line 12 ("User authorization already recorded in the session/register takes precedence") |
| Evidence | ROADMAP.md states APPROVED "under the owner's instruction to approve draft/proposed artifacts and create the plan". DEC-012 records approval of Stage 4 artifacts and explicitly lists Stage 5 as future work. No DEC-013 or any register entry records owner approval of ROADMAP.md, its 61 tasks or the READY promotion of SP-001/ENG-P1-001. PROJECT-REVIEW.md line 39 confirms the roadmap was "authored subsequently" to the handoff. |
| Consequence | The roadmap's APPROVED status rests on an inferred standing instruction, which the project's own contract says must be recorded in the register. Any later dispute about whether ENG-P1-001 was authorized has no evidence. |
| Recommended fix | Owner reads ROADMAP.md and records DEC-013 (approve / approve with conditions / reject), including the exact revision approved. Until then, ROADMAP.md status should read "PROPOSED, awaiting DEC-013" and SP-001 should return to APPROVED. |
| Blocks implementation | Yes. READY promotion of ENG-P1-001 requires recorded stage-gate authorization (quality-gates.md "READY audit"). |

### REV-003 — The claimed documentary audit was never recorded

| Field | Record |
| --- | --- |
| Classification | Objective defect |
| Severity | HIGH |
| Affected files | `ROADMAP.md` line 9 and lines 173–175; `PLAN.md` line 131; `docs/validation/quality-gates.md` lines 49–51 |
| Evidence | ROADMAP.md dashboard: "Latest verified checkpoint: documentary design/spec audit recorded in quality gates". ROADMAP.md "Documentary validation record": "Results are recorded in quality-gates.md." PLAN.md: "Documentary audit is recorded in quality-gates.md." quality-gates.md §"Stage 4 documentary record" contains one future-tense sentence: "The final documentary audit **will** record actual file/heading links, IDs, spec dependency graph…" and no results. Only GDD-020 records executed checks, and those cover the GDD/Art Bible only. |
| Consequence | Three documents cite a verification record that does not exist. The project's own rule (AGENTS.md: "No invented tool runs…") is violated by its planning artifacts. The reviewer's reproduction found the underlying content passes (0 broken links, acyclic graphs, full spec coverage, see "Tools and commands actually run"), so this is a missing record, not a hidden failure. |
| Recommended fix | Record the actual Stage 4/5 documentary results in quality-gates.md with date, method and counts (this review's numbers may be cited as independent reproduction), or change the three claims to "pending". |
| Blocks implementation | Yes for the Stage 5 gate statement "internally consistent"; no for code work once recorded. |

### REV-004 — All Stage 3–5 artifacts are uncommitted

| Field | Record |
| --- | --- |
| Classification | Objective defect |
| Severity | HIGH |
| Affected files | Working tree versus `HEAD` `b553738`; `PLAN.md` lines 188–192 ("Checkpoint after each stage and commit"); `AGENTS.md` line 32 (tasks require "explicit file ownership and base commit") |
| Evidence | `git ls-files` lists 22 files, none from Stages 3–5. `git status` shows the GDD, Art Bible, architecture, ADRs, 18 specs, validation docs, ROADMAP.md, CHANGELOG.md and PROJECT-REVIEW.md as untracked, and AGENTS.md, PLAN.md, README.md, the decision register and research files as modified. The planner's own checkpoint rule was not followed. |
| Consequence | Approval "belongs to a specific revision" (Track G) but no revision exists; a disk failure or careless reset loses Stages 3–5; ENG-P1-001 has no base commit; this review cannot cite a commit hash for the content it inspected. |
| Recommended fix | Owner (push is denied to agents) commits in stage-grouped commits, for example `docs(stage-3): gdd and art bible`, `docs(stage-4): architecture, adrs, specs, validation`, `docs(stage-5): roadmap`, `docs(stage-6): review handoff`, then records those hashes in the register entries. Do not blanket-add: `node_modules/` was unignored until this session. |
| Blocks implementation | Yes. ENG-P1-001 needs a base commit containing the approved contracts it implements. |

### REV-005 — ENG-P1-001 READY packet omits files its own validation requires

| Field | Record |
| --- | --- |
| Classification | Objective defect |
| Severity | MEDIUM |
| Affected files | `ROADMAP.md` line 26 (packet "Allowed" list); `docs/specs/sp-001-workspace.md` lines 126–131 and 137; `docs/architecture/architecture.md` §2 (cargo-deny 0.18.4, license allowlist) |
| Evidence | The packet allows root `Cargo.toml`/`Cargo.lock`/`rust-toolchain.toml`, the two crate manifests, `lib.rs`, the bin entry, two `bootstrap_contract.rs` tests and a `.gitignore` addition. Its required check V01 includes `cargo deny check`, and the architecture's license allowlist can only be expressed in `deny.toml`, which SP-001 lists as allowed scope but the packet does not. `scripts/check.ps1` is likewise in SP-001 scope but not in the packet. cargo-deny must also be installed (`cargo install` is not in the host allow list). |
| Consequence | The first implementer either edits a forbidden file, runs `cargo deny check` without the approved policy, or stops. The packet also depends on an owner-side tool install it does not mention. |
| Recommended fix | Add `deny.toml` (and optionally `scripts/check.ps1`) to the packet, or move `cargo deny check` and `deny.toml` explicitly to QA-P1-001 and drop it from V01 for ENG-P1-001. State the prerequisite tool installs (Rust 1.99.1 via rustup, cargo-deny 0.18.4) as owner setup. |
| Blocks implementation | Yes for completing ENG-P1-001 V01 as written; the task can start on the Cargo manifests meanwhile. |

### REV-006 — Overlapping allowed paths between SP-002 and SP-011

| Field | Record |
| --- | --- |
| Classification | Objective defect |
| Severity | MEDIUM |
| Affected files | `docs/specs/sp-002-simulation.md` line 193 (`crates/omega_game/src/sim/`); `docs/specs/sp-011-run-states.md` line 98 (`crates/omega_game/src/sim/transitions.rs`, `src/sim/run.rs`); compare `sp-007-combat.md` line 469 which explicitly excludes `stats.rs`/`energy.rs` owned by SP-006 |
| Evidence | SP-002's allowed scope is the whole `sim/` directory; SP-011 claims two files inside it. SP-016 shares `omega-sim.rs` with SP-002 and says "serially assigned"; the SP-002/SP-011 overlap carries no such note. |
| Consequence | Today the dependent tasks are sequential (GAME-P1-001 before GAME-P2-010), so no live conflict exists, but the rule "no task is READY with a conflicting file owner" cannot be checked mechanically, and a parallel worker would be allowed to touch the other spec's files. |
| Recommended fix | Add "excluding transitions.rs and run.rs (SP-011)" to SP-002's allowed scope, mirroring SP-007. |
| Blocks implementation | No now; yes for any parallel assignment of SP-002 and SP-011 tasks. |

### REV-007 — Boss warning start is unspecified

| Field | Record |
| --- | --- |
| Classification | Objective defect (underspecified) |
| Severity | MEDIUM |
| Affected files | `docs/gdd.md` §10.2 line 319 and §11 lines 342–353 and 377; `docs/specs/sp-008-encounters.md` AC1 |
| Evidence | GDD-010: "Bosses spawn once on their authored waves at `(0, 250)` after a 120-tick visible warning." The Wave record carries `boss_id?` and batch `request_tick`s, but no boss request tick; the Boss record carries `warning_ticks` but no start rule. Nothing states whether the boss warning begins at wave tick 0, 60, or elsewhere. |
| Consequence | SP-008-AC1 ("bosses are scheduled only on 5/10") cannot fix the spawn tick; two implementers would produce different wave-5 pacing and the DA-12 fixture has no authoritative start time. |
| Recommended fix | Add `boss_request_tick` to the Wave schema (with validation `request + warning_ticks < duration`) and author it for waves 5 and 10, or state in GDD-010 that the boss warning starts at wave tick 0. Record as a GDD revision 2 change through a change request. |
| Blocks implementation | No for Phase 1–2; yes for GAME-P3-004 (The Press) and GAME-P3-005. |

### REV-008 — `base_rate` has no declared exact unit

| Field | Record |
| --- | --- |
| Classification | Objective defect (numeric policy inconsistency) |
| Severity | MEDIUM |
| Affected files | `docs/gdd.md` §7 line 138 ("regen is represented as tenths … not arbitrary precision text"), §7.1 line 150, §12 example `"base_rate": 2.0` and weapon table values such as `0.65`; `docs/specs/sp-006-stats.md` ("checked integers/rationals … floor/ceil exactly once"); `docs/architecture/architecture.md` §6 ("Content decimals convert to exact declared units before simulation") |
| Evidence | Regen declares its unit (tenths). `base_rate` and the E(p) multipliers (1.60) are decimals with no declared denominator. `interval_ticks = ceil(60 / min(requested_rate, 10))` is boundary-sensitive: a floating evaluation of `60 / (2.0 * 1.6 * 1.25)` may or may not land exactly on 15 depending on evaluation order. |
| Consequence | SP-006-AC1's exact-match tests and DA-04 depend on an implementation choice the GDD does not make; two correct-looking implementations could disagree by one tick at boundaries. |
| Recommended fix | Declare `base_rate` as an integer in hundredths of attacks per second (bolt `200`, rocket `65`) and E(p) as `{0, 100, 160, 200}` percent; define the interval as `ceil(6000 / min(rate_centi * E_pct / 100 * (100 + attack_speed_pct) / 100, 1000))` evaluated in integers/rationals. Apply through a change request; no balance value changes. |
| Blocks implementation | No for ENG-P1-001; yes for GAME-P2-001 (SP-006) READY. |

### REV-009 — Projectile readability at the fixed full-arena camera is unverified and arithmetically tight

| Field | Record |
| --- | --- |
| Classification | Unverified assumption |
| Severity | MEDIUM |
| Affected files | `docs/gdd.md` §5 (arena 1200×675 LU shown in full), §9.2 (projectile radii 3–12 LU), §10.1 (enemy projectile radii 5–8 LU); `docs/art-bible.md` §6 ("2 physical pixels for critical projectile edges at 1280x720"), §4 (4 texels/LU) |
| Evidence | At 1280×720 the arena maps to about 1.07 px per LU. Needle Spitter (radius 3 LU) renders about 6.4 px in diameter, Bolt Driver (radius 4) about 8.5 px, Rivet Guard shots (radius 6) about 12.8 px. A 2 px outline on both sides leaves 2–4 px of interior for friendly shots. No native readability test exists; AA-04 is scheduled in Beta (QA-P4-001). |
| Consequence | If the smallest friendly and hostile cores are not distinguishable at 720p, the fix is either larger visual cores (presentation only) or larger radii (gameplay change) and the decision arrives late if measured in Beta. |
| Recommended fix | Pull a minimal AA-04 check forward into GAME-P1-004 (proof assets) using the real camera at 1280×720 and 1920×1080; record screenshots. Keep radii P. Consider allowing a visual core larger than the collision circle for projectiles under 4 LU radius, documented as presentation only, if the test fails. |
| Blocks implementation | No. |

### REV-010 — Several Alpha/Beta tasks are too broad for one reviewable change

| Field | Record |
| --- | --- |
| Classification | Preference (planning judgment), with an objective trigger in the roadmap's own rule |
| Severity | MEDIUM |
| Affected files | `ROADMAP.md` GAME-P3-002, GAME-P3-003 (four distinct AI profiles each), GAME-P3-011 (five dependencies, whole lifecycle), GAME-P4-002 (twelve weapon visuals), GAME-P4-003 (ten enemies, two bosses and arena art), lines 16 and 131 ("Split an oversized task … before starting it") |
| Evidence | Each profile in GDD §10.1 has its own state machine (keep-distance, charge, stationary volley, beam, spread, ground strike, orbit). Four per "M" task plus fixtures exceeds the "bounded integration/content batch" definition. GAME-P4-003 bundles 13 art families with provenance records. |
| Consequence | Review quality drops and partial batches get marked DONE under pressure; the roadmap already forbids that, so splitting will happen anyway but with ID churn mid-phase. |
| Recommended fix | Pre-split now with stable IDs: enemy profiles into pairs by behavior family (GAME-P3-002a/b, -003a/b as new IDs, not suffixes), weapon visuals into three batches of four, enemy/boss/arena art into common enemies 1–5, 6–10, bosses, arena. Keep GAME-P3-011 but move Boot/Menu/selection into its own task. |
| Blocks implementation | No. |

### REV-011 — Committed client settings let an agent edit its own permissions

| Field | Record |
| --- | --- |
| Classification | Objective defect (security) |
| Severity | MEDIUM |
| Affected files | `.claude/settings.json` allow list: `Edit(.claude/*)`, `Write(*.md)`, `Edit(*.md)`; previous `hooks` block (fixed this session) |
| Evidence | `Edit(.claude/*)` matches `.claude/settings.json` itself, so a session may extend its own allow list or delete deny entries without a prompt. The reviewer used exactly this permission in this session, at the owner's request, to fix the hook block and register the plugin; the capability is real. The pre-existing hook used keys `match`/`command` instead of the Claude Code schema (`matcher` plus `hooks[].type/command`) and was therefore inert. |
| Consequence | A compromised or confused agent can silently widen its sandbox. Because the file is committed, every contributor's client inherits it. |
| Recommended fix | Narrow `Edit(.claude/*)` to `Edit(.claude/skills/*)` and `Edit(.claude/agents/*)`; keep settings edits behind a prompt. Review `Write(*.md)` (it covers AGENTS.md and every spec, which AGENTS.md says are serially owned). |
| Blocks implementation | No. |

### REV-012 — Unrelated npm dependency and plain-HTTP tooling at repository root

| Field | Record |
| --- | --- |
| Classification | Objective defect (supply chain / hygiene) |
| Severity | MEDIUM |
| Affected files | `package.json` (`"ai-memory": "^1.0.0"`), `bun.lock`, `node_modules/` (untracked, previously unignored), `docs/AI_MEMORY_SETUP.md` (`http://aimemory.sh/...`, `AI_MEMORY_TOKEN`), `.env.example`; `docs/architecture/architecture.md` §1 ("Existing local Node/AI-memory setup is not a game dependency") |
| Evidence | The package is an open caret range, its publisher/provenance is not recorded anywhere, the setup guide links an HTTP (not HTTPS) install page and expects a token in `.env`, and until this session nothing ignored `node_modules/` or `.env`. The architecture explicitly disowns this tooling. |
| Consequence | Risk of committing `node_modules/` or a token; an unverified package executes on every `bun install` on the owner's machine; a future RC asset/dependency audit (REL-P5-002) must explain why a Node package sits in a Rust game repository. |
| Recommended fix | Move the AI-memory tooling out of the game repository or into a clearly separate `tools/ai-memory/` with an exact pinned version and recorded provenance; use HTTPS in the guide; keep `.gitignore` entries (added this session) for `node_modules/`, `.env`, `run.log`, `DONE.txt`. |
| Blocks implementation | No. |

### REV-013 — Overnight runner script is inconsistent with its own intent

| Field | Record |
| --- | --- |
| Classification | Objective defect (low impact) |
| Severity | LOW |
| Affected files | `run-overnight.ps1` lines 7, 18, 33, 49–53 |
| Evidence | `$maxRetries = 0` is logged but never enforced (the loop is unbounded until `DONE.txt` exists). `& claude code --continue` invokes a `code` subcommand; the deleted `overnight-context.md` used `claude -c -p "$PROMPT"`. On exit the script restores `powercfg` timeouts to fixed 10/20/30 minutes rather than the user's previous values. |
| Consequence | A night-long loop can run with no retry cap and leave the machine's power settings changed; the command line may fail immediately on current CLI versions. Not a planning-contract defect; AGENTS.md already says setup scripts do not bypass gates. |
| Recommended fix | Verify the flag with `claude --help`, enforce or remove `$maxRetries`, capture and restore prior `powercfg` values, or delete the script if the overnight workflow is abandoned. |
| Blocks implementation | No. |

### REV-014 — Owner-only push sits on the critical path before any game code

| Field | Record |
| --- | --- |
| Classification | Objective defect (undocumented dependency) |
| Severity | LOW |
| Affected files | `ROADMAP.md` QA-P1-001 ("real CI log"), GAME-P1-001 and GAME-P1-002 (depend on QA-P1-001); `.claude/settings.json` deny `git push` |
| Evidence | A real CI log requires pushing to GitHub; agents are denied `git push`; the roadmap does not name the owner step; two Phase 1 game tasks wait on it. |
| Consequence | Hidden owner action on the critical path; an agent reaching QA-P1-001 can only prepare the workflow file. |
| Recommended fix | Add "owner pushes and attaches the run URL" to QA-P1-001's acceptance; consider letting GAME-P1-001/002 depend on ENG-P1-001 only, since BUILD-001 local verification does not need hosted CI. |
| Blocks implementation | No. |

### REV-015 — SP-012 depends on SP-004 without a stated need

| Field | Record |
| --- | --- |
| Classification | Preference |
| Severity | LOW |
| Affected files | `docs/specs/index.md` (SP-012 → SP-001, SP-004); `ROADMAP.md` ENG-P1-003 → ENG-P1-002 |
| Evidence | ENG-CAM-001 is a viewport/inverse mapping; ENG-VIS-001 is catalog metadata and projection. Neither spec text uses circle/capsule/sweep queries. |
| Consequence | Camera and visual-catalog work waits on geometry for no technical reason, lengthening the Foundation chain. |
| Recommended fix | Drop the SP-004 dependency from SP-012 and let ENG-P1-003 depend on ENG-P1-001 only, unless the planner records the concrete shared type that requires it. |
| Blocks implementation | No. |

### REV-016 — Dependency versions and Bevy feature names remain unbuilt claims

| Field | Record |
| --- | --- |
| Classification | Unverified assumption |
| Severity | LOW |
| Affected files | `docs/architecture/architecture.md` §2; `docs/research/sources.md` E4-01–E4-15 |
| Evidence | The planner honestly labels the pins as "checked, not proven compatible by a project build". This review did not re-fetch any source. Features `std`, `multi_threaded`, `bevy_state`, `2d`, `ui`, `audio` and `resolver = "3"` with edition 2024 are plausible for the stated release but unconfirmed here. |
| Consequence | ENG-P1-001 may surface a first incompatibility; the roadmap already routes that through a change request. |
| Recommended fix | None beyond executing ENG-P1-001; record `cargo metadata --locked` output and the resolved feature graph as its evidence. |
| Blocks implementation | No. |

### REV-017 — Ram Cart charge cannot reach its trigger distance

| Field | Record |
| --- | --- |
| Classification | Unverified assumption (balance hypothesis) |
| Severity | LOW |
| Affected files | `docs/gdd.md` §10.1 Ram Cart paragraph |
| Evidence | Charge 420 LU/s for 24 ticks covers 168 LU; the charge triggers when the player is within 220 LU. A player standing still at 170–220 LU is never reached; the charge only threatens a player who approaches or stands inside about 168 LU plus radii. The Press ram (500 LU/s × 30 ticks = 250 LU) does not have this gap. |
| Consequence | Either an intended generous dodge window or a weak enemy; only playtesting can decide. |
| Recommended fix | Keep as is; add it to the first balance pass checklist (GAME-P4-006) with the alternative of trigger 180 LU or 30 charge ticks. |
| Blocks implementation | No. |

### REV-018 — Scope document not updated after GDD approval of controller/accessibility

| Field | Record |
| --- | --- |
| Classification | Objective defect (document sync) |
| Severity | LOW |
| Affected files | `docs/decisions/scope-and-non-goals.md` §Required for 1.0; `decision-register.md` DEC-010 ("provisional GDD recommendations, not approved 1.0 requirements … until approved with the GDD"); `docs/gdd.md` §15.2 ("requiring approval with this GDD"), status line ("APPROVED explicitly by the owner") |
| Evidence | DEC-010 made controller support and the accessibility baseline conditional on GDD approval. The GDD was approved on 2026-10-24, so they became 1.0 requirements, and the roadmap plans them (GAME-P3-013/014, QA-P4-001). The scope document still omits them from "Required for 1.0". |
| Consequence | Minor, but the scope document is cited as the approved scope authority and a reader could cut controller support as "not required". |
| Recommended fix | Add one bullet under Required for 1.0 citing GDD-015 and DEC-010's resolution. |
| Blocks implementation | No. |

### REV-019 — REA inspection offer received without the required prerequisites

| Field | Record |
| --- | --- |
| Classification | Objective defect (process), INFO for the documents |
| Severity | INFO |
| Affected files | `docs/legal/ip-policy.md` §Restricted inspection; `AGENTS.md` line 28; `docs/research/open-questions.md` U10 |
| Evidence | During this review the owner supplied the local Brotato installation path so REA could read the executable. No EULA/law review, no scoped question and no approval record exist; GDD-019 states REA is "unneeded for these original rules". The reviewer did not inspect, install or record the path. |
| Consequence | None yet. If inspection proceeds informally it would violate the project's own gate and contaminate original-design provenance. |
| Recommended fix | Record under U10 that the offer was made and declined pending prerequisites; if the owner wants inspection, first write the EULA/law note and a scoped question per ip-policy.md, then decide whether any approved rule actually needs it. |
| Blocks implementation | No. |

### REV-020 — Specs map acceptance criteria to test files, not named tests

| Field | Record |
| --- | --- |
| Classification | Preference |
| Severity | LOW |
| Affected files | `docs/specs/_template.md` ("Map each criterion to a named future test"); every SP-0xx "Test cases and commands" section |
| Evidence | Specs name one integration test file per domain (`combat_contract`, `shop_contract` …) and a single command; no criterion names its test function or fixture. The specs otherwise satisfy the review's "what to build, what not to touch, data ownership, systems, failure behavior, acceptance, validation" checklist by reference to architecture.md and the GDD. |
| Consequence | Acceptable at APPROVED; at READY promotion each task must add criterion-to-test names or evidence mapping becomes vague. |
| Recommended fix | Require the per-criterion test name in the task promotion step (quality-gates.md "READY audit"), not in every spec now. |
| Blocks implementation | No. |

### REV-021 — Economy and content arithmetic are consistent; balance remains a hypothesis

| Field | Record |
| --- | --- |
| Classification | INFO |
| Severity | INFO |
| Affected files | `docs/gdd.md` §8–§11 |
| Evidence | Reproduced: 408 spawn requests; expected common scrap at 100 % kill rate ≈ 1169 plus 75 boss scrap across nine shops, with base prices 12–34 inflated up to ×1.64, giving roughly 25–35 purchases over a run; all worked examples match; all unlocks reachable by Patch; 12 weapons = 6 initial + 2 + 2 + 1 + 1; 40 items = 20 + 5 × 4. Spark Hub (speed 0) spawns on boundary anchors, so about half of each radial volley enters a wall immediately. |
| Consequence | No contradiction found in the loop walk (new run, nine shops, two bosses, defeat/victory/abandon, unlock timing, save failure). Degenerate builds cannot be judged from arithmetic; energy spread 2/2/1/1 maximizes total rate multiplier (5.2) versus 3/3 (4.0), consistent with the stated hypothesis. |
| Recommended fix | None; carry the Spark Hub placement and the 2/2/1/1 dominance question into the first balance pass. |
| Blocks implementation | No. |

## Answers to the ten questions

1. **Is any Bevy fork justified?** No. No requirement in the GDD or architecture needs a private Bevy API; the three engine contracts are pure math, a viewport mapping and catalog metadata. An exception would need a reproduced blocker on the pinned release that neither a plugin nor a public-API workaround resolves, a minimal regression-tested patch, and an upstream issue, exactly as DEC-001 states. Nothing today meets that bar.
2. **Premature abstractions?** The `omega_2d` crate is marginal: its only concrete consumer is this game and the "another bounded 2D sprite game" reuse case is hypothetical. It is cheap enough to keep because the boundary is a dependency direction, not a framework, but any addition beyond ENG-GEOM-001, ENG-CAM-001 and ENG-VIS-001 should require a second consumer. Retained facilities are each justified by a concrete requirement: geometry by GDD-005 shapes, camera by GDD-004 fixed arena plus manual aim, visual catalog by GDD-016/ART-010 skin replacement. The two-slot save protocol, SHA-256 hashes and `omega-sim`/`omega-content` binaries are proportionate. Nothing speculative (physics, spatial index, atlases, allocators, property-testing library) is preapproved.
3. **Underspecified or contradictory?** Stage status (REV-001), boss warning start (REV-007), `base_rate` unit (REV-008), SP-002/SP-011 ownership (REV-006), scope document lag (REV-018). The combat, economy, state machine, save and unlock rules otherwise walk cleanly end to end.
4. **Too broad or fragmented?** Too broad: GAME-P3-002/003, GAME-P3-011, GAME-P4-002/003 (REV-010). Nothing is over-fragmented; Phase 1 and 2 slices are proportionate. Stage 5 exists as ROADMAP.md, so this is an actual task review, not a proposed decomposition; the review of PLAN.md as the status artifact is what fails (REV-001).
5. **Unverified assumptions and cheapest experiments?** Dependency compatibility and feature names → ENG-P1-001 (`cargo metadata --locked`, headless smoke). Projectile readability → one screenshot pass in GAME-P1-004 (REV-009). resvg export fidelity → ENG-P1-005 proof assets. Same-build repeatability → SP-002 twin-fixture test. Owner throughput → measure ENG-P1-001 through QA-P1-002 cycle time before dating anything.
6. **Cut or postpone?** Recommended cuts (not approved changes): drop the Node/AI-memory tooling from the repository (REV-012); remove `run-overnight.ps1` or fix it (REV-013); defer atlases, Criterion and property-testing crates as already planned. Already approved deferrals that should stay deferred: run resume, additional difficulties, other platforms, storefront. The approved 4/12/10/2/40 envelope should be reassessed only after QA-P2-001 produces throughput data, as the roadmap says.
7. **Can one developer maintain this at 20–40 h/week?** Plausibly for the engine and systems: the architecture is small, upstream-only and headless-testable. The risk is content: ten AI profiles, two bosses, twelve weapon payloads, 40 items, 56 original art families and a cleared audio set are the bulk of Alpha/Beta. Evidence that would sharpen the judgment: measured cycle time of the first six Foundation tasks, and the actual hours for one enemy profile and one production art family.
8. **What most increases the probability of 1.0?** Commit the planning state, fix REV-001/002/003, run ENG-P1-001 immediately to convert dependency claims into evidence, reach QA-P2-001 (first native playtest) with the smallest content subset, and defer every art/audio batch until the prototype loop is judged fun. Keep one active integration task and resist adding engine facilities before a second consumer exists.
9. **Is the Evidence → Decision → GDD → ADR → Spec → Task → Acceptance chain complete?** Mechanically yes: every GDD/ART requirement maps to specs in `index.md`, every spec maps to at least one task, every task names its spec and AC/V references, graphs are acyclic, links resolve. Missing links: (a) Stage 5 approval record (REV-002); (b) the documentary audit record the chain cites (REV-003); (c) per-criterion test names (REV-020); (d) controller/accessibility promotion to Required (REV-018); (e) `boss_request_tick` has no requirement, schema or task (REV-007).
10. **Can an implementation agent begin now without guessing?** No. Unmet objective conditions for ENG-P1-001: the canonical status file denies the roadmap exists (REV-001); no recorded Stage 5 approval (REV-002); no base commit containing the contracts (REV-004); `deny.toml`/cargo-deny not in the packet though V01 requires them (REV-005). Everything else needed by ENG-P1-001 (pins, feature gates, allowed files, headless criteria, report format) is concrete.

## Mandatory corrections

1. Resolve REV-001: one status authority, consistent stage/READY statements in PLAN.md, README.md, CHANGELOG.md, PROJECT-REVIEW.md and AGENTS.md step 1.
2. Resolve REV-002: owner records DEC-013 for ROADMAP.md at a named revision, or ROADMAP.md reverts to PROPOSED and SP-001 to APPROVED.
3. Resolve REV-003: record the actual documentary results in quality-gates.md or retract the three claims.
4. Resolve REV-004: commit Stages 3–6 in stage-grouped commits; record hashes in the register.
5. Resolve REV-005: fix the ENG-P1-001 packet file list and prerequisite tools.
6. Before promoting any SP-006 or SP-008 task: resolve REV-008 and REV-007 through change requests.

## Revised execution order

1. Owner: commit current planning state (REV-004); fix REV-001 text; record DEC-013 (REV-002); record documentary results (REV-003). Estimated effort: under one working session; no design change.
2. Planner or owner: patch the ENG-P1-001 packet (REV-005), SP-002 scope (REV-006), scope document (REV-018), optional REV-014/REV-015 simplifications; narrow `.claude/settings.json` (REV-011); move or pin the Node tooling (REV-012).
3. Re-promote SP-001/ENG-P1-001 to READY and start it. Record `cargo metadata --locked`, resolved features and cargo-deny output as evidence (REV-016).
4. Continue Phase 1 as planned, adding the early readability screenshot to GAME-P1-004 (REV-009).
5. Before Phase 2 GAME-P2-001: file and approve the `base_rate` unit change request (REV-008). Before Phase 3 GAME-P3-004: file the boss request tick change request (REV-007). Pre-split the broad Alpha/Beta tasks when Phase 3 planning is refreshed (REV-010).
6. Keep REA deferred (REV-019); revisit only with the ip-policy prerequisites recorded.

## Verdict

Scope of this verdict: complete implementation-ready planning as present in the working tree on 2026-10-24 (Stages 1–6 artifacts including ROADMAP.md), not the design quality of the GDD, Art Bible, architecture, ADRs or specs in isolation. Taken alone, those documents are internally consistent, honestly labeled and proportionate to a solo Windows 1.0; the defects are governance and record-keeping gaps that currently prevent any task from being honestly READY.

CHANGES REQUIRED
