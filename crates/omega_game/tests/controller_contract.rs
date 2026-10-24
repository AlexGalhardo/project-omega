//! SP-005 controller model: dead zones, analog movement and aim, edges, source switching,
//! navigation repeat, remapping conflicts and UI focus. Native device checks are separate.

use bevy::math::DVec2;
use omega_game::input::bindings::*;
use omega_game::input::gamepad::*;
use omega_game::input::{Edge, FrameInput, InputBuffer};
use omega_game::sim::{AimInput, SimPaused};
use omega_game::ui::*;

fn near(a: f64, b: f64) -> bool {
	(a - b).abs() < 1e-9
}

#[test]
fn radial_dead_zone_rescales_and_caps() {
	assert_eq!(
		radial_deadzone(DVec2::new(0.19, 0.0), MOVE_DEADZONE),
		DVec2::ZERO
	);
	assert_eq!(
		radial_deadzone(DVec2::new(0.2, 0.0), MOVE_DEADZONE),
		DVec2::ZERO
	);
	let mid = radial_deadzone(DVec2::new(0.6, 0.0), MOVE_DEADZONE);
	assert!(near(mid.x, 0.5) && near(mid.y, 0.0));
	let diagonal = radial_deadzone(DVec2::new(1.0, 1.0), MOVE_DEADZONE);
	assert!(near(diagonal.length(), 1.0), "capped at one");
	assert!(near(diagonal.x, diagonal.y), "direction preserved");
}

fn buffer_with(pad: PadFrame) -> InputBuffer {
	let mut buffer = InputBuffer::default();
	buffer.begin_frame(FrameInput::default());
	buffer.set_pad(&pad);
	buffer
}

#[test]
fn analog_movement_scales_with_the_stick_and_digital_wins_when_pressed() {
	let mut buffer = buffer_with(PadFrame {
		left_stick: DVec2::new(0.6, 0.0),
		..PadFrame::default()
	});
	let input = buffer.sample_tick(DVec2::ZERO).input;
	assert!(near(input.movement.x, 0.5) && near(input.movement.y, 0.0));
	buffer.begin_frame(FrameInput {
		right: true,
		..FrameInput::default()
	});
	let digital = buffer.sample_tick(DVec2::ZERO).input;
	assert_eq!(digital.movement, DVec2::X, "a pressed key is a full axis");
}

#[test]
fn aim_stick_aims_normalized_and_centering_returns_to_auto() {
	let mut buffer = buffer_with(PadFrame {
		right_stick: DVec2::new(0.0, 0.4),
		..PadFrame::default()
	});
	match buffer.sample_tick(DVec2::ZERO).input.aim {
		AimInput::Manual(d) => assert!(near(d.x, 0.0) && near(d.y, 1.0), "{d:?}"),
		other => panic!("{other:?}"),
	}
	let weak = PadFrame {
		right_stick: DVec2::new(0.0, 0.2),
		..PadFrame::default()
	};
	buffer.begin_frame(FrameInput::default());
	buffer.set_pad(&weak);
	assert_eq!(
		buffer.sample_tick(DVec2::ZERO).input.aim,
		AimInput::Auto,
		"inside the 0.25 dead zone"
	);
}

#[test]
fn buttons_become_edges_once_and_back_pauses() {
	let pad = PadFrame {
		south_pressed: true,
		shoulder_right_pressed: true,
		..PadFrame::default()
	};
	assert_eq!(pad.edges(), vec![Edge::Confirm, Edge::OpenEnergy]);
	let back = PadFrame {
		back_pressed: true,
		..PadFrame::default()
	};
	let mut buffer = buffer_with(back);
	assert!(buffer.sample_tick(DVec2::ZERO).pause_requested);
	assert!(
		!buffer.sample_tick(DVec2::ZERO).pause_requested,
		"consumed once"
	);
}

#[test]
fn ac4_controller_loss_pauses_before_the_next_tick_and_drops_edges() {
	let mut buffer = buffer_with(PadFrame {
		south_pressed: true,
		..PadFrame::default()
	});
	let mut paused = SimPaused(false);
	buffer.focus_lost(&mut paused);
	assert!(paused.0);
	assert_eq!(
		buffer.pending_len(),
		0,
		"confirm cannot leak after the loss"
	);
}

