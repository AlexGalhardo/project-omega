## Run summary

Label `2026-10-24-before`, character `char_patch`, content profile Prototype, content revision `d5f2248cfb2c807bd49cb112836f4e6dff1f58daa637a343f7cdb9373c335f75`, seeds 0..200 (200 per policy), 3 waves. Percentiles: nearest-rank, index ceil(p*N)-1.

| Policy | Runs | Win rate | Combat s p50 | Combat s p95 | Deaths by wave | Distinct winning builds |
| --- | ---: | ---: | ---: | ---: | --- | ---: |
| idle | 200 | 100.0 % | 180.0 | 180.0 | - | 1/200 |
| kiter | 200 | 100.0 % | 180.0 | 180.0 | - | 28/200 |
| greedy | 200 | 100.0 % | 180.0 | 180.0 | - | 16/200 |
| defensive | 200 | 100.0 % | 180.0 | 180.0 | - | 24/200 |
| random | 200 | 100.0 % | 180.0 | 180.0 | - | 169/200 |
| human_like | 200 | 100.0 % | 180.0 | 180.0 | - | 53/200 |

### Input styles inside a policy

| Policy | Style | Runs | Win rate | Cleared wave 1 | Cleared wave 2 |
| --- | --- | ---: | ---: | ---: | ---: |
| human_like | aggressive | 100 | 100.0 % | 100.0 % | 100.0 % |
| human_like | cautious | 100 | 100.0 % | 100.0 % | 100.0 % |

## Prototype bands

| Check | Target | Observed | Status |
| --- | --- | --- | --- |
| idle dies in waves 1-2 | all idle runs end in wave 1 or 2 | 0.0 % | MISS |
| human_like clears wave 1 | >= 90 % | 100.0 % | pass |
| human_like clears the last wave | 50-80 % | 100.0 % | MISS |
| kiter win rate | <= 75 % | 100.0 % | MISS |
| survival curve drop | <= 25 points per wave (non-boss) | worst 0.0 points (human_like) | pass |
| no item or weapon above +15 points when owned | <= 15 points (cells with >= 30 runs on both sides) | worst +0.0 points (item_bearings under kiter) | pass |
| every weapon is picked | >= 3 % of offers, policies other than random and idle | lowest wpn_saw 48.2 % | pass |
| energy: spread and single both win | >= 5 winning runs of each pattern class | single 730 wins, spread 470 wins | pass |
| characters within 15 points | human_like win rates within 15 points | one character, three 60 s waves | n/a (prototype) |
| full-run length | 12-20 minutes of combat (full game) | one character, three 60 s waves | n/a (prototype) |

## Survival (share of runs that cleared the wave)

| Policy | Wave 1 | Wave 2 | Wave 3 |
| --- | ---: | ---: | ---: |
| idle | 100.0 % | 100.0 % | 100.0 % |
| kiter | 100.0 % | 100.0 % | 100.0 % |
| greedy | 100.0 % | 100.0 % | 100.0 % |
| defensive | 100.0 % | 100.0 % | 100.0 % |
| random | 100.0 % | 100.0 % | 100.0 % |
| human_like | 100.0 % | 100.0 % | 100.0 % |

## Damage taken and scrap earned per wave (mean)

| Policy | Wave | Damage taken | Scrap earned |
| --- | ---: | ---: | ---: |
| idle | 1 | 0.0 | 48.0 |
| idle | 2 | 0.0 | 60.0 |
| idle | 3 | 0.0 | 72.0 |
| kiter | 1 | 0.0 | 47.1 |
| kiter | 2 | 0.0 | 59.5 |
| kiter | 3 | 0.0 | 71.2 |
| greedy | 1 | 0.0 | 48.0 |
| greedy | 2 | 0.0 | 60.0 |
| greedy | 3 | 0.0 | 72.0 |
| defensive | 1 | 0.0 | 45.0 |
| defensive | 2 | 0.0 | 57.5 |
| defensive | 3 | 0.0 | 69.3 |
| random | 1 | 0.1 | 45.5 |
| random | 2 | 2.6 | 58.1 |
| random | 3 | 7.4 | 69.5 |
| human_like | 1 | 3.1 | 46.6 |
| human_like | 2 | 1.6 | 59.0 |
| human_like | 3 | 2.8 | 70.5 |

## Shop economy per visit (mean scrap)

| Policy | Visit | Held on entry | Spent | Unspent |
| --- | ---: | ---: | ---: | ---: |
| idle | 1 | 48.0 | 0.0 | 48.0 |
| idle | 2 | 108.0 | 0.0 | 108.0 |
| kiter | 1 | 47.1 | 44.5 | 2.6 |
| kiter | 2 | 62.0 | 58.1 | 3.9 |
| greedy | 1 | 48.0 | 38.1 | 9.9 |
| greedy | 2 | 69.9 | 67.7 | 2.1 |
| defensive | 1 | 45.0 | 41.5 | 3.5 |
| defensive | 2 | 60.9 | 53.5 | 7.4 |
| random | 1 | 45.5 | 28.7 | 16.8 |
| random | 2 | 74.9 | 42.1 | 32.8 |
| human_like | 1 | 46.6 | 37.4 | 9.2 |
| human_like | 2 | 68.2 | 62.8 | 5.4 |

