# 02 - Player stats, weapons and combat

Task: RES-P2-001. Date: 2026-10-24. Build: Steam build 23429717 (owner copy, not inspected for rules; see [01-inventory.md](01-inventory.md)).

## Method

REA could not read the game data package (see note 01). This note is original prose from public knowledge of the game's mechanics and general design reasoning. Labels: Verified means confirmed by tool or direct play in this task; Inferred means widely reported or deduced from design logic; Unknown means not established. Nothing here was verified by direct play in this task, so no item is labelled Verified. Exact numbers are deliberately omitted where memory is uncertain.

## Findings

### Stat set and stacking

- The player has a broad stat sheet: survivability (max HP, regeneration, armor, dodge, lifesteal), damage (a general percentage plus separate melee, ranged and elemental scaling), offense modifiers (attack speed, critical chance, range), and utility (movement speed, luck, pickup range, resource-gain and engineering-style stats). Inferred.
- Items and level-up picks add or subtract from stats; most stats are plain sums. Some stats have hard caps (dodge is the commonly cited one). Inferred.
- Weapon damage is built from a weapon-specific base plus fractions of several stats, so a given weapon rewards particular stats. Inferred.
- A stat can go negative through item drawbacks. Inferred.

### Weapons and tiers

- The player holds a fixed small number of weapons at once (six is the commonly cited count). Inferred.
- Weapons have four quality tiers. Two copies of the same weapon at the same tier can be combined into one higher-tier copy. Inferred.
- Weapons fire automatically at a target in range; melee weapons swing at close targets and ranged weapons fire projectiles, some piercing or exploding. Inferred.
- Fire rate is a cooldown per weapon, shortened by the attack-speed stat, with a floor so it cannot become instant. Inferred. Exact floor: Unknown.
- Higher tiers raise damage and sometimes cooldown, range or special properties. Inferred.

### Damage, armor, crit

- Armor reduces incoming damage with a diminishing-returns curve (each extra point helps less); negative armor increases damage taken. Inferred. Exact constant: Unknown.
- Critical hits multiply damage by a weapon-specific factor; crit chance is a stat. Inferred.
- Every damaging hit deals at least a minimal positive amount. Inferred.

### Hit rules and recovery

- Projectiles can pierce a limited number of targets; explosions hit an area. Inferred.
- Knockback is a weapon property and pushes enemies away; some enemies resist it. Inferred.
- The player gets a brief protection window after taking a hit, so overlapping enemies cannot deal damage every frame. Inferred. Duration: Unknown.
- Dodge, when it triggers, ignores a hit entirely. Inferred.

## Comparison with GDD

| Topic | Brotato (as described) | GDD | Decision |
| --- | --- | --- | --- |
| Number of stats | Many, with per-weapon-type scaling | Eight flat stats, no class scaling (section 7) | Keep GDD: the Foundry scope is intentionally small and "no hidden conversions" aids testing. |
| Armor curve | Diminishing returns, uncapped, negative allowed | `armor/(100+armor)` with armor clamped 0-80 (section 7.2) | Keep GDD: same family of curve, capped and integer-exact; revisit only through balance simulation (rule 1.12). |
| Dodge and lifesteal | Present | Absent (section 7.3) | Keep GDD: avoids RNG that hides skill and keeps healing sources minimal. |
| Weapon tiers | Four tiers, merge two copies | Three tiers by duplicate purchase, no merge inventory (section 7.2, 8) | Keep GDD: simpler purchase flow already specified by SP-009. |
| Weapon slots | About six slots, no power budget | Fixed slot count plus a six-point reactor budget (section 7.1) | Keep GDD: reactor allocation is the project's own differentiator. |
| Fire rate | Cooldown shortened by attack speed | Integer-exact interval from energy and attack speed (CR-002) | Keep GDD. |
| Crit | Per-weapon multiplier | Fixed 150/100 factor | Keep GDD: one parameter fewer. |
| Hit protection | Short window after a hit | 36 ticks (0.6 s) | Keep GDD: same mechanism; duration is a balance parameter. |
| Dodge and negative stats | Allowed | Clamped ranges | Keep GDD. |

No change request is filed: no difference is clearly better design than the GDD rule.

## Open questions

Exact armor constant, fire-rate floor, protection duration and crit multipliers are Unknown. They are tuning parameters; the GAME tasks implement the GDD numbers regardless.

## GDD sections informed

Sections 5-7 and 9.2.
