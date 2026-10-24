# CR-001 — Boss request tick in the Wave schema

Date: 2026-10-24. Author: Foundation integrator (from review finding REV-007). Status: **Approved** under [DEC-014](../decision-register.md#dec-014--gdd-revision-2-exact-rate-units-and-boss-request-tick). Blocking scope: GAME-P3-004, GAME-P3-005.

## Evidence

GDD revision 1, section 10.2: "Bosses spawn once on their authored waves at `(0, 250)` after a 120-tick visible warning." The Wave record (section 12) carries `boss_id?` and batch `request_tick`s but no boss request tick. Nothing states when the boss warning begins. SP-008-AC1 cannot fix the spawn tick.

## Alternatives

1. Add `boss_request_tick` to the Wave record and author it per boss wave. Validation: `boss_request_tick + warning_ticks < duration_ticks`.
2. State in GDD-010 that the boss warning always starts at wave tick 0. No schema change, but no authoring control.

## Recommendation and resolution

Alternative 1, authored as `0` for `wave_05` and `wave_10` (boss materializes at tick 120, one second before the second common batch warning ends). This keeps the current pacing intent and gives playtesting one data knob. Applied in GDD revision 2 sections 10.2, 11 and 12, and in the SP-003 schema.

## Validation

SP-003 content tests reject a boss wave without `boss_request_tick`, an ordinary wave with one, and a value that violates the duration rule.
