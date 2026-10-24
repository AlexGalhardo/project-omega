## Run summary

Label `quick`, character `char_patch`, content profile Prototype, content revision `d5f2248cfb2c807bd49cb112836f4e6dff1f58daa637a343f7cdb9373c335f75`, seeds 0..200 (200 per policy), 3 waves. Percentiles: nearest-rank, index ceil(p*N)-1.

| Policy | Runs | Win rate | Combat s p50 | Combat s p95 | Deaths by wave | Distinct winning builds |
| --- | ---: | ---: | ---: | ---: | --- | ---: |
| idle | 200 | 100.0 % | 180.0 | 180.0 | - | 1/200 |
| human_like | 200 | 100.0 % | 180.0 | 180.0 | - | 53/200 |

## Prototype bands

| Check | Target | Observed | Status |
| --- | --- | --- | --- |
| idle dies in waves 1-2 | all idle runs end in wave 1 or 2 | 0.0 % | MISS |
| human_like clears wave 1 | >= 90 % | 100.0 % | pass |
| human_like clears the last wave | 50-80 % | 100.0 % | MISS |
| kiter win rate | <= 75 % | - | not run |
| survival curve drop | <= 25 points per wave (non-boss) | worst 0.0 points (human_like) | pass |
| no item or weapon above +15 points when owned | <= 15 points (cells with >= 30 runs on both sides) | worst +0.0 points (item_bearings under human_like) | pass |
| every weapon is picked | >= 3 % of offers, policies other than random and idle | lowest wpn_bolt 39.2 % | pass |
| energy: spread and single both win | >= 5 winning runs of each pattern class | single 200 wins, spread 200 wins | pass |
| characters within 15 points | human_like win rates within 15 points | one character, three 60 s waves | n/a (prototype) |
| full-run length | 12-20 minutes of combat (full game) | one character, three 60 s waves | n/a (prototype) |

## Survival (share of runs that cleared the wave)

| Policy | Wave 1 | Wave 2 | Wave 3 |
| --- | ---: | ---: | ---: |
| idle | 100.0 % | 100.0 % | 100.0 % |
| human_like | 100.0 % | 100.0 % | 100.0 % |

## Damage taken and scrap earned per wave (mean)

| Policy | Wave | Damage taken | Scrap earned |
| --- | ---: | ---: | ---: |
| idle | 1 | 0.0 | 48.0 |
| idle | 2 | 0.0 | 60.0 |
| idle | 3 | 0.0 | 72.0 |
| human_like | 1 | 3.1 | 46.6 |
| human_like | 2 | 1.6 | 59.0 |
| human_like | 3 | 2.8 | 70.5 |

## Shop economy per visit (mean scrap)

| Policy | Visit | Held on entry | Spent | Unspent |
| --- | ---: | ---: | ---: | ---: |
| idle | 1 | 48.0 | 0.0 | 48.0 |
| idle | 2 | 108.0 | 0.0 | 108.0 |
| human_like | 1 | 46.6 | 37.4 | 9.2 |
| human_like | 2 | 68.2 | 62.8 | 5.4 |

## Time to kill (seconds from spawn to death)

| Policy | Enemy | Kills | p50 | p95 | max |
| --- | --- | ---: | ---: | ---: | ---: |
| idle | `enemy_chaser` | 11459 | 5.78 | 7.27 | 8.13 |
| idle | `enemy_runner` | 6541 | 3.47 | 4.42 | 4.88 |
| human_like | `enemy_chaser` | 11121 | 4.78 | 11.82 | 17.25 |
| human_like | `enemy_runner` | 6491 | 3.38 | 7.30 | 9.63 |

### Weapon pick rate and win delta when owned

| ID | human_like pick | human_like win delta |
| --- | ---: | ---: |
| `wpn_bolt` | 39.2 % | - |
| `wpn_saw` | 88.2 % | - |

### Item pick rate and win delta when owned

| ID | human_like pick | human_like win delta |
| --- | ---: | ---: |
| `item_bearings` | 35.6 % | +0 (n=37/163) |
| `item_brace` | 32.7 % | +0 (n=33/167) |
| `item_coupler` | 75.5 % | +0 (n=77/123) |
| `item_magnet` | 58.9 % | +0 (n=63/137) |
| `item_plating` | 13.4 % | +0 (n=15/185) |
| `item_ratchet` | 68.0 % | +0 (n=68/132) |
| `item_sealant` | 2.3 % | +0 (n=2/198) |
| `item_sight` | 27.3 % | +0 (n=24/176) |

Pick rate is purchases over offers shown. Win delta is the win rate of runs that owned the entry at the start of the last wave minus the rest, in points (n owned / n other).

## Energy allocation at the start of the last wave

| Policy | Pattern | Runs | Wins | Win rate |
| --- | --- | ---: | ---: | ---: |
| idle | 3 | 200 | 200 | 100.0 % |
| human_like | 3-3 | 200 | 200 | 100.0 % |
