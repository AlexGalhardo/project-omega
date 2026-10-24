//! Typed content records (GDD-012, revision 2) and their per-record parsing.
//! Cross-record rules (references, counts, reachability) live in `validate`.

use super::reader::{R, Rec};
use bevy::math::DVec2;
use omega_2d::geometry::Bounds2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StatId {
	MaxHp,
	Armor,
	DamagePct,
	AttackSpeedPct,
	MoveSpeedPct,
	CritPct,
	/// Tenths of HP per second.
	RegenTenths,
	PickupRadius,
}

impl StatId {
	const NAMES: [&'static str; 8] = [
		"max_hp",
		"armor",
		"damage_pct",
		"attack_speed_pct",
		"move_speed_pct",
		"crit_pct",
		"regen_tenths",
		"pickup_radius",
	];

	fn parse(s: &str) -> Self {
		match s {
			"max_hp" => Self::MaxHp,
			"armor" => Self::Armor,
			"damage_pct" => Self::DamagePct,
			"attack_speed_pct" => Self::AttackSpeedPct,
			"move_speed_pct" => Self::MoveSpeedPct,
			"crit_pct" => Self::CritPct,
			"regen_tenths" => Self::RegenTenths,
			_ => Self::PickupRadius,
		}
	}
}

/// Signed additive modifier in the stat's integer unit (GDD-006).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatModifier {
	pub stat: StatId,
	pub value: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Availability {
	Initial,
	Unlock(String),
}

