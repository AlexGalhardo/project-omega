use super::{Command, MenuView, QaUi, Session, Snapshot, apply_debug, apply_ui, capture};
use crate::GameCorePlugin;
use crate::combat::actor::{ArenaRes, ContentRes, spawn_player};
use crate::content::{Profile, load_dir};
use crate::input::{AimOrigin, FrameInput, InputBuffer};
use crate::sim::run::{Run, RunState, begin_run};
use crate::sim::{RunSeed, SimHarness, SimPaused};
use crate::ui::{UiMode, edge_action, take_screen_edges};
use bevy::prelude::*;
use std::collections::BTreeSet;
use std::path::PathBuf;
use std::sync::Arc;

/// Either the foundation sandbox (player plus wave 1 on the full fixture) or the real
/// prototype loop, driven by the real `GameTick` schedule and the same [`InputBuffer`]
/// the native client uses, with one frame per fixed tick and no window, renderer or
/// wall clock.
pub struct HeadlessHost {
	harness: SimHarness,
	window: [u32; 2],
	lifecycle: bool,
	screen_was_open: bool,
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
			lifecycle: false,
			screen_was_open: false,
			menu: None,
		})
	}

	/// The shipped prototype content through `begin_run`: Wave, Shop, Victory/Defeat and
	/// Summary as in the native client, with the same Confirm/Back edge routing.
	pub fn prototype(seed: RunSeed, window: [u32; 2]) -> Result<Self, String> {
		let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/content");
		let content = load_dir(&dir, Profile::Prototype).map_err(|e| format!("content: {e:?}"))?;
		let arena = content.catalogs.arenas["arena_foundry"].bounds;
		let mut app = App::new();
		app.add_plugins(GameCorePlugin)
			.init_resource::<InputBuffer>()
			.init_resource::<UiMode>()
			.init_resource::<QaUi>();
		let mut harness = SimHarness::with_app(app, seed);
		let world = harness.app_mut().world_mut();
		world.insert_resource(ContentRes(Arc::new(content)));
		world.insert_resource(ArenaRes(arena));
		begin_run(world, "char_patch", BTreeSet::new()).map_err(|e| format!("run: {e:?}"))?;
		Ok(Self {
			harness,
			window,
			lifecycle: true,
			screen_was_open: false,
			menu: None,
		})
	}

	fn screen_open(&self) -> bool {
		let world = self.harness.app().world();
		world.resource::<SimPaused>().0
			|| world
				.get_resource::<Run>()
				.is_some_and(|r| r.state != RunState::Wave)
	}

	/// What the client's `keyboard_actions` does after each frame: while a screen is open
	/// it owns Confirm and Back and turns them into typed requests.
	fn route_ui_edges(&mut self) {
		let open = self.screen_open();
		let mut was_open = self.screen_was_open;
		let mut edges = None;
		self.with_buffer(|buffer, _| {
			edges = Some(take_screen_edges(buffer, &mut was_open, open));
		});
		self.screen_was_open = open;
		let world = self.harness.app_mut().world_mut();
		let state = world.get_resource::<Run>().map(|r| r.state);
		if let (Some(edges), Some(state)) = (edges, state) {
			let mode = *world.resource::<UiMode>();
			if let Some(action) = edge_action(state, mode, edges) {
				apply_ui(world, action);
			}
		}
	}

	/// Direct world access for tests that arrange a situation before driving it.
	pub fn world_mut(&mut self) -> &mut World {
		self.harness.app_mut().world_mut()
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
		let (focus, lifecycle) = (self.menu.is_some(), self.lifecycle);
		let origin = self.harness.app().world().resource::<AimOrigin>().0;
		let mut sample = None;
		self.with_buffer(|buffer, paused| {
			// The lifecycle host sets focus after each frame, like the client's Update.
			if !lifecycle {
				buffer.ui_focus = focus;
			}
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
		if self.lifecycle {
			self.route_ui_edges();
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
					Command::Ui(action) => apply_ui(self.harness.app_mut().world_mut(), action),
					Command::Debug(op) => {
						if let Err(e) = apply_debug(self.harness.app_mut().world_mut(), op) {
							session.reject(e);
						}
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
