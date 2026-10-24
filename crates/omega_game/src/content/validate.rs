//! Cross-record validation: references, global ID uniqueness, profile counts,
//! wave/boss rules, unlock reachability and award coverage (GDD-012, GDD-013).

use super::model::*;
use super::{Catalogs, ContentError, Profile};
use std::collections::{BTreeMap, BTreeSet};

/// Player hurtbox radius shared by every character (GDD-004).
pub const PLAYER_RADIUS_LU: f64 = 14.0;
/// Visible warning before a common enemy materializes (GDD-011).
pub const COMMON_WARNING_TICKS: u32 = 45;

const PROTOTYPE_CHARACTERS: &[&str] = &["char_patch"];
const PROTOTYPE_WEAPONS: &[&str] = &["wpn_bolt", "wpn_saw"];
const PROTOTYPE_ENEMIES: &[&str] = &["enemy_chaser", "enemy_runner"];
const PROTOTYPE_ITEMS: &[&str] = &[
	"item_plating",
	"item_brace",
	"item_coupler",
	"item_ratchet",
	"item_bearings",
	"item_sight",
	"item_sealant",
	"item_magnet",
];

struct Check<'a> {
	errors: &'a mut Vec<ContentError>,
}

impl Check<'_> {
	fn fail(&mut self, file: &str, path: impl AsRef<str>, message: impl Into<String>) {
		self.errors
			.push(ContentError::new(file, path.as_ref(), message.into()));
	}
}

pub fn validate(c: &Catalogs, profile: Profile, errors: &mut Vec<ContentError>) {
	let mut ck = Check { errors };
	unique_ids(c, &mut ck);
	counts(c, profile, &mut ck);
	characters(c, &mut ck);
	weapons(c, &mut ck);
	arenas(c, &mut ck);
	waves(c, profile, &mut ck);
	shop(c, &mut ck);
	unlocks(c, &mut ck);
	coverage(c, &mut ck);
}

fn unique_ids(c: &Catalogs, ck: &mut Check) {
	let mut seen: BTreeMap<&str, &str> = BTreeMap::new();
	let families: [(&str, Vec<&str>); 9] = [
		(
			"characters.json",
			c.characters.keys().map(String::as_str).collect(),
		),
		(
			"weapons.json",
			c.weapons.keys().map(String::as_str).collect(),
		),
		("items.json", c.items.keys().map(String::as_str).collect()),
		(
			"enemies.json",
			c.enemies.keys().map(String::as_str).collect(),
		),
		("bosses.json", c.bosses.keys().map(String::as_str).collect()),
		("arenas.json", c.arenas.keys().map(String::as_str).collect()),
		(
			"waves.json",
			c.waves.values().map(|w| w.id.as_str()).collect(),
		),
		(
			"unlocks.json",
			c.unlocks.keys().map(String::as_str).collect(),
		),
		("shop.json", vec![c.shop.id.as_str()]),
	];
	for (file, ids) in families {
		for id in ids {
			if let Some(first) = seen.insert(id, file) {
				ck.fail(
					file,
					format!("{id}.id"),
					format!("id is already used in {first}"),
				);
			}
		}
	}
}

fn expect_count(ck: &mut Check, file: &str, actual: usize, expected: usize) {
	if actual != expected {
		ck.fail(
			file,
			"records",
			format!("profile requires exactly {expected} records, found {actual}"),
		);
	}
}

fn expect_set<'a>(
	ck: &mut Check,
	file: &str,
	actual: impl Iterator<Item = &'a String>,
	expected: &[&str],
) {
	let actual: BTreeSet<&str> = actual.map(String::as_str).collect();
	let expected: BTreeSet<&str> = expected.iter().copied().collect();
	if actual != expected {
		ck.fail(
			file,
			"records",
			format!("prototype profile requires exactly {expected:?}, found {actual:?}"),
		);
	}
}

