use super::metrics::{KillRecord, RunRecord, ShopRecord, WaveRecord};
use super::observe::Observer;
use super::policy::{Agent, Policy, ShopAction, ShopObs};
use super::seed_for;
use crate::GameCorePlugin;
use crate::combat::actor::{ArenaRes, ContentRes, Loadout, Player, Scrap};
use crate::content::{ContentSnapshot, Profile, load_dir};
use crate::shop::{Offer, RunBuild, ShopCommand, ShopOutcome, ShopVisit, apply};
use crate::sim::run::{Run, RunState, begin_run, start_next_wave};
use crate::sim::{PresentationKind, SimHarness};
use bevy::prelude::*;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::sync::Arc;

/// A full run with no player input ends well inside this; it only guards a stuck loop.
const TICK_CAP: u64 = 1_000_000;
const MAX_CONSECUTIVE_REJECTS: u32 = 3;

pub fn load_content(dir: &Path, profile: Profile) -> Result<Arc<ContentSnapshot>, String> {
	load_dir(dir, profile)
		.map(Arc::new)
		.map_err(|e| format!("cannot load content from {}: {e:?}", dir.display()))
}

fn offer_id(offer: &Offer) -> Option<String> {
	match offer {
		Offer::Weapon { id, .. } | Offer::Item { id, .. } => Some(id.clone()),
		Offer::Sold | Offer::Empty => None,
	}
}

struct WaveCtx {
	wave: u32,
	start_tick: u64,
	start_scrap: u32,
	damage: u32,
	kills: u32,
	weapons: Vec<String>,
	tiers: Vec<u8>,
	items: Vec<String>,
	energy: [u8; 4],
}

fn open_wave(world: &mut World, wave: u32, tick: u64) -> WaveCtx {
	let loadout = world
		.query_filtered::<&Loadout, With<Player>>()
		.iter(world)
		.next()
		.cloned();
	let mut equipped: Vec<(String, u8)> = loadout
		.iter()
		.flat_map(|l| l.slots.iter().flatten())
		.map(|w| (w.weapon_id.clone(), w.tier))
		.collect();
	equipped.sort();
	WaveCtx {
		wave,
		start_tick: tick,
		start_scrap: world.resource::<Scrap>().0,
		damage: 0,
		kills: 0,
		weapons: equipped.iter().map(|(id, _)| id.clone()).collect(),
		tiers: equipped.iter().map(|(_, tier)| *tier).collect(),
		items: world.resource::<RunBuild>().items.iter().cloned().collect(),
		energy: loadout.map_or([0; 4], |l| l.energy.slots()),
	}
}

fn close_wave(world: &mut World, ctx: &WaveCtx, tick: u64, cleared: bool) -> WaveRecord {
	let hp_end = world
		.query_filtered::<&crate::combat::actor::Health, With<Player>>()
		.iter(world)
		.next()
		.map_or(0, |h| h.current);
	WaveRecord {
		wave: ctx.wave,
		cleared,
		combat_ticks: tick - ctx.start_tick,
		damage_taken: ctx.damage,
		scrap_earned: world.resource::<Scrap>().0.saturating_sub(ctx.start_scrap),
		kills: ctx.kills,
		hp_end,
		weapons: ctx.weapons.clone(),
		tiers: ctx.tiers.clone(),
		items: ctx.items.clone(),
		energy: ctx.energy,
	}
}

fn shop_phase(world: &mut World, agent: &mut Agent) -> ShopRecord {
	agent.begin_shop();
	let scrap_before = world.resource::<Scrap>().0;
	let visit_number = world.resource::<ShopVisit>().visit;
	let mut record = ShopRecord {
		visit: visit_number,
		scrap_before,
		spent: 0,
		unspent: 0,
		refreshed: false,
		offered: world
			.resource::<ShopVisit>()
			.offers
			.iter()
			.filter_map(offer_id)
			.collect(),
		bought: Vec::new(),
	};
	let mut rejects = 0;
	loop {
		let visit = world.resource::<ShopVisit>().clone();
		let scrap = world.resource::<Scrap>().0;
		let loadout = world
			.query_filtered::<&Loadout, With<Player>>()
			.iter(world)
			.next()
			.cloned();
		let Some(loadout) = loadout else { break };
		let obs = ShopObs {
			scrap,
			visit: &visit,
			loadout: &loadout,
		};
		let Some(action) = agent.shop_action(&obs) else {
			break;
		};
		let revision = visit.revision;
		let command = match action {
			ShopAction::Buy {
				offer,
				replace_slot,
			} => ShopCommand::Buy {
				offer,
				revision,
				replace_slot,
			},
			ShopAction::Refresh => ShopCommand::Refresh { revision },
			ShopAction::Energy { slot, add: true } => ShopCommand::AddEnergy { slot, revision },
			ShopAction::Energy { slot, add: false } => ShopCommand::RemoveEnergy { slot, revision },
		};
		match apply(world, command) {
			Ok(
				ShopOutcome::Bought { offer, price } | ShopOutcome::Replaced { offer, price, .. },
			) => {
				rejects = 0;
				record.spent += price;
				if let Some(id) = visit.offers.get(offer).and_then(offer_id) {
					record.bought.push(id);
				}
			}
			Ok(ShopOutcome::Refreshed { price }) => {
				rejects = 0;
				record.spent += price;
				record.refreshed = true;
				record.offered.extend(
					world
						.resource::<ShopVisit>()
						.offers
						.iter()
						.filter_map(offer_id),
				);
			}
			Ok(ShopOutcome::EnergyChanged) => rejects = 0,
			Err(_) => {
				rejects += 1;
				if rejects >= MAX_CONSECUTIVE_REJECTS {
					break;
				}
			}
		}
	}
	record.unspent = world.resource::<Scrap>().0;
	record
}

