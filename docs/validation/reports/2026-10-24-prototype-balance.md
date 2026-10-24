# Prototype balance report (2026-10-24)

Task GAME-P2-013. Base commit `0fffd87` plus the uncommitted GAME-P2-013 working tree. Environment: Windows 11, Rust 1.99.1, Bevy 0.20.0 vendored fork. Method and bands: [balancing.md](../balancing.md). Change request: [CR-004](../../decisions/change-requests/cr-004-prototype-pressure-calibration.md).

Every band in [balancing.md](../balancing.md) is **P** (provisional). This report tests the prototype loop (Patch, three 60 s waves, two shops, Bolt Driver and Chain Saw, Scrapper and Wire Rat, eight items) and records numbers, not fun.

## Verdict

- Before tuning the prototype was trivial: all six policies won 100 % of 200 seeds, `idle` took 0 damage in all three waves.
- CR-004 changes only enemy stats and the three wave counts. After it, six of the eight measurable bands pass in the 6 policies x 200 seeds matrix; the other two are recorded misses with a proposal (survival curve of the one-weapon `kiter`, and Chain Saw ownership measured on a supplementary 2000-seed `random` run).
- Two bands cannot be measured with the prototype content (characters, full-run length).

## Method

- `omega-sim --scenario balance` runs complete runs through the real `GameTick` schedule: begin run, wave, shop, next wave, victory or defeat. Seeds are `SHA-256("omega-balance-seed" || index)`; every policy gets the same seed indices.
- Policies produce `TickInput` and shop commands from player-visible facts only: own position and HP, enemy and scrap positions, scrap held, shop offers and prices, loadout and reactor points, public content data. They never read RNG streams, unspawned batches or enemy HP.
- `idle` never moves and never buys. `kiter` flees enemies within 300 LU, slides along walls, collects scrap when safe and buys only weapons with range of at least 200 LU (so it stays on the Bolt Driver). `greedy` chases scrap then walks into weapon range of the nearest enemy and stands there; it prefers damage and attack speed. `defensive` flees within 380 LU, prefers HP, armor and regeneration, and powers two points per weapon. `random` walks in random directions and takes uniformly random legal shop actions. `human_like` drives the QA bot input model (`qa::bot::Bot`) headlessly through the same `InputBuffer` as the client; it alternates the `cautious` and `aggressive` profiles by seed parity and buys with balanced weights.
- Metrics read the world and the presentation stream after each tick. A contract test steps 900 ticks with and without the observer and compares the trace (tick, ID counters, RNG stream positions) for equality.
- Percentiles use the nearest-rank method of the performance plan. Matrices: 6 policies x 200 seeds (seeds 0 to 199). Win means clearing wave 3.

## Bands before and after

| Check | Target | Before (content revision `d5f2248c`) | After (content revision `e31c1d06`) | Status after |
| --- | --- | --- | --- | --- |
| `idle` dies in waves 1-2 | all idle runs | 0.0 % (idle won 100 %) | 100.0 % (all 200 die in wave 2) | pass |
| `human_like` clears wave 1 | >= 90 % | 100.0 % | 93.5 % | pass |
| `human_like` clears wave 3 | 50-80 % | 100.0 % | 59.5 % | pass |
| `kiter` win rate | <= 75 % | 100.0 % | 37.5 % | pass |
| Survival curve drop | <= 25 points per wave | 0.0 (nothing ever died) | `human_like` 93.5, 81.5, 59.5 (worst drop 22 points); `kiter` 100, 100, 37.5 (62.5 points) | MISS (kiter) |
| No item or weapon above +15 points when owned | <= 15 points, cells with >= 50 runs per side | +0.0 (no losses, uninformative) | +9.9 (`item_plating`, `human_like`); supplementary 2000-seed `random`: `wpn_saw` +20.7 | MISS (saw, supplementary) |
| Every weapon is picked | >= 3 % of offers | lowest `wpn_saw` 48.2 % | lowest `wpn_saw` 50.0 % | pass |
| Energy: spreading and concentrating both win | both present among wins | single 730 wins, spread 470 wins | single 75 wins, spread 527 wins | pass |
| Characters within 15 points | `human_like` | not applicable | not applicable: one character in the prototype | n/a |
| Full-run length | 12-20 min | not applicable | not applicable: three 60 s waves | n/a |

