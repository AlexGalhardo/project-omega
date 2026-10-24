//! Bevy UI widgets for the HUD, shop, pause, abandon and result screens (SP-014).
//! Widgets only render `ScreenView`/`HudView` data and queue typed `UiAction` requests.

use super::menus::{QuitRequested, RebindCapture, apply_rebind};
use super::{
	ScreenView, UiAction, UiFocus, UiMode, apply_action, back_action, edge_action, hints, hud_text,
	hud_view, screen_view, take_screen_edges,
};
use crate::encounters::WaveRuntime;
use crate::input::gamepad::{InputSource, NavDirection, NavRepeat};
use crate::input::{ActiveSource, InputBuffer, PadNav};
use crate::profile::Settings;
use crate::sim::run::{Run, RunState};
use crate::sim::{SimPaused, SimTick};
use bevy::prelude::*;
use bevy::ui::InteractionDisabled;
use bevy::ui_widgets::{Activate, Button};

const TEXT_COLOR: Color = Color::srgb_u8(0xe8, 0xee, 0xf4);
const MUTED_COLOR: Color = Color::srgb_u8(0x8a, 0x97, 0xa6);
const PANEL_COLOR: Color = Color::srgba(0.05, 0.08, 0.11, 0.92);
const BUTTON_COLOR: Color = Color::srgb_u8(0x2a, 0x3a, 0x4c);
const BUTTON_OFF_COLOR: Color = Color::srgb_u8(0x1a, 0x22, 0x2c);

#[derive(Component)]
struct HudText;

#[derive(Component)]
struct ScreenRoot;

#[derive(Component)]
struct MessageText;

#[derive(Resource, Default)]
struct ScreenWasOpen(bool);

#[derive(Resource, Default)]
struct ActionQueue(Vec<UiAction>);

#[derive(Resource, Default)]
pub struct UiMessage(pub String);

/// The panel and focus index currently built, to rebuild only on change.
#[derive(Resource, Default)]
struct LastScreen(Option<ScreenView>, Option<usize>);

const FOCUS_COLOR: Color = Color::srgb_u8(0x3f, 0x6e, 0x9a);

pub struct UiClientPlugin;

impl Plugin for UiClientPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<ScreenWasOpen>()
			.init_resource::<ActionQueue>()
			.init_resource::<UiMessage>()
			.init_resource::<UiMode>()
			.init_resource::<LastScreen>()
			.init_resource::<UiFocus>()
			.add_systems(Startup, spawn_roots)
			.add_systems(
				Update,
				(
					rebind_capture,
					quit_when_requested,
					hint_trigger,
					apply_ui_scale,
					keyboard_actions,
					run_actions,
					refresh_screen,
					refresh_hud,
				)
					.chain(),
			);
	}
}

/// Opens the first-run note that matches the screen, pausing combat until it is dismissed.
fn hint_trigger(
	settings: Option<Res<Settings>>,
	run: Option<Res<Run>>,
	runtime: Option<Res<WaveRuntime>>,
	tick: Res<SimTick>,
	mut mode: ResMut<UiMode>,
	mut paused: ResMut<SimPaused>,
) {
	let (Some(settings), Some(run)) = (settings, run) else {
		return;
	};
	if *mode != UiMode::Normal {
		return;
	}
	let first_wave = run.wave == 1
		&& runtime
			.as_ref()
			.is_some_and(|w| tick.0.saturating_sub(w.tick0) <= 2);
	if let Some(hint) = hints::pending(&settings, run.state, first_wave) {
		*mode = UiMode::Hint(hint);
		paused.0 = true;
	}
}

fn apply_ui_scale(settings: Option<Res<Settings>>, mut scale: ResMut<UiScale>) {
	if let Some(settings) = settings
		&& settings.is_changed()
	{
		scale.0 = hints::ui_scale(&settings);
	}
}

/// While a rebind is pending the next key press becomes the binding; Escape cancels.
fn rebind_capture(world: &mut World) {
	let Some(capture) = world.get_resource::<RebindCapture>().copied() else {
		return;
	};
	let key = world
		.resource::<ButtonInput<KeyCode>>()
		.get_just_pressed()
		.next()
		.copied();
	let Some(key) = key else {
		return;
	};
	let message = if key == KeyCode::Escape {
		world.remove_resource::<RebindCapture>();
		"Rebind cancelled.".to_string()
	} else {
		apply_rebind(world, capture.0, &format!("{key:?}"))
	};
	world.resource_mut::<UiMessage>().0 = message;
}

fn quit_when_requested(requested: Option<Res<QuitRequested>>, mut exit: MessageWriter<AppExit>) {
	if requested.is_some() {
		exit.write(AppExit::Success);
	}
}

