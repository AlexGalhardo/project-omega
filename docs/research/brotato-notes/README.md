# Brotato behavior notes

Original prose notes from studying the owner's purchased Steam copy of Brotato, under [DEC-019](../../decisions/decision-register.md#dec-019--scoped-rea-inspection-of-the-owners-brotato-copy) and the [IP policy](../../legal/ip-policy.md). This repository is public.

## Rules for every note

- Describe behavior in your own words: rules, formulas, timings, state flow, data model shape. Record uncertainty and how the observation was made (REA static analysis, ordinary play).
- Never paste or attach extracted material: no decompiled code, GDScript, resource files, raw data tables, string dumps, sprites, audio, or screenshots of extracted assets. Raw REA output stays outside the repository (for example `%LOCALAPPDATA%\ProjectOmega\rea\`).
- One file per topic, named `NN-topic.md`, with: date, game build (Steam build ID), tool and version, question, finding, confidence (Verified / Inferred / Unknown), and the GDD section it could inform.
- A finding changes the game only through a change request against the GDD; the re-creation keeps original names, art, audio and text.

## Index

- [01-inventory.md](01-inventory.md): REA setup, install inventory, tool limits (RES-P1-001).
- [02-combat-stats-weapons.md](02-combat-stats-weapons.md): stats, weapons, combat (RES-P2-001).
- [03-shop-economy-run-flow.md](03-shop-economy-run-flow.md): shop, economy, run flow (RES-P2-002).
