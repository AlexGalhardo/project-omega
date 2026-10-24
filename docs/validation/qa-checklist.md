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

## How to walk it

- Scripted: `omega-game --headless --e2e-script tests/e2e/foundation.json` (CI, no GPU) or without `--headless` on the reference machine; reports and screenshots land in `artifacts/e2e/`.
- Simulated human: `omega-game --headless --qa-bot <cautious|aggressive|chaotic> --seed <64 hex> [--frames N]`; the report lists findings with the seed and tick that reproduce them.
- Screenshot items (letterbox bars, fallback visual) are reviewed by eye from a windowed run.