impl Availability {
	pub fn is_initial(&self) -> bool {
		matches!(self, Self::Initial)
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct Character {
	pub id: String,
	pub display_name: String,
	pub visual_id: String,
	pub starting_weapon_id: String,
	pub modifier: StatModifier,
	pub availability: Availability,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WeaponPayload {
	Projectile {
		speed_lu_s: f64,
		radius_lu: f64,
		pierce: u32,
		angles_degrees: Vec<f64>,
	},
	ExplosiveProjectile {
		speed_lu_s: f64,
		radius_lu: f64,
		explosion_radius_lu: f64,
		angles_degrees: Vec<f64>,
	},
	Capsule {
		reach_lu: f64,
		radius_lu: f64,
	},
	Circle {
		offset_lu: f64,
		radius_lu: f64,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub struct Weapon {
	pub id: String,
	pub display_name: String,
	pub visual_id: String,
	pub base_damage: u32,
	/// Hundredths of attacks per second (CR-002).
	pub base_rate_centi: u32,
	pub range_lu: f64,
	pub windup_ticks: u32,
	pub recovery_ticks: u32,
	pub payload: WeaponPayload,
	pub knockback_lu: f64,
	pub base_price: u32,
	pub availability: Availability,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Item {
	pub id: String,
	pub display_name: String,
	pub icon_visual_id: String,
	pub modifiers: Vec<StatModifier>,
	pub base_price: u32,
	pub availability: Availability,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AimFrame {
	/// Angles relative to the aim locked at windup start.
	Locked,
	/// Angles in world space (radial volleys).
	World,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HostilePayload {
	Projectiles {
		aim: AimFrame,
		angles_degrees: Vec<f64>,
		speed_lu_s: f64,
		radius_lu: f64,
		range_lu: f64,
		damage: u32,
	},
	/// One-tick capsule from the attacker's current origin along the locked aim.
	Beam {
		reach_lu: f64,
		radius_lu: f64,
		damage: u32,
	},
	/// Circle centered on the player's position locked at windup start.
	GroundCircle { radius_lu: f64, damage: u32 },
	/// Swept body charge along the locked direction until the tick limit or a wall.
	Charge {
		speed_lu_s: f64,
		ticks: u32,
		contact_damage: u32,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnemyAttack {
	/// `None` means any distance.
	pub start_range_lu: Option<f64>,
	pub interval_ticks: u32,
	pub windup_ticks: u32,
	pub recovery_ticks: u32,
	pub payload: HostilePayload,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Behavior {
	Chase,
	KeepDistance {
		desired_distance_lu: f64,
		attack: EnemyAttack,
	},
	Orbit {
		orbit_radius_lu: f64,
		attack: EnemyAttack,
	},
	Stationary {
		attack: EnemyAttack,
	},
	ChaseStrike {
		attack: EnemyAttack,
	},
	Charge {
		initial_wait_ticks: u32,
		trigger_range_lu: f64,
		windup_ticks: u32,
		recovery_ticks: u32,
		charge: HostilePayload,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub struct Enemy {
	pub id: String,
	pub display_name: String,
	pub visual_id: String,
	pub base_hp: u32,
	pub armor: u32,
	pub radius_lu: f64,
	pub speed_lu_s: f64,
	pub contact_damage: u32,
	pub scrap: u32,
	pub knockback_multiplier: f64,
	pub behavior: Behavior,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BossAttack {
	pub id: String,
	pub windup_ticks: u32,
	pub recovery_ticks: u32,
	pub payload: HostilePayload,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RecoveryMovement {
	Chase {
		speed_lu_s: f64,
	},
	KeepDistance {
		speed_lu_s: f64,
		desired_distance_lu: f64,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub struct Boss {
	pub id: String,
	pub display_name: String,
	pub visual_id: String,
	pub hp: u32,
	pub armor: u32,
	pub radius_lu: f64,
	pub contact_damage: u32,
	pub scrap: u32,
	pub spawn_position: DVec2,
	pub warning_ticks: u32,
	pub initial_recovery_ticks: u32,
	pub recovery_movement: RecoveryMovement,
	pub attacks: Vec<BossAttack>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Arena {
	pub id: String,
	pub display_name: String,
	pub visual_id: String,
	pub bounds: Bounds2,
	pub player_spawn: DVec2,
	pub camera_aspect: [u32; 2],
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpawnBatch {
	pub request_tick: u32,
	pub count: u32,
	/// `(enemy_id, weight)` in authored order.
	pub enemies: Vec<(String, u32)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BossSpawn {
	pub boss_id: String,
	pub request_tick: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Wave {
	pub id: String,
	pub number: u32,
	pub arena_id: String,
	pub duration_ticks: u32,
	pub batches: Vec<SpawnBatch>,
	pub boss: Option<BossSpawn>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShopRules {
	pub id: String,
	pub offer_count: u32,
	pub weapon_quota: u32,
	pub item_quota: u32,
	pub refresh_limit: u32,
	pub refresh_cost_formula_id: String,
	pub price_formula_id: String,
	pub selection_policy_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnlockCondition {
	/// Successfully complete this wave alive in any one run.
	CompleteWave(u32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unlock {
	pub id: String,
	pub display_name: String,
	pub condition: UnlockCondition,
	pub reward_ids: Vec<String>,
}

// Per-record parsing. Ranges here are the GDD-012 domains; references and counts
// are checked after every family has parsed.

const MAX_LU: f64 = 100_000.0;
const MAX_TICKS: u32 = 1_000_000;
const MAX_AMOUNT: u32 = 1_000_000;

fn availability(r: &mut Rec) -> R<Availability> {
	let initial = match r.opt("initial") {
		None => false,
		Some(_) => r.bool("initial")?,
	};
	match (initial, r.opt_id("unlock_id")?) {
		(true, None) => Ok(Availability::Initial),
		(false, Some(u)) => Ok(Availability::Unlock(u)),
		(true, Some(_)) => Err(r.error(
			"unlock_id",
			"an initial record cannot also require an unlock",
		)),
		(false, None) => Err(r.error("initial", "set initial=true or provide unlock_id")),
	}
}

fn modifier(mut r: Rec) -> R<StatModifier> {
	let stat = StatId::parse(&r.choice("stat", &StatId::NAMES)?);
	let value = r.int("value", -1000, 1000)? as i32;
	if value == 0 {
		return Err(r.error("value", "modifier must not be zero"));
	}
	r.finish()?;
	Ok(StatModifier { stat, value })
}

pub fn character(mut r: Rec) -> R<Character> {
	let c = Character {
		id: r.id("id")?,
		display_name: r.string("display_name")?,
		visual_id: r.id("visual_id")?,
		starting_weapon_id: r.id("starting_weapon_id")?,
		modifier: modifier(r.object("modifier")?)?,
		availability: availability(&mut r)?,
	};
	r.finish()?;
	Ok(c)
}

fn angles(r: &mut Rec) -> R<Vec<f64>> {
	r.numbers("angles_degrees", -360.0, 360.0)
}

fn weapon_payload(mut r: Rec) -> R<WeaponPayload> {
	let kind = r.choice(
		"kind",
		&["projectile", "explosive_projectile", "capsule", "circle"],
	)?;
	let p = match kind.as_str() {
		"projectile" => WeaponPayload::Projectile {
			speed_lu_s: r.positive("speed_lu_s")?,
			radius_lu: r.positive("radius_lu")?,
			pierce: r.uint("pierce", 0, 3)?,
			angles_degrees: angles(&mut r)?,
		},
		"explosive_projectile" => WeaponPayload::ExplosiveProjectile {
			speed_lu_s: r.positive("speed_lu_s")?,
			radius_lu: r.positive("radius_lu")?,
			explosion_radius_lu: r.positive("explosion_radius_lu")?,
			angles_degrees: angles(&mut r)?,
		},
		"capsule" => WeaponPayload::Capsule {
			reach_lu: r.positive("reach_lu")?,
			radius_lu: r.positive("radius_lu")?,
		},
		_ => WeaponPayload::Circle {
			offset_lu: r.number("offset_lu", 0.0, MAX_LU)?,
			radius_lu: r.positive("radius_lu")?,
		},
	};
	r.finish()?;
	Ok(p)
}

pub fn weapon(mut r: Rec) -> R<Weapon> {
	let w = Weapon {
		id: r.id("id")?,
		display_name: r.string("display_name")?,
		visual_id: r.id("visual_id")?,
		base_damage: r.uint("base_damage", 1, MAX_AMOUNT)?,
		base_rate_centi: r.uint("base_rate_centi", 1, 100_000)?,
		range_lu: r.positive("range_lu")?,
		windup_ticks: r.uint("windup_ticks", 0, MAX_TICKS)?,
		recovery_ticks: r.uint("recovery_ticks", 0, MAX_TICKS)?,
		payload: weapon_payload(r.object("payload")?)?,
		knockback_lu: r.number("knockback_lu", 0.0, MAX_LU)?,
		base_price: r.uint("base_price", 1, MAX_AMOUNT)?,
		availability: availability(&mut r)?,
	};
	r.finish()?;
	Ok(w)
}

pub fn item(mut r: Rec) -> R<Item> {
	let id = r.id("id")?;
	let display_name = r.string("display_name")?;
	let icon_visual_id = r.id("icon_visual_id")?;
	let mods = r.objects("modifiers")?;
	if !(1..=2).contains(&mods.len()) {
		return Err(r.error("modifiers", "an item has one or two modifiers"));
	}
	let modifiers = mods.into_iter().map(modifier).collect::<R<Vec<_>>>()?;
	if modifiers.len() == 2 && modifiers[0].stat == modifiers[1].stat {
		return Err(r.error("modifiers", "an item cannot repeat a stat"));
	}
	let i = Item {
		id,
		display_name,
		icon_visual_id,
		modifiers,
		base_price: r.uint("base_price", 1, MAX_AMOUNT)?,
		availability: availability(&mut r)?,
	};
	r.finish()?;
	Ok(i)
}

pub fn hostile_payload(mut r: Rec) -> R<HostilePayload> {
	let kind = r.choice("kind", &["projectiles", "beam", "ground_circle", "charge"])?;
	let p = match kind.as_str() {
		"projectiles" => HostilePayload::Projectiles {
			aim: match r.choice("aim", &["locked", "world"])?.as_str() {
				"locked" => AimFrame::Locked,
				_ => AimFrame::World,
			},
			angles_degrees: angles(&mut r)?,
			speed_lu_s: r.positive("speed_lu_s")?,
			radius_lu: r.positive("radius_lu")?,
			range_lu: r.positive("range_lu")?,
			damage: r.uint("damage", 1, MAX_AMOUNT)?,
		},
		"beam" => HostilePayload::Beam {
			reach_lu: r.positive("reach_lu")?,
			radius_lu: r.positive("radius_lu")?,
			damage: r.uint("damage", 1, MAX_AMOUNT)?,
		},
		"ground_circle" => HostilePayload::GroundCircle {
			radius_lu: r.positive("radius_lu")?,
			damage: r.uint("damage", 1, MAX_AMOUNT)?,
		},
		_ => HostilePayload::Charge {
			speed_lu_s: r.positive("speed_lu_s")?,
			ticks: r.uint("ticks", 1, MAX_TICKS)?,
			contact_damage: r.uint("contact_damage", 1, MAX_AMOUNT)?,
		},
	};
	r.finish()?;
	Ok(p)
}

fn enemy_attack(mut r: Rec) -> R<EnemyAttack> {
	let start_range_lu = match r.opt("start_range_lu") {
		None | Some(serde_json::Value::Null) => None,
		Some(_) => Some(r.positive("start_range_lu")?),
	};
	let a = EnemyAttack {
		start_range_lu,
		interval_ticks: r.uint("interval_ticks", 1, MAX_TICKS)?,
		windup_ticks: r.uint("windup_ticks", 0, MAX_TICKS)?,
		recovery_ticks: r.uint("recovery_ticks", 0, MAX_TICKS)?,
		payload: hostile_payload(r.object("payload")?)?,
	};
	if matches!(a.payload, HostilePayload::Charge { .. }) {
		return Err(r.error(
			"payload.kind",
			"charge is a behavior profile, not a timed attack",
		));
	}
	r.finish()?;
	Ok(a)
}

fn behavior(mut r: Rec) -> R<Behavior> {
	let profile = r.choice(
		"profile",
		&[
			"chase",
			"keep_distance",
			"orbit",
			"stationary",
			"chase_strike",
			"charge",
		],
	)?;
	let b = match profile.as_str() {
		"chase" => Behavior::Chase,
		"keep_distance" => Behavior::KeepDistance {
			desired_distance_lu: r.positive("desired_distance_lu")?,
			attack: enemy_attack(r.object("attack")?)?,
		},
		"orbit" => Behavior::Orbit {
			orbit_radius_lu: r.positive("orbit_radius_lu")?,
			attack: enemy_attack(r.object("attack")?)?,
		},
		"stationary" => Behavior::Stationary {
			attack: enemy_attack(r.object("attack")?)?,
		},
		"chase_strike" => Behavior::ChaseStrike {
			attack: enemy_attack(r.object("attack")?)?,
		},
		_ => {
			let charge = hostile_payload(r.object("charge")?)?;
			if !matches!(charge, HostilePayload::Charge { .. }) {
				return Err(r.error("charge.kind", "charge profile requires a charge payload"));
			}
			Behavior::Charge {
				initial_wait_ticks: r.uint("initial_wait_ticks", 0, MAX_TICKS)?,
				trigger_range_lu: r.positive("trigger_range_lu")?,
				windup_ticks: r.uint("windup_ticks", 0, MAX_TICKS)?,
				recovery_ticks: r.uint("recovery_ticks", 0, MAX_TICKS)?,
				charge,
			}
		}
	};
	r.finish()?;
	Ok(b)
}

pub fn enemy(mut r: Rec) -> R<Enemy> {
	let e = Enemy {
		id: r.id("id")?,
		display_name: r.string("display_name")?,
		visual_id: r.id("visual_id")?,
		base_hp: r.uint("base_hp", 1, MAX_AMOUNT)?,
		armor: r.uint("armor", 0, 80)?,
		radius_lu: r.positive("radius_lu")?,
		speed_lu_s: r.number("speed_lu_s", 0.0, MAX_LU)?,
		contact_damage: r.uint("contact_damage", 1, MAX_AMOUNT)?,
		scrap: r.uint("scrap", 1, MAX_AMOUNT)?,
		knockback_multiplier: r.number("knockback_multiplier", 0.0, 1.0)?,
		behavior: behavior(r.object("behavior")?)?,
	};
	r.finish()?;
	Ok(e)
}

fn point(r: &mut Rec, key: &str) -> R<DVec2> {
	let v = r.numbers(key, -MAX_LU, MAX_LU)?;
	match v[..] {
		[x, y] => Ok(DVec2::new(x, y)),
		_ => Err(r.error(key, "expected [x, y]")),
	}
}

fn recovery_movement(mut r: Rec) -> R<RecoveryMovement> {
	let kind = r.choice("kind", &["chase", "keep_distance"])?;
	let m = match kind.as_str() {
		"chase" => RecoveryMovement::Chase {
			speed_lu_s: r.positive("speed_lu_s")?,
		},
		_ => RecoveryMovement::KeepDistance {
			speed_lu_s: r.positive("speed_lu_s")?,
			desired_distance_lu: r.positive("desired_distance_lu")?,
		},
	};
	r.finish()?;
	Ok(m)
}

fn boss_attack(mut r: Rec) -> R<BossAttack> {
	let a = BossAttack {
		id: r.id("id")?,
		windup_ticks: r.uint("windup_ticks", 1, MAX_TICKS)?,
		recovery_ticks: r.uint("recovery_ticks", 0, MAX_TICKS)?,
		payload: hostile_payload(r.object("payload")?)?,
	};
	if matches!(a.payload, HostilePayload::Beam { .. }) {
		return Err(r.error(
			"payload.kind",
			"boss attacks support projectiles, ground_circle and charge",
		));
	}
	r.finish()?;
	Ok(a)
}

pub fn boss(mut r: Rec) -> R<Boss> {
	let id = r.id("id")?;
	let display_name = r.string("display_name")?;
	let visual_id = r.id("visual_id")?;
	let hp = r.uint("hp", 1, MAX_AMOUNT)?;
	let armor = r.uint("armor", 0, 80)?;
	let radius_lu = r.positive("radius_lu")?;
	let contact_damage = r.uint("contact_damage", 1, MAX_AMOUNT)?;
	let scrap = r.uint("scrap", 1, MAX_AMOUNT)?;
	let spawn_position = point(&mut r, "spawn_position")?;
	let warning_ticks = r.uint("warning_ticks", 1, MAX_TICKS)?;
	let initial_recovery_ticks = r.uint("initial_recovery_ticks", 0, MAX_TICKS)?;
	let recovery_movement = recovery_movement(r.object("recovery_movement")?)?;
	let attacks = r.objects("attacks")?;
	if attacks.is_empty() {
		return Err(r.error("attacks", "a boss needs at least one attack"));
	}
	let b = Boss {
		id,
		display_name,
		visual_id,
		hp,
		armor,
		radius_lu,
		contact_damage,
		scrap,
		spawn_position,
		warning_ticks,
		initial_recovery_ticks,
		recovery_movement,
		attacks: attacks.into_iter().map(boss_attack).collect::<R<_>>()?,
	};
	r.finish()?;
	Ok(b)
}

pub fn arena(mut r: Rec) -> R<Arena> {
	let id = r.id("id")?;
	let display_name = r.string("display_name")?;
	let visual_id = r.id("visual_id")?;
	let mut b = r.object("bounds_lu")?;
	let (min_x, max_x) = (
		b.number("min_x", -MAX_LU, MAX_LU)?,
		b.number("max_x", -MAX_LU, MAX_LU)?,
	);
	let (min_y, max_y) = (
		b.number("min_y", -MAX_LU, MAX_LU)?,
		b.number("max_y", -MAX_LU, MAX_LU)?,
	);
	if min_x >= max_x || min_y >= max_y {
		return Err(b.error("max_x", "bounds must have positive width and height"));
	}
	b.finish()?;
	let bounds = Bounds2::new(DVec2::new(min_x, min_y), DVec2::new(max_x, max_y))
		.map_err(|e| r.error("bounds_lu", e.to_string()))?;
	let player_spawn = point(&mut r, "player_spawn")?;
	let mut aspect = r.object("camera_aspect")?;
	let camera_aspect = [
		aspect.uint("width", 1, 1000)?,
		aspect.uint("height", 1, 1000)?,
	];
	aspect.finish()?;
	r.choice("common_spawn_anchor_policy", &["boundary_eight_clockwise"])?;
	let a = Arena {
		id,
		display_name,
		visual_id,
		bounds,
		player_spawn,
		camera_aspect,
	};
	r.finish()?;
	Ok(a)
}

pub fn wave(mut r: Rec) -> R<Wave> {
	let id = r.id("id")?;
	let number = r.uint("number", 1, 1000)?;
	let arena_id = r.id("arena_id")?;
	let duration_ticks = r.uint("duration_ticks", 1, MAX_TICKS)?;
	let mut batches = Vec::new();
	for mut b in r.objects("batches")? {
		let request_tick = b.uint("request_tick", 0, MAX_TICKS)?;
		let count = b.uint("count", 1, 10_000)?;
		let mut enemies = Vec::new();
		for mut e in b.objects("enemies")? {
			enemies.push((e.id("id")?, e.uint("weight", 1, 1_000_000)?));
			e.finish()?;
		}
		if enemies.is_empty() {
			return Err(b.error("enemies", "a batch needs at least one eligible enemy"));
		}
		b.finish()?;
		batches.push(SpawnBatch {
			request_tick,
			count,
			enemies,
		});
	}
	let boss = match (r.opt_id("boss_id")?, r.opt("boss_request_tick")) {
		(None, None) => None,
		(Some(boss_id), Some(_)) => Some(BossSpawn {
			boss_id,
			request_tick: r.uint("boss_request_tick", 0, MAX_TICKS)?,
		}),
		(Some(_), None) => {
			return Err(r.error("boss_request_tick", "required when boss_id is set (CR-001)"));
		}
		(None, Some(_)) => return Err(r.error("boss_request_tick", "only allowed with boss_id")),
	};
	let w = Wave {
		id,
		number,
		arena_id,
		duration_ticks,
		batches,
		boss,
	};
	r.finish()?;
	Ok(w)
}

pub fn shop_rules(mut r: Rec) -> R<ShopRules> {
	let s = ShopRules {
		id: r.id("id")?,
		offer_count: r.uint("offer_count", 0, 100)?,
		weapon_quota: r.uint("weapon_quota", 0, 100)?,
		item_quota: r.uint("item_quota", 0, 100)?,
		refresh_limit: r.uint("refresh_limit", 0, 100)?,
		refresh_cost_formula_id: r
			.choice("refresh_cost_formula_id", &["refresh_8_plus_2_per_wave"])?,
		price_formula_id: r.choice("price_formula_id", &["inflation_8_pct_per_wave_ceil"])?,
		selection_policy_id: r.choice(
			"selection_policy_id",
			&["uniform_without_replacement_fill_other"],
		)?,
	};
	r.finish()?;
	Ok(s)
}

pub fn unlock(mut r: Rec) -> R<Unlock> {
	let id = r.id("id")?;
	let display_name = r.string("display_name")?;
	let mut c = r.object("condition")?;
	c.choice("kind", &["complete_wave"])?;
	let condition = UnlockCondition::CompleteWave(c.uint("wave", 1, 1000)?);
	c.finish()?;
	let path = r.field_path("reward_ids");
	let file = r.file();
	let reward_ids = r
		.array("reward_ids")?
		.iter()
		.enumerate()
		.map(|(i, v)| {
			v.as_str()
				.filter(|s| super::reader::valid_id(s))
				.map(str::to_string)
				.ok_or_else(|| {
					super::ContentError::new(file, &format!("{path}[{i}]"), "expected id".into())
				})
		})
		.collect::<R<Vec<_>>>()?;
	if reward_ids.is_empty() {
		return Err(r.error("reward_ids", "an unlock needs at least one reward"));
	}
	let u = Unlock {
		id,
		display_name,
		condition,
		reward_ids,
	};
	r.finish()?;
	Ok(u)
}
