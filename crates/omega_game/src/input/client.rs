//! Bevy device sampling for the native client: keyboard, mouse and window focus.

use super::{AimOrigin, Edge, FrameInput, InputBuffer};
use crate::sim::{SimPaused, TickInput};
use bevy::math::DVec2;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowFocused};
use omega_2d::camera::CurrentViewport;

pub struct InputClientPlugin;

impl Plugin for InputClientPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<InputBuffer>()
			.init_resource::<AimOrigin>()
			.add_systems(PreUpdate, (sample_devices, handle_focus).chain())
			.add_systems(FixedPreUpdate, fill_tick_input);
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
