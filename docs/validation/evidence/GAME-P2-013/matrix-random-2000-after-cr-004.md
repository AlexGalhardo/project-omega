## Run summary

Label `2026-10-24-random-2000-after`, character `char_patch`, content profile Prototype, content revision `e31c1d064b80cb4ea008063ae9223c4c1b382c672083465548b31b6a7f1ee7ad`, seeds 1000..3000 (2000 per policy), 3 waves. Percentiles: nearest-rank, index ceil(p*N)-1.

| Policy | Runs | Win rate | Combat s p50 | Combat s p95 | Deaths by wave | Distinct winning builds |
| --- | ---: | ---: | ---: | ---: | --- | ---: |
| random | 2000 | 5.2 % | 105.8 | 180.0 | w2: 1334, w3: 561 | 103/105 |

### Input styles inside a policy

| Policy | Style | Runs | Win rate | Cleared wave 1 | Cleared wave 2 |
| --- | --- | ---: | ---: | ---: | ---: |

## Prototype bands

| Check | Target | Observed | Status |
| --- | --- | --- | --- |
| idle dies in waves 1-2 | all idle runs end in wave 1 or 2 | - | not run |
| human_like clears wave 1 | >= 90 % | - | not run |
| human_like clears the last wave | 50-80 % | - | not run |
| kiter win rate | <= 75 % | - | not run |
| survival curve drop | <= 25 points per wave (non-boss) | - | not run |
| no item or weapon above +15 points when owned | <= 15 points (cells with >= 50 runs on both sides) | worst +20.7 points (wpn_saw under random) | MISS |
| every weapon is picked | >= 3 % of offers, policies other than random and idle | - | not run |
| energy: spread and single both win | >= 5 winning runs of each pattern class | single 1 wins, spread 104 wins | MISS |
| characters within 15 points | human_like win rates within 15 points | one character, three 60 s waves | n/a (prototype) |
| full-run length | 12-20 minutes of combat (full game) | one character, three 60 s waves | n/a (prototype) |

## Survival (share of runs that cleared the wave)

| Policy | Wave 1 | Wave 2 | Wave 3 |
| --- | ---: | ---: | ---: |
| random | 100.0 % | 33.3 % | 5.2 % |

## Damage taken and scrap earned per wave (mean)

| Policy | Wave | Damage taken | Scrap earned |
| --- | ---: | ---: | ---: |
| random | 1 | 3.3 | 68.5 |
| random | 2 | 106.9 | 83.6 |
| random | 3 | 121.7 | 86.1 |

## Shop economy per visit (mean scrap)

| Policy | Visit | Held on entry | Spent | Unspent |
| --- | ---: | ---: | ---: | ---: |
| random | 1 | 68.5 | 38.3 | 30.2 |
| random | 2 | 158.2 | 62.4 | 95.7 |

## Time to kill (seconds from spawn to death)

| Policy | Enemy | Kills | p50 | p95 | max |
| --- | --- | ---: | ---: | ---: | ---: |
| random | `enemy_chaser` | 126194 | 6.38 | 13.83 | 29.47 |
| random | `enemy_runner` | 68377 | 3.53 | 6.98 | 13.50 |

### Weapon pick rate and win delta when owned

| ID | random pick | random win delta |
| --- | ---: | ---: |
| `wpn_bolt` | 29.3 % | - |
| `wpn_saw` | 31.0 % | +21 (n=508/158) |

### Item pick rate and win delta when owned

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

## Energy allocation at the start of the last wave

| Policy | Pattern | Runs | Wins | Win rate |
| --- | --- | ---: | ---: | ---: |
| random | 1 | 64 | 0 | 0.0 % |
| random | 1-1 | 20 | 1 | 5.0 % |
| random | 2 | 95 | 0 | 0.0 % |
| random | 2-1 | 59 | 8 | 13.6 % |
| random | 2-2 | 27 | 13 | 48.1 % |
| random | 3 | 241 | 1 | 0.4 % |
| random | 3-1 | 84 | 24 | 28.6 % |
| random | 3-2 | 56 | 41 | 73.2 % |
| random | 3-3 | 20 | 17 | 85.0 % |
