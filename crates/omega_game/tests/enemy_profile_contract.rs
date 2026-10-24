//! SP-008 / GDD 10.1: the ten common enemy profiles through the real `GameTick` schedule.
//! Tick numbers are relative to the enemy's first processed tick, which is tick 1 here.

use bevy::math::DVec2;
use bevy::prelude::*;
use omega_game::combat::actor::*;
use omega_game::content::{Profile, load_dir};
use omega_game::encounters::ai::{AiState, EnemyAi};
use omega_game::encounters::hostile::HostileProjectile;
use omega_game::sim::*;
use std::path::PathBuf;
use std::sync::Arc;

struct Bed {
	h: SimHarness,
	player: Entity,
	records: Vec<(u64, PresentationKind)>,
}

fn bed(player_at: DVec2) -> Bed {
	let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/content/full");
	let content = load_dir(&dir, Profile::Full).expect("full fixture");
	let arena = content.catalogs.arenas["arena_foundry"].bounds;
	let mut app = App::new();
	app.add_plugins(omega_game::GameCorePlugin);
	let mut h = SimHarness::with_app(app, RunSeed::from_hex(&"77".repeat(32)).unwrap());
	h.app_mut().insert_resource(ContentRes(Arc::new(content)));
	h.app_mut().insert_resource(ArenaRes(arena));
	let player = spawn_player(h.app_mut().world_mut(), "char_patch", player_at).unwrap();
	let world = h.app_mut().world_mut();
	world.get_mut::<Loadout>(player).unwrap().slots[0] = None;
	let mut health = world.get_mut::<Health>(player).unwrap();
	health.current = 1_000_000;
	health.max = 1_000_000;
	Bed {
		h,
		player,
		records: Vec::new(),
	}
}

impl Bed {
	fn enemy(&mut self, id: &str, at: DVec2) -> Entity {
		let world = self.h.app_mut().world_mut();
		let content = world.resource::<ContentRes>().0.clone();
		let def = &content.catalogs.enemies[id];
		let e = spawn_enemy(world, id, at, 1_000_000, def.contact_damage).unwrap();
		world
			.entity_mut(e)
			.insert(EnemyAi::new(def.behavior.clone(), 1, def.contact_damage));
		e
	}

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

	fn pose(&self, e: Entity) -> DVec2 {
		self.h.app().world().get::<Pose2>(e).unwrap().0
	}

	fn world(&mut self) -> &mut World {
		self.h.app_mut().world_mut()
	}

	fn ticks_of(&self, f: impl Fn(&PresentationKind) -> bool) -> Vec<u64> {
		self.records
			.iter()
			.filter(|(_, k)| f(k))
			.map(|(t, _)| *t)
			.collect()
	}

	fn damage_to_player(&self) -> Vec<(u64, u32)> {
		self.records
			.iter()
			.filter_map(|(t, k)| match k {
				PresentationKind::PlayerDamaged { damage, .. } => Some((*t, *damage)),
				_ => None,
			})
			.collect()
	}

	fn count<T: Component>(&mut self) -> usize {
		let w = self.world();
		w.query::<&T>().iter(w).count()
	}
}

fn cue_started(k: &PresentationKind) -> bool {
	matches!(k, PresentationKind::CueStarted { .. })
}

fn cue_ended(k: &PresentationKind) -> bool {
	matches!(
		k,
		PresentationKind::CueEnded {
			cancelled: false,
			..
		}
	)
}

fn spawned(k: &PresentationKind) -> bool {
	matches!(k, PresentationKind::HostileProjectileSpawned { .. })
}

#[test]
fn gunner_approaches_retreats_and_holds_inside_the_220_band() {
	let mut b = bed(DVec2::ZERO);
	let far = b.enemy("enemy_gunner", DVec2::new(400.0, 0.0));
	let near = b.enemy("enemy_gunner", DVec2::new(0.0, 100.0));
	let held = b.enemy("enemy_gunner", DVec2::new(-230.0, 0.0));
	b.run(60);
	assert!(
		(b.pose(far).x - (400.0 - 55.0)).abs() < 1e-6,
		"approach at 55 LU/s"
	);
	assert!(
		(b.pose(near).y - (100.0 + 55.0)).abs() < 1e-6,
		"retreat below 200"
	);
	assert_eq!(
		b.pose(held),
		DVec2::new(-230.0, 0.0),
		"inside 200..240 it holds"
	);
}

