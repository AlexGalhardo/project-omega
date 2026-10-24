# CR-004 — Prototype pressure calibration (enemy stats and three-wave counts)

Date: 2026-10-24. Author: GAME-P2-013 (balance simulator). Status: **Proposed**; applied to the prototype content so the simulator measures it, awaiting integrator and owner review for the GDD edit. Blocking scope: none; GAME-P2-013 closes with the numbers applied. If the owner rejects it, revert the nine content values below in both content directories.

## Evidence

Base commit `0fffd87`, content revision `d5f2248cfb2c807bd49cb112836f4e6dff1f58daa637a343f7cdb9373c335f75`. Command (release build): `omega-sim --scenario balance --policies all --seeds 200 --label 2026-10-24-before`. Result: all six policies (`idle`, `kiter`, `greedy`, `defensive`, `random`, `human_like`) won 200 of 200 runs, and `idle` took 0 damage in all three waves. The auto-aimed Bolt Driver at three reactor points (48 damage per second) kills every Scrapper batch of four before it arrives.

Bands of [balancing.md](../../validation/balancing.md) that this misses: `idle` dies in waves 1-2 (observed: never dies); `human_like` clears wave 3 in 50-80 % (observed 100 %); `kiter` win rate <= 75 % (observed 100 %). The full report is [2026-10-24-prototype-balance.md](../../validation/reports/2026-10-24-prototype-balance.md).

Affected GDD numbers: section 10.1 (common enemy table, all marked P) and section 11 (batch counts, P). Affected task: GAME-P2-013.

## Alternatives

1. Raise enemy pressure (this CR): enemy stats plus prototype wave counts. Touches only P numbers of two enemies and three waves.
2. Counts only: with the original enemies and counts 6, 12 and 20 (scratch run, 100 seeds) `idle` dies in wave 3 in every run instead of wave 2, and `kiter` and `human_like` still win 100 % with 0 and 12 mean damage in wave 2.
3. Enemy HP only: `idle` kept winning (scratch run, Scrapper 40 HP, Wire Rat 22 HP).
4. Weaken the player (Bolt Driver damage or rate, Patch HP): changes the starting fantasy of GDD-009 and every shop price relation, and still leaves the swarm unpressured.
5. Keep the numbers and lower the bands: the bands are the owner-facing contract; a trivial 3-wave loop is not a band problem.

## Recommendation and numbers

Alternative 1. Exact before and after values:

| Record | Field | Before | After |
| --- | --- | ---: | ---: |
| `enemy_chaser` (Scrapper) | `base_hp` | 24 | 36 |
| `enemy_chaser` (Scrapper) | `speed_lu_s` | 70 | 78 |
| `enemy_chaser` (Scrapper) | `contact_damage` | 8 | 10 |
| `enemy_runner` (Wire Rat) | `base_hp` | 14 | 18 |
| `enemy_runner` (Wire Rat) | `speed_lu_s` | 125 | 170 |
| `enemy_runner` (Wire Rat) | `contact_damage` | 6 | 8 |
| `wave_01` | `count` of each of its 6 batches | 4 | 6 |
| `wave_02` | `count` of each of its 6 batches | 5 | 12 |
| `wave_03` | `count` of each of its 6 batches | 6 | 20 |

Constraints kept: Wire Rat (170) stays below the player's base speed (180 LU/s); wave scaling formulas, radii, armor, scrap values, shop rules and prices are unchanged.

Result with the same 6 x 200 matrix (content revision `e31c1d064b80cb4ea008063ae9223c4c1b382c672083465548b31b6a7f1ee7ad`): `idle` 0.0 % wins and all 200 runs die in wave 2; `human_like` clears wave 1 in 93.5 % and wave 3 in 59.5 %; `kiter` wins 37.5 %; `greedy` 100 %, `defensive` 98.5 %, `random` 5.5 %. Two bands stay missed and are recorded in the report with proposals (survival-curve cliff of the one-weapon `kiter`; Chain Saw ownership delta on a supplementary 2000-seed `random` run). Neither justifies another prototype change now.

## GDD edit for the integrator

- Section 10.1 table: Scrapper HP 24 -> 36, speed 70 -> 78, contact damage 8 -> 10; Wire Rat HP 14 -> 18, speed 125 -> 170, contact damage 6 -> 8. The worked example (wave-4 Chaser `ceil(24*1.54)=37` HP, `ceil(8*1.24)=10` damage) becomes `ceil(36*1.54)=56` HP and `ceil(10*1.24)=13` damage.
- Section 11: the counts 6, 12 and 20 were measured with only Scrapper and Wire Rat (no Rivet Guard in wave 3) against a two-weapon shop. Record them as the **prototype profile** counts (section 17 Prototype row) and leave the full-game wave 1-3 counts (4, 5, 6) for QA-P3-003, which re-tunes them with all enemy types and weapons. The full-profile fixtures (`crates/omega_game/tests/fixtures/content/full`) were not touched.

## Validation

`cargo test -p omega_game --no-default-features --locked` (all suites, including the equal-revision test of the two prototype content directories) and `balance_contract` pass with the new content. The matrices are reproducible with the commands in the report.
