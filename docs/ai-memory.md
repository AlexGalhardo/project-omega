# ai-memory setup

Cross-session memory for Claude Code comes from [akitaonrails/ai-memory](https://github.com/akitaonrails/ai-memory), a native Windows binary installed per user. It is developer tooling only: not a game dependency, not a CI prerequisite, and nothing in this repository configures it. The earlier npm package `ai-memory@1.0.0` listed in `package.json` was an empty placeholder with no code and was removed on 2026-10-24.

## State verified on 2026-10-24

| Item | Value |
| --- | --- |
| Version | ai-memory 3.2.2, release zip `ai-memory-windows-x86_64.zip` (SHA-256 `84814a55…169b`, matched the published `.sha256`) |
| Binary | `%LOCALAPPDATA%\Programs\ai-memory\ai-memory.exe` |
| Data dir | `%LOCALAPPDATA%\ai-memory` (config, owner-only bearer token in `auth-tokens\default`) |
| Server | `http://127.0.0.1:49374`, started at logon by `HKCU\Software\Microsoft\Windows\CurrentVersion\Run\ai-memory`; `/healthz` returned 200 |
| Claude Code wiring | `ai-memory setup --yes --agent claude-code`: lifecycle hooks in `~/.claude/settings.json`, MCP registration in `~/.claude.json`, six `ai-memory-*` skills in `~/.claude/skills/` |
| Project scope | `default/alexgalhardo-project-omega` (derived from the git remote) |

Kaspersky quarantined the first install's `ai-memory.exe`. Add exclusions for `%LOCALAPPDATA%\Programs\ai-memory` and `%LOCALAPPDATA%\ai-memory` in Kaspersky, or the hooks stop capturing.

## Check that it works

```powershell
ai-memory status          # server reachable, counts
ai-memory doctor          # capture coverage for this project; warns if hooks are missing
```

After a session, `ai-memory doctor` should show captured Claude Code sessions for `alexgalhardo-project-omega`. Use the `ai-memory-retrieval` skill (or the MCP tools) to ask about earlier sessions.

## Rules

- Memory is a convenience, not a source of truth: decisions live in `docs/decisions/`, status in `PLAN.md`, evidence in `docs/validation/evidence/`.
- Never store secrets in memory pages. The bearer token stays in the data dir, never in this repository.