#[test]
fn gunner_first_windup_follows_one_full_interval_then_freezes_and_emits_aimed() {
	let mut b = bed(DVec2::ZERO);
	let e = b.enemy("enemy_gunner", DVec2::new(220.0, 0.0));
	b.run(150);
	assert!(
		b.ticks_of(cue_started).is_empty(),
		"no windup before a full interval"
	);
	b.run(1);
	assert_eq!(b.ticks_of(cue_started), vec![151]);
	let locked = b.pose(e);
	b.run(29);
	assert_eq!(b.pose(e), locked, "windup freezes movement");
	assert!(b.ticks_of(spawned).is_empty());
	b.run(1);
	assert_eq!(b.ticks_of(cue_ended), vec![181]);
	assert_eq!(b.ticks_of(spawned), vec![181]);
	let aim = b.records.iter().find_map(|(_, k)| match k {
		PresentationKind::HostileProjectileSpawned {
			direction, origin, ..
		} => Some((*direction, *origin)),
		_ => None,
	});
	assert_eq!(aim, Some(([-1.0, 0.0], [220.0, 0.0])));
	b.run(29);
	assert_eq!(b.pose(e), locked, "recovery freezes movement too");
	b.run(1);
	b.run(120);
	assert_eq!(
		b.ticks_of(cue_started),
		vec![151, 301],
		"next windup one interval after the last start"
	);
}

#[test]
fn gunner_projectile_hits_the_player_for_the_authored_damage_once() {
	let mut b = bed(DVec2::ZERO);
	b.enemy("enemy_gunner", DVec2::new(220.0, 0.0));
	b.run(181 + 70);
	let hits = b.damage_to_player();
	assert_eq!(hits.len(), 1, "{hits:?}");
	assert_eq!(hits[0].1, 8);
	assert_eq!(b.count::<HostileProjectile>(), 0, "consumed on impact");
}

#[test]
fn out_of_range_attacker_stays_ready_and_keeps_moving() {
	let mut b = bed(DVec2::new(-560.0, 0.0));
	let e = b.enemy("enemy_gunner", DVec2::new(580.0, 0.0));
	b.run(200);
	assert!(b.ticks_of(cue_started).is_empty());
	assert!(b.pose(e).x < 580.0, "it keeps seeking range");
}

#[test]
fn scatter_fires_three_projectiles_around_the_locked_aim_under_one_attack_id() {
	let mut b = bed(DVec2::ZERO);
	b.enemy("enemy_scatter", DVec2::new(180.0, 0.0));
	b.run(196);
	assert_eq!(b.ticks_of(cue_started), vec![151]);
	assert_eq!(b.ticks_of(spawned), vec![196, 196, 196]);
	let shots: Vec<(u64, [f64; 2])> = b
		.records
		.iter()
		.filter_map(|(_, k)| match k {
			PresentationKind::HostileProjectileSpawned {
				attack, direction, ..
			} => Some((*attack, *direction)),
			_ => None,
		})
		.collect();
	assert!(shots.iter().all(|s| s.0 == shots[0].0), "one attack ID");
	let angles: Vec<f64> = shots
		.iter()
		.map(|(_, d)| d[1].atan2(d[0]).to_degrees())
		.collect();
	let rel: Vec<f64> = angles
		.iter()
		.map(|a| ((a - 180.0 + 540.0) % 360.0) - 180.0)
		.collect();
	for expected in [-20.0, 0.0, 20.0] {
		assert!(rel.iter().any(|a| (a - expected).abs() < 1e-6), "{rel:?}");
	}
	b.run(60);
	let hits = b.damage_to_player();
	assert_eq!(
		hits.len(),
		1,
		"pellets of one attack cannot multiply damage"
	);
	assert_eq!(hits[0].1, 10);
}