fn counts(c: &Catalogs, profile: Profile, ck: &mut Check) {
	match profile {
		Profile::Full => {
			expect_count(ck, "characters.json", c.characters.len(), 4);
			expect_count(ck, "weapons.json", c.weapons.len(), 12);
			expect_count(ck, "enemies.json", c.enemies.len(), 10);
			expect_count(ck, "bosses.json", c.bosses.len(), 2);
			expect_count(ck, "items.json", c.items.len(), 40);
			expect_count(ck, "waves.json", c.waves.len(), 10);
			expect_count(ck, "unlocks.json", c.unlocks.len(), 4);
		}
		Profile::Prototype => {
			expect_set(
				ck,
				"characters.json",
				c.characters.keys(),
				PROTOTYPE_CHARACTERS,
			);
			expect_set(ck, "weapons.json", c.weapons.keys(), PROTOTYPE_WEAPONS);
			expect_set(ck, "enemies.json", c.enemies.keys(), PROTOTYPE_ENEMIES);
			expect_set(ck, "items.json", c.items.keys(), PROTOTYPE_ITEMS);
			expect_count(ck, "bosses.json", c.bosses.len(), 0);
			expect_count(ck, "waves.json", c.waves.len(), 3);
			expect_count(ck, "unlocks.json", c.unlocks.len(), 0);
		}
	}
	expect_count(ck, "arenas.json", c.arenas.len(), 1);
}

fn characters(c: &Catalogs, ck: &mut Check) {
	for ch in c.characters.values() {
		let Some(w) = c.weapons.get(&ch.starting_weapon_id) else {
			ck.fail(
				"characters.json",
				format!("{}.starting_weapon_id", ch.id),
				"unknown weapon",
			);
			continue;
		};
		// The starting weapon must be available whenever the character is.
		if !w.availability.is_initial() && w.availability != ch.availability {
			ck.fail(
				"characters.json",
				format!("{}.starting_weapon_id", ch.id),
				format!(
					"weapon `{}` is not available when this character unlocks",
					w.id
				),
			);
		}
	}
	if !c.characters.values().any(|ch| ch.availability.is_initial()) {
		ck.fail(
			"characters.json",
			"records",
			"at least one character must be initial",
		);
	}
}

fn weapons(c: &Catalogs, ck: &mut Check) {
	for w in c.weapons.values() {
		if let WeaponPayload::Projectile { angles_degrees, .. }
		| WeaponPayload::ExplosiveProjectile { angles_degrees, .. } = &w.payload
			&& angles_degrees.len() > 16
		{
			ck.fail(
				"weapons.json",
				format!("{}.payload.angles_degrees", w.id),
				"at most 16 projectiles",
			);
		}
	}
}

fn arenas(c: &Catalogs, ck: &mut Check) {
	for a in c.arenas.values() {
		let size = a.bounds.size();
		let [aw, ah] = a.camera_aspect;
		// Exact aspect check without floating division: width * ah == height * aw.
		if (size.x * f64::from(ah) - size.y * f64::from(aw)).abs() > 1e-9 {
			ck.fail(
				"arenas.json",
				format!("{}.camera_aspect", a.id),
				"bounds do not match camera aspect",
			);
		}
		let inner = a
			.bounds
			.clamp_circle_center(a.player_spawn, PLAYER_RADIUS_LU);
		if inner != a.player_spawn {
			ck.fail(
				"arenas.json",
				format!("{}.player_spawn", a.id),
				"spawn lacks full body clearance",
			);
		}
	}
}

fn waves(c: &Catalogs, profile: Profile, ck: &mut Check) {
	let expected_numbers: BTreeSet<u32> = (1..=c.waves.len() as u32).collect();
	let numbers: BTreeSet<u32> = c.waves.keys().copied().collect();
	if numbers != expected_numbers {
		ck.fail(
			"waves.json",
			"records",
			"wave numbers must cover 1..=N without gaps",
		);
	}
	let mut boss_uses: BTreeMap<&str, u32> = BTreeMap::new();
	for w in c.waves.values() {
		let p = |f: &str| format!("{}.{f}", w.id);
		if !c.arenas.contains_key(&w.arena_id) {
			ck.fail("waves.json", p("arena_id"), "unknown arena");
		}
		if w.batches.is_empty() {
			ck.fail(
				"waves.json",
				p("batches"),
				"a wave needs at least one batch",
			);
		}
		for (i, b) in w.batches.iter().enumerate() {
			if b.request_tick + COMMON_WARNING_TICKS >= w.duration_ticks {
				ck.fail(
					"waves.json",
					p(&format!("batches[{i}].request_tick")),
					"request plus warning must end before the wave",
				);
			}
			for (j, (id, _)) in b.enemies.iter().enumerate() {
				if !c.enemies.contains_key(id) {
					ck.fail(
						"waves.json",
						p(&format!("batches[{i}].enemies[{j}].id")),
						format!("unknown enemy `{id}`"),
					);
				}
			}
		}
		let boss_wave = profile == Profile::Full && (w.number == 5 || w.number == 10);
		match (&w.boss, boss_wave) {
			(Some(spawn), true) => match c.bosses.get(&spawn.boss_id) {
				Some(boss) => {
					*boss_uses.entry(boss.id.as_str()).or_default() += 1;
					if spawn.request_tick + boss.warning_ticks >= w.duration_ticks {
						ck.fail(
							"waves.json",
							p("boss_request_tick"),
							"boss request plus warning must end before the wave",
						);
					}
				}
				None => ck.fail(
					"waves.json",
					p("boss_id"),
					format!("unknown boss `{}`", spawn.boss_id),
				),
			},
			(Some(_), false) => ck.fail(
				"waves.json",
				p("boss_id"),
				"bosses appear only on waves 5 and 10 of the full profile",
			),
			(None, true) => ck.fail("waves.json", p("boss_id"), "waves 5 and 10 require a boss"),
			(None, false) => {}
		}
	}
	for b in c.bosses.values() {
		if boss_uses.get(b.id.as_str()).copied().unwrap_or(0) != 1 {
			ck.fail(
				"bosses.json",
				format!("{}.id", b.id),
				"a boss must be referenced by exactly one wave",
			);
		}
	}
}

