//! Native client (feature `client`). Foundation scope: the fixed full-arena camera,
//! a static preview of the proof visual catalog (ENG-P1-004, GAME-P1-004), native
//! screenshots for readability review, and the PERF-01 empty-scene benchmark.

pub mod presentation;

use crate::GameCorePlugin;
use crate::diagnostics::{Metric, Report, ReportMetadata};
use crate::sim::{RngStreams, RunSeed};
use bevy::math::DVec2;
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::window::{PresentMode, WindowResolution};
use omega_2d::camera::LogicalCamera;
use omega_2d::geometry::Bounds2;
use omega_2d::visuals::{
	Resolved, VisualCatalog, VisualCategory, VisualId, VisualPose, pose_transform, sprite_parts,
};
use presentation::{ActiveSkin, LivePresentationPlugin};
use std::path::PathBuf;

/// Arena `arena_foundry` playable rectangle (GDD-004).
const ARENA_MIN: DVec2 = DVec2::new(-600.0, -337.5);
const ARENA_MAX: DVec2 = DVec2::new(600.0, 337.5);
const FLOOR_DEEP: Color = Color::srgb_u8(0x15, 0x1d, 0x27);

#[derive(Debug, Clone)]
pub struct BenchmarkOptions {
	pub id: String,
	pub duration_seconds: f64,
	pub output: PathBuf,
}

#[derive(Debug, Clone)]
pub struct ClientOptions {
	pub width: u32,
	pub height: u32,
	/// Catalog under `assets/presentation/`.
	pub catalog: String,
	pub screenshot: Option<PathBuf>,
	pub benchmark: Option<BenchmarkOptions>,
	/// Prototype sandbox: spawn the player and run wave 1 live instead of the static preview.
	pub play: bool,
}

impl Default for ClientOptions {
	fn default() -> Self {
		Self {
			width: 1280,
			height: 720,
			catalog: "proof-catalog.json".into(),
			screenshot: None,
			benchmark: None,
			play: false,
		}
	}
}

/// Packaged builds ship `assets/` next to the executable; development runs use the
/// repository `assets/`.
pub fn asset_root() -> PathBuf {
	let beside_exe = std::env::current_exe()
		.ok()
		.and_then(|p| p.parent().map(|d| d.join("assets")))
		.filter(|p| p.join("presentation").is_dir());
	beside_exe.unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets"))
}

#[derive(Resource)]
struct Options(ClientOptions);

#[derive(Resource, Default)]
struct FrameCounter(u32);

#[derive(Resource, Default)]
struct BenchSamples {
	elapsed: f64,
	frame_ms: Vec<f64>,
}

/// Runs the native client until the window closes, the screenshot is written or the
/// benchmark finishes.
pub fn run(options: ClientOptions) -> AppExit {
	let root = asset_root();
	let catalog_path = root.join("presentation").join(&options.catalog);
	let skin = match std::fs::read(&catalog_path)
		.map_err(|e| vec![e.to_string()])
		.and_then(|b| {
			VisualCatalog::from_json(&b, |f| root.join(f).is_file())
				.map_err(|errs| errs.iter().map(ToString::to_string).collect())
		}) {
		Ok(s) => s,
		Err(errors) => {
			eprintln!("cannot load visual catalog {}:", catalog_path.display());
			errors.iter().for_each(|e| eprintln!("  {e}"));
			return AppExit::error();
		}
	};
	let present_mode = if options.benchmark.is_some() {
		PresentMode::AutoNoVsync
	} else {
		PresentMode::AutoVsync
	};
	let mut app = App::new();
	app.add_plugins(
		DefaultPlugins
			.set(WindowPlugin {
				primary_window: Some(Window {
					title: "Project Omega".into(),
					resolution: WindowResolution::new(options.width, options.height)
						.with_scale_factor_override(1.0),
					present_mode,
					..default()
				}),
				..default()
			})
			.set(AssetPlugin {
				file_path: root.to_string_lossy().into_owned(),
				..default()
			}),
	)
	.add_plugins((GameCorePlugin, crate::input::InputClientPlugin))
	.insert_resource(ClearColor(Color::BLACK))
	.insert_resource(RngStreams::new(&dev_seed()))
	.insert_resource(dev_seed())
	.insert_resource(ActiveSkin(skin))
	.insert_resource(Options(options))
	.init_resource::<FrameCounter>()
	.init_resource::<BenchSamples>()
	.add_systems(Startup, spawn_preview)
	.add_systems(Update, (screenshot_and_exit, benchmark));
	if app.world().resource::<Options>().0.play {
		app.add_plugins(LivePresentationPlugin)
			.add_systems(Startup, start_play.after(spawn_preview));
	}
	app.run()
}

/// Fixed seed until the run lifecycle (GAME-P2-010) derives one per run.
fn dev_seed() -> RunSeed {
	RunSeed::from_hex(&"5a".repeat(32)).expect("static seed")
}

/// Development sandbox: the full fixture catalog stands in until GAME-P2-009 ships the
/// exact prototype subset and GAME-P2-010 the lifecycle.
fn start_play(world: &mut World) {
	use crate::combat::actor::{ArenaRes, ContentRes, spawn_player};
	use crate::content::{Profile, load_dir};
	let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/content/full");
	let content = match load_dir(&dir, Profile::Full) {
		Ok(c) => c,
		Err(e) => {
			error!("cannot load play content: {e:?}");
			return;
		}
	};
	let arena = content.catalogs.arenas["arena_foundry"].clone();
	world.insert_resource(ContentRes(std::sync::Arc::new(content)));
	world.insert_resource(ArenaRes(arena.bounds));
	if let Err(e) = spawn_player(world, "char_patch", arena.player_spawn) {
		error!("cannot spawn player: {e:?}");
	}
	if let Err(e) = crate::encounters::start_wave(world, 1) {
		error!("cannot start wave: {e:?}");
	}
}

