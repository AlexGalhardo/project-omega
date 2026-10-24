//! SP-013-AC2/AC4 (catalog side): the proof catalogs validate against the exported
//! files, frames are 8-bit RGBA PNGs, and the raster substitute skin keeps every
//! logical extent and pivot. Export flags, hashes and reproducibility are checked by
//! scripts/validate-assets.ps1 -Reproduce.

use omega_2d::visuals::VisualCatalog;
use std::path::{Path, PathBuf};

fn assets() -> PathBuf {
	PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets")
}

fn load(name: &str) -> VisualCatalog {
	let root = assets();
	let bytes = std::fs::read(root.join("presentation").join(name)).unwrap();
	VisualCatalog::from_json(&bytes, |f| root.join(f).is_file())
		.unwrap_or_else(|e| panic!("{name}: {e:?}"))
}

/// (width, height, is 8-bit RGBA) from the PNG header.
fn png_header(path: &Path) -> (u32, u32, bool) {
	let b = std::fs::read(path).unwrap();
	assert_eq!(
		&b[..8],
		b"\x89PNG\r\n\x1a\n",
		"{} is not a PNG",
		path.display()
	);
	let be = |o: usize| u32::from_be_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]]);
	(be(16), be(20), b[24] == 8 && b[25] == 6)
}

#[test]
fn proof_catalogs_resolve_to_rgba_textures() {
	for name in ["proof-catalog.json", "proof-catalog-pixel.json"] {
		let catalog = load(name);
		for record in catalog.records() {
			for frame in &record.frames {
				let (w, h, rgba) = png_header(&assets().join(frame));
				assert!(rgba, "{frame} must be RGBA");
				if let Some(extent) = record.world_extent() {
					// Texture aspect equals logical aspect (no unintended distortion).
					let ratio = f64::from(w) / f64::from(h);
					assert!(
						(ratio - extent.x / extent.y).abs() < 0.02,
						"{frame}: {w}x{h} vs {extent}"
					);
				}
			}
		}
	}
}

#[test]
fn raster_substitute_keeps_logical_metadata() {
	let svg = load("proof-catalog.json");
	let pixel = load("proof-catalog-pixel.json");
	assert_ne!(svg.skin_id(), pixel.skin_id());
	let a = svg.records().collect::<Vec<_>>();
	let b = pixel.records().collect::<Vec<_>>();
	assert_eq!(a.len(), b.len());
	for (x, y) in a.iter().zip(&b) {
		assert_eq!(x.visual_id, y.visual_id);
		assert_eq!(x.extent, y.extent, "{}", x.visual_id);
		assert_eq!(x.pivot_normalized, y.pivot_normalized, "{}", x.visual_id);
		assert_eq!(x.draw_layer, y.draw_layer, "{}", x.visual_id);
	}
	let patch_svg = svg
		.records()
		.find(|r| r.visual_id.as_str() == "visual_char_patch")
		.unwrap();
	let patch_px = pixel
		.records()
		.find(|r| r.visual_id.as_str() == "visual_char_patch")
		.unwrap();
	let (w1, ..) = png_header(&assets().join(&patch_svg.frames[0]));
	let (w2, ..) = png_header(&assets().join(&patch_px.frames[0]));
	assert_eq!(
		(w1, w2),
		(160, 40),
		"different texture resolutions, same 40 LU extent"
	);
}
