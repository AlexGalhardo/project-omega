# QA checklist

Exploratory checks walked by the QA bot and by screenshot review at every phase end (ROADMAP rule 1.11). Each phase appends its items; a failed item becomes a bug task with a reproducing test.

## Foundation

- [ ] Launch, wait for assets, quit cleanly (exit code 0)
- [ ] Resize the window across 16:9, 21:9 and 5:4; arena stays fully visible with letterboxing; clicks in bars are ignored
- [ ] Focus loss and regain; no simulated ticks while unfocused
- [ ] Swap to the pixel skin catalog; trace unchanged
- [ ] Missing cosmetic visual shows its category fallback
