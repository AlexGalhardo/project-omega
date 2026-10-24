# 04 - Enemies, bosses, waves and spawning

Task: RES-P3-001. Date: 2026-10-24. Build: Steam build 23429717 (owner copy, not inspected for rules; see [01-inventory.md](01-inventory.md)).

## Method

REA could not read the game data package (`Brotato.pck`, see note 01), and the executable holds only engine code, so no REA step answers a mechanics question. This note is original prose from public knowledge of the game and general design reasoning. Labels follow the README: Verified (tool or direct play), Inferred (widely reported or deduced), Unknown. No direct play was done in this task, so nothing is Verified. Exact numbers are omitted or given as ranges where memory is uncertain.

## Findings

### Enemy behavior families

- Most basic enemies simply walk straight at the player and hurt on contact. Inferred.
- Fast variants exist that trade health for speed. Inferred.
- Ranged enemies stop or slow at a distance and fire single or multi-shot projectiles at the player. Inferred.
- Some enemies charge in a straight line after a short pause, and some leave harmful ground areas or spawn smaller enemies when they die. Inferred.
- Stationary or slow summoner types create further enemies over time. Inferred.
- Enemy variety grows over the run: early waves show only a few families, later waves mix many. Inferred.
- Enemies do not block each other; large crowds overlap. Inferred.

### Telegraphs and timings

- Newly spawning enemies appear as a ground marker for a short moment before becoming dangerous. Inferred. Duration: Unknown, likely around a second or less.
- Charging and some ranged attacks show a brief wind-up animation rather than a drawn lane. Inferred.
- Projectile speeds are slow enough to dodge by moving, but are rarely avoidable once many are on screen. Inferred.

### Spawn placement and safety

- Enemies spawn at random points away from the player, not only on the arena edge. Inferred.
- A minimum distance from the player is respected so enemies do not appear on top of the player. Inferred. Distance: Unknown.
- Spawning happens in repeated groups over the wave rather than a single burst; group size and rate rise with the wave. Inferred.
- Some waves are scripted as hordes (one large burst, then a lull) and some bring an extra enemy type. Inferred.

### Wave scaling

- Enemy health and damage grow with wave number; health scaling is the stronger of the two. Inferred. Curve shape and constants: Unknown.
- Difficulty settings (danger levels) layer extra scaling and extra enemy modifiers on top of the base curve. Inferred.
- Enemy count per wave rises over the run, with some waves deliberately lighter. Inferred.
- Defeated enemies drop currency; stronger enemies drop more. Inferred.

### Elites and bosses

- An elite is a tougher variant of a normal enemy with extra health, a distinct look and a bigger reward. Inferred. Elites appear from the middle of a run on specific waves, sometimes together with other enemies. Inferred.
- Bosses appear on the late fixed waves (an early boss around the middle and a final boss at the end of the standard run). Inferred. Their health is far above ordinary enemies and they come with normal wave spawns. Inferred.
- A boss has a few attack patterns (aimed shots, radial or spread volleys, charges, summons) chosen in a loop or at random, and may change behavior as its health drops. Inferred. Schedule and phase thresholds: Unknown.
- Whether the wave clears when the timer ends with a boss still alive: Unknown.

## Comparison with GDD

| Topic | Brotato (as described) | GDD | Decision |
| --- | --- | --- | --- |
| Enemy family count | Many families, grows through the run | Ten common types (section 10.1) | Keep GDD: scoped to a ten-wave prototype; the ten types already cover chase, fast, tank, ranged, charge, stationary, beam, spread and orbit. |
| Telegraph style | Short spawn marker; wind-up animation, no lane | Explicit warning, lane or circle with fixed tick durations | Keep GDD: readable, testable telegraphs are an accessibility goal (section 15). |
| Spawn location | Random inside the arena with a minimum distance | Eight boundary anchors, 180 LU exclusion, postpone and cancel rules | Keep GDD: deterministic, auditable and keeps the arena center clear. |
| Spawn rhythm | Repeating groups, horde waves, light waves | Six fixed batches per wave, count per row | Keep GDD: authored counts are data; a horde wave can be authored later without a rule change. |
| HP and damage scaling | Strong HP scaling, weaker damage scaling, plus difficulty layers | Linear +18% HP and +8% damage per wave step, no difficulty layers | Keep GDD: same direction (HP faster than damage); exact curve belongs to RES-P4-001 and simulation. |
| Elites | Special tougher variants | None | Keep GDD: out of scope for 1.0; bosses give the spike. Revisit in a later release scope. |
| Boss count and placement | Mid-run and final | Waves 5 and 10 | Keep GDD: equivalent shape. |
| Boss phases | Possible health-based behavior changes | No hidden phases or summons | Keep GDD: avoids unannounced behavior; readability and accessibility rules. |
| Boss kill needed to clear wave | Unknown | Boss death is required | Keep GDD: stated and tested. |

No change request is filed.

## Open questions

Spawn marker duration, minimum spawn distance, HP and damage curves, elite wave schedule and boss phase thresholds are Unknown. They matter only for tuning; the GDD stays authoritative.

## GDD sections informed

Sections 10 and 11.
