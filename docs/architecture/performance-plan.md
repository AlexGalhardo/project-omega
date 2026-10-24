# Performance measurement plan

Status: APPROVED by owner blanket approval, 2026-10-24. **Recommendation accepted:** measure before setting budgets or selecting optimizations. **Verified inventory only:** E4-L01 in the [ledger](../research/sources.md#stage-4-verification). No scenario has been run.

## Reference and reporting

Initial reference: Ryzen 7 5700X3D, RTX 4060, 32 GiB, Windows 11 Pro build 10.0.26300, driver 32.0.16.1742. Use x86_64-pc-windows-msvc release builds, record rustc/Cargo/commit/lockfile/content hashes, power mode, display resolution, driver, graphics backend and sync mode. Measure 1280x720 and 1920x1080; run uncapped for throughput and with normal presentation settings for pacing. Minimum shipping hardware is still unknown and must be selected before Beta qualification. Hosted CI is not a graphics performance reference.

## Scenarios

| ID | Workload / inputs | Purpose |
| --- | --- | --- |
| PERF-01 | Boot/Menu, then empty arena; cold and warm asset load separately | Startup, idle CPU/GPU and baseline memory. |
| PERF-02 | GDD ten-wave schedules; fixed seed and recorded movement/manual-aim input | Representative progression, spawning, boss and shop transitions. |
| PERF-03 | Controlled populations: 64 enemies/128 projectiles; 256/1024; 1024/4096 | Scaling exploration; these counts are test inputs, not supported-count promises or budgets. |
| PERF-04 | Same populations with bodies clustered in a 200-LU disk; piercing/explosions active | Collision candidate pressure and repeated-hit filtering. |
| PERF-05 | 256 spawn requests in one tick, then deaths/despawns and pickup settlement | Structural change, allocation and event-queue spikes. |
| PERF-06 | Two-hour automated repeated run/shop/teardown cycles; fixed seed list | Memory/entity/audio-handle growth and long-session failure detection. |
| PERF-07 | Visual catalogs swapped at Menu; SVG-derived and raster alternative, same inputs | Trace equality plus load/memory/render comparison. |
| PERF-08 | Fresh build, no-change build and single game-module edit rebuild | Cold/warm build cost before dynamic linking/linker/profile changes. |

For steady scenarios, warm up 10 seconds and capture 120 seconds, three independent runs. Report each run and aggregate separately; never pool samples from differing settings. Spawn spikes report maximum and surrounding tick/frame traces. Long-session measurements include start, periodic samples and settled end-state. Inputs/scenarios are checked into validation fixtures when implemented.

## Metrics and method

Record CPU frame wall time and simulation tick time separately; render/GPU timings if supported; P50/P95/P99 using nearest-rank sorted samples (`ceil(p*N)-1` index), maximum and sample count. Capture allocation count/bytes with an explicitly documented instrumented build, plus process working set; instrumented results are not normal-build frame-time claims. Record live entities by role, archetypes, pending spawn/attack/presentation queues, draw calls, sprite count, texture memory estimates, audio voices and asset handles. Unsupported metrics are labeled Unsupported, never zero. Record actual tools/counter source and overhead.

Compare identical workload/configuration before and after an optimization. Only choose spatial partitioning, pooling, component restructuring, atlases or custom allocators when a measured bottleneck and regression checks justify them. Investigate memory/handle growth after complete teardown rather than assuming every temporary peak is a leak. No P95/P99 threshold is set yet. After baseline, propose hardware-specific budgets for owner review and link them to release gates; benchmark variation does not automatically fail normal correctness CI.
