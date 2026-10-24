# Balancing method and simulation

Status: method approved with DEC-020 (2026-10-24); every target band below is **P** (provisional) until the owner plays and confirms it. The GDD owns the numbers; this document owns how they are tested. A balance change is a GDD/content change with its simulation evidence, never an untracked tweak.

## Goal

The game should feel fair, tense and varied: early waves teach, later waves pressure, most builds are viable, no single choice dominates, and losing feels like the player's mistake rather than an invisible spike. Simulation cannot prove fun; it finds numbers that are clearly wrong before a human plays.

## Simulator

`omega-sim --scenario balance` runs complete runs headlessly through the real `GameTick` schedule (same rules as the client, no renderer):

- **Bot policies:** `idle` (never moves; lower bound), `kiter` (keeps distance, collects scrap), `greedy` (maximizes scrap and damage purchases), `defensive` (prefers HP/armor/regen), `random` (uniform legal choices), `human_like` (the QA bot of ROADMAP rule 1.11 driven headlessly: reaction delay, aim error, imperfect dodging). Shop choices and energy allocation are part of each policy.
- **Matrix:** every character × every policy × N seeds (at least 200 per cell for decisions; 50 for quick checks). Seeds and content revision are recorded.
- **Output:** `artifacts/balance/<date>-<label>.json` plus a Markdown summary in `docs/validation/reports/`. Reports follow the performance plan's nearest-rank percentiles.

## Metrics

| Metric | Why it matters |
| --- | --- |
| Survival rate per wave (by character and policy) | Difficulty curve and spikes |
| Win rate (wave 10 cleared) | Overall difficulty |
| Run length (active combat time) | Pacing |
| Time-to-kill per enemy type and wave | Weapon/enemy tuning |
| Damage taken per wave, source breakdown | Unfair or invisible threats |
| Scrap earned and spent per shop; unspent scrap | Economy pacing |
| Purchase and pick rates; win rate when owned | Dominant or useless items/weapons |
| Energy allocation patterns and their win rates | Reactor design (GDD-007 hypothesis) |
| Build diversity (distinct final loadouts among wins) | Variety |

## Provisional target bands (P)

| Check | Target |
| --- | --- |
| `idle` policy | Dies in waves 1–2 (the game demands movement) |
| `human_like` policy, Prototype (three waves) | Clears wave 1 in ≥ 90 % of runs; clears wave 3 in 50–80 % |
| `human_like` policy, full game | Win rate 15–40 %; median death wave 6–9 among losses |
| Strong policy (`kiter`) | Win rate ≤ 75 %; the game stays beatable but not trivial |
| Survival curve | No single wave drops survival by more than 25 percentage points versus the previous wave, except boss waves (≤ 35) |
| Weapons and items | No item or weapon raises win rate by more than 15 points when owned (controlling for character); no weapon is picked in less than 3 % of `random`-free policies |
| Characters | Win rates within 15 points of each other for `human_like` |
| Energy | Spreading and concentrating power both appear among winning builds |
| Run length | Full winning run 12–20 minutes of combat including bosses |

A target missed is a finding: propose a GDD/content change, re-run the same matrix, and keep before/after reports. When the owner disagrees with a band after playing, the band changes here with a note.

## Where it happens in the roadmap

- Prototype: the simulator and bots are built and tuned on the prototype content (ROADMAP rule 1.12, task GAME-P2-013).
- Alpha: full-content simulation and first tuning (QA-P3-003).
- Beta: balance pass with human playtest notes plus simulation (GAME-P4-006).
- RC: balance regression guard against the frozen content (QA-P5-001 and QA-P5-003).
