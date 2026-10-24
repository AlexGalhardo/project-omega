//! Headless-capable QA harness (QA-P1-003, ROADMAP rule 1.11): scripted e2e timelines
//! and the seeded QA bot, both producing frames for the same [`crate::input::InputBuffer`]
//! the player uses. Nothing here needs a window or GPU; the native client only hosts
//! the same [`Session`] against its real window.

pub mod bot;
pub mod cli;
mod host;
pub mod script;
mod snapshot;

pub use cli::{Finished, QaRun, Session, run_headless, run_headless_session, write_report};
pub use host::HeadlessHost;
pub use snapshot::{CameraFacts, MenuView, PlayerFacts, Snapshot, arena, camera_facts, capture};

use crate::input::FrameInput;

/// Side effect a driver asks its host to perform before the frame is applied.
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
	Resize { width: u32, height: u32 },
	FocusLost,
	Resume,
	Checkpoint(script::Checkpoint),
}

/// One frame of driver output: commands run in order, then `frame` goes to the buffer.
/// A `done` step carries commands only; its frame is not applied.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Step {
	pub frame: FrameInput,
	pub commands: Vec<Command>,
	pub done: bool,
}

/// A source of player-equivalent input. Scripts and bots both implement it.
pub trait InputDriver: Send + Sync {
	fn next(&mut self, obs: &Snapshot) -> Step;

	/// Why the driver stopped early (invalid timeline, wait that cannot finish).
	fn failure(&self) -> Option<&str> {
		None
	}
}
