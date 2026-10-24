//! Bevy device sampling for the native client: keyboard, mouse and window focus.

use super::bindings::{Action, Bindings};
use super::gamepad::{InputSource, NavDirection, PadFrame};
use super::{AimOrigin, Edge, FrameInput, InputBuffer};
use crate::profile::Settings;
use crate::sim::{SimPaused, TickInput};
use bevy::input::gamepad::{Gamepad, GamepadButton, GamepadConnection, GamepadConnectionEvent};
use bevy::math::DVec2;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowFocused};
use omega_2d::camera::CurrentViewport;

pub struct InputClientPlugin;

impl Plugin for InputClientPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<InputBuffer>()
			.init_resource::<AimOrigin>()
			.init_resource::<ActiveSource>()
			.init_resource::<PadNav>()
			.add_systems(
				PreUpdate,
				(
					sample_devices,
					sample_gamepad,
					handle_focus,
					handle_disconnect,
				)
					.chain(),
			)
			.add_systems(FixedPreUpdate, fill_tick_input);
	}
}

/// Which device family the player last used deliberately.
#[derive(Resource, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ActiveSource(pub InputSource);

/// D-pad or left-stick navigation request of the current frame, for UI focus.
#[derive(Resource, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PadNav(pub Option<NavDirection>);

fn sample_gamepad(
	pads: Query<&Gamepad>,
	keys: Res<ButtonInput<KeyCode>>,
	mouse: Res<ButtonInput<MouseButton>>,
	mut active: ResMut<ActiveSource>,
	mut nav: ResMut<PadNav>,
	mut buffer: ResMut<InputBuffer>,
) {
	let pad = pads.iter().next().map_or_else(PadFrame::default, |g| {
		let left = g.left_stick();
		let right = g.right_stick();
		let dpad = g.dpad();
		PadFrame {
			left_stick: DVec2::new(f64::from(left.x), f64::from(left.y)),
			right_stick: DVec2::new(f64::from(right.x), f64::from(right.y)),
			dpad_up: dpad.y > 0.5,
			dpad_down: dpad.y < -0.5,
			dpad_left: dpad.x < -0.5,
			dpad_right: dpad.x > 0.5,
			south_pressed: g.just_pressed(GamepadButton::South),
			back_pressed: g.just_pressed(GamepadButton::East)
				|| g.just_pressed(GamepadButton::Start),
			shoulder_right_pressed: g.just_pressed(GamepadButton::RightTrigger),
		}
	});
	let keyboard_action =
		keys.get_just_pressed().next().is_some() || mouse.get_just_pressed().next().is_some();
	active.0 = active.0.after(keyboard_action, pad.is_active());
	nav.0 = if pad.dpad_up || pad.left_stick.y > 0.6 {
		Some(NavDirection::Up)
	} else if pad.dpad_down || pad.left_stick.y < -0.6 {
		Some(NavDirection::Down)
	} else if pad.dpad_left || pad.left_stick.x < -0.6 {
		Some(NavDirection::Left)
	} else if pad.dpad_right || pad.left_stick.x > 0.6 {
		Some(NavDirection::Right)
	} else {
		None
	};
	buffer.set_pad(&pad);
}

/// An active controller that disconnects pauses before the next tick (DA-16, SP-005-AC4).
fn handle_disconnect(
	mut events: MessageReader<GamepadConnectionEvent>,
	active: Res<ActiveSource>,
	mut buffer: ResMut<InputBuffer>,
	mut paused: ResMut<SimPaused>,
) {
	for event in events.read() {
		if matches!(event.connection, GamepadConnection::Disconnected)
			&& active.0 == InputSource::Controller
		{
			buffer.focus_lost(&mut paused);
		}
	}
}

