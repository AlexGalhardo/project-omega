//! SP-012-AC2/AC3: catalog extent/pivot are independent of texture resolution,
//! invalid catalogs are rejected, a cosmetic fallback stays visible and an absent
//! essential visual is a typed error.

use bevy::math::DVec2;
use omega_2d::visuals::*;
use serde_json::{Value, json};

fn record(id: &str, category: &str, frame: &str) -> Value {
	json!({
		"visual_id": id, "category": category, "source_asset_id": format!("src_{id}"),
		"provenance_id": "prov_test", "extent": { "world_lu": [40, 40] },
		"pivot_normalized": [0.5, 0.5], "sampling": "linear", "draw_layer": "actor",
		"essential": category == "player", "frames": [frame]
	})
}

fn catalog(records: Vec<Value>) -> Value {
	json!({
		"schema_version": 1,
		"skin_id": "test",
		"category_fallbacks": { "enemy": "visual_fallback_enemy" },
		"records": records
	})
}

fn base(player_frame: &str) -> Vec<Value> {
	vec![
		record("visual_char_patch", "player", player_frame),
		record("visual_fallback_enemy", "enemy", "fallback.png"),
	]
}

fn load(v: &Value) -> Result<VisualCatalog, Vec<CatalogError>> {
	VisualCatalog::from_json(v.to_string().as_bytes(), |_| true)
}

fn paths(v: &Value) -> Vec<String> {
	load(v).unwrap_err().into_iter().map(|e| e.path).collect()
}

fn id(s: &str) -> VisualId {
	VisualId::new(s).unwrap()
}

#[test]
fn ac2_replacing_texture_resolution_keeps_extent_and_pivot() {
	let svg_skin = load(&catalog(base("sprites/patch_160.png"))).unwrap();
	let pixel_skin = load(&catalog(base("sprites/patch_64_pixel.png"))).unwrap();
	let a = svg_skin.get(&id("visual_char_patch")).unwrap();
	let b = pixel_skin.get(&id("visual_char_patch")).unwrap();
	assert_eq!(a.world_extent(), Some(DVec2::new(40.0, 40.0)));
	assert_eq!(a.world_extent(), b.world_extent());
	assert_eq!(a.anchor(), b.anchor());
	// Art Bible section 4: the 44.8-unit core of a 64-unit view box spans 28 LU.
	let view_box = [0.0, 0.0, 64.0, 64.0];
	let left = a
		.source_to_local(DVec2::new(32.0 - 22.4, 32.0), view_box)
		.unwrap();
	let top = a
		.source_to_local(DVec2::new(32.0, 32.0 - 22.4), view_box)
		.unwrap();
	assert!((left.x + 14.0).abs() < 1e-9 && left.y.abs() < 1e-9);
	assert!(
		(top.y - 14.0).abs() < 1e-9,
		"SVG +Y down maps to world +Y up"
	);
}

#[test]
fn ac3_invalid_extent_pivot_and_frames_are_rejected() {
	let mut records = base("p.png");
	records[0]["extent"] = json!({ "world_lu": [0, 40] });
	records[0]["pivot_normalized"] = json!([1.5, 0.5]);
	records[1]["frames"] = json!([]);
	let p = paths(&catalog(records));
	assert!(p.contains(&"visual_char_patch.extent".into()), "{p:?}");
	assert!(
		p.contains(&"visual_char_patch.pivot_normalized".into()),
		"{p:?}"
	);
	assert!(p.contains(&"visual_fallback_enemy.frames".into()), "{p:?}");

	let missing = catalog(base("missing.png")).to_string();
	let errors = VisualCatalog::from_json(missing.as_bytes(), |f| f != "missing.png").unwrap_err();
	assert!(
		errors
			.iter()
			.any(|e| e.path == "visual_char_patch.frames[0]")
	);
}

#[test]
fn ac3_fallback_cycle_unknown_and_cross_category_are_rejected() {
	let mut records = base("p.png");
	let mut a = record("visual_a", "enemy", "a.png");
	let mut b = record("visual_b", "enemy", "b.png");
	a["fallback_visual_id"] = json!("visual_b");
	b["fallback_visual_id"] = json!("visual_a");
	records.extend([a, b]);
	let errors = load(&catalog(records)).unwrap_err();
	assert!(errors.iter().any(|e| e.message == "fallback cycle"));

	let mut unknown = base("p.png");
	unknown[1]["fallback_visual_id"] = json!("visual_zzz");
	assert!(paths(&catalog(unknown)).contains(&"visual_fallback_enemy.fallback_visual_id".into()));

	let mut cross = base("p.png");
	cross[1]["fallback_visual_id"] = json!("visual_char_patch");
	assert!(load(&catalog(cross)).is_err());

	let mut dup = base("p.png");
	dup.push(record("visual_char_patch", "player", "q.png"));
	assert!(paths(&catalog(dup)).contains(&"visual_char_patch.visual_id".into()));

	let mut extra = catalog(base("p.png"));
	extra["unexpected"] = json!(1);
	assert!(load(&extra).is_err(), "unknown catalog fields are rejected");

	let mut version = catalog(base("p.png"));
	version["schema_version"] = json!(2);
	assert!(paths(&version).contains(&"catalog.schema_version".into()));
}

#[test]
fn ac3_missing_cosmetic_uses_visible_fallback_and_essential_errors() {
	let catalog = load(&catalog(base("p.png"))).unwrap();
	let missing = id("visual_enemy_runner");
	match catalog
		.resolve(&missing, VisualCategory::Enemy, false)
		.unwrap()
	{
		Resolved::Fallback(r) => assert_eq!(r.visual_id.as_str(), "visual_fallback_enemy"),
		other => panic!("expected fallback, got {other:?}"),
	}
	assert_eq!(
		catalog.resolve(&missing, VisualCategory::Enemy, true),
		Err(PresentationError::EssentialMissing(missing.clone()))
	);
	assert_eq!(
		catalog.resolve(&missing, VisualCategory::Item, false),
		Err(PresentationError::NoFallback(missing))
	);
	assert!(matches!(
		catalog.resolve(&id("visual_char_patch"), VisualCategory::Player, true),
		Ok(Resolved::Direct(_))
	));
}

#[test]
fn visual_ids_reject_paths_and_uppercase() {
	assert!(VisualId::new("sprites/patch.png").is_err());
	assert!(VisualId::new("Visual").is_err());
	assert!(VisualId::new("9visual").is_err());
	assert!(VisualId::new("").is_err());
	assert!(VisualId::new("visual_char_patch").is_ok());
}
