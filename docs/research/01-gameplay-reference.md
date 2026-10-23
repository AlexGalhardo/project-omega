# Track A — Gameplay reference and design principles

Status: Stage 1 research, 2026-10-23. Recommendations are not approved game requirements. Evidence labels and dated source records are canonical in [sources.md](sources.md). Policy: [IP policy](../legal/ip-policy.md).

## Findings and evidence boundaries

**Verified (source reports, not team gameplay):** the current Steam description advertises default automatic firing, optional manual aim, runs under 30 minutes, waves of 20–90 seconds, material collection, and shops between waves [A01]. These are product descriptions, not a measured distribution of play times. Normal-run structure is reported as 20 waves [A03]. Do not generalize this to every mode, DLC, console, or mobile port.

The research consulted public descriptions and wiki pages. No executable, package, extracted data, recorded playthrough, or live gameplay was inspected. Therefore no claim below establishes exact timing, game feel, or behavior of a specific installed build. **Unknown:** reference build to observe. The Weapons page identifies patch 1.1.6.3, while the current Danger Levels page includes Nightmare and the Progress page has older scope wording [A04, A07, A08]. Treat this as version skew, not one coherent current ruleset.

| Area | Public evidence / uncertainty | Design implication (Inferred; origin: inferred) |
| --- | --- | --- |
| Core loop | **Verified:** survival, collection, upgrades, shop, next wave [A01, A03]. | Separate real-time execution from deliberate purchasing; measure both combat time and decision time. |
| Spawning and pacing | **Verified:** enemy types include different behavior roles; wiki reports a population cap and replacement behavior [A06]. **Unknown:** current spawn distributions and exact warning timings. | Pressure needs a composition and timing model, not only more HP. Do not adopt the reference cap as a performance target. |
| Movement and targeting | **Verified:** optional manual aim [A01]. **Unknown:** target priority, tie-breaking, target retention, movement acceleration, diagonal normalization. | Auto-aim policy affects positioning skill and weapon usefulness; make it an explicit original rule. |
| Melee / ranged | **Verified:** melee sweeps/thrusts and projectile attacks have distinct coverage; piercing and bounce extend projectile hits [A04]. | Crowd coverage, reach, cooldown, and exposure should create different roles. |
| Hitboxes and knockback | **Verified:** wiki describes knockback away from the player, even for projectile hits [A16]. **Unknown:** collision geometry, hit-repeat rules, resistance, displacement curve. | Separate attack visuals, hurtboxes, hit memory, and knockback response. |
| Damage and scaling | **Verified:** weapon-specific stat scaling and percentage damage are documented [A04, A09]. **Unknown:** complete interaction and rounding order across effects. | Tooltips should explain the same calculation combat uses; do not reproduce a guessed global formula. |
| Mitigation and crits | **Verified:** armor, chance-based dodge with caps/exceptions, and crit have separate documented behaviors [A10, A15, A23]. **Unknown:** current damage-floor, invulnerability and simultaneous-hit order. | Defensive choices need visible marginal value and predictable boundary behavior. |
| Healing | **Verified:** regeneration is time-based; life steal is reported as a chance to restore one HP, subject to a 0.1-second lockout [A13, A14]. | Proc frequency can matter more than nominal damage; caps and exclusions must be explicit. |
| Stats / characters | **Verified:** the stat system includes offense, defense, mobility, economy, and character-specific modifiers [A02, A09]. | Character constraints can create replayability with shared systems; exceptions also increase test combinations. |
| Weapons | **Verified:** most characters use six slots; four tiers, pairwise upgrades and class-count bonuses are reported, with character exceptions. Weapons can belong to multiple classes [A04]. | Slots and combining impose opportunity costs; class incentives shape specialization. Exact counts and bonuses are owner decisions. |
| Items / synergies | **Verified:** public item descriptions mix bonuses, penalties, and conditional effects [A17]. | Prefer a small set of understandable interactions before adding many bespoke triggers. |
| Shop / rerolls | **Verified:** four offers, locking, increasing reroll costs, and selection biases are reported [A05]. | Give players ways to manage randomness; model an entire purchase as an atomic transaction. |
| Materials / economy | **Verified:** pickups contribute currency and XP; uncollected materials have a carryover mechanism [A03, A12]. | Collection competes with safety. Currency, XP, and deferred rewards need distinct accounting. |
| Progression / difficulty | **Verified:** challenge-based unlocks and cumulative danger modifiers are documented; current page also lists Nightmare [A07, A08]. | Separate unlocking variety from permanent power and from difficulty selection. |

## Formula audit

These are descriptions of public reports, not approved values for the original game.

| Topic | Evidence status | What remains unconfirmed |
| --- | --- | --- |
| Armor | **Inferred:** the positive-armor table is consistent with a multiplier `1 / (1 + A / 15)` before final rounding [A10]. | Exact coefficient representation, negative-armor branch, damage floor, intermediate/final rounding, exceptions. Do not promote the fitted expression to verified code behavior. |
| Attack speed | **Verified:** the wiki explicitly warns that simple percentage scaling has exceptions, melee range effects, and minimum-duration behavior [A11]. | Complete current piecewise formula, simulation quantization, animation contribution. |
| Crit | **Verified:** reported effective chance is bounded and combines weapon and character effects [A15]. | Whether RNG is per hit/target/projectile, interaction with pierce and secondary effects, multiplier rounding. |
| Regeneration | **Verified:** reported examples include 1 HP per 5 seconds at stat 1 and per second at stat 10 [A13]. | First tick, fractional accumulation, pause boundaries, complete formula. |
| Harvesting | **Verified:** wiki reports wave-end currency/XP and growth with rounding [A02]. | Character/DLC exceptions and ordering relative to other wave-end effects. |
| Prices / offers | **Verified:** offer rules depend on wave, ownership and luck [A05]. | Exact current joint distribution, price rounding, lock/reroll/reset interactions. |