#[test]
fn source_switches_only_on_deliberate_actions() {
	let source = InputSource::KeyboardMouse;
	assert_eq!(
		source.after(false, false),
		InputSource::KeyboardMouse,
		"passive mouse movement"
	);
	assert_eq!(source.after(false, true), InputSource::Controller);
	assert_eq!(
		InputSource::Controller.after(false, false),
		InputSource::Controller
	);
	assert_eq!(
		InputSource::Controller.after(true, false),
		InputSource::KeyboardMouse
	);
	let idle = PadFrame {
		left_stick: DVec2::new(0.1, 0.1),
		..PadFrame::default()
	};
	assert!(
		!idle.is_active(),
		"stick drift inside the dead zone is not an action"
	);
}

#[test]
fn navigation_repeats_after_035_then_every_012_seconds() {
	let mut repeat = NavRepeat::default();
	let down = Some(NavDirection::Down);
	let mut fired = Vec::new();
	for step in 0..=60 {
		let t = f64::from(step) * 0.01;
		if repeat.update(down, t).is_some() {
			fired.push((t * 100.0).round() as i64);
		}
	}
	assert_eq!(fired, vec![0, 35, 47, 59]);
	assert_eq!(repeat.update(None, 0.7), None);
	assert_eq!(repeat.update(down, 0.71), down, "a new press fires at once");
	assert_eq!(
		repeat.update(Some(NavDirection::Up), 0.72),
		Some(NavDirection::Up)
	);
}

#[test]
fn rebinding_detects_conflicts_and_keeps_back_reachable() {
	let mut b = Bindings::default();
	assert_eq!(
		b.rebind(Action::MoveUp, "KeyS"),
		Err(BindError::Conflict {
			with: Action::MoveDown
		})
	);
	assert_eq!(
		b.inputs(Action::MoveUp),
		["KeyW", "ArrowUp"],
		"unchanged on conflict"
	);
	b.rebind(Action::MoveUp, "KeyI").unwrap();
	assert_eq!(b.action_for("KeyI"), Some(Action::MoveUp));
	b.unbind(Action::Back, "Escape").unwrap();
	b.unbind(Action::Back, "GamepadEast").unwrap();
	assert_eq!(
		b.unbind(Action::Back, "GamepadMenu"),
		Err(BindError::BackUnreachable)
	);
	b.restore_defaults();
	assert_eq!(b, Bindings::default());
}

#[test]
fn bindings_round_trip_through_settings_and_report_unknown_actions() {
	let mut b = Bindings::default();
	b.rebind(Action::OpenEnergy, "KeyQ").unwrap();
	let stored = b.to_settings();
	assert_eq!(stored.len(), 1, "only differences are stored");
	let (loaded, unknown) = Bindings::from_settings(&stored);
	assert_eq!(loaded, b);
	assert!(unknown.is_empty());
	let mut odd = stored.clone();
	odd.insert("retired_action".into(), "KeyZ".into());
	odd.insert("back".into(), String::new());
	let (loaded, unknown) = Bindings::from_settings(&odd);
	assert_eq!(unknown, vec!["retired_action".to_string()]);
	assert!(
		!loaded.inputs(Action::Back).is_empty(),
		"Back is never left unbound"
	);
}

fn view() -> ScreenView {
	let b = |label: &str, action, enabled| Button {
		label: label.into(),
		action,
		enabled,
		same_row_as_next: false,
	};
	ScreenView {
		title: "Shop".into(),
		lines: Vec::new(),
		buttons: vec![
			b("off", UiAction::Refresh, false),
			b("buy", UiAction::Buy(0), true),
			b("start", UiAction::Start, true),
			b("cancel", UiAction::CancelReplace, true),
		],
	}
}

#[test]
fn focus_skips_disabled_buttons_wraps_and_activates_only_enabled() {
	let v = view();
	let mut focus = UiFocus::default();
	focus.clamp(&v);
	assert_eq!(focus.index, 1, "starts on the first enabled button");
	focus.step(&v, true);
	assert_eq!(focus.activate(&v), Some(UiAction::Start));
	focus.step(&v, true);
	focus.step(&v, true);
	assert_eq!(
		focus.index, 1,
		"wraps past the end and skips the disabled one"
	);
	focus.step(&v, false);
	assert_eq!(focus.index, 3, "wraps backwards");
	focus.index = 0;
	assert_eq!(
		focus.activate(&v),
		None,
		"a disabled button never activates"
	);
}

#[test]
fn back_never_buys_or_starts() {
	assert_eq!(back_action(&view()), Some(UiAction::CancelReplace));
	let mut shop = view();
	shop.buttons.pop();
	assert_eq!(back_action(&shop), None);
}
