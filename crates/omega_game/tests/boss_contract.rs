//! SP-008 bosses (GDD 10.2): fixed warning with uncapped postponement, the Press and Foreman
//! schedules, completion blocking (DA-12) and boss knockback immunity.

use bevy::math::DVec2;
use bevy::prelude::*;
use omega_game::combat::actor::*;
use omega_game::content::{Profile, load_dir};
use omega_game::encounters::boss::{BossAi, BossWarning};
use omega_game::encounters::{BossPhase, WaveRuntime, start_wave};
use omega_game::sim::*;
use std::path::PathBuf;
use std::sync::Arc;

struct Bed {
	h: SimHarness,
	player: Entity,
	records: Vec<(u64, PresentationKind)>,
}

fn bed(wave: u32, player_at: DVec2) -> Bed {
	let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/content/full");
	let content = load_dir(&dir, Profile::Full).expect("full fixture");
	let arena = content.catalogs.arenas["arena_foundry"].bounds;
	let mut app = App::new();
	app.add_plugins(omega_game::GameCorePlugin);
	let mut h = SimHarness::with_app(app, RunSeed::from_hex(&"99".repeat(32)).unwrap());
	h.app_mut().insert_resource(ContentRes(Arc::new(content)));
	h.app_mut().insert_resource(ArenaRes(arena));
	let player = spawn_player(h.app_mut().world_mut(), "char_patch", player_at).unwrap();
	let world = h.app_mut().world_mut();
	world.get_mut::<Loadout>(player).unwrap().slots[0] = None;
	let mut health = world.get_mut::<Health>(player).unwrap();
	health.current = 1_000_000;
	health.max = 1_000_000;
	start_wave(world, wave).unwrap();
	// Only the boss under test: no common batches.
	world.resource_mut::<WaveRuntime>().cursor = usize::MAX;
	Bed {
		h,
		player,
		records: Vec::new(),
	}
}

impl Bed {
	fn run(&mut self, ticks: u32) {
		for _ in 0..ticks {
			self.h.step(TickInput::default());
			let tick = self.h.tick();
			self.records.extend(
				self.h
					.drain_presentation()
					.into_iter()
					.map(|r| (tick, r.kind)),
			);
		}
	}

	fn world(&mut self) -> &mut World {
		self.h.app_mut().world_mut()
	}

	fn boss(&mut self) -> Option<(Entity, DVec2)> {
		let w = self.world();
		w.query_filtered::<(Entity, &Pose2), With<BossAi>>()
			.iter(w)
			.next()
			.map(|(e, p)| (e, p.0))
	}

	fn ticks_of(&self, f: impl Fn(&PresentationKind) -> bool) -> Vec<u64> {
		self.records
			.iter()
			.filter(|(_, k)| f(k))
			.map(|(t, _)| *t)
			.collect()
	}

	fn damage(&self) -> Vec<(u64, u32)> {
		self.records
			.iter()
			.filter_map(|(t, k)| match k {
				PresentationKind::PlayerDamaged { damage, .. } => Some((*t, *damage)),
				_ => None,
			})
			.collect()
	}
}

fn started(k: &PresentationKind) -> bool {
	matches!(k, PresentationKind::CueStarted { .. })
}

fn spawned(k: &PresentationKind) -> bool {
	matches!(k, PresentationKind::HostileProjectileSpawned { .. })
}

#[test]
fn boss_is_absent_and_untargetable_until_its_120_tick_warning_ends() {
	let mut b = bed(5, DVec2::ZERO);
	b.run(119);
	assert!(b.boss().is_none(), "pending bosses are not entities");
	let w = b.world();
	assert_eq!(w.query::<&BossWarning>().iter(w).count(), 1);
	assert_eq!(w.resource::<WaveRuntime>().boss, BossPhase::Warning);
	b.run(1);
	let (_, at) = b.boss().expect("materialized at wave tick 120");
	assert!(
		(at - DVec2::new(0.0, 250.0)).length() < 1.0,
		"spawned at (0, 250): {at:?}"
	);
	assert_eq!(b.world().resource::<WaveRuntime>().boss, BossPhase::Alive);
}

#[test]
fn da18_boss_warning_keeps_postponing_without_a_cap_and_blocks_completion() {
	let mut b = bed(5, DVec2::new(0.0, 240.0));
	b.run(120 + 30 * 14);
	assert!(
		b.boss().is_none(),
		"occupied spawn: no cancel after ten postponements"
	);
	let w = b.world();
	let warning = w.query::<&BossWarning>().iter(w).next().cloned().unwrap();
	assert!(warning.postponements > 10);
	b.run(3600);
	assert!(
		!b.world().resource::<WaveRuntime>().completed,
		"pending boss blocks completion"
	);
	let player = b.player;
	b.world().get_mut::<Pose2>(player).unwrap().0 = DVec2::new(0.0, -200.0);
	b.run(31);
	assert!(b.boss().is_some(), "spawns once the zone is clear");
}