Energy classes: single is one powered weapon (`kiter`, 3 points on the Bolt Driver), spread is two powered weapons (3-3, 2-2, 3-2 and similar).

## Quick matrix (2 policies x 200 seeds, original content)

`idle` and `human_like`, seeds 0 to 199, content revision `d5f2248c`: both won 200 of 200 runs and `idle` took 0 damage in every wave. This was the trigger for CR-004; the full matrix below confirmed it for all six policies.

## Content changes (CR-004)

| Record | Field | Before | After |
| --- | --- | ---: | ---: |
| `enemy_chaser` (Scrapper) | `base_hp` | 24 | 36 |
| `enemy_chaser` (Scrapper) | `speed_lu_s` | 70 | 78 |
| `enemy_chaser` (Scrapper) | `contact_damage` | 8 | 10 |
| `enemy_runner` (Wire Rat) | `base_hp` | 14 | 18 |
| `enemy_runner` (Wire Rat) | `speed_lu_s` | 125 | 170 |
| `enemy_runner` (Wire Rat) | `contact_damage` | 6 | 8 |
| `wave_01` | batch `count` (6 batches) | 4 | 6 |
| `wave_02` | batch `count` (6 batches) | 5 | 12 |
| `wave_03` | batch `count` (6 batches) | 6 | 20 |

Both `assets/content` and `crates/omega_game/tests/fixtures/content/prototype` carry these values (the existing revision-equality test passes). Wire Rat stays below the player's base speed of 180 LU/s, so the player can still outrun every enemy at base speed.

Exploration that led here (40 to 200 seeds per configuration on scratch copies of the content, not committed): raising enemy HP alone left `idle` winning; doubling HP, speed and damage with counts 8, 10, 12 killed `human_like` in wave 1 (33 % wins) while `kiter` still took 0 damage; a Wire Rat at 200 LU/s (faster than the player) was not needed once counts reached 12 and 20; `kiter` win rate moves from about 100 % to about 30 % across a single enemy of wave-3 batch size, so the cliff cannot be placed gently by counts.

### Win rate and damage

| Policy | Win rate before | Win rate after | Deaths by wave before | Deaths by wave after | Mean damage taken per wave before | Mean damage taken per wave after |
| --- | ---: | ---: | --- | --- | --- | --- |
| idle | 100.0 % | 0.0 % | - | w2: 200 | 0, 0, 0 | 0, 124 |
| kiter | 100.0 % | 37.5 % | - | w3: 125 | 0, 0, 0 | 0, 2, 135 |
| greedy | 100.0 % | 100.0 % | - | - | 0, 0, 0 | 0, 22, 52 |
| defensive | 100.0 % | 98.5 % | - | w2: 3 | 0, 0, 0 | 0, 24, 39 |
| random | 100.0 % | 5.5 % | - | w2: 137, w3: 52 | 0, 3, 7 | 3, 108, 122 |
| human_like | 100.0 % | 59.5 % | - | w1: 13, w2: 24, w3: 44 | 3, 2, 3 | 45, 56, 73 |

### Survival (share of runs that cleared the wave)

| Policy | Wave 1 before | Wave 2 before | Wave 3 before | Wave 1 after | Wave 2 after | Wave 3 after |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| idle | 100.0 % | 100.0 % | 100.0 % | 100.0 % | 0.0 % | 0.0 % |
| kiter | 100.0 % | 100.0 % | 100.0 % | 100.0 % | 100.0 % | 37.5 % |
| greedy | 100.0 % | 100.0 % | 100.0 % | 100.0 % | 100.0 % | 100.0 % |
| defensive | 100.0 % | 100.0 % | 100.0 % | 100.0 % | 98.5 % | 98.5 % |
| random | 100.0 % | 100.0 % | 100.0 % | 100.0 % | 31.5 % | 5.5 % |
| human_like | 100.0 % | 100.0 % | 100.0 % | 93.5 % | 81.5 % | 59.5 % |

### Shop economy per visit (mean scrap)

