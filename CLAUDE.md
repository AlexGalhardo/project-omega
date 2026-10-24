# Claude adapter

Read [AGENTS.md](AGENTS.md) as the canonical project operating contract, then follow its minimal context-loading order. Start every session at [ROADMAP.md](ROADMAP.md): its header names the current phase and next task, section 1 holds the plan rules.

Primary objectives (DEC-017, details in AGENTS.md): (1) study the owner's purchased Brotato copy with REA under the DEC-019 scope and re-create the game with original code and art; (2) build the project's own 2D engine as the vendored Bevy fork in `engine/bevy/`, developed in parallel with the game.

Honor this client's actual permissions in .claude/settings.json. Do not assume availability of other agents, skills, models or native hardware. Repository artifacts are English; owner conversation is Brazilian Portuguese.

## Session start routine (mandatory, every session)

1. `/caveman` (level from `.caveman.json`). The SessionStart hook announces it; if no "CAVEMAN MODE ACTIVE" line appears, activate it manually. Chat style only: repository files, commits, specs, evidence and change requests stay plain English.
2. Skill `karpathy-guidelines`.
3. Skill `graphify`: `graphify update .` when `graphify-out/graph.json` is missing or stale, then `graphify query` before reading many files.
4. Skill `project-omega` for the project map and rules. Rust work also uses `rust-skills`; Bevy ECS work uses `bevy-ecs` (read the vendored fork for exact 0.20 APIs).

## Optional token-saving helpers

The caveman plugin (JuliusBrussee/caveman, project-scoped copy in `.claude/skills/`, agents in `.claude/agents/`, default mode in `.caveman.json`):

- Locate code or map a directory: `cavecrew-investigator`; broad cold-start exploration: `caveman-explore`.
- Bounded 1-2 file edit inside an assigned task: `cavecrew-builder`; diff review: `cavecrew-reviewer` or `/caveman-review`.
- Commit messages: `/caveman-commit` (Conventional Commits, English).
- `/caveman-compress` may shrink this adapter only. Never run it on AGENTS.md, GDD, ADRs, specs, ROADMAP.md or PLAN.md.

ai-memory (akitaonrails/ai-memory, native Windows install) provides cross-session memory through Claude Code hooks and MCP; see [docs/ai-memory.md](docs/ai-memory.md).

## Commands

```powershell
powershell -NoProfile -File scripts/check.ps1        # all local gates (fmt, clippy, tests, client check, deny, markdown)
cargo test --workspace --no-default-features --locked
cargo run -p omega_game --locked --bin omega-game     # native client
bun run lint                                          # markdownlint
cargo run -p omega_game --locked --bin omega-game -- --play --fast-forward-ticks 3700 --skip-hints   # live sandbox (shop after wave 1)
cargo run -p omega_game --features client --locked --bin omega-game -- --headless --e2e-script tests/e2e/prototype-loop.json   # e2e, no GPU
cargo run -p omega_game --features client --locked --bin omega-game -- --headless --qa-bot cautious --seed <64 hex>          # simulated human QA
cargo run --release -p omega_game --no-default-features --bin omega-sim -- --scenario balance --policies all --seeds 200 --label <date>-<name>
cargo run --release -p omega_game --no-default-features --bin omega-sim -- --scenario perf02 --seed <64 hex> --output <json>
```

## Code map (game crate)

`crates/omega_game/src/`: `content` (strict JSON catalogs), `sim` (schedule, RNG, `run.rs` lifecycle), `combat` (stats, weapons, damage), `encounters` (waves, enemy AI, bosses), `shop`, `profile` and `persist` (grants, settings, two-slot saves), `input` (keyboard, mouse, controller, bindings), `ui` (view models, menus, widgets), `audio`, `balance` (simulator), `qa` (e2e runner and bot), `diagnostics` (PERF reports), `client` (native app). Rules stay headless; `client` and `*/client.rs` hold the Bevy glue.