#[test]
fn sentry_stays_put_and_fires_an_eight_way_world_frame_volley() {
	let mut b = bed(DVec2::new(-300.0, 0.0));
	let e = b.enemy("enemy_sentry", DVec2::new(200.0, 100.0));
	b.run(226);
	assert_eq!(
		b.ticks_of(cue_started),
		vec![181],
		"interval 180 from materialization"
	);
	assert_eq!(b.ticks_of(spawned), vec![226; 8], "windup 45");
	assert_eq!(b.pose(e), DVec2::new(200.0, 100.0));
	let mut angles: Vec<i64> = b
		.records
		.iter()
		.filter_map(|(_, k)| match k {
			PresentationKind::HostileProjectileSpawned { direction, .. } => Some(
				direction[1]
					.atan2(direction[0])
					.to_degrees()
					.rem_euclid(360.0)
					.round() as i64,
			),
			_ => None,
		})
		.collect();
	angles.sort_unstable();
	assert_eq!(angles, vec![0, 45, 90, 135, 180, 225, 270, 315]);
}

#[test]
fn skimmer_orbits_clockwise_at_180_and_shoots() {
	let mut b = bed(DVec2::ZERO);
	let e = b.enemy("enemy_skimmer", DVec2::new(180.0, 0.0));
	b.run(60);
	let p = b.pose(e);
	assert!(p.y < -50.0, "clockwise from east heads south: {p:?}");
	assert!(
		(p.length() - 180.0).abs() < 6.0,
		"stays near the orbit radius: {}",
		p.length()
	);
	b.run(61);
	assert_eq!(b.ticks_of(cue_started), vec![121], "interval 120");
	b.run(24);
	assert_eq!(b.ticks_of(spawned), vec![145], "windup 24");
}

#[test]
fn rammer_waits_winds_up_charges_with_charge_damage_and_resets() {
	let mut b = bed(DVec2::ZERO);
	let e = b.enemy("enemy_rammer", DVec2::new(200.0, 0.0));
	b.run(60);
	assert!(
		b.ticks_of(cue_started).is_empty(),
		"waits 60 ticks after materialization"
	);
	b.run(1);
	assert_eq!(b.ticks_of(cue_started), vec![61]);
	let at_lock = b.pose(e);
	b.run(45);
	assert_eq!(b.pose(e), at_lock, "windup holds still");
	assert!(matches!(
		b.world().get::<EnemyAi>(e).unwrap().state,
		AiState::Charging { .. }
	));
	b.run(24 + 2);
	let hits = b.damage_to_player();
	assert_eq!(hits.len(), 1, "{hits:?}");
	assert_eq!(hits[0].1, 24, "charge contact damage replaces the base 10");
	let travelled = at_lock.x - b.pose(e).x;
	assert!(
		(travelled - 168.0).abs() < 1e-6,
		"420 LU/s for 24 ticks: {travelled}"
	);
	b.run(45);
	assert_eq!(
		b.world().get::<EnemyStats>(e).unwrap().contact_damage,
		10,
		"base contact restored"
	);
	b.run(60);
	assert_eq!(
		b.ticks_of(cue_started).len(),
		2,
		"wait resets to 60 and repeats"
	);
}

#[test]
fn rammer_charge_ends_at_a_wall_without_extra_damage_area() {
	let mut b = bed(DVec2::new(-570.0, 0.0));
	let e = b.enemy("enemy_rammer", DVec2::new(-400.0, 0.0));
	b.run(61 + 45 + 30);
	let x = b.pose(e).x;
	assert!(
		(x - (-600.0 + 18.0)).abs() < 1e-6,
		"stopped at the wall inset: {x}"
	);
	assert!(matches!(
		b.world().get::<EnemyAi>(e).unwrap().state,
		AiState::Recovery { .. } | AiState::Moving
	));
}

#[test]
fn da17_crusher_strike_stays_on_the_locked_center_when_either_actor_moves() {
	let mut b = bed(DVec2::ZERO);
	let e = b.enemy("enemy_crusher", DVec2::new(60.0, 0.0));
	b.run(151);
	assert_eq!(b.ticks_of(cue_started), vec![151]);
	let anchor = b.records.iter().find_map(|(_, k)| match k {
		PresentationKind::CueStarted { anchor, shape, .. } => Some((*anchor, *shape)),
		_ => None,
	});
	assert_eq!(
		anchor,
		Some((
			CueAnchorFact::Locked([0.0, 0.0]),
			CueShapeFact::Circle { radius: 50.0 }
		))
	);
	b.world().get_mut::<Pose2>(e).unwrap().0 = DVec2::new(300.0, 200.0);
	b.run(54);
	let strikes: Vec<_> = b
		.damage_to_player()
		.into_iter()
		.filter(|h| h.1 == 22)
		.collect();
	assert_eq!(
		strikes.len(),
		1,
		"player stood on the locked center: {:?}",
		b.damage_to_player()
	);
}