| Policy | Visit | Held on entry before | Held on entry after | Spent before | Spent after | Unspent before | Unspent after |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| idle | 1 | 48.0 | 72.0 | 0.0 | 0.0 | 48.0 | 72.0 |
| kiter | 1 | 47.1 | 70.4 | 44.5 | 60.3 | 2.6 | 10.1 |
| kiter | 2 | 62.0 | 149.9 | 58.1 | 139.7 | 3.9 | 10.3 |
| greedy | 1 | 48.0 | 72.0 | 38.1 | 67.1 | 9.9 | 4.9 |
| greedy | 2 | 69.9 | 148.9 | 67.7 | 141.1 | 2.1 | 7.8 |
| defensive | 1 | 45.0 | 67.3 | 41.5 | 64.5 | 3.5 | 2.8 |
| defensive | 2 | 60.9 | 138.0 | 53.5 | 125.6 | 7.4 | 12.4 |
| random | 1 | 45.5 | 68.6 | 28.7 | 38.6 | 16.8 | 30.0 |
| random | 2 | 74.9 | 157.6 | 42.1 | 63.9 | 32.8 | 93.6 |
| human_like | 1 | 46.6 | 69.1 | 37.4 | 65.1 | 9.2 | 4.0 |
| human_like | 2 | 68.2 | 143.5 | 62.8 | 133.8 | 5.4 | 9.7 |

Enemy counts raised scrap income (greedy earns about 72, 144 and 240 per wave instead of 48, 60 and 72), and bots now spend about 90 % or more of what they hold (`idle` and `random` aside); shop prices were not changed.

## Findings and proposals for the missed bands

### Survival curve of `kiter` (MISS)

Observed: `kiter` clears waves 1 and 2 in every run and wave 3 in 37.5 %, a 62.5-point drop. `human_like` (22 points at worst), `greedy` and `defensive` stay inside the band. The cliff is consistent with a one-weapon build: with the Bolt Driver alone (48 damage per second at three points) wave 3 is the first wave whose 20 enemies per batch exceed its kill rate, and once enemies accumulate the run is lost. Counts of 18 and 19 per batch in the exploratory runs gave 82 % and 31 % `kiter` win rates with the earlier enemy set (Wire Rat at 200 LU/s), so no integer count satisfies both "`kiter` <= 75 %" and "drop <= 25 points".

Proposal (not applied): no further prototype change. The cliff measures the absence of a second ranged weapon in a three-wave slice; re-evaluate in QA-P3-003 with ten waves and six weapons, where progressive pressure can spread the loss over several waves. If the owner wants it now, the alternative is a third prototype weapon.

### Chain Saw ownership (MISS, supplementary)

Observed: in the 6 x 200 matrix no weapon cell has 50 runs per side (survivors nearly all own the Chain Saw), so the band is not measurable there. The `random` policy buys independently of skill; 2000 seeds (1000 to 2999) of it give `wpn_saw` owners +20.7 points win rate over non-owners at the start of wave 3 (508 against 158 runs). Item cells stay below +15 in every matrix, including 800 `human_like` seeds (worst +13.4).

#### Weapon pick rate and win delta when owned

| ID | random pick | random win delta |
| --- | ---: | ---: |
| `wpn_bolt` | 29.3 % | - |
| `wpn_saw` | 31.0 % | +21 (n=508/158) |

#### Item pick rate and win delta when owned

| ID | random pick | random win delta |
| --- | ---: | ---: |
| `item_bearings` | 33.6 % | +6 (n=174/492) |
| `item_brace` | 35.4 % | +10 (n=200/466) |
| `item_coupler` | 33.0 % | +4 (n=175/491) |
| `item_magnet` | 33.1 % | +8 (n=183/483) |
| `item_plating` | 30.7 % | +7 (n=170/496) |
| `item_ratchet` | 35.1 % | +5 (n=216/450) |
| `item_sealant` | 32.7 % | +6 (n=177/489) |
| `item_sight` | 33.3 % | +1 (n=182/484) |

Pick rate is purchases over offers shown. Win delta is the win rate of runs that owned the entry at the start of the last wave minus the rest, in points (n owned / n other).

Proposal (not applied): lower Chain Saw `base_damage` from 20 to 16 together with a wave recalibration. Measured on scratch content: saw 17 gives +17.1 (with `wave_03` count 19 to keep `kiter` below 75 %), saw 16 gives +11.1 (1000 seeds) but pushes `human_like` to 48 % wins, below the 50 % band, so it needs a new `wave_03` count and a re-run of the whole matrix. A second CR for one weapon of a two-weapon slice is not justified before the other weapons exist; re-measure in QA-P3-003. Until then the report states the effect: the second weapon is worth roughly 20 points of win rate for an otherwise unskilled player.

