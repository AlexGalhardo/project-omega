use super::{Sampling, VisualPose, VisualRecord};
use bevy::image::{ImageLoaderSettings, ImageSampler};
use bevy::prelude::*;
use bevy::sprite::Anchor;

/// Components that draw one catalog frame at its logical extent and pivot.
pub struct SpriteParts {
	pub sprite: Sprite,
	pub anchor: Anchor,
}

/// Builds a sprite whose size is the record's logical extent, so a texture of any
/// resolution occupies the same world area. UI records return `None`.
pub fn sprite_parts(
	record: &VisualRecord,
	frame: usize,
	assets: &AssetServer,
) -> Option<SpriteParts> {
	let extent = record.world_extent()?;
	let path = record.frames.get(frame)?.clone();
	let sampling = record.sampling;
	let image = assets
		.load_builder()
		.with_settings(move |s: &mut ImageLoaderSettings| {
			s.sampler = match sampling {
				Sampling::Linear => ImageSampler::linear(),
				Sampling::Nearest => ImageSampler::nearest(),
			};
		})
		.load(path);
	Some(SpriteParts {
		sprite: Sprite {
			image,
			custom_size: Some(extent.as_vec2()),
			..default()
		},
		anchor: Anchor(record.anchor().as_vec2()),
	})
}

/// World transform for a pose on the record's semantic layer.
pub fn pose_transform(pose: VisualPose, record: &VisualRecord) -> Transform {
	Transform::from_translation(pose.position.as_vec2().extend(record.draw_layer.z()))
		.with_rotation(Quat::from_rotation_z(pose.facing_radians as f32))
}
