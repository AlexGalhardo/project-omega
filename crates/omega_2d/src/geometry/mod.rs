//! ENG-GEOM-001: pure circle, capsule, sweep and bounds queries in logical units (LU).
//!
//! Every query is a pure function over validated shapes. Tangency counts as overlap.
//! Comparisons use [`EPSILON`] so values that are tangent on paper stay tangent after
//! translation; the tolerance never inflates a hurtbox beyond that rounding slack.

use bevy::math::DVec2;

/// Absolute LU tolerance for overlap and time-of-impact comparisons.
pub const EPSILON: f64 = 1e-9;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeometryError {
	NonFinite,
	NegativeRadius,
	InvertedBounds,
}

impl std::fmt::Display for GeometryError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(match self {
			Self::NonFinite => "coordinate or radius is not finite",
			Self::NegativeRadius => "radius is negative",
			Self::InvertedBounds => "bounds minimum exceeds maximum",
		})
	}
}

impl std::error::Error for GeometryError {}

fn finite(v: DVec2) -> Result<DVec2, GeometryError> {
	if v.is_finite() {
		Ok(v)
	} else {
		Err(GeometryError::NonFinite)
	}
}

fn radius(r: f64) -> Result<f64, GeometryError> {
	if !r.is_finite() {
		Err(GeometryError::NonFinite)
	} else if r < 0.0 {
		Err(GeometryError::NegativeRadius)
	} else {
		Ok(r)
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
	center: DVec2,
	radius: f64,
}

impl Circle {
	pub fn new(center: DVec2, radius_lu: f64) -> Result<Self, GeometryError> {
		Ok(Self {
			center: finite(center)?,
			radius: radius(radius_lu)?,
		})
	}

	pub fn center(&self) -> DVec2 {
		self.center
	}

	pub fn radius(&self) -> f64 {
		self.radius
	}

	pub fn translated(&self, offset: DVec2) -> Result<Self, GeometryError> {
		Self::new(self.center + offset, self.radius)
	}
}

/// Segment `start..end` swept by `radius`. A zero-length segment is a circle.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Capsule {
	start: DVec2,
	end: DVec2,
	radius: f64,
}

impl Capsule {
	pub fn new(start: DVec2, end: DVec2, radius_lu: f64) -> Result<Self, GeometryError> {
		Ok(Self {
			start: finite(start)?,
			end: finite(end)?,
			radius: radius(radius_lu)?,
		})
	}

	pub fn start(&self) -> DVec2 {
		self.start
	}

	pub fn end(&self) -> DVec2 {
		self.end
	}

	pub fn radius(&self) -> f64 {
		self.radius
	}
}

/// Axis-aligned rectangle with inclusive edges.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bounds2 {
	min: DVec2,
	max: DVec2,
}

impl Bounds2 {
	pub fn new(min: DVec2, max: DVec2) -> Result<Self, GeometryError> {
		let (min, max) = (finite(min)?, finite(max)?);
		if min.x > max.x || min.y > max.y {
			return Err(GeometryError::InvertedBounds);
		}
		Ok(Self { min, max })
	}

	pub fn min(&self) -> DVec2 {
		self.min
	}

	pub fn max(&self) -> DVec2 {
		self.max
	}

	pub fn size(&self) -> DVec2 {
		self.max - self.min
	}

	pub fn contains(&self, point: DVec2) -> bool {
		point.cmpge(self.min).all() && point.cmple(self.max).all()
	}

	/// Clamps a circle center so the whole circle stays inside the bounds. When the
	/// circle is wider than the bounds on an axis, the center goes to that axis midpoint.
	pub fn clamp_circle_center(&self, center: DVec2, radius_lu: f64) -> DVec2 {
		let axis = |c: f64, lo: f64, hi: f64| {
			let (lo, hi) = (lo + radius_lu, hi - radius_lu);
			if lo > hi {
				(lo + hi) * 0.5
			} else {
				c.clamp(lo, hi)
			}
		};
		DVec2::new(
			axis(center.x, self.min.x, self.max.x),
			axis(center.y, self.min.y, self.max.y),
		)
	}
}

pub fn circles_overlap(a: &Circle, b: &Circle) -> bool {
	a.center.distance(b.center) <= a.radius + b.radius + EPSILON
}

/// Distance from `point` to the closed segment `start..end`; degenerate segments are points.
pub fn point_segment_distance(point: DVec2, start: DVec2, end: DVec2) -> f64 {
	let seg = end - start;
	let len2 = seg.length_squared();
	if len2 == 0.0 {
		return point.distance(start);
	}
	let t = ((point - start).dot(seg) / len2).clamp(0.0, 1.0);
	point.distance(start + seg * t)
}

pub fn capsule_circle_overlap(capsule: &Capsule, circle: &Circle) -> bool {
	point_segment_distance(circle.center, capsule.start, capsule.end)
		<= capsule.radius + circle.radius + EPSILON
}

/// First time of impact `t` in `[0, 1]` when `moving` travels by `motion` toward a
/// stationary `target`. Returns `Some(0.0)` when they already overlap (including
/// tangency and equal centers) and `None` when they never touch during the motion.
/// Zero motion reduces to a static overlap test.
pub fn sweep_circle(moving: &Circle, motion: DVec2, target: &Circle) -> Option<f64> {
	if circles_overlap(moving, target) {
		return Some(0.0);
	}
	let reach = moving.radius + target.radius;
	let rel = moving.center - target.center;
	let a = motion.length_squared();
	if a == 0.0 {
		return None;
	}
	let b = 2.0 * rel.dot(motion);
	let c = rel.length_squared() - reach * reach;
	let disc = b * b - 4.0 * a * c;
	if disc < 0.0 {
		// Grazing within tolerance: closest approach equals reach.
		let t = (-b / (2.0 * a)).clamp(0.0, 1.0);
		let closest = (rel + motion * t).length();
		return (closest <= reach + EPSILON).then_some(t);
	}
	let t = (-b - disc.sqrt()) / (2.0 * a);
	(0.0..=1.0).contains(&t).then_some(t)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn clamp_keeps_radius_inside() {
		let b = Bounds2::new(DVec2::new(-600.0, -337.5), DVec2::new(600.0, 337.5)).unwrap();
		let c = b.clamp_circle_center(DVec2::new(1000.0, -1000.0), 14.0);
		assert_eq!(c, DVec2::new(586.0, -323.5));
	}
}