#[test]
fn press_stamp_follows_a_60_tick_recovery_and_locks_on_the_player() {
	let mut b = bed(5, DVec2::ZERO);
	b.run(179);
	assert!(b.ticks_of(started).is_empty());
	b.run(1);
	assert_eq!(
		b.ticks_of(started),
		vec![180],
		"60 recovery ticks after materializing at 120"
	);
	let shape = b.records.iter().find_map(|(_, k)| match k {
		PresentationKind::CueStarted {
			shape,
			anchor,
			ticks,
			..
		} => Some((*shape, *anchor, *ticks)),
		_ => None,
	});
	assert_eq!(
		shape,
		Some((
			CueShapeFact::Circle { radius: 90.0 },
			CueAnchorFact::Locked([0.0, 0.0]),
			72
		))
	);
	let frozen = b.boss().unwrap().1;
	b.run(71);
	assert_eq!(b.boss().unwrap().1, frozen, "windup freezes movement");
	b.run(1);
	assert_eq!(
		b.damage(),
		vec![(252, 32)],
		"raw damage 32, no wave scaling"
	);
}

#[test]
fn press_alternates_to_the_ram_and_charge_damage_replaces_contact() {
	let mut b = bed(5, DVec2::ZERO);
	b.run(313);
	assert_eq!(
		b.ticks_of(started),
		vec![180, 313],
		"stamp then ram after 60 recovery ticks"
	);
	b.run(60 + 40);
	let hits: Vec<_> = b.damage().into_iter().filter(|h| h.1 == 30).collect();
	assert_eq!(hits.len(), 1, "{:?}", b.damage());
	b.run(72 + 20);
	let (boss, _) = b.boss().unwrap();
	assert_eq!(
		b.world().get::<EnemyStats>(boss).unwrap().contact_damage,
		18
	);
}

#[test]
fn foreman_holds_260_and_alternates_volley_then_discharge() {
	let mut b = bed(10, DVec2::ZERO);
	b.run(120 + 89);
	assert!(b.ticks_of(started).is_empty());
	b.run(1);
	assert_eq!(
		b.ticks_of(started),
		vec![210],
		"materializes at 120, recovers 90, then winds up"
	);
	b.run(60);
	assert_eq!(
		b.ticks_of(spawned),
		vec![270; 5],
		"five projectiles after a 60-tick windup"
	);
	b.run(61 + 90);
	assert_eq!(
		b.ticks_of(started),
		vec![210, 331],
		"discharge begins after 60 recovery ticks"
	);
	assert_eq!(b.ticks_of(spawned).len(), 5 + 12);
	let boss = b.boss().unwrap().1;
	assert!(
		(boss.distance(DVec2::ZERO) - 250.0).abs() < 31.0,
		"keeps near 260: {boss:?}"
	);
}

#[test]
fn killing_the_boss_releases_wave_completion_only_after_the_timer() {
	let mut b = bed(5, DVec2::ZERO);
	b.run(200);
	let (boss, _) = b.boss().unwrap();
	b.world().despawn(boss);
	b.run(1);
	assert_eq!(
		b.world().resource::<WaveRuntime>().boss,
		BossPhase::Defeated
	);
	assert!(
		!b.world().resource::<WaveRuntime>().completed,
		"timer still running"
	);
	b.run(3600);
	assert!(b.world().resource::<WaveRuntime>().completed);
}

#[test]
fn a_live_boss_at_timer_expiry_keeps_the_wave_open() {
	let mut b = bed(5, DVec2::ZERO);
	b.run(3600 + 10);
	assert!(b.boss().is_some());
	assert!(
		!b.world().resource::<WaveRuntime>().completed,
		"DA-12: boss alive stays in Wave"
	);
}

#[test]
fn boss_takes_no_knockback_from_player_attacks() {
	let mut b = bed(5, DVec2::ZERO);
	let player = b.player;
	b.world().get_mut::<Loadout>(player).unwrap().slots[0] = Some(EquippedWeapon {
		weapon_id: "wpn_bolt".into(),
		tier: 1,
	});
	b.run(120);
	let hp_before = {
		let (boss, _) = b.boss().unwrap();
		b.world().get::<Health>(boss).unwrap().current
	};
	b.run(55);
	let (boss, at) = b.boss().unwrap();
	let hp_after = b.world().get::<Health>(boss).unwrap().current;
	assert!(hp_after < hp_before, "the bolt landed");
	assert!(at.y <= 250.0, "a hit never pushes the boss away: {at:?}");
}
