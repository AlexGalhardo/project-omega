# 01 - REA setup and inventory of the owner's Brotato copy

Task: RES-P1-001. Date: 2026-10-24. Scope: [DEC-019](../../decisions/decision-register.md#dec-019--scoped-rea-inspection-of-the-owners-brotato-copy).

## Method

- REA `rea-agents@6.7.0` with Ghidra 12.1.4 (Temurin JDK 21), through the Claude Code MCP server.
- Readiness check on a small existing Windows executable (a system utility, not a project artifact): open, binary overview and counts succeeded. Verified.
- Read-only open attempts on the owner's files; Steam default path found at the DEC-019 location. Steam build ID `23429717` (app 1942280). Verified from the local Steam manifest.
- Raw output and a short log live in `%LOCALAPPDATA%\ProjectOmega\rea\`. Nothing from the game is stored in this repository.

## Findings

| Question | Finding | Confidence |
| --- | --- | --- |
| Is the install present at the DEC-019 path? | Yes: engine executable, one data package, the Steam API library and a small workshop helper. | Verified |
| Can REA open the data package (`.pck`)? | No. REA rejects it as an unsupported format. | Verified |
| Can REA analyze the engine executable? | It opens as a 64-bit PE (about 39 MB). Import and auto-analysis exceeded the 330 s startup deadline twice (initial attempt plus one retry), so no overview was obtained. | Verified |
| What would the executable answer if analyzed? | Only engine internals (how the generic engine loads scenes and scripts). The game rules are data and scripts in the package, not native code. | Inferred |
| Can game rules be read through REA now? | No. Rules, formulas, tables and timings are unreachable with the supported tools. | Verified |

## Consequences for RES-P2-001 and RES-P2-002

- Those notes cannot cite static analysis. They are written from public knowledge of the game, general design reasoning and ordinary play by the owner, with explicit confidence labels.
- Anything numeric that is not Verified stays an Unknown or Inferred and is listed as an open question in `questions-to-dev.md`.
- Raising `REA_GHIDRA_STARTUP_TIMEOUT_MS` would likely let the executable import, but it still would not expose game rules. No further tool (for example a package extractor) is added without the owner's decision (rule 1.10).

## GDD sections informed

None directly. This note records tooling limits only.
