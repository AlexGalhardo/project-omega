//! Live presentation of the authoritative simulation (ENG-VIS-001 integration).
//!
//! One view entity per simulation entity, keyed by `SimId`. Views read poses and facts;
//! nothing here writes simulation state or feeds sizes and timings back into rules.

use crate::combat::actor::{Body, ContentRes, Enemy, Pickup, Player, Pose2, PreviousPose2};
use crate::combat::weapons::Projectile;
use crate::encounters::SpawnWarning;
use crate::sim::{PresentationQueue, PresentationRecord, SimId};
use bevy::ecs::query::QueryData;
use bevy::math::DVec2;
use bevy::prelude::*;
use omega_2d::visuals::{
	DrawLayer, Resolved, VisualCatalog, VisualCategory, VisualId, VisualPose, pose_transform,
	sprite_parts,
};
use std::collections::{BTreeMap, BTreeSet};

/// Skin currently used to draw the simulation; swapped only between runs.
#[derive(Resource)]
pub struct ActiveSkin(pub VisualCatalog);

/// `SimId` to view entity.
#[derive(Resource, Default, Debug)]
pub struct RenderMap(pub BTreeMap<SimId, Entity>);

#[derive(Component, Debug, Clone, Copy)]
pub struct SimView(pub SimId);

/// Presentation records drained this rendered frame, for audio, HUD and effects.
#[derive(Resource, Default, Debug)]
pub struct FrameRecords(pub Vec<PresentationRecord>);

const PICKUP_COLOR: Color = Color::srgb_u8(0xf2, 0xc9, 0x4c);
const PICKUP_SIZE_LU: f32 = 10.0;

/// What a simulation entity looks like, decided from its kind only.
#[derive(Debug, Clone, PartialEq)]
pub enum ViewPlan {
	Visual {
		id: String,
		category: VisualCategory,
		/// Exact collision diameter for projectile cores.
		diameter: Option<f32>,
	},
	Marker {
		diameter: f32,
	},
}

#[derive(QueryData)]
pub struct SimEntity {
	id: &'static SimId,
	pose: &'static Pose2,
	previous: Option<&'static PreviousPose2>,
	body: Option<&'static Body>,
	player: Has<Player>,
	enemy: Option<&'static Enemy>,
	projectile: Has<Projectile>,
	pickup: Has<Pickup>,
	warning: Option<&'static SpawnWarning>,
}

/// Linear interpolation used for rendering only.
pub fn lerp_pose(previous: DVec2, current: DVec2, alpha: f64) -> DVec2 {
	previous + (current - previous) * alpha.clamp(0.0, 1.0)
}

fn plan(item: &SimEntityItem, content: Option<&ContentRes>) -> Option<ViewPlan> {
	if item.player {
		return Some(ViewPlan::Visual {
			id: "visual_char_patch".into(),
			category: VisualCategory::Player,
			diameter: None,
		});
	}
	if let Some(enemy) = item.enemy {
		let id = content
			.and_then(|c| c.0.catalogs.enemies.get(&enemy.id))
			.map_or_else(
				|| "visual_unknown_enemy".to_string(),
				|d| d.visual_id.clone(),
			);
		return Some(ViewPlan::Visual {
			id,
			category: VisualCategory::Enemy,
			diameter: None,
		});
	}
	if item.projectile {
		return Some(ViewPlan::Visual {
			id: "visual_projectile_friendly_round".into(),
			category: VisualCategory::Projectile,
			diameter: item.body.map(|b| (b.0 * 2.0) as f32),
		});
	}
	if item.warning.is_some() {
		return Some(ViewPlan::Visual {
			id: "visual_warning_ground_circle".into(),
			category: VisualCategory::Warning,
			diameter: None,
		});
	}
	item.pickup.then_some(ViewPlan::Marker {
		diameter: PICKUP_SIZE_LU,
	})
}

fn spawn_view(
	commands: &mut Commands,
	assets: &AssetServer,
	skin: &VisualCatalog,
	plan: &ViewPlan,
	sim_id: SimId,
	at: DVec2,
) -> Option<Entity> {
	match plan {
		ViewPlan::Marker { diameter } => Some(
			commands
				.spawn((
					SimView(sim_id),
					Sprite::from_color(PICKUP_COLOR, Vec2::splat(*diameter)),
					Transform::from_translation(at.as_vec2().extend(DrawLayer::Pickup.z())),
				))
				.id(),
		),
		ViewPlan::Visual {
			id,
			category,
			diameter,
		} => {
			let id = VisualId::new(id.as_str()).ok()?;
			// Essential cues must never vanish silently: unresolved ids log and fall back.
			let resolved = match skin.resolve(&id, *category, false) {
				Ok(r) => r,
				Err(e) => {
					error!("{e}");
					return None;
				}
			};
			if matches!(resolved, Resolved::Fallback(_)) {
				debug!("visual {id} missing; category fallback in use");
			}
			let record = resolved.record();
			let mut parts = sprite_parts(record, 0, assets)?;
			if let Some(d) = diameter {
				parts.sprite.custom_size = Some(Vec2::splat(*d));
			}
			let transform = pose_transform(
				VisualPose {
					position: at,
					facing_radians: 0.0,
				},
				record,
			);
			Some(
				commands
					.spawn((SimView(sim_id), parts.sprite, parts.anchor, transform))
					.id(),
			)
		}
	}
}

/// Creates, moves and removes view entities for this frame.
#[allow(clippy::too_many_arguments)]
pub fn sync_views(
	mut commands: Commands,
	assets: Res<AssetServer>,
	skin: Res<ActiveSkin>,
	content: Option<Res<ContentRes>>,
	fixed: Res<Time<Fixed>>,
	mut map: ResMut<RenderMap>,
	sims: Query<SimEntity>,
	mut views: Query<&mut Transform, With<SimView>>,
) {
	let alpha = f64::from(fixed.overstep_fraction());
	let mut seen = BTreeSet::new();
	for item in &sims {
		let current = item.warning.map_or(item.pose.0, |w| w.position);
		let at = item
			.previous
			.map_or(current, |p| lerp_pose(p.0, current, alpha));
		seen.insert(*item.id);
		if let Some(&view) = map.0.get(item.id) {
			if let Ok(mut t) = views.get_mut(view) {
				t.translation.x = at.x as f32;
				t.translation.y = at.y as f32;
			}
			continue;
		}
		let Some(plan) = plan(&item, content.as_deref()) else {
			continue;
		};
		if let Some(view) = spawn_view(&mut commands, &assets, &skin.0, &plan, *item.id, at) {
			map.0.insert(*item.id, view);
		}
	}
	map.0.retain(|id, view| {
		let keep = seen.contains(id);
		if !keep {
			commands.entity(*view).despawn();
		}
		keep
	});
}

/// Drains the simulation queue exactly once per rendered frame.
pub fn drain_records(mut queue: ResMut<PresentationQueue>, mut frame: ResMut<FrameRecords>) {
	frame.0 = queue.drain();
}

pub struct LivePresentationPlugin;

impl Plugin for LivePresentationPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<RenderMap>()
			.init_resource::<FrameRecords>()
			.add_systems(Update, (drain_records, sync_views).chain());
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn interpolation_is_clamped_and_linear() {
		let (a, b) = (DVec2::new(0.0, 0.0), DVec2::new(10.0, -4.0));
		assert_eq!(lerp_pose(a, b, 0.0), a);
		assert_eq!(lerp_pose(a, b, 0.5), DVec2::new(5.0, -2.0));
		assert_eq!(lerp_pose(a, b, 2.0), b);
	}
}
