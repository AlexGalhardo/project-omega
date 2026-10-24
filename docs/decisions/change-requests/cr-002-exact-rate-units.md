# CR-002 — Exact integer units for fire rate and energy multiplier

Date: 2026-10-24. Author: Foundation integrator (from review finding REV-008). Status: **Approved** under [DEC-014](../decision-register.md#dec-014--gdd-revision-2-exact-rate-units-and-boss-request-tick). Blocking scope: GAME-P2-001 (SP-006).

## Evidence

GDD revision 1 declares regen in tenths but writes `base_rate` (`2.0`, `0.65`) and E(p) (`1.60`) as decimals with no denominator. `interval_ticks = ceil(60 / min(requested_rate, 10))` is boundary-sensitive under floating-point evaluation order, so two correct-looking implementations can disagree by one tick.

## Alternatives

1. Integer units: `base_rate_centi` in hundredths of attacks per second, E(p) in percent, interval evaluated with integer ceiling division.
2. Keep decimals and mandate one floating evaluation order. Fragile; rejected.

## Recommendation and resolution

Alternative 1. Content field `base_rate_centi` (bolt `200`, rocket `65`); `E_pct(p) = {0, 100, 160, 200}`. With `attack_speed_pct` clamped as in section 7:

`requested_centi_num = base_rate_centi * E_pct * (100 + attack_speed_pct)` (units: 1/1 000 000 attacks per second, numerator over `10_000`), and

`interval_ticks = max(ceil(6000 * 10_000 / min(requested_centi_num, 1000 * 10_000)), windup_ticks + recovery_ticks, 1)`,

computed in checked `u64` arithmetic. Worked examples are unchanged: 2.0 attacks/s at two points and +25% gives `200*160*125 = 4_000_000`, `ceil(60_000_000 / 4_000_000) = 15` ticks; one point and +15% gives `200*100*115 = 2_300_000`, `ceil(60_000_000/2_300_000) = 27` ticks. No balance value changes. Applied in GDD revision 2 sections 7.1, 9.2 and 12.

## Validation

SP-006 unit tests cover the worked examples and the 10 attacks/s cap boundary with integer expectations.
