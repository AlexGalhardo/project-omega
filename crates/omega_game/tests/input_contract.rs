//! SP-005 acceptance: input buffering across zero/many tick frames, aim mapping,
//! letterbox rejection, focus pause and modal edge ownership.

use bevy::math::DVec2;
use omega_2d::camera::ViewportMapping;
use omega_2d::geometry::Bounds2;
use omega_game::input::*;
use omega_game::sim::{AimInput, SimPaused};

fn arena() -> Bounds2 {
	Bounds2::new(DVec2::new(-600.0, -337.5), DVec2::new(600.0, 337.5)).unwrap()
}

fn frame(edges: Vec<Edge>) -> FrameInput {
	FrameInput {
		right: true,
		edges,
		..FrameInput::default()
	}
}

#[test]
fn ac1_held_repeats_and_edge_is_consumed_once_across_three_ticks() {
	let mut buffer = InputBuffer::default();
	buffer.begin_frame(frame(vec![Edge::Cancel]));
	let samples: Vec<_> = (0..3).map(|_| buffer.sample_tick(DVec2::ZERO)).collect();
	assert!(samples.iter().all(|s| s.input.movement == DVec2::X));
	assert_eq!(samples.iter().filter(|s| s.pause_requested).count(), 1);
	assert!(samples[0].pause_requested);
	assert_eq!(buffer.pending_len(), 0);
}

#[test]
fn ac2_zero_tick_frame_keeps_the_edge_for_the_next_tick() {
	let mut buffer = InputBuffer::default();
	buffer.begin_frame(frame(vec![Edge::Cancel]));
	// A second render frame with no fixed ticks in between must not lose or double it.
	buffer.begin_frame(frame(vec![]));
	assert_eq!(buffer.pending_len(), 1);
	assert!(buffer.sample_tick(DVec2::ZERO).pause_requested);
	assert!(!buffer.sample_tick(DVec2::ZERO).pause_requested);
}

#[test]
fn ac3_diagonal_has_unit_length_and_opposites_cancel() {
	let mut buffer = InputBuffer::default();
	buffer.begin_frame(FrameInput {
		up: true,
		right: true,
		..FrameInput::default()
	});
	let m = buffer.sample_tick(DVec2::ZERO).input.movement;
	assert!((m.length() - 1.0).abs() < 1e-12);
	buffer.begin_frame(FrameInput {
		left: true,
		right: true,
		..FrameInput::default()
	});
	assert_eq!(buffer.sample_tick(DVec2::ZERO).input.movement, DVec2::ZERO);
}

#[test]
fn ac3_manual_aim_maps_cursor_and_ignores_letterbox() {
	// 1920x1200 window: 16:9 content with bars above and below.
	let mapping = ViewportMapping::new(arena(), 1920.0, 1200.0).unwrap();
	let center = mapping.world_to_screen(DVec2::new(100.0, 0.0));
	let bar = DVec2::new(960.0, 5.0);
	assert!(mapping.screen_to_world(bar).is_none());

	let mut buffer = InputBuffer::default();
	let aim_frame = |cursor| FrameInput {
		manual_aim_held: true,
		cursor_world: cursor,
		..FrameInput::default()
	};
	// Default last direction is +X before any valid cursor.
	buffer.begin_frame(aim_frame(mapping.screen_to_world(bar)));
	assert_eq!(
		buffer.sample_tick(DVec2::ZERO).input.aim,
		AimInput::Manual(DVec2::X)
	);
	// Cursor above the player aims +Y.
	buffer.begin_frame(aim_frame(Some(DVec2::new(0.0, 200.0))));
	assert_eq!(
		buffer.sample_tick(DVec2::ZERO).input.aim,
		AimInput::Manual(DVec2::Y)
	);
	// Letterbox point creates no new direction.
	buffer.begin_frame(aim_frame(mapping.screen_to_world(bar)));
	assert_eq!(
		buffer.sample_tick(DVec2::ZERO).input.aim,
		AimInput::Manual(DVec2::Y)
	);
	// A vector under 1 LU keeps the last direction.
	buffer.begin_frame(aim_frame(Some(DVec2::new(0.5, 0.0))));
	assert_eq!(
		buffer.sample_tick(DVec2::ZERO).input.aim,
		AimInput::Manual(DVec2::Y)
	);
	// The mapped cursor of a real point measures from the player origin.
	buffer.begin_frame(aim_frame(mapping.screen_to_world(center)));
	let AimInput::Manual(dir) = buffer.sample_tick(DVec2::new(0.0, 0.0)).input.aim else {
		panic!("manual aim expected");
	};
	assert!((dir - DVec2::X).length() < 1e-9);
	// Release returns immediately to auto aim.
	buffer.begin_frame(FrameInput::default());
	assert_eq!(buffer.sample_tick(DVec2::ZERO).input.aim, AimInput::Auto);
}

#[test]
fn ac4_focus_loss_pauses_and_resume_discards_stale_edges() {
	let mut buffer = InputBuffer::default();
	let mut paused = SimPaused(false);
	buffer.begin_frame(frame(vec![Edge::Confirm, Edge::Cancel]));
	buffer.focus_lost(&mut paused);
	assert!(paused.0);
	assert_eq!(buffer.pending_len(), 0);
	assert_eq!(buffer.sample_tick(DVec2::ZERO).input.movement, DVec2::ZERO);
	buffer.begin_frame(frame(vec![Edge::Cancel]));
	buffer.resume(&mut paused);
	assert!(!paused.0);
	assert!(!buffer.sample_tick(DVec2::ZERO).pause_requested);
}

#[test]
fn ac4_confirm_and_back_do_not_leak_through_a_modal() {
	let mut buffer = InputBuffer::default();
	// Combat: Confirm is not a gameplay action and nothing reaches the UI.
	buffer.begin_frame(frame(vec![Edge::Confirm]));
	assert!(!buffer.sample_tick(DVec2::ZERO).pause_requested);
	assert_eq!(buffer.take_ui_edges(), UiEdges::default());
	// Modal open: Back is Back, not Pause, and is delivered exactly once.
	buffer.ui_focus = true;
	buffer.begin_frame(frame(vec![Edge::Cancel]));
	assert!(!buffer.sample_tick(DVec2::ZERO).pause_requested);
	assert_eq!(
		buffer.take_ui_edges(),
		UiEdges {
			confirm: true,
			back: true,
			open_energy: false
		}
	);
	assert_eq!(buffer.take_ui_edges(), UiEdges::default());
}
