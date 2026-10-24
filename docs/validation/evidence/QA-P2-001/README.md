# Evidence: QA-P2-001

Status: IN_REVIEW (headless evidence complete; the owner's native playtest is the Prototype exit, see the local questions file). Base commit: `0fffd87`. Environment: Windows 11, Rust 1.99.1, Bevy 0.20.0 vendored fork, Ryzen 7 5700X3D, release build.

## Commands

| Command | Exit | Result |
| --- | --- | --- |
| `cargo test -p omega_game --no-default-features --locked --test combat_contract --test shop_contract --test run_contract` | 0 | V07 (21), V09 (14), V11 (9) seeded loop cases pass |
| `cargo test -p omega_game --no-default-features --locked --test perf02_contract` | 0 | 2 passed |
| `omega-sim --scenario perf02 --seed 5a..5a --output docs/validation/evidence/QA-P2-001/perf02-prototype-runN.json` (N = 1..3, release) | 0 | three independent runs, 10,800 ticks, 2 shop visits, final state Victory |
| `powershell -NoProfile -File scripts/check.ps1` | 0 | all gates passed |

## PERF-02 prototype workload (simulation tick only)

Workload: three waves of the shipped prototype content, scripted strafing and manual-aim sweep, scripted shop purchases, fixed seed. Timed region is one `GameTick`; render, GPU, allocation and working-set metrics are unsupported in this build (listed in each JSON). Percentiles are nearest-rank.

| Run | P50 us | P95 us | P99 us | Max us |
| --- | ---: | ---: | ---: | ---: |
| 1 | 54.5 | 116.1 | 153.5 | 1026.5 |
| 2 | 51.4 | 111.7 | 155.8 | 906.8 |
| 3 | 55.9 | 122.7 | 161.6 | 914.8 |

Peak populations in every run: 8 live enemies, 3 live projectiles. No threshold is set (performance plan: measure first). The simulation costs under 1% of a 16.6 ms frame at this scale; rendering is not measured here.

## Limits

Workload is prototype scale. The native pacing, readability and economy observations need the owner; frame times need the windowed benchmark (PERF-02 render side arrives with the Beta measurement tasks).
