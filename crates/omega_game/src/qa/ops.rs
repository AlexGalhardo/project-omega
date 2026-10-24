//! World-side effects of QA commands: typed UI requests (both hosts) and the debug
//! operations only the headless host may run.

use super::snapshot::QaUi;
use crate::combat::actor::{Health, Player, Scrap};
use crate::encounters::WaveRuntime;
use crate::input::InputBuffer;
use crate::sim::run::Run;
use crate::sim::{SimPaused, SimTick};
use crate::ui::{UiAction, apply_action};
use bevy::prelude::*;
use serde::Deserialize;

/// Headless-only state edits that set up a situation no honest input can reach quickly.
/// Scripts name them `debug_headless`, and the windowed host refuses them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebugOp {
	/// Player health to zero; death is then resolved by the real transition.
	KillPlayer,
	/// Backdates the wave start so the real completion check fires on the next tick.
	ExpireWave,
	SetScrap(u32),
}

/// Applies a typed UI request exactly like a widget would and records its message.
pub fn apply_ui(world: &mut World, action: UiAction) {
	let message = if action == UiAction::Resume {
		world.resource_scope(|world, mut buffer: Mut<InputBuffer>| {
			buffer.resume(&mut world.resource_mut::<SimPaused>());
		});
		String::new()
	} else {
		apply_action(world, action)
	};
	let mut qa = world.get_resource_or_insert_with(QaUi::default);
	qa.message.clone_from(&message);
	qa.actions += 1;
	#[cfg(feature = "client")]
	if let Some(mut shown) = world.get_resource_mut::<crate::ui::client::UiMessage>() {
		shown.0 = message;
	}
}

pub fn apply_debug(world: &mut World, op: DebugOp) -> Result<(), String> {
	match op {
		DebugOp::KillPlayer => {
			let mut health = world.query_filtered::<&mut Health, With<Player>>();
			let mut health = health
				.iter_mut(world)
				.next()
				.ok_or("kill_player: no player")?;
			health.current = 0;
		}
		DebugOp::ExpireWave => {
			let tick = world.resource::<SimTick>().0;
			let live = world
				.get_resource::<Run>()
				.is_some_and(|r| r.state == crate::sim::run::RunState::Wave);
			let mut runtime = world
				.get_resource_mut::<WaveRuntime>()
				.filter(|_| live)
				.ok_or("expire_wave: no live wave")?;
			runtime.tick0 = tick.saturating_sub(runtime.duration);
		}
		DebugOp::SetScrap(value) => world.insert_resource(Scrap(value)),
	}
	Ok(())
}
