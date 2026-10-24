# QA checklist

Exploratory checks walked by the QA bot and by screenshot review at every phase end (ROADMAP rule 1.11). Each phase appends its items; a failed item becomes a bug task with a reproducing test.

## Foundation

- [ ] Launch, wait for assets, quit cleanly (exit code 0)
- [ ] Resize the window across 16:9, 21:9 and 5:4; arena stays fully visible with letterboxing; clicks in bars are ignored
- [ ] Focus loss and regain; no simulated ticks while unfocused
- [ ] Pause spam (Escape bursts, pause during resize and focus loss); no tick while paused, resume only on explicit resume
- [ ] Invalid input (opposite keys held, Confirm and Open Energy during combat, cursor in a letterbox bar); no crash, no stuck state
- [ ] Swap to the pixel skin catalog; trace unchanged
- [ ] Missing cosmetic visual shows its category fallback (catalog fallback)

## Prototype

- [ ] Shop double purchase: buying the same offer twice (key, button, mashed) takes scrap once and installs once; the second request is rejected and scrap, build and offers are unchanged (`tests/e2e/shop-double-purchase.json`)
- [ ] Shop refresh spam: only one refresh per visit; further refreshes are rejected with a reason and scrap is unchanged (`tests/e2e/shop-refresh-spam.json`)
- [ ] Shop unaffordable and invalid purchases: an unaffordable, sold or out-of-range offer is rejected with a reason and changes nothing (`tests/e2e/shop-unaffordable.json`)
- [ ] Zero-energy start: starting the next wave with no powered weapon is rejected with the reason, the shop stays open and a later valid start works (`tests/e2e/shop-zero-energy-start.json`)
- [ ] Energy limits: no more than 6 points in total and 3 per slot, none on an empty slot; rejected requests change nothing, and unspent points grant nothing (`tests/e2e/shop-energy-allocation.json`)
- [ ] Death during the shop transition: dying in the same tick the wave completes settles Defeat once and never opens a shop (`tests/e2e/death-at-wave-completion.json`)
- [ ] Death while in the shop does not end the run; the next wave starts with full HP (`tests/e2e/death-in-shop.json`)
- [ ] Abandon: opening the dialog and cancelling changes nothing; confirming settles Abandoned once and shows the Summary (`tests/e2e/abandon-cancel.json`, `tests/e2e/abandon-confirm.json`)
- [ ] Full loop: move, fight, wave 1 to the shop, buy, allocate energy, wave 2, wave 3 to Victory or Defeat, Continue to the Summary (`tests/e2e/prototype-loop.json`)
- [ ] Result screens ignore stray requests: Buy, Refresh, Start and Energy after Victory, Defeat or Summary change nothing
- [ ] Confirm or Open Energy pressed during combat does not start the next wave when the shop opens
- [ ] Bot: 3 profiles x 20 seeds play the whole prototype loop with no finding and no panic (`omega-game --headless --qa-bot <profile> --seed <hex> --frames 60000`)
- [ ] Screenshot review (owner): shop readability, disabled-button reasons, result screens, abandon dialog

## How to walk it

- Scripted: `omega-game --headless --e2e-script tests/e2e/foundation.json` or any `tests/e2e/*.json` (CI, no GPU; `"mode": "prototype"` scripts drive the real run loop) or without `--headless` on the reference machine; reports and screenshots land in `artifacts/e2e/`.
- Simulated human: `omega-game --headless --qa-bot <cautious|aggressive|chaotic> --seed <64 hex> [--frames N]`; the report lists findings with the seed and tick that reproduce them.
- Screenshot items (letterbox bars, fallback visual) are reviewed by eye from a windowed run.