## Time to kill (seconds from spawn to death)

| Policy | Enemy | Kills | p50 | p95 | max |
| --- | --- | ---: | ---: | ---: | ---: |
| idle | `enemy_chaser` | 11459 | 5.78 | 7.27 | 8.13 |
| idle | `enemy_runner` | 6541 | 3.47 | 4.42 | 4.88 |
| kiter | `enemy_chaser` | 11227 | 5.30 | 8.75 | 10.00 |
| kiter | `enemy_runner` | 6541 | 3.12 | 5.48 | 7.55 |
| greedy | `enemy_chaser` | 11459 | 3.85 | 6.98 | 8.17 |
| greedy | `enemy_runner` | 6541 | 2.92 | 5.62 | 6.53 |
| defensive | `enemy_chaser` | 10700 | 6.57 | 15.17 | 18.82 |
| defensive | `enemy_runner` | 6475 | 3.87 | 8.23 | 10.00 |
| random | `enemy_chaser` | 10825 | 5.63 | 13.83 | 18.70 |
| random | `enemy_runner` | 6488 | 3.62 | 7.92 | 13.63 |
| human_like | `enemy_chaser` | 11121 | 4.78 | 11.82 | 17.25 |
| human_like | `enemy_runner` | 6491 | 3.38 | 7.30 | 9.63 |

### Weapon pick rate and win delta when owned

| ID | kiter pick | kiter win delta | greedy pick | greedy win delta | defensive pick | defensive win delta | random pick | random win delta | human_like pick | human_like win delta |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `wpn_bolt` | 100.0 % | - | 47.8 % | - | 100.0 % | - | 22.3 % | - | 39.2 % | - |
| `wpn_saw` | 0.0 % | - | 98.5 % | - | 6.0 % | +0 (n=24/176) | 23.7 % | +0 (n=119/81) | 88.2 % | - |

### Item pick rate and win delta when owned

| ID | kiter pick | kiter win delta | greedy pick | greedy win delta | defensive pick | defensive win delta | random pick | random win delta | human_like pick | human_like win delta |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `item_bearings` | 55.9 % | +0 (n=57/143) | 26.0 % | +0 (n=27/173) | 54.4 % | +0 (n=56/144) | 25.7 % | +0 (n=39/161) | 35.6 % | +0 (n=37/163) |
| `item_brace` | 15.9 % | +0 (n=17/183) | 8.5 % | +0 (n=9/191) | 39.4 % | +0 (n=37/163) | 27.4 % | +0 (n=37/163) | 32.7 % | +0 (n=33/167) |
| `item_coupler` | 68.0 % | +0 (n=68/132) | 55.0 % | +0 (n=55/145) | 14.2 % | +0 (n=15/185) | 25.5 % | +0 (n=38/162) | 75.5 % | +0 (n=77/123) |
| `item_magnet` | 100.0 % | +0 (n=110/90) | 32.4 % | +0 (n=35/165) | 55.0 % | +0 (n=66/134) | 35.2 % | +0 (n=57/143) | 58.9 % | +0 (n=63/137) |
| `item_plating` | 15.5 % | +0 (n=17/183) | 12.2 % | +0 (n=14/186) | 51.6 % | +0 (n=47/153) | 31.2 % | +0 (n=44/156) | 13.4 % | +0 (n=15/185) |
| `item_ratchet` | 63.9 % | +0 (n=62/138) | 52.9 % | +0 (n=54/146) | 11.1 % | +0 (n=11/189) | 23.6 % | +0 (n=33/167) | 68.0 % | +0 (n=68/132) |
| `item_sealant` | 4.7 % | +0 (n=4/196) | 0.0 % | - | 12.5 % | +0 (n=11/189) | 24.2 % | +0 (n=31/169) | 2.3 % | +0 (n=2/198) |
| `item_sight` | 44.9 % | +0 (n=40/160) | 43.9 % | +0 (n=36/164) | 3.0 % | +0 (n=3/197) | 26.1 % | +0 (n=37/163) | 27.3 % | +0 (n=24/176) |

Pick rate is purchases over offers shown. Win delta is the win rate of runs that owned the entry at the start of the last wave minus the rest, in points (n owned / n other).

## Energy allocation at the start of the last wave

| Policy | Pattern | Runs | Wins | Win rate |
| --- | --- | ---: | ---: | ---: |
| idle | 3 | 200 | 200 | 100.0 % |
| kiter | 3 | 200 | 200 | 100.0 % |
| greedy | 3-3 | 200 | 200 | 100.0 % |
| defensive | 2 | 176 | 176 | 100.0 % |
| defensive | 2-2 | 24 | 24 | 100.0 % |
| random | 1 | 37 | 37 | 100.0 % |
| random | 1-1 | 4 | 4 | 100.0 % |
| random | 2 | 47 | 47 | 100.0 % |
| random | 2-1 | 9 | 9 | 100.0 % |
| random | 2-2 | 6 | 6 | 100.0 % |
| random | 3 | 70 | 70 | 100.0 % |
| random | 3-1 | 14 | 14 | 100.0 % |
| random | 3-2 | 10 | 10 | 100.0 % |
| random | 3-3 | 3 | 3 | 100.0 % |
| human_like | 3-3 | 200 | 200 | 100.0 % |