/// Keys a binding can name; the name is the `KeyCode` variant, as stored in settings.
const KEY_NAMES: &[(&str, KeyCode)] = &[
	("KeyA", KeyCode::KeyA),
	("KeyB", KeyCode::KeyB),
	("KeyC", KeyCode::KeyC),
	("KeyD", KeyCode::KeyD),
	("KeyE", KeyCode::KeyE),
	("KeyF", KeyCode::KeyF),
	("KeyG", KeyCode::KeyG),
	("KeyH", KeyCode::KeyH),
	("KeyI", KeyCode::KeyI),
	("KeyJ", KeyCode::KeyJ),
	("KeyK", KeyCode::KeyK),
	("KeyL", KeyCode::KeyL),
	("KeyM", KeyCode::KeyM),
	("KeyN", KeyCode::KeyN),
	("KeyO", KeyCode::KeyO),
	("KeyP", KeyCode::KeyP),
	("KeyQ", KeyCode::KeyQ),
	("KeyR", KeyCode::KeyR),
	("KeyS", KeyCode::KeyS),
	("KeyT", KeyCode::KeyT),
	("KeyU", KeyCode::KeyU),
	("KeyV", KeyCode::KeyV),
	("KeyW", KeyCode::KeyW),
	("KeyX", KeyCode::KeyX),
	("KeyY", KeyCode::KeyY),
	("KeyZ", KeyCode::KeyZ),
	("Digit0", KeyCode::Digit0),
	("Digit1", KeyCode::Digit1),
	("Digit2", KeyCode::Digit2),
	("Digit3", KeyCode::Digit3),
	("Digit4", KeyCode::Digit4),
	("Digit5", KeyCode::Digit5),
	("Digit6", KeyCode::Digit6),
	("Digit7", KeyCode::Digit7),
	("Digit8", KeyCode::Digit8),
	("Digit9", KeyCode::Digit9),
	("ArrowUp", KeyCode::ArrowUp),
	("ArrowDown", KeyCode::ArrowDown),
	("ArrowLeft", KeyCode::ArrowLeft),
	("ArrowRight", KeyCode::ArrowRight),
	("Enter", KeyCode::Enter),
	("Space", KeyCode::Space),
	("Escape", KeyCode::Escape),
	("Tab", KeyCode::Tab),
];

fn key_from_name(name: &str) -> Option<KeyCode> {
	KEY_NAMES.iter().find(|(n, _)| *n == name).map(|(_, k)| *k)
}

/// True when any key bound to `action` satisfies `test`.
fn bound(bindings: &Bindings, action: Action, test: impl Fn(KeyCode) -> bool) -> bool {
	bindings
		.inputs(action)
		.iter()
		.filter_map(|n| key_from_name(n))
		.any(test)
}

fn sample_devices(
	keys: Res<ButtonInput<KeyCode>>,
	mouse: Res<ButtonInput<MouseButton>>,
	window: Single<&Window, With<PrimaryWindow>>,
	viewport: Option<Res<CurrentViewport>>,
	settings: Option<Res<Settings>>,
	mut buffer: ResMut<InputBuffer>,
) {
	let (bindings, _) = settings.as_ref().map_or_else(
		|| (Bindings::default(), Vec::new()),
		|s| Bindings::from_settings(&s.bindings),
	);
	let held = |action: Action| bound(&bindings, action, |k| keys.pressed(k));
	let pressed = |action: Action| bound(&bindings, action, |k| keys.just_pressed(k));
	let cursor_world = viewport.and_then(|v| {
		let cursor = window.physical_cursor_position()?;
		v.0.screen_to_world(DVec2::new(f64::from(cursor.x), f64::from(cursor.y)))
	});
	let mut edges = Vec::new();
	if pressed(Action::Back) {
		edges.push(Edge::Cancel);
	}
	if pressed(Action::Confirm) {
		edges.push(Edge::Confirm);
	}
	if pressed(Action::OpenEnergy) {
		edges.push(Edge::OpenEnergy);
	}
	let mouse_aim = bindings
		.inputs(Action::ManualAim)
		.iter()
		.any(|n| n == "MouseRight")
		&& mouse.pressed(MouseButton::Right);
	buffer.begin_frame(FrameInput {
		up: held(Action::MoveUp),
		down: held(Action::MoveDown),
		left: held(Action::MoveLeft),
		right: held(Action::MoveRight),
		manual_aim_held: mouse_aim || held(Action::ManualAim),
		cursor_world,
		edges,
	});
}

fn handle_focus(
	mut focus: MessageReader<WindowFocused>,
	mut buffer: ResMut<InputBuffer>,
	mut paused: ResMut<SimPaused>,
) {
	if focus.read().any(|f| !f.focused) {
		buffer.focus_lost(&mut paused);
	}
}

fn fill_tick_input(
	mut buffer: ResMut<InputBuffer>,
	origin: Res<AimOrigin>,
	mut paused: ResMut<SimPaused>,
	mut tick_input: ResMut<TickInput>,
) {
	let sample = buffer.sample_tick(origin.0);
	*tick_input = sample.input;
	if sample.pause_requested {
		paused.0 = true;
	}
}
