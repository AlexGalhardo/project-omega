//! Bevy UI widgets for the HUD, shop, pause, abandon and result screens (SP-014).
//! Widgets only render `ScreenView`/`HudView` data and queue typed `UiAction` requests.

use super::{
	ScreenView, UiAction, UiMode, apply_action, edge_action, hud_text, hud_view, screen_view,
	take_screen_edges,
};
use crate::input::InputBuffer;
use crate::sim::SimPaused;
use crate::sim::run::{Run, RunState};
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

#[derive(Resource, Default)]
struct LastScreen(Option<ScreenView>);

pub struct UiClientPlugin;

impl Plugin for UiClientPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<ScreenWasOpen>()
			.init_resource::<ActionQueue>()
			.init_resource::<UiMessage>()
			.init_resource::<UiMode>()
			.init_resource::<LastScreen>()
			.add_systems(Startup, spawn_roots)
			.add_systems(
				Update,
				(keyboard_actions, run_actions, refresh_screen, refresh_hud).chain(),
			);
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
			left: percent(18),
			right: percent(18),
			top: percent(8),
			flex_direction: FlexDirection::Column,
			row_gap: px(6),
			padding: UiRect::all(px(14)),
			display: Display::None,
			..default()
		},
		BackgroundColor(PANEL_COLOR),
	));
}

fn keyboard_actions(
	keys: Res<ButtonInput<KeyCode>>,
	mode: Res<UiMode>,
	paused: Res<SimPaused>,
	run: Option<Res<Run>>,
	mut buffer: ResMut<InputBuffer>,
	mut queue: ResMut<ActionQueue>,
	mut was_open: ResMut<ScreenWasOpen>,
) {
	let Some(run) = run else {
		return;
	};
	let screen_open = run.state != RunState::Wave || paused.0;
	let edges = take_screen_edges(&mut buffer, &mut was_open.0, screen_open);
	if !screen_open {
		return;
	}
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
	if world.resource::<LastScreen>().0 == view {
		return;
	}
	world.resource_mut::<LastScreen>().0 = view.clone();
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
	let mut children = vec![
		world
			.spawn(text_node(view.title.clone(), 26.0, TEXT_COLOR))
			.id(),
	];
	for line in &view.lines {
		children.push(world.spawn(text_node(line.clone(), 15.0, MUTED_COLOR)).id());
	}
	for button in &view.buttons {
		let color = if button.enabled {
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
				padding: UiRect::axes(px(10), px(5)),
				..default()
			},
		));
		entity.with_child(text_node(button.label.clone(), 16.0, text));
		if button.enabled {
			let action = button.action;
			entity.observe(move |_: On<Activate>, mut queue: ResMut<ActionQueue>| {
				queue.0.push(action);
			});
		} else {
			entity.insert(InteractionDisabled);
		}
		let entity = entity.id();
		children.push(entity);
	}
	world.entity_mut(root).add_children(&children);
}

fn refresh_hud(world: &mut World) {
	let hud = hud_view(world).map(|h| hud_text(&h)).unwrap_or_default();
	let message = world.resource::<UiMessage>().0.clone();
	let mut hud_query = world.query_filtered::<&mut Text, (With<HudText>, Without<MessageText>)>();
	if let Some(mut text) = hud_query.iter_mut(world).next()
		&& text.0 != hud
	{
		text.0 = hud;
	}
	let mut msg_query = world.query_filtered::<&mut Text, (With<MessageText>, Without<HudText>)>();
	if let Some(mut text) = msg_query.iter_mut(world).next()
		&& text.0 != message
	{
		text.0 = message;
	}
}
