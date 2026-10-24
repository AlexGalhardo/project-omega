# 03 - Shop, economy and run flow

Task: RES-P2-002. Date: 2026-10-24. Build: Steam build 23429717 (owner copy, not inspected for rules; see [01-inventory.md](01-inventory.md)).

## Method

REA could not read the game data package (see note 01). This note is original prose from public knowledge and general design reasoning. Labels follow the README: Verified, Inferred, Unknown. No item was confirmed by direct play in this task, so none is Verified. Exact prices, weights and counts are deliberately not stated.

## Findings

### Currency

- Defeated enemies drop a currency that is collected by moving over it or by an automatic pull toward the player. Inferred.
- Currency also acts as experience: collecting it grants progress toward level-ups, which offer a choice of stat boosts between waves. Inferred.
- Uncollected drops are swept up at wave end. Inferred.
- A stat can increase currency gained, and unspent currency can pay a small return at wave end. Inferred. Rates: Unknown.

### Shop

- After each wave the player sees a row of offers (four is the commonly cited count) mixing weapons and items. Inferred.
- Rarity weighting shifts toward higher rarities as the run advances and with the luck stat. Inferred.
- Prices rise with wave number (inflation) and differ by item quality. Inferred. Formula: Unknown.
- Reroll replaces the row. Its cost grows with every reroll in the same visit and also with wave number. Inferred.
- Individual offers can be locked to carry over to the next visit. Inferred.
- Many items stack, some with a per-run limit. Inferred.

### Run flow

- A run is a fixed sequence of timed waves; wave length grows from short to a plateau, with a boss encounter late in the sequence. Inferred.
- Between waves: level-up picks, then the shop, then the next wave starts on player command. Inferred.
- Death ends the run; there is no carry-over of currency. Inferred.

## Comparison with GDD

| Topic | Brotato (as described) | GDD | Decision |
| --- | --- | --- | --- |
| Currency as XP and level-ups | Currency also drives level-up stat picks | Currency is purely a shop currency (section 8) | Keep GDD: reactor allocation and the shop are the progression levers; no level-up screen. |
| End-of-wave interest | Small return on unspent currency | None, explicit (section 8) | Keep GDD: avoids hoarding incentive. |
| Offers per shop | About four | Four (two weapons, two items) | Keep GDD: equivalent. |
| Rarity and luck weighting | Weighted, shifts with progress | Uniform draw, no luck (section 8) | Keep GDD: fewer systems; small content pool does not support rarity layers. |
| Price inflation | Rises per wave | Linear 8% per wave step | Keep GDD: same idea, exact formula chosen for testability. |
| Reroll | Unlimited, escalating cost, plus lock | One paid refresh per visit, no lock | Keep GDD: avoids unlimited gold sink and lock UI; revisit after QA-P4 balance data. |
| Item stacking | Many stack | Unique passives, max stack one | Keep GDD. |
| Run length | Long, many waves | Ten waves, flat 60 s | Keep GDD: scoped; note early short waves as a possible onboarding aid (owner question). |
| Wave-to-shop flow | Wave, level-up, shop, start | Wave, shop, explicit start | Keep GDD: same structure minus the level-up step. |

No change request is filed. Early short waves are the only candidate worth the owner's attention and are raised as a question rather than a CR because wave timing is already implemented.

## Open questions

Exact price curve, reroll cost curve, rarity weights, interest rate: Unknown, only tuning knowledge.

## GDD sections informed

Sections 3, 4 and 8.
