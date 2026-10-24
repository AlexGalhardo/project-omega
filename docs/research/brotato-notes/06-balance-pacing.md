# 06 - Balance numbers and pacing

Task: RES-P4-001. Date: 2026-10-24. Build: Steam build 23429717 (owner copy, not inspected for rules; see [01-inventory.md](01-inventory.md)).

## Method

REA could not read the game data package (see note 01), so no balance number was extracted. This note is original prose from public knowledge and design reasoning, and builds on notes [02](02-combat-stats-weapons.md), [03](03-shop-economy-run-flow.md), [04](04-enemies-bosses-waves.md) and [05](05-characters-unlocks-saves.md). Labels: Verified, Inferred, Unknown. Nothing is Verified because no play measurement was taken in this task. Numbers are given as ranges or left Unknown.

## Findings

### Run length

- A standard run lasts roughly half an hour to an hour of active play, driven by about twenty waves whose duration ramps from short to about a minute. Inferred.
- Total time is dominated by combat; shop and level-up pauses add several minutes. Inferred.

### Damage and HP curves

- Enemy health grows faster than enemy damage across waves. Inferred.
- Player damage must therefore grow through stacked items and weapon upgrades; a build that does not scale falls behind in the second half. Inferred.
- Defense and healing become necessary around the middle of the run as damage per hit rises. Inferred. Thresholds: Unknown.

### Economy pacing

- Currency income per wave rises with enemy count and enemy value; prices rise with wave number, so purchasing power grows slower than raw income. Inferred.
- A typical wave allows a small number of purchases, so choices are tight early and looser later. Inferred. Counts: Unknown.
- Level-up picks and shop items are both stat sources, so currency is a power source through two channels. Inferred.

### Item and weapon power bands

- Rarity tiers form power bands; upgrading a weapon tier is a large step relative to a common item. Inferred.
- Some items trade one stat for another, so one item is rarely strictly better. Inferred.
- Weapon classes differ in reliability: melee needs closeness, ranged is safer but weaker per shot. Inferred.

### Difficulty spikes

- Spikes come from boss waves, horde waves and the introduction of ranged or special enemies. Inferred.
- The first few waves are deliberately easy to teach movement. Inferred.
- Perceived difficulty depends strongly on build coherence, not wave number alone. Inferred.

## Comparison with GDD

| Topic | Brotato (as described) | GDD | Decision |
| --- | --- | --- | --- |
| Run length | Long (about 30-60 min) | Ten waves, at least 10 min active combat | Keep GDD: prototype scope; the lower bound is stated openly (section 3). |
| HP versus damage growth | HP faster than damage | +18% HP and +8% damage per wave step | Keep GDD: consistent direction. |
| Late-run dependency on scaling | Stacked items required | Unique items, no stacking, clamped stats | Keep GDD: clamps bound power; balance by simulation (GAME-P2-013, QA-P3-001). |
| Defense necessity | Rises mid-run | Armor, regen, capped HP | Keep GDD: equivalent pressure via boss waves. |
| Income versus prices | Prices grow slower than income | Flat scrap per kill, +8% price per wave | Keep GDD: check with the balance simulator and tune by data, not by this note. |
| Purchases per shop | Tight early, loose later | Four offers, one refresh | Keep GDD: same intent. |
| Difficulty spikes | Boss, horde and special enemy waves | Boss waves 5 and 10, plus type mixes | Keep GDD: equivalent. |
| Early onboarding | Short early waves | Flat 60 s | Keep GDD for now; same open question as Q-003. |

No change request is filed. All differences are scope-level; tuning stays with the balance simulator and `docs/validation/balancing.md`.

## Open questions

All numeric curves are Unknown. A play measurement of wave length, income per wave and time to clear would turn parts of this note into Verified; the owner may do it optionally.

## GDD sections informed

Sections 3, 7, 8, 10 and 11.
