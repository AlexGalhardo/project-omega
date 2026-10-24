//! SP-012-AC4 / AA-03 / AA-04: cues copy authoritative shape and anchor facts, locked
//! and following anchors behave differently, cancellation is immediate and reduced
//! effects keep every essential cue.

use bevy::math::DVec2;
use omega_2d::visuals::cues::*;

fn ground(radius: f64, total: u32) -> PhaseCue {
	PhaseCue {
		shape: CueShape::Circle { radius },
		anchor: CueAnchor::Locked(DVec2::new(100.0, 40.0)),
		total_ticks: total,
		elapsed_ticks: 0,
	}
}

fn beam() -> PhaseCue {
	PhaseCue {
		shape: CueShape::Capsule {
			reach: 600.0,
			radius: 6.0,
		},
		anchor: CueAnchor::FollowOrigin {
			direction: DVec2::X,
		},
		total_ticks: 72,
		elapsed_ticks: 0,
	}
}

#[test]
fn locked_ground_cue_stays_put_when_the_attacker_is_displaced() {
	let cue = ground(50.0, 54);
	let before = cue.view(DVec2::new(0.0, 0.0), EffectsLevel::Full);
	let after = cue.view(DVec2::new(-80.0, 30.0), EffectsLevel::Full);
	assert_eq!(before.origin, DVec2::new(100.0, 40.0));
	assert_eq!(
		after.origin, before.origin,
		"DA-17: strike stays on the locked center"
	);
	assert_eq!(after.shape, CueShape::Circle { radius: 50.0 });
	assert_eq!(after.direction, None);
}

#[test]
fn beam_cue_follows_the_displaced_origin_with_the_locked_direction() {
	let cue = beam();
	let moved = cue.view(DVec2::new(-30.0, 12.0), EffectsLevel::Full);
	assert_eq!(moved.origin, DVec2::new(-30.0, 12.0));
	assert_eq!(moved.direction, Some(DVec2::X));
	assert_eq!(
		moved.shape,
		CueShape::Capsule {
			reach: 600.0,
			radius: 6.0
		}
	);
}

#[test]
fn progress_equals_elapsed_over_total_and_never_overruns() {
	let mut set = CueSet::default();
	set.start(7, ground(90.0, 72));
	for _ in 0..36 {
		set.tick();
	}
	assert_eq!(set.get(7).unwrap().progress(), 0.5);
	for _ in 0..200 {
		set.tick();
	}
	assert_eq!(set.get(7).unwrap().progress(), 1.0);
	assert_eq!(set.get(7).unwrap().elapsed_ticks, 72);
}

#[test]
fn zero_length_cue_is_complete() {
	assert_eq!(ground(10.0, 0).progress(), 1.0);
}

#[test]
fn cancellation_and_emission_remove_the_cue_at_once_and_only_that_cue() {
	let mut set = CueSet::default();
	set.start(1, ground(50.0, 54));
	set.start(2, beam());
	assert!(set.end(1));
	assert!(!set.end(1), "ending twice is a no-op");
	assert_eq!(set.len(), 1);
	assert!(set.get(2).is_some());
	let ids: Vec<u64> = set.iter().map(|(id, _)| id).collect();
	assert_eq!(ids, vec![2]);
}

#[test]
fn reduced_effects_drop_decorations_but_keep_shape_anchor_and_progress() {
	let mut cue = beam();
	cue.elapsed_ticks = 36;
	let full = cue.view(DVec2::ZERO, EffectsLevel::Full);
	let reduced = cue.view(DVec2::ZERO, EffectsLevel::Reduced);
	assert!(full.decorations);
	assert!(!reduced.decorations);
	assert_eq!(
		(reduced.origin, reduced.direction),
		(full.origin, full.direction)
	);
	assert_eq!(reduced.shape, full.shape);
	assert_eq!(reduced.progress, full.progress);
}
