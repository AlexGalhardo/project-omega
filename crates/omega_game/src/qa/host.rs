use super::{Command, MenuView, Session, Snapshot, capture};
use crate::GameCorePlugin;
use crate::combat::actor::{ArenaRes, ContentRes, spawn_player};
use crate::content::{Profile, load_dir};
use crate::input::{AimOrigin, FrameInput, InputBuffer};
use crate::sim::{RunSeed, SimHarness, SimPaused};
use bevy::prelude::*;
use std::path::PathBuf;
use std::sync::Arc;

/// The prototype sandbox (player plus wave 1) driven by the real `GameTick` schedule and
/// the same [`InputBuffer`] the native client uses, with one frame per fixed tick and no
/// window, renderer or wall clock.
pub struct HeadlessHost {
	harness: SimHarness,
	window: [u32; 2],
	pub menu: Option<MenuView>,
}

impl HeadlessHost {
	pub fn new(seed: RunSeed, window: [u32; 2]) -> Result<Self, String> {
		let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/content/full");
		let content = load_dir(&dir, Profile::Full).map_err(|e| format!("content: {e:?}"))?;
		let arena = content.catalogs.arenas["arena_foundry"].clone();
		let mut app = App::new();
		app.add_plugins(GameCorePlugin)
			.init_resource::<InputBuffer>();
		let mut harness = SimHarness::with_app(app, seed);
		let world = harness.app_mut().world_mut();
		world.insert_resource(ContentRes(Arc::new(content)));
		world.insert_resource(ArenaRes(arena.bounds));
		spawn_player(world, "char_patch", arena.player_spawn)
			.map_err(|e| format!("player: {e:?}"))?;
		crate::encounters::start_wave(world, 1).map_err(|e| format!("wave: {e:?}"))?;
		Ok(Self {
			harness,
			window,
			menu: None,
		})
	}

	pub fn snapshot(&mut self) -> Snapshot {
		let mut snap = capture(self.harness.app_mut().world_mut(), self.window);
		snap.menu = self.menu;
		snap
	}

	fn with_buffer(&mut self, f: impl FnOnce(&mut InputBuffer, &mut SimPaused)) {
		self.harness
			.app_mut()
			.world_mut()
			.resource_scope(|world, mut buffer: Mut<InputBuffer>| {
				f(&mut buffer, &mut world.resource_mut::<SimPaused>());
			});
	}

	/// Feeds one rendered frame to the input buffer and runs the tick it earns.
	pub fn advance(&mut self, frame: FrameInput) {
		let focus = self.menu.is_some();
		let origin = self.harness.app().world().resource::<AimOrigin>().0;
		let mut sample = None;
		self.with_buffer(|buffer, paused| {
			buffer.ui_focus = focus;
			buffer.begin_frame(frame);
			// A paused client runs no fixed step, so edges stay pending exactly like here.
			if !paused.0 {
				let s = buffer.sample_tick(origin);
				if s.pause_requested {
					paused.0 = true;
				}
				sample = Some(s.input);
			}
		});
		if let Some(input) = sample {
			self.harness.step(input);
		}
	}

	/// Runs the session to completion or until `max_frames`; returns an error text when
	/// the budget ran out first.
	pub fn run(&mut self, session: &mut Session, max_frames: u64) -> Option<String> {
		for _ in 0..max_frames {
			let snap = self.snapshot();
			let step = session.next(&snap);
			for command in step.commands {
				match command {
					Command::Resize { width, height } => self.window = [width, height],
					Command::FocusLost => self.with_buffer(|b, p| b.focus_lost(p)),
					Command::Resume => self.with_buffer(|b, p| b.resume(p)),
					Command::Checkpoint(cp) => {
						let at = self.snapshot();
						session.checkpoint(&cp, &at);
					}
				}
			}
			if step.done {
				return None;
			}
			self.advance(step.frame);
		}
		Some(format!("frame budget of {max_frames} exhausted"))
	}
}