fn text_node(text: impl Into<String>, size: f32, color: Color) -> impl Bundle {
	(
		Text::new(text),
		TextFont::from_font_size(size),
		TextColor(color),
	)
}

fn spawn_roots(mut commands: Commands) {
	commands.spawn((
		HudText,
		text_node("", 16.0, TEXT_COLOR),
		Node {
			position_type: PositionType::Absolute,
			left: px(12),
			top: px(8),
			..default()
		},
	));
	commands.spawn((
		MessageText,
		text_node("", 16.0, Color::srgb_u8(0xf2, 0xc9, 0x4c)),
		Node {
			position_type: PositionType::Absolute,
			left: px(12),
			bottom: px(8),
			..default()
		},
	));
	commands.spawn((
		ScreenRoot,
		Node {
			position_type: PositionType::Absolute,
			left: percent(3),
			right: percent(3),
			top: percent(3),
			flex_direction: FlexDirection::Row,
			column_gap: px(16),
			padding: UiRect::all(px(12)),
			display: Display::None,
			..default()
		},
		BackgroundColor(PANEL_COLOR),
	));
}

#[allow(clippy::too_many_arguments)]
fn keyboard_actions(
	keys: Res<ButtonInput<KeyCode>>,
	mode: Res<UiMode>,
	paused: Res<SimPaused>,
	run: Option<Res<Run>>,
	mut buffer: ResMut<InputBuffer>,
	mut queue: ResMut<ActionQueue>,
	active: Res<ActiveSource>,
	nav: Res<PadNav>,
	time: Res<Time<Real>>,
	last: Res<LastScreen>,
	mut focus: ResMut<UiFocus>,
	mut repeat: Local<NavRepeat>,
	mut was_open: ResMut<ScreenWasOpen>,
	rebind: Option<Res<RebindCapture>>,
) {
	if rebind.is_some() {
		buffer.take_ui_edges();
		return;
	}
	let state = run.as_ref().map(|r| r.state);
	let menu_like = state.is_none() || matches!(*mode, UiMode::Hint(_));
	let screen_open = state != Some(RunState::Wave) || paused.0;
	let edges = take_screen_edges(&mut buffer, &mut was_open.0, screen_open);
	if !screen_open {
		return;
	}
	let controller = active.0 == InputSource::Controller;
	if (controller || menu_like)
		&& let Some(view) = &last.0
	{
		focus.clamp(view);
		let dir = if controller {
			repeat.update(nav.0, time.elapsed_secs_f64())
		} else if keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::KeyW) {
			Some(NavDirection::Up)
		} else if keys.just_pressed(KeyCode::ArrowDown)
			|| keys.just_pressed(KeyCode::KeyS)
			|| keys.just_pressed(KeyCode::Tab)
		{
			Some(NavDirection::Down)
		} else {
			None
		};
		match dir {
			Some(NavDirection::Up | NavDirection::Left) => focus.step(view, false),
			Some(NavDirection::Down | NavDirection::Right) => focus.step(view, true),
			None => {}
		}
		if edges.confirm
			&& let Some(action) = focus.activate(view)
		{
			queue.0.push(action);
		}
		if edges.back
			&& let Some(action) = back_action(view)
		{
			queue.0.push(action);
		}
		return;
	}
	let Some(run) = run else {
		return;
	};
	let digit = [
		KeyCode::Digit1,
		KeyCode::Digit2,
		KeyCode::Digit3,
		KeyCode::Digit4,
	]
	.iter()
	.position(|&k| keys.just_pressed(k));
	queue.0.extend(edge_action(run.state, *mode, edges));
	match (run.state, *mode) {
		(_, UiMode::ConfirmAbandon) => {}
		(RunState::Wave, _) => {
			if keys.just_pressed(KeyCode::KeyX) {
				queue.0.push(UiAction::AskAbandon);
			}
		}
		(RunState::Shop, UiMode::Replace(_)) => {
			if let Some(slot) = digit {
				queue.0.push(UiAction::ReplaceSlot(slot));
			}
		}
		(RunState::Shop, _) => {
			if let Some(offer) = digit {
				queue.0.push(UiAction::Buy(offer));
			}
			if keys.just_pressed(KeyCode::KeyF) {
				queue.0.push(UiAction::Refresh);
			}
			let remove = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
			for (slot, key) in [KeyCode::KeyQ, KeyCode::KeyW, KeyCode::KeyE, KeyCode::KeyR]
				.into_iter()
				.enumerate()
			{
				if keys.just_pressed(key) {
					queue.0.push(UiAction::Energy { slot, add: !remove });
				}
			}
		}
		_ => {}
	}
}