### Other observations

- `greedy` wins 100 % and `defensive` 98.5 %: standing in Chain Saw range with two powered weapons beats the prototype waves. The `kiter` band is met only because `kiter` never buys the saw. Watch this in QA-P3-003; a melee-first build should not be strictly safest.
- `human_like` is bimodal: `cautious` wins 100 % and `aggressive` 19 % (87 %, 63 %, 19 % cleared per wave). The overall 59.5 % is the average of two player styles, so the owner's playtest should say which one feels right.
- `idle` dies in wave 2 in every run (median combat time 90.6 s), never in wave 1: wave 1 has 6 Scrappers per batch and is survivable standing still.
- Scrapper time to kill (spawn to death) for `human_like`: median 6.5 s, p95 14.5 s; Wire Rat median 3.5 s, p95 6.9 s.
- Prototype damage sources are contact only (no ranged or area enemy exists yet), so the per-source damage breakdown of the method is a single bucket.

## Detail of the final matrix

The tables below are the generated output of `omega-sim --scenario balance --policies all --seeds 200 --label 2026-10-24-after --md ...` and are reproduced unedited.

### Run summary

Label `2026-10-24-after`, character `char_patch`, content profile Prototype, content revision `e31c1d064b80cb4ea008063ae9223c4c1b382c672083465548b31b6a7f1ee7ad`, seeds 0..200 (200 per policy), 3 waves. Percentiles: nearest-rank, index ceil(p*N)-1.

| Policy | Runs | Win rate | Combat s p50 | Combat s p95 | Deaths by wave | Distinct winning builds |
| --- | ---: | ---: | ---: | ---: | --- | ---: |
| idle | 200 | 0.0 % | 90.6 | 100.7 | w2: 200 | 0/0 |
| kiter | 200 | 37.5 % | 178.1 | 180.0 | w3: 125 | 8/75 |
| greedy | 200 | 100.0 % | 180.0 | 180.0 | - | 50/200 |
| defensive | 200 | 98.5 % | 180.0 | 180.0 | w2: 3 | 93/197 |
| random | 200 | 5.5 % | 105.7 | 180.0 | w2: 137, w3: 52 | 11/11 |
| human_like | 200 | 59.5 % | 180.0 | 180.0 | w1: 13, w2: 24, w3: 44 | 67/119 |

#### Input styles inside a policy

| Policy | Style | Runs | Win rate | Cleared wave 1 | Cleared wave 2 |
| --- | --- | ---: | ---: | ---: | ---: |
| human_like | aggressive | 100 | 19.0 % | 87.0 % | 63.0 % |
| human_like | cautious | 100 | 100.0 % | 100.0 % | 100.0 % |

### Prototype bands

| Check | Target | Observed | Status |
| --- | --- | --- | --- |
| idle dies in waves 1-2 | all idle runs end in wave 1 or 2 | 100.0 % | pass |
| human_like clears wave 1 | >= 90 % | 93.5 % | pass |
| human_like clears the last wave | 50-80 % | 59.5 % | pass |
| kiter win rate | <= 75 % | 37.5 % | pass |
| survival curve drop | <= 25 points per wave (non-boss) | worst 62.5 points (kiter) | MISS |
| no item or weapon above +15 points when owned | <= 15 points (cells with >= 50 runs on both sides) | worst +9.9 points (item_plating under human_like) | pass |
| every weapon is picked | >= 3 % of offers, policies other than random and idle | lowest wpn_saw 50.0 % | pass |
| energy: spread and single both win | >= 5 winning runs of each pattern class | single 75 wins, spread 527 wins | pass |
| characters within 15 points | human_like win rates within 15 points | one character, three 60 s waves | n/a (prototype) |
| full-run length | 12-20 minutes of combat (full game) | one character, three 60 s waves | n/a (prototype) |

### Survival (share of runs that cleared the wave)

| Policy | Wave 1 | Wave 2 | Wave 3 |
| --- | ---: | ---: | ---: |
| idle | 100.0 % | 0.0 % | 0.0 % |
| kiter | 100.0 % | 100.0 % | 37.5 % |
| greedy | 100.0 % | 100.0 % | 100.0 % |
| defensive | 100.0 % | 98.5 % | 98.5 % |
| random | 100.0 % | 31.5 % | 5.5 % |
| human_like | 93.5 % | 81.5 % | 59.5 % |

