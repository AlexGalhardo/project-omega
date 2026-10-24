//! DA-20 / SP-013-AC4 (simulation side): swapping the presentation catalog leaves
//! the authoritative trace unchanged, because simulation never reads presentation.

use bevy::prelude::*;
use omega_2d::visuals::VisualCatalog;
use omega_game::sim::{RunSeed, SimHarness, TickInput};
use std::path::PathBuf;

#[derive(Resource)]
struct Skin(#[allow(dead_code)] VisualCatalog);

fn harness_with(catalog: &str) -> SimHarness {
	let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets");
	let bytes = std::fs::read(root.join("presentation").join(catalog)).unwrap();
	let skin = VisualCatalog::from_json(&bytes, |f| root.join(f).is_file()).unwrap();
	let mut app = App::new();
	app.insert_resource(Skin(skin));
	SimHarness::with_app(app, RunSeed([3; 32]))
}

#[test]
fn da20_catalog_swap_keeps_trace_identical() {
	let inputs = vec![TickInput::default(); 1200];
	let svg = harness_with("proof-catalog.json").run(&inputs);
	let pixel = harness_with("proof-catalog-pixel.json").run(&inputs);
	assert_eq!(svg.len(), 1200);
	assert_eq!(svg, pixel);
}
