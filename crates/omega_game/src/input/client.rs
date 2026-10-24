//! Bevy device sampling for the native client: keyboard, mouse and window focus.

use super::gamepad::{InputSource, NavDirection, PadFrame};
use super::{AimOrigin, Edge, FrameInput, InputBuffer};
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

fn sample_devices(
	keys: Res<ButtonInput<KeyCode>>,
	mouse: Res<ButtonInput<MouseButton>>,
	window: Single<&Window, With<PrimaryWindow>>,
	viewport: Option<Res<CurrentViewport>>,
	mut buffer: ResMut<InputBuffer>,
) {
	let held = |codes: &[KeyCode]| codes.iter().any(|&k| keys.pressed(k));
	let cursor_world = viewport.and_then(|v| {
		let cursor = window.physical_cursor_position()?;
		v.0.screen_to_world(DVec2::new(f64::from(cursor.x), f64::from(cursor.y)))
	});
	let mut edges = Vec::new();
	if keys.just_pressed(KeyCode::Escape) {
		edges.push(Edge::Cancel);
	}
	if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::Space) {
		edges.push(Edge::Confirm);
	}
	if keys.just_pressed(KeyCode::KeyE) {
		edges.push(Edge::OpenEnergy);
	}
	buffer.begin_frame(FrameInput {
		up: held(&[KeyCode::KeyW, KeyCode::ArrowUp]),
		down: held(&[KeyCode::KeyS, KeyCode::ArrowDown]),
		left: held(&[KeyCode::KeyA, KeyCode::ArrowLeft]),
		right: held(&[KeyCode::KeyD, KeyCode::ArrowRight]),
		manual_aim_held: mouse.pressed(MouseButton::Right),
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