### Damage taken and scrap earned per wave (mean)

| Policy | Wave | Damage taken | Scrap earned |
| --- | ---: | ---: | ---: |
| idle | 1 | 0.0 | 72.0 |
| idle | 2 | 123.8 | 38.8 |
| kiter | 1 | 0.0 | 70.4 |
| kiter | 2 | 1.8 | 139.8 |
| kiter | 3 | 134.5 | 151.6 |
| greedy | 1 | 0.0 | 72.0 |
| greedy | 2 | 21.5 | 144.0 |
| greedy | 3 | 51.6 | 240.0 |
| defensive | 1 | 0.0 | 67.3 |
| defensive | 2 | 24.5 | 134.5 |
| defensive | 3 | 38.9 | 226.0 |
| random | 1 | 3.4 | 68.6 |
| random | 2 | 107.9 | 83.8 |
| random | 3 | 121.7 | 88.1 |
| human_like | 1 | 44.9 | 68.6 |
| human_like | 2 | 55.6 | 137.5 |
| human_like | 3 | 73.3 | 218.7 |

### Shop economy per visit (mean scrap)

| Policy | Visit | Held on entry | Spent | Unspent |
| --- | ---: | ---: | ---: | ---: |
| idle | 1 | 72.0 | 0.0 | 72.0 |
| kiter | 1 | 70.4 | 60.3 | 10.1 |
| kiter | 2 | 149.9 | 139.7 | 10.3 |
| greedy | 1 | 72.0 | 67.1 | 4.9 |
| greedy | 2 | 148.9 | 141.1 | 7.8 |
| defensive | 1 | 67.3 | 64.5 | 2.8 |
| defensive | 2 | 138.0 | 125.6 | 12.4 |
| random | 1 | 68.6 | 38.6 | 30.0 |
| random | 2 | 157.6 | 63.9 | 93.6 |
| human_like | 1 | 69.1 | 65.1 | 4.0 |
| human_like | 2 | 143.5 | 133.8 | 9.7 |

### Time to kill (seconds from spawn to death)

| Policy | Enemy | Kills | p50 | p95 | max |
| --- | --- | ---: | ---: | ---: | ---: |
| idle | `enemy_chaser` | 10446 | 6.47 | 9.13 | 12.37 |
| idle | `enemy_runner` | 3596 | 3.50 | 5.13 | 7.13 |
| kiter | `enemy_chaser` | 21453 | 6.85 | 20.17 | 57.55 |
| kiter | `enemy_runner` | 17757 | 3.88 | 8.17 | 15.85 |
| greedy | `enemy_chaser` | 26417 | 4.93 | 7.53 | 8.20 |
| greedy | `enemy_runner` | 19183 | 3.03 | 5.18 | 5.93 |
| defensive | `enemy_chaser` | 23538 | 8.40 | 17.58 | 54.27 |
| defensive | `enemy_runner` | 18947 | 4.42 | 7.77 | 9.52 |
| random | `enemy_chaser` | 12730 | 6.38 | 13.68 | 21.55 |
| random | `enemy_runner` | 6692 | 3.57 | 7.00 | 10.50 |
| human_like | `enemy_chaser` | 21721 | 6.50 | 14.52 | 17.35 |
| human_like | `enemy_runner` | 15943 | 3.47 | 6.90 | 8.07 |

#### Weapon pick rate and win delta when owned

| ID | kiter pick | kiter win delta | greedy pick | greedy win delta | defensive pick | defensive win delta | random pick | random win delta | human_like pick | human_like win delta |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `wpn_bolt` | 100.0 % | - | 100.0 % | - | 100.0 % | - | 30.3 % | - | 100.0 % | - |
| `wpn_saw` | 0.0 % | - | 66.7 % | - | 67.1 % | - | 30.6 % | +24 (n=46/17) | 74.2 % | - |

#### Item pick rate and win delta when owned