fn shop(c: &Catalogs, ck: &mut Check) {
	let s = &c.shop;
	let expected = [
		("offer_count", s.offer_count, 4),
		("weapon_quota", s.weapon_quota, 2),
		("item_quota", s.item_quota, 2),
		("refresh_limit", s.refresh_limit, 1),
	];
	for (field, actual, want) in expected {
		if actual != want {
			ck.fail(
				"shop.json",
				format!("{}.{field}", s.id),
				format!("must be {want} (GDD-008)"),
			);
		}
	}
}

fn unlocks(c: &Catalogs, ck: &mut Check) {
	let max_wave = c.waves.len() as u32;
	let mut awarded: BTreeMap<&str, &str> = BTreeMap::new();
	for u in c.unlocks.values() {
		let UnlockCondition::CompleteWave(n) = u.condition;
		if n == 0 || n > max_wave {
			ck.fail(
				"unlocks.json",
				format!("{}.condition.wave", u.id),
				"wave is not reachable in this profile",
			);
		}
		for (i, reward) in u.reward_ids.iter().enumerate() {
			let path = format!("{}.reward_ids[{i}]", u.id);
			let availability = c
				.characters
				.get(reward)
				.map(|r| &r.availability)
				.or_else(|| c.weapons.get(reward).map(|r| &r.availability))
				.or_else(|| c.items.get(reward).map(|r| &r.availability));
			match availability {
				None => ck.fail("unlocks.json", path, format!("unknown reward `{reward}`")),
				Some(Availability::Initial) => ck.fail(
					"unlocks.json",
					path,
					format!("`{reward}` is initial and cannot be awarded"),
				),
				Some(Availability::Unlock(owner)) if owner != &u.id => ck.fail(
					"unlocks.json",
					path,
					format!("`{reward}` declares unlock `{owner}`"),
				),
				Some(_) => {}
			}
			if awarded.insert(reward, &u.id).is_some() {
				ck.fail(
					"unlocks.json",
					format!("{}.reward_ids[{i}]", u.id),
					format!("`{reward}` is awarded more than once"),
				);
			}
		}
	}
	let gated = c
		.characters
		.values()
		.map(|r| ("characters.json", &r.id, &r.availability))
		.chain(
			c.weapons
				.values()
				.map(|r| ("weapons.json", &r.id, &r.availability)),
		)
		.chain(
			c.items
				.values()
				.map(|r| ("items.json", &r.id, &r.availability)),
		);
	for (file, id, availability) in gated {
		if let Availability::Unlock(u) = availability {
			if !c.unlocks.contains_key(u) {
				ck.fail(
					file,
					format!("{id}.unlock_id"),
					format!("unknown unlock `{u}`"),
				);
			} else if !awarded.contains_key(id.as_str()) {
				ck.fail(
					file,
					format!("{id}.unlock_id"),
					format!("unlock `{u}` does not award it"),
				);
			}
		}
	}
}

fn coverage(c: &Catalogs, ck: &mut Check) {
	let spawned: BTreeSet<&str> = c
		.waves
		.values()
		.flat_map(|w| w.batches.iter())
		.flat_map(|b| b.enemies.iter().map(|(id, _)| id.as_str()))
		.collect();
	for id in c.enemies.keys() {
		if !spawned.contains(id.as_str()) {
			ck.fail(
				"enemies.json",
				format!("{id}.id"),
				"enemy appears in no wave",
			);
		}
	}
}
