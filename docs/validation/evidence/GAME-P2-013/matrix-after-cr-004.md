## Run summary

Label `2026-10-24-after`, character `char_patch`, content profile Prototype, content revision `e31c1d064b80cb4ea008063ae9223c4c1b382c672083465548b31b6a7f1ee7ad`, seeds 0..200 (200 per policy), 3 waves. Percentiles: nearest-rank, index ceil(p*N)-1.

| Policy | Runs | Win rate | Combat s p50 | Combat s p95 | Deaths by wave | Distinct winning builds |
| --- | ---: | ---: | ---: | ---: | --- | ---: |
| idle | 200 | 0.0 % | 90.6 | 100.7 | w2: 200 | 0/0 |
| kiter | 200 | 37.5 % | 178.1 | 180.0 | w3: 125 | 8/75 |
| greedy | 200 | 100.0 % | 180.0 | 180.0 | - | 50/200 |
| defensive | 200 | 98.5 % | 180.0 | 180.0 | w2: 3 | 93/197 |
| random | 200 | 5.5 % | 105.7 | 180.0 | w2: 137, w3: 52 | 11/11 |
| human_like | 200 | 59.5 % | 180.0 | 180.0 | w1: 13, w2: 24, w3: 44 | 67/119 |

### Input styles inside a policy

| Policy | Style | Runs | Win rate | Cleared wave 1 | Cleared wave 2 |
| --- | --- | ---: | ---: | ---: | ---: |
| human_like | aggressive | 100 | 19.0 % | 87.0 % | 63.0 % |
| human_like | cautious | 100 | 100.0 % | 100.0 % | 100.0 % |

## Prototype bands

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

## Survival (share of runs that cleared the wave)

| Policy | Wave 1 | Wave 2 | Wave 3 |
| --- | ---: | ---: | ---: |
| idle | 100.0 % | 0.0 % | 0.0 % |
| kiter | 100.0 % | 100.0 % | 37.5 % |
| greedy | 100.0 % | 100.0 % | 100.0 % |
| defensive | 100.0 % | 98.5 % | 98.5 % |
| random | 100.0 % | 31.5 % | 5.5 % |
| human_like | 93.5 % | 81.5 % | 59.5 % |

## Damage taken and scrap earned per wave (mean)

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

## Shop economy per visit (mean scrap)

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

## Time to kill (seconds from spawn to death)

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

### Weapon pick rate and win delta when owned

| ID | kiter pick | kiter win delta | greedy pick | greedy win delta | defensive pick | defensive win delta | random pick | random win delta | human_like pick | human_like win delta |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `wpn_bolt` | 100.0 % | - | 100.0 % | - | 100.0 % | - | 30.3 % | - | 100.0 % | - |
| `wpn_saw` | 0.0 % | - | 66.7 % | - | 67.1 % | - | 30.6 % | +24 (n=46/17) | 74.2 % | - |

### Item pick rate and win delta when owned

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

## Energy allocation at the start of the last wave

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
