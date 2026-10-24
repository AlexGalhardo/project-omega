# 05 - Characters, unlocks, progression and saves

Task: RES-P3-002. Date: 2026-10-24. Build: Steam build 23429717 (owner copy, not inspected for rules; see [01-inventory.md](01-inventory.md)).

## Method

REA could not read the game data package (see note 01), so there is no tool evidence. This note is original prose from public knowledge and design reasoning. Labels: Verified, Inferred, Unknown. No direct play was done in this task, so nothing is Verified. Counts and thresholds are given as ranges or left Unknown.

## Findings

### Characters and starting kits

- The roster is large (dozens of characters). Inferred.
- Each character starts with a stat package, usually a mix of bonuses and penalties, often with a weapon restriction or a unique rule that changes how a run is played. Inferred.
- Many characters let the player pick a starting weapon from a pool; some restrict the pool. Inferred.
- Characters are balanced as play styles, not as strict upgrades. Inferred.

### Unlock conditions

- Most characters are unlocked by achievements such as winning a run, reaching a wave count or meeting a condition with a particular character or item type. Inferred.
- Weapons, items and further characters can also be gated by progress. Inferred.
- Unlocks are permanent and are announced after a run ends. Inferred.
- Difficulty tiers unlock by winning on the previous tier. Inferred.

### What persists between runs

- Unlocked content, per-character win or best-wave records and the difficulty reached persist. Inferred.
- Run state (currency, items, weapons) does not carry into a new run. Inferred.
- A run in progress can be resumed after quitting from the pause menu, at wave boundaries. Inferred.
- Settings (audio, display, input) persist. Inferred.

### Menu and result flow

- Start leads to character selection, then weapon selection, then difficulty, then the run. Inferred.
- When a run ends, a summary shows the result, the build and progress such as new unlocks, then returns to a menu. Inferred.
- A pause menu shows owned stats and items. Inferred.

### Accessibility and input

- Keyboard-and-mouse and controller are both supported, with rebinding. Inferred.
- Movement is by direction input; aiming and firing are automatic in the default mode. Inferred.
- Options include volume, screen shake and some visual toggles. Inferred. Depth of accessibility options: Unknown.

## Comparison with GDD

| Topic | Brotato (as described) | GDD | Decision |
| --- | --- | --- | --- |
| Roster size | Dozens | Four (section 9.1) | Keep GDD: scoped; four silhouettes cover the visual budget. |
| Character differences | Multi-stat packages plus unique rules | One weapon plus one additive modifier | Keep GDD: avoids hidden rules and keeps balance testable. |
| Unlock source | Achievements of varied kinds | Four wave-based challenges (section 13) | Keep GDD: deterministic, shown with progress. |
| Difficulty tiers | Several, unlocked by winning | None | Keep GDD: no configurable difficulty in 1.0 (section 15.2). |
| Run resume | Resumable at wave boundary | No resume (section 14) | Keep GDD: scope; the rationale is documented. Revisit for a later release. |
| Persistent records | Per-character best results | Completed-run and victory counts only | Keep GDD: smaller save schema. Possible later addition. |
| Menu flow | Character, weapon, difficulty, run | Character with fixed starting weapon | Keep GDD: fewer steps, same intent. |
| Input and accessibility | Rebinding, basic options | Full baseline in section 15.2 | Keep GDD: stricter than the reference; no change. |

No change request is filed.

## Open questions

Exact unlock thresholds, roster rules and the extent of accessibility options in the original are Unknown. They do not affect the four-character prototype.

## GDD sections informed

Sections 9.1, 13, 14 and 15.
