# AI Memory Integration Setup

Project has `ai-memory` configured for persistent context across Claude Code sessions.

## What is AI Memory?

AI Memory (aimemory.sh) enables Claude to maintain persistent memory across disconnections, context compressions, and sessions. This is critical for:

- **Long-running stages** (Research, Brainstorming, Architecture specs)
- **Session recovery** after context compression (`/compact`)
- **Checkpoint preservation** across overnight runs
- **Decision traceability** without re-reading all history

## Installation

ai-memory npm package is installed locally:

```bash
bun list | grep ai-memory
```

For the full CLI tool and managed instance, follow the guide at: **http://aimemory.sh/pt-br/install/**

## Configuration

Create `.env` from `.env.example`:

```bash
cp .env.example .env
```

### Self-Hosted (Local Database)

```env
AI_MEMORY_ENABLED=true
AI_MEMORY_DB=./ai-memory.db
```

Initialize database once:

```bash
ai-memory init --db ./ai-memory.db
```

### Managed Instance

Use the aimemory.sh hosted service:

```env
AI_MEMORY_ENABLED=true
AI_MEMORY_ENDPOINT=https://api.aimemory.sh
AI_MEMORY_TOKEN=<your-token>
```

## Usage During Overnight Execution

When running `run-overnight.ps1`:

1. **Memory scope:** Set to `project-omega` to keep decisions, specs, and checkpoints persistent.
2. **Session recovery:** After `/compact`, Claude retrieves prior decisions and stages automatically.
3. **Checkpoint marking:** Include `[memory: save]` in commit messages for critical checkpoints (stage gates, owner approvals).
4. **Query stored context:** Use natural language in prompts like "what did we decide about fork strategy?" — ai-memory provides prior context.

## Memory Categories

Store in memory:

- **Decisions:** Fork strategy, scope, architecture choices (→ `docs/decisions/`)
- **Stage gates:** Owner approvals and approval dates
- **Blockers:** When a task cannot proceed, why, and escalation status
- **Checkpoints:** End-of-stage summaries, verified completions

## Verification

To verify ai-memory is working:

1. Complete Stage 1 Research
2. Save memory: `ai-memory save --name "stage-1-complete" --tags "research,checkpoint"`
3. Trigger `/compact`
4. Resume with `/loop`
5. Ask Claude: "What were the key findings from Track C (Bevy fork strategy)?"
6. If ai-memory is connected, Claude references prior findings without re-reading all files.

## Troubleshooting

| Issue | Solution |
|-------|----------|
| `ai-memory not found` | Install from aimemory.sh or ensure `bun add ai-memory` succeeded |
| Memory not persisting | Check `.env` configuration and that `AI_MEMORY_ENABLED=true` |
| Session recovery slow | Ensure no network/API timeout; check `AI_MEMORY_ENDPOINT` is reachable |
| Token expired | Regenerate token from aimemory.sh dashboard |

## References

- **Official guide:** http://aimemory.sh/pt-br/install/
- **Project memory files:** (auto-managed in `.ai-memory/` or cloud)
- **Context compression:** Use `/compact` freely; memory survives compression

---

**Setup date:** 2026-10-23  
**Status:** Configured, awaiting ai-memory connection verification