## Black-box experiments proposed — none executed

**Recommendation; origin: original design.** First record platform, visible game version, DLC, mode, character, difficulty, accessibility settings, equipment, and capture frame rate. Use ordinary gameplay only. Record rejected trials and confidence intervals; presentation frames are not proof of internal simulation ticks. Where ordinary play cannot isolate a variable, keep the result Unknown instead of modifying packages.

| ID | Question and method | Measurements / discriminating result |
| --- | --- | --- |
| EXP-A01 | How does armor affect received damage? Compare repeated hits from the same enemy type/wave with controlled positive, zero, and negative armor. | Raw HP deltas and displayed armor; choose input cases where candidate rounding rules diverge. Do not mix enemy damage rolls with armor effects. |
| EXP-A02 | What determines attack cadence? Record a single unchanged weapon at several speed/range values, including negative speed. | Inter-attack intervals over many attacks; distinguish target reacquisition delay, animation and cooldown. If frame resolution cannot distinguish formulas, report a bound. |
| EXP-A03 | How does targeting work? Arrange nearest, aligned and equal-distance candidates through ordinary positioning, testing auto/manual modes. | First target, retention after movement, switches; repeated symmetric cases reveal possible tie instability, not the internal data structure. |
| EXP-A04 | What can one attack hit? Observe melee sweeps and single/piercing projectiles with overlapping enemies. | Per-target hits, projectile survival, repeat hits, knockback direction and displacement. Treat occluded numbers as missing data. |
| EXP-A05 | What is spawn pacing? Record complete waves with comparable settings and multiple runs. | Warning onset to activation, time-binned counts, roles, spawn locations, visible crowd density. Screen occlusion limits population estimates. |
| EXP-A06 | How does the economy evolve? Log offers, prices, locks, reroll number, wave, luck, owned weapons and materials. | Deterministic price changes first; estimate probabilities with uncertainty separately. A handful of shops cannot confirm rare-event chances. |
| EXP-A07 | How do healing/crit/procs interact? Hold one healing source/weapon at a time, log HP and damage events at known stats. | Tick phase, healing caps, crit frequency and repeated-hit behavior. Stop when co-occurring effects prevent attribution. |
| EXP-A08 | How does the game communicate danger? Observe full ordinary runs and annotate damage, warnings, HUD checks and shop duration. | Missed threats, warning lead times, screen obstruction, feedback latency, decision duration. This evaluates usability; it cannot establish design intent. |

## Comparisons: principles, not feature accumulation

All factual anchors are **Verified as developer store descriptions**, accessed 2026-10-23; all lessons are **Inferred**, not measured causal claims.

| Reference | Factual anchor | Principle and trade-off for this project |
| --- | --- | --- |
| Vampire Survivors [A18] | Minimal-input survival with persistent upgrades. | Low input complexity can leave attention for navigation and build choices; persistent power complicates fair difficulty comparisons. |
| 20 Minutes Till Dawn [A19] | Directional aim, active firing, upgrade combinations and persistent runes. | Manual aiming is a major attention-budget choice, with controller and accessibility consequences. Decide it early. |
| Halls of Torment [A20] | Exploration, quest progression, bosses and recoverable equipment across runs. | Spatial objectives can interrupt passive circling; equipment persistence adds inventory, saves and balancing work. |
| Death Must Die [A21] | Blessing combinations, equipment and selectable risk/reward. | Thematic families can make synergies legible; stacking several progression axes creates a larger content and test matrix. |
| Soulstone Survivors [A22] | Many skills, characters, crafted weapons and progression systems. | Combinatorial builds need consistent effect vocabulary and readable threats; its advertised volume is not a solo-project scope target. |

## Game feel, recommendation and alternatives

**Recommendation; origin: original design:** borrow the cadence principle of short combat segments followed by meaningful decisions. Choose one original differentiator during Stage 2 and test it in a small vertical slice. Keep wave timers, HP, currency, attack readiness, and dangerous telegraphs legible; provide independent controls for screen shake, flashes and effect density. Camera framing should preserve threat visibility. These are proposed usability requirements, not observed Brotato internals.

An uninterrupted survival loop reduces shop/UI work but changes the reference cadence. Manual combat adds agency but competes with navigation and build planning. Persistent equipment increases long-term goals but adds save, inventory, and balance scope. None is selected yet.

## Risks and unknowns

- **Unknown:** owner's intended run duration, aiming mode, slot count, progression type, difficulty range and content counts.
- **Unknown:** firsthand feel, exact hit geometry, invulnerability timing, boss timing, camera rules and current reference build.
- **Inferred risk:** importing reference exceptions produces complexity without proving fun or originality.
- **Recommendation:** use the public model to inform original choices; run only experiments that can change a design decision. No exact Brotato reproduction is required for 1.0.