fn run_actions(world: &mut World) {
	let actions = std::mem::take(&mut world.resource_mut::<ActionQueue>().0);
	for action in actions {
		let message = if action == UiAction::Resume {
			world.resource_scope(|world, mut buffer: Mut<InputBuffer>| {
				let mut paused = world.resource_mut::<SimPaused>();
				buffer.resume(&mut paused);
			});
			String::new()
		} else {
			apply_action(world, action)
		};
		world.resource_mut::<UiMessage>().0 = message;
	}
}

fn refresh_screen(world: &mut World) {
	let mode = *world.resource::<UiMode>();
	let paused = world.resource::<SimPaused>().0;
	let view = screen_view(world, mode, paused);
	let focused = (world.resource::<ActiveSource>().0 == InputSource::Controller)
		.then(|| world.resource::<UiFocus>().index);
	{
		let last = world.resource::<LastScreen>();
		if last.0 == view && last.1 == focused {
			return;
		}
	}
	if let Some(view) = &view {
		world.resource_mut::<UiFocus>().clamp(view);
	}
	let mut last = world.resource_mut::<LastScreen>();
	last.0 = view.clone();
	last.1 = focused;
	let Some(root) = world
		.query_filtered::<Entity, With<ScreenRoot>>()
		.iter(world)
		.next()
	else {
		return;
	};
	if let Some(children) = world.get::<Children>(root) {
		let doomed: Vec<Entity> = children.iter().collect();
		for child in doomed {
			world.despawn(child);
		}
	}
	let Some(view) = view else {
		world.get_mut::<Node>(root).expect("root node").display = Display::None;
		return;
	};
	world.get_mut::<Node>(root).expect("root node").display = Display::Flex;
	let column = |gap: f32| Node {
		flex_direction: FlexDirection::Column,
		flex_basis: px(0),
		flex_grow: 1.0,
		row_gap: px(gap),
		..default()
	};
	let info = world.spawn(column(3.0)).id();
	let title = world
		.spawn(text_node(view.title.clone(), 24.0, TEXT_COLOR))
		.id();
	world.entity_mut(info).add_child(title);
	for line in &view.lines {
		let text = world.spawn(text_node(line.clone(), 14.0, MUTED_COLOR)).id();
		world.entity_mut(info).add_child(text);
	}
	let actions = world.spawn(column(5.0)).id();
	let mut row: Option<Entity> = None;
	for (i, button) in view.buttons.iter().enumerate() {
		let color = if focused == Some(i) {
			FOCUS_COLOR
		} else if button.enabled {
			BUTTON_COLOR
		} else {
			BUTTON_OFF_COLOR
		};
		let text = if button.enabled {
			TEXT_COLOR
		} else {
			MUTED_COLOR
		};
		let mut entity = world.spawn((
			Button,
			BackgroundColor(color),
			Node {
				padding: UiRect::axes(px(10), px(4)),
				flex_grow: 1.0,
				..default()
			},
		));
		entity.with_child(text_node(button.label.clone(), 15.0, text));
		if button.enabled {
			let action = button.action;
			entity.observe(move |_: On<Activate>, mut queue: ResMut<ActionQueue>| {
				queue.0.push(action);
			});
		} else {
			entity.insert(InteractionDisabled);
		}
		let entity = entity.id();
		let parent = match row {
			Some(existing) => existing,
			None => world
				.spawn(Node {
					flex_direction: FlexDirection::Row,
					column_gap: px(5),
					..default()
				})
				.id(),
		};
		world.entity_mut(parent).add_child(entity);
		if button.same_row_as_next {
			row = Some(parent);
		} else {
			world.entity_mut(actions).add_child(parent);
			row = None;
		}
	}
	world.entity_mut(root).add_children(&[info, actions]);
}

fn refresh_hud(world: &mut World) {
	let hud = hud_view(world).map(|h| hud_text(&h)).unwrap_or_default();
	let message = world.resource::<UiMessage>().0.clone();
	let panel_open = world.resource::<LastScreen>().0.is_some();
	let mut hud_query =
		world.query_filtered::<(&mut Text, &mut Node), (With<HudText>, Without<MessageText>)>();
	if let Some((mut text, mut node)) = hud_query.iter_mut(world).next() {
		if text.0 != hud {
			text.0 = hud;
		}
		node.display = if panel_open {
			Display::None
		} else {
			Display::Flex
		};
	}
	let mut msg_query = world.query_filtered::<&mut Text, (With<MessageText>, Without<HudText>)>();
	if let Some(mut text) = msg_query.iter_mut(world).next()
		&& text.0 != message
	{
		text.0 = message;
	}
}