fn arena() -> Bounds2 {
	Bounds2::new(ARENA_MIN, ARENA_MAX).expect("static arena bounds")
}

fn spawn_visual(
	commands: &mut Commands,
	assets: &AssetServer,
	skin: &VisualCatalog,
	id: &str,
	category: VisualCategory,
	at: DVec2,
) {
	let id = VisualId::new(id).expect("static visual id");
	// Proof scene treats everything as cosmetic so a missing record shows its fallback.
	match skin.resolve(&id, category, false) {
		Ok(resolved) => {
			let record = resolved.record();
			if let Some(parts) = sprite_parts(record, 0, assets) {
				let pose = VisualPose {
					position: at,
					facing_radians: 0.0,
				};
				commands.spawn((parts.sprite, parts.anchor, pose_transform(pose, record)));
			}
			if matches!(resolved, Resolved::Fallback(_)) {
				info!(
					"visual {id} missing; showing category fallback {}",
					record.visual_id
				);
			}
		}
		Err(e) => error!("{e}"),
	}
}

/// Static preview: player, smallest friendly and hostile projectile cores, a locked
/// ground warning, a missing enemy visual shown through its fallback, and an icon.
fn spawn_preview(
	mut commands: Commands,
	assets: Res<AssetServer>,
	skin: Res<ActiveSkin>,
	options: Res<Options>,
) {
	commands.spawn((Camera2d, LogicalCamera { world: arena() }));
	commands.spawn((
		Sprite::from_color(FLOOR_DEEP, (ARENA_MAX - ARENA_MIN).as_vec2()),
		Transform::from_xyz(0.0, 0.0, omega_2d::visuals::DrawLayer::ArenaFloor.z()),
	));
	if options.0.play {
		return;
	}
	let s = &skin.0;
	spawn_visual(
		&mut commands,
		&assets,
		s,
		"visual_char_patch",
		VisualCategory::Player,
		DVec2::ZERO,
	);
	spawn_visual(
		&mut commands,
		&assets,
		s,
		"visual_projectile_friendly_round",
		VisualCategory::Projectile,
		DVec2::new(60.0, 0.0),
	);
	spawn_visual(
		&mut commands,
		&assets,
		s,
		"visual_projectile_hostile_pointed",
		VisualCategory::Projectile,
		DVec2::new(60.0, 40.0),
	);
	spawn_visual(
		&mut commands,
		&assets,
		s,
		"visual_warning_ground_circle",
		VisualCategory::Warning,
		DVec2::new(220.0, -120.0),
	);
	spawn_visual(
		&mut commands,
		&assets,
		s,
		"visual_enemy_chaser",
		VisualCategory::Enemy,
		DVec2::new(-220.0, 120.0),
	);
	if let Some(icon) = s.get(&VisualId::new("visual_item_plating").expect("static id")) {
		commands.spawn((
			ImageNode::new(assets.load(icon.frames[0].clone())),
			Node {
				position_type: PositionType::Absolute,
				right: px(16),
				top: px(16),
				width: px(64),
				height: px(64),
				..default()
			},
		));
	}
}

fn screenshot_and_exit(
	mut commands: Commands,
	options: Res<Options>,
	mut frames: ResMut<FrameCounter>,
	mut exit: MessageWriter<AppExit>,
) {
	let Some(path) = &options.0.screenshot else {
		return;
	};
	frames.0 += 1;
	// Wait for textures to load and the camera to fit the window before capturing.
	if frames.0 == 60 {
		commands
			.spawn(Screenshot::primary_window())
			.observe(save_to_disk(path.clone()));
	}
	if frames.0 == 90 {
		exit.write(AppExit::Success);
	}
}

const BENCH_WARMUP_SECONDS: f64 = 2.0;

fn benchmark(
	options: Res<Options>,
	time: Res<Time<Real>>,
	mut samples: ResMut<BenchSamples>,
	window: Single<&Window>,
	mut exit: MessageWriter<AppExit>,
) {
	let Some(bench) = &options.0.benchmark else {
		return;
	};
	samples.elapsed += time.delta_secs_f64();
	if samples.elapsed < BENCH_WARMUP_SECONDS {
		return;
	}
	samples.frame_ms.push(time.delta_secs_f64() * 1000.0);
	if samples.elapsed < BENCH_WARMUP_SECONDS + bench.duration_seconds {
		return;
	}
	let mut metadata = ReportMetadata::new(&bench.id, "none".into());
	let size = window.physical_size();
	metadata.settings = vec![
		("resolution".into(), format!("{}x{}", size.x, size.y)),
		("present_mode".into(), "AutoNoVsync".into()),
		("warmup_seconds".into(), BENCH_WARMUP_SECONDS.to_string()),
		("capture_seconds".into(), bench.duration_seconds.to_string()),
		(
			"scene".into(),
			"Foundation proof preview (empty arena plus proof visuals)".into(),
		),
	];
	let report = Report {
		metadata,
		metrics: vec![Metric::from_samples(
			"cpu_frame_wall_time",
			"ms",
			&samples.frame_ms,
		)],
		unsupported: vec![
			"gpu_frame_time".into(),
			"allocation_count".into(),
			"working_set".into(),
			"draw_calls".into(),
			"texture_memory".into(),
		],
	};
	if let Some(dir) = bench.output.parent() {
		let _ = std::fs::create_dir_all(dir);
	}
	match std::fs::write(&bench.output, report.to_json()) {
		Ok(()) => info!("wrote {}", bench.output.display()),
		Err(e) => error!("cannot write {}: {e}", bench.output.display()),
	}
	exit.write(AppExit::Success);
}