/// A bot that left every weapon unpowered re-powers the first one, as a player must.
fn start_wave_powered(world: &mut World) -> Result<(), String> {
	if start_next_wave(world).is_ok() {
		return Ok(());
	}
	for slot in 0..4 {
		let revision = world.resource::<ShopVisit>().revision;
		if apply(world, ShopCommand::AddEnergy { slot, revision }).is_ok() {
			break;
		}
	}
	start_next_wave(world).map_err(|e| format!("cannot start the next wave: {e:?}"))
}

pub fn run_one(
	content: &Arc<ContentSnapshot>,
	character: &str,
	policy: Policy,
	seed_index: u64,
) -> Result<RunRecord, String> {
	let seed = seed_for(seed_index);
	// Every prototype wave uses the one arena; the full game will read it per wave.
	let arena = content
		.catalogs
		.arenas
		.values()
		.next()
		.ok_or("content has no arena")?
		.bounds;
	let mut app = App::new();
	app.add_plugins(GameCorePlugin);
	let mut harness = SimHarness::with_app(app, seed);
	let world = harness.app_mut().world_mut();
	world.insert_resource(ContentRes(content.clone()));
	world.insert_resource(ArenaRes(arena));
	begin_run(world, character, BTreeSet::new()).map_err(|e| format!("cannot begin run: {e:?}"))?;
	let mut observer = Observer::new(world);
	let mut agent = Agent::new(
		policy,
		content.clone(),
		seed,
		seed_index,
		(arena.min(), arena.max()),
	);
	agent.begin_wave(1);
	let mut ctx = open_wave(world, 1, 0);
	let mut waves = Vec::new();
	let mut shops = Vec::new();
	let mut kills = Vec::new();
	let mut spawned: BTreeMap<u64, (String, u64)> = BTreeMap::new();
	let mut combat_ticks = 0u64;
	loop {
		let state = harness.app().world().resource::<Run>().state;
		match state {
			RunState::Wave => {
				if harness.tick() > TICK_CAP {
					return Err(format!("{policy} seed {seed_index} exceeded the tick cap"));
				}
				let world = harness.app_mut().world_mut();
				let seen = observer.observe(world);
				let input = agent.tick(world, &seen);
				harness.step(input);
				for rec in harness.drain_presentation() {
					match rec.kind {
						PresentationKind::EnemySpawned { sim_id, .. } => {
							let kind = observer
								.enemy_kind(harness.app().world(), sim_id)
								.unwrap_or_else(|| "unknown".into());
							spawned.insert(sim_id, (kind, rec.tick));
						}
						PresentationKind::EnemyDied { target, .. } => {
							if let Some((enemy, born)) = spawned.remove(&target) {
								ctx.kills += 1;
								kills.push(KillRecord {
									enemy,
									wave: ctx.wave,
									ttk_ticks: rec.tick - born,
								});
							}
						}
						PresentationKind::PlayerDamaged { damage, .. } => ctx.damage += damage,
						_ => {}
					}
				}
			}
			RunState::Shop => {
				let tick = harness.tick();
				let world = harness.app_mut().world_mut();
				waves.push(close_wave(world, &ctx, tick, true));
				combat_ticks += tick - ctx.start_tick;
				spawned.clear();
				shops.push(shop_phase(world, &mut agent));
				start_wave_powered(world)?;
				let next = world.resource::<Run>().wave;
				agent.begin_wave(next);
				ctx = open_wave(world, next, tick);
			}
			RunState::Victory | RunState::Defeat | RunState::Summary => {
				let tick = harness.tick();
				let victory = state == RunState::Victory;
				let world = harness.app_mut().world_mut();
				waves.push(close_wave(world, &ctx, tick, victory));
				combat_ticks += tick - ctx.start_tick;
				let run = world.resource::<Run>().clone();
				let waves_cleared = run.result.map_or(0, |r| r.waves_completed);
				return Ok(RunRecord {
					policy,
					seed_index,
					style: agent.style_name(),
					victory,
					waves_cleared,
					combat_ticks,
					final_tick: tick,
					waves,
					shops,
					kills,
				});
			}
		}
	}
}