| ID | kiter pick | kiter win delta | greedy pick | greedy win delta | defensive pick | defensive win delta | random pick | random win delta | human_like pick | human_like win delta |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `item_bearings` | 100.0 % | +31 (n=188/12) | 52.2 % | +0 (n=94/106) | 83.6 % | +0 (n=117/80) | 44.3 % | +7 (n=23/40) | 65.9 % | -1 (n=83/80) |
| `item_brace` | 100.0 % | -1 (n=179/21) | 34.4 % | +0 (n=67/133) | 92.8 % | +0 (n=126/71) | 27.8 % | -11 (n=20/43) | 45.8 % | -8 (n=69/94) |
| `item_coupler` | 100.0 % | -24 (n=167/33) | 92.2 % | +0 (n=153/47) | 48.6 % | +0 (n=70/127) | 34.0 % | +8 (n=17/46) | 79.3 % | -20 (n=101/62) |
| `item_magnet` | 100.0 % | -4 (n=178/22) | 61.7 % | +0 (n=111/89) | 64.1 % | +0 (n=96/101) | 34.0 % | +13 (n=19/44) | 80.5 % | -1 (n=98/65) |
| `item_plating` | 100.0 % | +1 (n=173/27) | 36.6 % | +0 (n=71/129) | 78.7 % | +0 (n=107/90) | 31.9 % | -11 (n=12/51) | 43.1 % | +10 (n=54/109) |
| `item_ratchet` | 100.0 % | +9 (n=176/24) | 90.3 % | +0 (n=139/61) | 50.3 % | +0 (n=75/122) | 28.0 % | -18 (n=20/43) | 76.2 % | -9 (n=94/69) |
| `item_sealant` | 100.0 % | +14 (n=169/31) | 26.0 % | +0 (n=45/155) | 78.0 % | +0 (n=91/106) | 36.1 % | +13 (n=19/44) | 34.1 % | -8 (n=43/120) |
| `item_sight` | 100.0 % | -7 (n=170/30) | 75.9 % | +0 (n=120/80) | 43.1 % | +0 (n=62/135) | 24.2 % | -7 (n=9/54) | 36.8 % | -7 (n=47/116) |

Pick rate is purchases over offers shown. Win delta is the win rate of runs that owned the entry at the start of the last wave minus the rest, in points (n owned / n other).

### Energy allocation at the start of the last wave

| Policy | Pattern | Runs | Wins | Win rate |
| --- | --- | ---: | ---: | ---: |
| kiter | 3 | 200 | 75 | 37.5 % |
| greedy | 3-3 | 200 | 200 | 100.0 % |
| defensive | 2-2 | 197 | 197 | 100.0 % |
| random | 1 | 8 | 0 | 0.0 % |
| random | 1-1 | 3 | 0 | 0.0 % |
| random | 2 | 9 | 0 | 0.0 % |
| random | 2-1 | 6 | 2 | 33.3 % |
| random | 2-2 | 3 | 3 | 100.0 % |
| random | 3 | 19 | 0 | 0.0 % |
| random | 3-1 | 9 | 1 | 11.1 % |
| random | 3-2 | 5 | 4 | 80.0 % |
| random | 3-3 | 1 | 1 | 100.0 % |
| human_like | 3-3 | 163 | 119 | 73.0 % |

## Limitations

- Bots are heuristics, not players. They show clearly wrong numbers, not fun. The `human_like` reaction model is the QA bot with its abuse paths (pause, focus loss, resize) ignored; its idle frames after such events are rare.
- One character and two weapons: character parity, weapon variety and energy patterns beyond single and double are untested.
- No boss and no ranged enemy yet; the survival bands for boss waves and the full-game bands (win rate 15-40 %, median death wave 6-9, 12-20 minute runs) belong to QA-P3-003.
- Ownership deltas are observational and carry sampling noise of about 5 points at 150 runs per side.

## Reproduce

```powershell
cargo build --release -p omega_game --no-default-features --locked --bin omega-sim
./target/release/omega-sim.exe --scenario balance --policies all --seeds 200 --label 2026-10-24-after --md artifacts/balance/2026-10-24-after.md
./target/release/omega-sim.exe --scenario balance --policies random --seeds 2000 --seed-start 1000 --label 2026-10-24-random-2000-after --md artifacts/balance/2026-10-24-random-2000-after.md
```

The baseline needs the original content: `git stash` the content change or pass `--content` with a copy of the files from base commit `0fffd87`. JSON artifacts (`artifacts/balance/<label>.json`, all run records included) are git-ignored; the Markdown outputs of the runs cited here are kept in `docs/validation/evidence/GAME-P2-013/`.
