//! Windowed host for QA sessions: the same [`Session`] the headless path uses, applied to
//! the real window, the shared [`InputBuffer`] and the 60 Hz fixed loop.
//!
//! Limits: the fixed loop and render frames are decoupled, so tick-exact checkpoints are
//! only reproducible headless; the windowed path asserts ranges. Real keyboard events
//! reaching the window in the same frame still add their edges to the buffer.

use crate::input::InputBuffer;
use crate::qa::{Command, Session, capture, write_report};
use crate::sim::SimPaused;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::window::PrimaryWindow;

/// Frames kept alive after the report is written so queued screenshots reach the disk.
const SETTLE_FRAMES: u32 = 30;

#[derive(Resource)]
struct QaHost {
	session: Option<Session>,
	/// Size the script asked for, reported until the window confirms it.
	requested: Option<[u32; 2]>,
	frames: u64,
	outcome: Option<bool>,
	settle: u32,
}

pub fn install(app: &mut App, session: Session) {
	app.insert_resource(QaHost {
		session: Some(session),
		requested: None,
		frames: 0,
		outcome: None,
		settle: SETTLE_FRAMES,
	})
	.add_systems(
		RunFixedMainLoop,
		drive.in_set(RunFixedMainLoopSystems::BeforeFixedMainLoop),
	);
}

fn window_size(world: &mut World, host: &mut QaHost) -> Option<[u32; 2]> {
	let size = world
		.query_filtered::<&Window, With<PrimaryWindow>>()
		.single(world)
		.ok()?
		.physical_size();
	let real = [size.x, size.y];
	if host.requested == Some(real) {
		host.requested = None;
	}
	Some(host.requested.unwrap_or(real))
}

fn drive(world: &mut World) {
	let Some(mut host) = world.remove_resource::<QaHost>() else {
		return;
	};
	step(world, &mut host);
	world.insert_resource(host);
}

fn step(world: &mut World, host: &mut QaHost) {
	if let Some(passed) = host.outcome {
		host.settle = host.settle.saturating_sub(1);
		if host.settle == 0 {
			world.write_message(if passed {
				AppExit::Success
			} else {
				AppExit::error()
			});
		}
		return;
	}
	let Some(window) = window_size(world, host) else {
		return;
	};
	let Some(mut session) = host.session.take() else {
		return;
	};
	host.frames += 1;
	let snap = capture(world, window);
	let step = session.next(&snap);
	for command in step.commands {
		match command {
			Command::Resize { width, height } => {
				host.requested = Some([width, height]);
				if let Ok(mut w) = world
					.query_filtered::<&mut Window, With<PrimaryWindow>>()
					.single_mut(world)
				{
					w.resolution.set(width as f32, height as f32);
				}
			}
			Command::FocusLost => world.resource_scope(|w, mut b: Mut<InputBuffer>| {
				b.focus_lost(&mut w.resource_mut::<SimPaused>());
			}),
			Command::Resume => world.resource_scope(|w, mut b: Mut<InputBuffer>| {
				b.resume(&mut w.resource_mut::<SimPaused>());
			}),
			Command::Checkpoint(cp) => {
				let at = capture(world, host.requested.unwrap_or(window));
				let path = session.out_dir.join(format!("{}.png", cp.name));
				let result = session.checkpoint(&cp, &at);
				if cp.screenshot {
					result.screenshot = Some(path.display().to_string());
					let _ = std::fs::create_dir_all(&session.out_dir);
					world
						.spawn(Screenshot::primary_window())
						.observe(save_to_disk(path));
				}
			}
		}
	}
	let over_budget = host.frames >= session.frame_budget();
	if step.done || over_budget {
		let error = over_budget.then(|| "frame budget exhausted".to_owned());
		let finished = session.finish(capture(world, window), "windowed", error);
		if let Err(e) = write_report(&finished) {
			error!("{e}");
			host.outcome = Some(false);
		} else {
			info!(
				"{} {}",
				if finished.passed { "PASS" } else { "FAIL" },
				finished.file.display()
			);
			host.outcome = Some(finished.passed);
		}
		return;
	}
	world.resource_mut::<InputBuffer>().begin_frame(step.frame);
	host.session = Some(session);
}
