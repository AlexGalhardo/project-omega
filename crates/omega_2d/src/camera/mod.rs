//! ENG-CAM-001: fixed-aspect logical viewport and inverse screen/world mapping.
//!
//! The whole world rectangle is always visible. Extra screen space becomes letterbox
//! (or pillarbox) bars. Screen coordinates are physical pixels, origin top-left, +Y
//! down; world coordinates are LU, +Y up.

use crate::geometry::Bounds2;
use bevy::math::DVec2;

#[cfg(feature = "client")]
mod client;
#[cfg(feature = "client")]
pub use client::{CurrentViewport, LogicalCamera, LogicalCameraPlugin};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraError {
	InvalidScreenSize,
	EmptyWorld,
}

impl std::fmt::Display for CameraError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(match self {
			Self::InvalidScreenSize => "screen size must be finite and positive",
			Self::EmptyWorld => "world bounds must have positive width and height",
		})
	}
}

impl std::error::Error for CameraError {}

/// Physical-pixel rectangle that shows the world, inside the screen.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScreenRect {
	pub origin: DVec2,
	pub size: DVec2,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ViewportMapping {
	world: Bounds2,
	screen: DVec2,
	content: ScreenRect,
	/// Physical pixels per LU, equal on both axes.
	scale: f64,
}

impl ViewportMapping {
	/// Recompute on resize or settings change, not per entity.
	pub fn new(world: Bounds2, screen_width: f64, screen_height: f64) -> Result<Self, CameraError> {
		let screen = DVec2::new(screen_width, screen_height);
		if !screen.is_finite() || screen.cmple(DVec2::ZERO).any() {
			return Err(CameraError::InvalidScreenSize);
		}
		let size = world.size();
		if size.cmple(DVec2::ZERO).any() {
			return Err(CameraError::EmptyWorld);
		}
		let scale = (screen.x / size.x).min(screen.y / size.y);
		let content_size = size * scale;
		Ok(Self {
			world,
			screen,
			content: ScreenRect {
				origin: (screen - content_size) * 0.5,
				size: content_size,
			},
			scale,
		})
	}

	pub fn world(&self) -> Bounds2 {
		self.world
	}

	pub fn screen_size(&self) -> DVec2 {
		self.screen
	}

	/// Area that shows the world; everything else is a noninteractive bar.
	pub fn content_rect(&self) -> ScreenRect {
		self.content
	}

	pub fn pixels_per_lu(&self) -> f64 {
		self.scale
	}

	pub fn world_to_screen(&self, world: DVec2) -> DVec2 {
		let rel = world - self.world.min();
		DVec2::new(
			self.content.origin.x + rel.x * self.scale,
			self.content.origin.y + self.content.size.y - rel.y * self.scale,
		)
	}

	/// `None` for points in letterbox bars or outside the screen, so they cannot aim.
	pub fn screen_to_world(&self, screen: DVec2) -> Option<DVec2> {
		let local = screen - self.content.origin;
		if !local.is_finite()
			|| local.cmplt(DVec2::ZERO).any()
			|| local.cmpgt(self.content.size).any()
		{
			return None;
		}
		Some(DVec2::new(
			self.world.min().x + local.x / self.scale,
			self.world.min().y + (self.content.size.y - local.y) / self.scale,
		))
	}
}
