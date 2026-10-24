//! SP-004 acceptance: ENG-GEOM-001 tangency, sweeps, degenerate inputs and invariants.

use bevy::math::DVec2;
use omega_2d::geometry::*;

fn circle(x: f64, y: f64, r: f64) -> Circle {
	Circle::new(DVec2::new(x, y), r).unwrap()
}

#[test]
fn ac1_tangent_circles_overlap_and_separated_do_not() {
	assert!(circles_overlap(
		&circle(0.0, 0.0, 14.0),
		&circle(27.0, 0.0, 13.0)
	));
	assert!(!circles_overlap(
		&circle(0.0, 0.0, 14.0),
		&circle(27.001, 0.0, 13.0)
	));
}

#[test]
fn ac1_tangent_capsule_overlaps_and_separated_does_not() {
	// Chain Saw lane: reach 80, radius 18 from origin along +X.
	let saw = Capsule::new(DVec2::ZERO, DVec2::new(80.0, 0.0), 18.0).unwrap();
	assert!(capsule_circle_overlap(&saw, &circle(40.0, 31.0, 13.0)));
	assert!(capsule_circle_overlap(&saw, &circle(111.0, 0.0, 13.0)));
	assert!(!capsule_circle_overlap(&saw, &circle(40.0, 31.01, 13.0)));
	assert!(!capsule_circle_overlap(&saw, &circle(111.01, 0.0, 13.0)));
}

#[test]
fn ac2_fast_sweep_reports_first_impact_between_endpoints() {
	let shot = circle(0.0, 0.0, 3.0);
	let target = circle(50.0, 0.0, 10.0);
	let t = sweep_circle(&shot, DVec2::new(100.0, 0.0), &target).expect("must hit");
	assert!(
		(t - 0.37).abs() < 1e-12,
		"first contact at x = 37, got t = {t}"
	);
	// Both endpoints miss; only the sweep catches the target.
	assert!(!circles_overlap(&shot, &target));
	assert!(!circles_overlap(
		&shot.translated(DVec2::new(100.0, 0.0)).unwrap(),
		&target
	));
}

#[test]
fn ac2_sweep_misses_beside_short_of_and_away_from_target() {
	let shot = circle(0.0, 0.0, 3.0);
	assert_eq!(
		sweep_circle(&shot, DVec2::new(100.0, 0.0), &circle(50.0, 13.1, 10.0)),
		None
	);
	assert_eq!(
		sweep_circle(&shot, DVec2::new(30.0, 0.0), &circle(50.0, 0.0, 10.0)),
		None
	);
	assert_eq!(
		sweep_circle(&shot, DVec2::new(-100.0, 0.0), &circle(50.0, 0.0, 10.0)),
		None
	);
}

#[test]
fn ac3_zero_motion_and_equal_centers_are_finite() {
	let a = circle(5.0, 5.0, 4.0);
	assert_eq!(
		sweep_circle(&a, DVec2::ZERO, &circle(5.0, 5.0, 4.0)),
		Some(0.0)
	);
	assert_eq!(sweep_circle(&a, DVec2::ZERO, &circle(50.0, 5.0, 4.0)), None);
	let degenerate = Capsule::new(DVec2::ONE, DVec2::ONE, 2.0).unwrap();
	assert!(capsule_circle_overlap(&degenerate, &circle(1.0, 4.0, 1.0)));
	let d = point_segment_distance(DVec2::new(3.0, 1.0), DVec2::ONE, DVec2::ONE);
	assert!(d.is_finite() && (d - 2.0).abs() < 1e-12);
}

#[test]
fn rejects_invalid_shapes() {
	assert_eq!(
		Circle::new(DVec2::new(f64::NAN, 0.0), 1.0),
		Err(GeometryError::NonFinite)
	);
	assert_eq!(
		Circle::new(DVec2::ZERO, f64::INFINITY),
		Err(GeometryError::NonFinite)
	);
	assert_eq!(
		Circle::new(DVec2::ZERO, -1.0),
		Err(GeometryError::NegativeRadius)
	);
	assert_eq!(
		Bounds2::new(DVec2::new(1.0, 0.0), DVec2::ZERO),
		Err(GeometryError::InvertedBounds)
	);
}

#[test]
fn arena_clamp_accounts_for_radius() {
	let arena = Bounds2::new(DVec2::new(-600.0, -337.5), DVec2::new(600.0, 337.5)).unwrap();
	assert_eq!(
		arena.clamp_circle_center(DVec2::new(-900.0, 400.0), 14.0),
		DVec2::new(-586.0, 323.5)
	);
	assert_eq!(
		arena.clamp_circle_center(DVec2::new(10.0, 20.0), 14.0),
		DVec2::new(10.0, 20.0)
	);
}

/// Deterministic xorshift so a failing case reproduces from its printed seed.
struct Gen(u64);

impl Gen {
	fn next(&mut self) -> f64 {
		self.0 ^= self.0 << 13;
		self.0 ^= self.0 >> 7;
		self.0 ^= self.0 << 17;
		(self.0 >> 11) as f64 / (1u64 << 53) as f64
	}

	fn range(&mut self, lo: f64, hi: f64) -> f64 {
		lo + (hi - lo) * self.next()
	}
}

#[test]
fn ac4_generated_translation_and_swap_invariants() {
	for seed in [1u64, 7, 42, 0x5eed, 0xdead_beef] {
		let mut g = Gen(seed);
		for case in 0..2000 {
			let a = circle(
				g.range(-600.0, 600.0),
				g.range(-340.0, 340.0),
				g.range(0.0, 60.0),
			);
			let b = circle(
				g.range(-600.0, 600.0),
				g.range(-340.0, 340.0),
				g.range(0.0, 60.0),
			);
			let shift = DVec2::new(g.range(-500.0, 500.0), g.range(-500.0, 500.0));
			let (ta, tb) = (a.translated(shift).unwrap(), b.translated(shift).unwrap());
			let gap = a.center().distance(b.center()) - a.radius() - b.radius();
			// Outside the declared tolerance band the answer must not change.
			if gap.abs() > 1e-6 {
				let base = circles_overlap(&a, &b);
				assert_eq!(
					base,
					circles_overlap(&b, &a),
					"swap seed {seed} case {case}"
				);
				assert_eq!(
					base,
					circles_overlap(&ta, &tb),
					"translate seed {seed} case {case}"
				);
			}
			let motion = DVec2::new(g.range(-40.0, 40.0), g.range(-40.0, 40.0));
			let t1 = sweep_circle(&a, motion, &b);
			let t2 = sweep_circle(&ta, motion, &tb);
			match (t1, t2) {
				(Some(x), Some(y)) => {
					assert!((x - y).abs() < 1e-6, "sweep seed {seed} case {case}")
				}
				(None, None) => {}
				// A grazing contact may flip only inside the tolerance band.
				(x, y) => {
					let t = x.or(y).unwrap();
					let closest = (a.center() + motion * t).distance(b.center());
					assert!(
						(closest - a.radius() - b.radius()).abs() < 1e-6,
						"sweep flip seed {seed} case {case}"
					);
				}
			}
			if let Some(t) = t1 {
				assert!((0.0..=1.0).contains(&t));
			}
		}
	}
}