#[test]
fn crusher_strike_misses_a_player_who_left_the_locked_circle() {
	let mut b = bed(DVec2::ZERO);
	b.enemy("enemy_crusher", DVec2::new(60.0, 0.0));
	b.run(151);
	let player = b.player;
	b.world().get_mut::<Pose2>(player).unwrap().0 = DVec2::new(-200.0, 0.0);
	b.run(54);
	assert!(b.damage_to_player().iter().all(|h| h.1 != 22));
}

#[test]
fn marksman_beam_follows_the_displaced_origin() {
	let mut b = bed(DVec2::ZERO);
	let e = b.enemy("enemy_marksman", DVec2::new(300.0, 0.0));
	b.run(181);
	assert_eq!(b.ticks_of(cue_started), vec![181]);
	let follow = b.records.iter().find_map(|(_, k)| match k {
		PresentationKind::CueStarted { anchor, .. } => Some(*anchor),
		_ => None,
	});
	assert_eq!(
		follow,
		Some(CueAnchorFact::Follow {
			direction: [-1.0, 0.0]
		})
	);
	b.world().get_mut::<Pose2>(e).unwrap().0 = DVec2::new(300.0, 100.0);
	b.run(72);
	assert!(
		b.damage_to_player().iter().all(|h| h.1 != 18),
		"lane moved with the attacker and now misses the player"
	);
}

#[test]
fn marksman_beam_hits_the_player_for_18_when_not_displaced() {
	let mut b = bed(DVec2::ZERO);
	b.enemy("enemy_marksman", DVec2::new(300.0, 0.0));
	b.run(181 + 72);
	let beams: Vec<_> = b
		.damage_to_player()
		.into_iter()
		.filter(|h| h.1 == 18)
		.collect();
	assert_eq!(beams.len(), 1);
	assert_eq!(beams[0].0, 253);
}

#[test]
fn dead_attacker_cancels_its_windup_without_emitting() {
	let mut b = bed(DVec2::ZERO);
	let e = b.enemy("enemy_gunner", DVec2::new(220.0, 0.0));
	b.run(160);
	b.world().despawn(e);
	b.run(60);
	assert!(b.ticks_of(spawned).is_empty());
	let cancelled = b.ticks_of(|k| {
		matches!(
			k,
			PresentationKind::CueEnded {
				cancelled: true,
				..
			}
		)
	});
	assert_eq!(cancelled.len(), 1);
}

#[test]
fn projectile_is_consumed_even_when_invulnerability_rejects_its_damage() {
	let mut b = bed(DVec2::ZERO);
	let player = b.player;
	b.world()
		.get_mut::<Invulnerability>(player)
		.unwrap()
		.next_accept_tick = u64::MAX;
	b.enemy("enemy_gunner", DVec2::new(220.0, 0.0));
	b.run(181 + 70);
	assert!(b.damage_to_player().is_empty());
	assert_eq!(b.count::<HostileProjectile>(), 0);
}

#[test]
fn hostile_damage_scales_with_the_wave_formula() {
	let mut b = bed(DVec2::ZERO);
	let e = b.enemy("enemy_gunner", DVec2::new(220.0, 0.0));
	b.world().get_mut::<EnemyAi>(e).unwrap().wave = 4;
	b.run(181 + 70);
	assert_eq!(b.damage_to_player()[0].1, 10, "ceil(8 * 1.24)");
}

#[test]
fn plate_hauler_takes_a_quarter_of_the_weapon_knockback() {
	let mut b = bed(DVec2::ZERO);
	let player = b.player;
	let content = b.world().resource::<ContentRes>().0.clone();
	b.world().get_mut::<Loadout>(player).unwrap().slots[0] = Some(EquippedWeapon {
		weapon_id: "wpn_bolt".into(),
		tier: 1,
	});
	let knockback = content.catalogs.weapons["wpn_bolt"].knockback_lu;
	let e = spawn_enemy(
		b.world(),
		"enemy_bulwark",
		DVec2::new(100.0, 0.0),
		1_000_000,
		0,
	)
	.unwrap();
	b.run(9);
	let moved = b.pose(e).x - 100.0;
	assert!(
		(moved - knockback * 0.25).abs() < 1e-6,
		"{moved} vs {}",
		knockback * 0.25
	);
}
