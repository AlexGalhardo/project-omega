//! HUD and shop view models (SP-014). Pure reads of the simulation: widgets render these
//! values and send typed commands back; nothing here mutates rules state.

pub mod compare;
pub mod hints;
pub mod menus;

#[cfg(feature = "client")]
pub mod client;

use crate::combat::actor::{ContentRes, Health, Loadout, Player, PlayerStats, Scrap};
use crate::combat::energy::TOTAL_POINTS;
use crate::combat::stats::fire_interval_ticks;
use crate::encounters::WaveRuntime;
use crate::profile::Settings;
use crate::shop::{Offer, ShopCommand, ShopOutcome, ShopReject, ShopVisit, apply, refresh_cost};
use crate::sim::run::{
	Outcome, Run, RunState, StartError, abandon, continue_to_summary, start_next_wave,
};
use crate::sim::{SimPaused, SimTick, TICKS_PER_SECOND};
use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct SlotView {
	pub name: Option<String>,
	pub tier: u8,
	pub points: u8,
	/// Attacks per second at the slot's current energy; `None` when unpowered or empty.
	pub rate: Option<f64>,
}

impl SlotView {
	pub fn pips(&self) -> String {
		(0..3)
			.map(|i| if i < self.points { '#' } else { '-' })
			.collect()
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct HudView {
	pub hp: u32,
	pub max_hp: u32,
	pub wave: u32,
	pub total_waves: u32,
	/// Seconds left, or the boss prompt once the timer expired with a boss pending.
	pub timer: String,
	pub scrap: u32,
	pub slots: [SlotView; 4],
	pub unallocated: u8,
}

pub fn slot_views(world: &mut World) -> Option<([SlotView; 4], u8)> {
	let content = world.resource::<ContentRes>().0.clone();
	let (loadout, stats) = world
		.query_filtered::<(&Loadout, &PlayerStats), With<Player>>()
		.iter(world)
		.next()
		.map(|(l, s)| (l.clone(), s.0))?;
	let points = loadout.energy.slots();
	let slots = std::array::from_fn(|i| match &loadout.slots[i] {
		None => SlotView {
			name: None,
			tier: 0,
			points: 0,
			rate: None,
		},
		Some(equipped) => {
			let weapon = content.catalogs.weapons.get(&equipped.weapon_id);
			let rate = weapon
				.and_then(|w| fire_interval_ticks(w, points[i], stats.attack_speed_pct))
				.filter(|_| points[i] > 0)
				.map(|ticks| f64::from(TICKS_PER_SECOND) / f64::from(ticks));
			SlotView {
				name: weapon.map(|w| w.display_name.clone()),
				tier: equipped.tier,
				points: points[i],
				rate,
			}
		}
	});
	Some((slots, loadout.energy.remainder()))
}

pub fn hud_view(world: &mut World) -> Option<HudView> {
	let run = world.get_resource::<Run>()?.clone();
	let (hp, max_hp) = world
		.query_filtered::<&Health, With<Player>>()
		.iter(world)
		.next()
		.map(|h| (h.current, h.max))?;
	let (slots, unallocated) = slot_views(world)?;
	let timer = world
		.get_resource::<WaveRuntime>()
		.map_or_else(String::new, |w| {
			let elapsed = world.resource::<SimTick>().0.saturating_sub(w.tick0);
			if elapsed >= w.duration {
				if w.boss_blocking {
					"Defeat the boss".to_string()
				} else {
					"0s".to_string()
				}
			} else {
				let left = (w.duration - elapsed).div_ceil(u64::from(TICKS_PER_SECOND));
				format!("{left}s")
			}
		});
	Some(HudView {
		hp,
		max_hp,
		wave: run.wave,
		total_waves: run.total_waves,
		timer,
		scrap: world.get_resource::<Scrap>().map_or(0, |s| s.0),
		slots,
		unallocated,
	})
}

#[derive(Debug, Clone, PartialEq)]
pub struct OfferView {
	pub label: String,
	pub price: Option<u32>,
	pub enabled: bool,
	/// Text reason shown when the offer cannot be bought.
	pub reason: Option<String>,
	/// Comparison lines from the player's actual stats.
	pub detail: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShopView {
	pub scrap: u32,
	pub offers: Vec<OfferView>,
	pub refresh_cost: u32,
	pub refresh_enabled: bool,
	pub refresh_reason: Option<String>,
	pub start_enabled: bool,
	pub start_reason: Option<String>,
	pub slots: [SlotView; 4],
	pub unallocated: u8,
	pub revision: u64,
}

pub fn shop_view(world: &mut World) -> Option<ShopView> {
	if world.get_resource::<Run>()?.state != RunState::Shop {
		return None;
	}
	let visit = world.get_resource::<ShopVisit>()?.clone();
	let content = world.resource::<ContentRes>().0.clone();
	let scrap = world.resource::<Scrap>().0;
	let (slots, unallocated) = slot_views(world)?;
	let offers = visit
		.offers
		.iter()
		.map(|offer| match offer {
			Offer::Empty => OfferView {
				label: "Empty".into(),
				price: None,
				enabled: false,
				reason: Some("Nothing left to offer".into()),
				detail: Vec::new(),
			},
			Offer::Sold => OfferView {
				label: "Sold".into(),
				price: None,
				enabled: false,
				reason: Some("Already bought this visit".into()),
				detail: Vec::new(),
			},
			Offer::Weapon { id, tier, price } => {
				let name = content
					.catalogs
					.weapons
					.get(id)
					.map_or(id.as_str(), |w| w.display_name.as_str());
				let verb = if *tier > 1 { "Upgrade" } else { "New" };
				priced(format!("{verb} {name} (tier {tier})"), *price, scrap)
			}
			Offer::Item { id, price } => {
				let name = content
					.catalogs
					.items
					.get(id)
					.map_or(id.as_str(), |i| i.display_name.as_str());
				priced(format!("Item {name}"), *price, scrap)
			}
		})
		.collect();
	let mut offers: Vec<OfferView> = offers;
	let (loadout, stats) = world
		.query_filtered::<(&Loadout, &PlayerStats), With<Player>>()
		.iter(world)
		.next()
		.map(|(l, s)| (l.clone(), s.0))?;
	for (view, offer) in offers.iter_mut().zip(&visit.offers) {
		view.detail = compare::offer_detail(&content, &loadout, &stats, offer);
	}
	let cost = refresh_cost(visit.visit);
	let (refresh_enabled, refresh_reason) = if visit.refresh_used {
		(false, Some("Refresh already used this visit".to_string()))
	} else if scrap < cost {
		(false, Some(format!("Need {} more scrap", cost - scrap)))
	} else {
		(true, None)
	};
	let powered = slots.iter().any(|s| s.points > 0 && s.name.is_some());
	Some(ShopView {
		scrap,
		offers,
		refresh_cost: cost,
		refresh_enabled,
		refresh_reason,
		start_enabled: powered,
		start_reason: (!powered).then(|| "Give at least one weapon an energy point".to_string()),
		slots,
		unallocated,
		revision: visit.revision,
	})
}

fn priced(label: String, price: u32, scrap: u32) -> OfferView {
	let affordable = scrap >= price;
	OfferView {
		label,
		price: Some(price),
		enabled: affordable,
		reason: (!affordable).then(|| format!("Need {} more scrap", price - scrap)),
		detail: Vec::new(),
	}
}

/// Text block for the HUD, shared by the widget and its tests.
pub fn hud_text(hud: &HudView) -> String {
	let mut text = format!(
		"HP {}/{}   Wave {}/{}   {}   Scrap {}\n",
		hud.hp, hud.max_hp, hud.wave, hud.total_waves, hud.timer, hud.scrap
	);
	for (i, slot) in hud.slots.iter().enumerate() {
		match &slot.name {
			None => text.push_str(&format!("[{}] empty\n", i + 1)),
			Some(name) => text.push_str(&format!(
				"[{}] {} T{} {}{}\n",
				i + 1,
				name,
				slot.tier,
				slot.pips(),
				slot.rate
					.map_or(" unpowered".to_string(), |r| format!(" {r:.2}/s")),
			)),
		}
	}
	text.push_str(&format!("Unallocated {}/{}", hud.unallocated, TOTAL_POINTS));
	text
}

#[derive(Resource, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum UiMode {
	#[default]
	Normal,
	/// Choosing which slot a new weapon replaces.
	Replace(usize),
	ConfirmAbandon,
	/// A first-run note is open; combat stays paused until it is dismissed.
	Hint(hints::Hint),
}

/// Typed requests widgets send to the rules modules.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiAction {
	Buy(usize),
	Refresh,
	Energy { slot: usize, add: bool },
	Start,
	ReplaceSlot(usize),
	CancelReplace,
	Resume,
	AskAbandon,
	ConfirmAbandon,
	CancelAbandon,
	Continue,
	DismissHint,
	SkipAllHints,
	ResetTutorials,
	Menu(menus::MenuAction),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Button {
	pub label: String,
	pub action: UiAction,
	pub enabled: bool,
	/// Draw the next button on the same row (the energy minus beside its plus).
	pub same_row_as_next: bool,
}

/// The full-screen panel for every non-combat state, as plain data.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ScreenView {
	pub title: String,
	pub lines: Vec<String>,
	pub buttons: Vec<Button>,
}

fn button(label: impl Into<String>, action: UiAction, enabled: bool) -> Button {
	Button {
		label: label.into(),
		action,
		enabled,
		same_row_as_next: false,
	}
}

fn pause_screen(world: &mut World) -> Option<ScreenView> {
	let stats = world
		.query_filtered::<&PlayerStats, With<Player>>()
		.iter(world)
		.next()
		.map(|s| s.0)?;
	Some(ScreenView {
		title: "Paused".into(),
		lines: vec![
			format!("Max HP {}   Armor {}", stats.max_hp, stats.armor),
			format!(
				"Damage {:+}%   Attack speed {:+}%",
				stats.damage_pct, stats.attack_speed_pct
			),
			format!(
				"Move speed {:+}%   Crit {}%",
				stats.move_speed_pct, stats.crit_pct
			),
			format!(
				"Regen {:.1} HP/s   Pickup radius {} LU",
				f64::from(stats.regen_tenths) / 10.0,
				stats.pickup_radius
			),
		],
		buttons: vec![
			button("Resume", UiAction::Resume, true),
			button("Abandon", UiAction::AskAbandon, true),
			button("Reset tutorials", UiAction::ResetTutorials, true),
		],
	})
}

fn replace_screen(shop: &ShopView, offer: usize, details: Vec<String>) -> ScreenView {
	let mut buttons: Vec<Button> = shop
		.slots
		.iter()
		.enumerate()
		.map(|(i, s)| {
			button(
				format!(
					"Replace [{}] {} T{}",
					i + 1,
					s.name.as_deref().unwrap_or("empty"),
					s.tier
				),
				UiAction::ReplaceSlot(i),
				s.name.is_some(),
			)
		})
		.collect();
	buttons.push(button("Cancel", UiAction::CancelReplace, true));
	let mut lines = vec![
		format!("{} needs a free slot.", shop.offers[offer].label),
		"Choose the weapon to lose; its energy moves to the new weapon.".into(),
	];
	lines.extend(details);
	ScreenView {
		title: "Replace a weapon".into(),
		lines,
		buttons,
	}
}

fn shop_screen(shop: &ShopView) -> ScreenView {
	let mut lines = vec![format!("Scrap {}", shop.scrap)];
	let mut buttons = Vec::new();
	for (i, offer) in shop.offers.iter().enumerate() {
		let price = offer
			.price
			.map_or(String::new(), |p| format!(" - {p} scrap"));
		buttons.push(button(
			format!("[{}] {}{}", i + 1, offer.label, price),
			UiAction::Buy(i),
			offer.enabled,
		));
		for detail in &offer.detail {
			lines.push(format!("  [{}] {detail}", i + 1));
		}
		if let Some(reason) = &offer.reason {
			lines.push(format!("  [{}] unavailable: {reason}", i + 1));
		}
	}
	buttons.push(button(
		format!("[F] Refresh offers - {} scrap", shop.refresh_cost),
		UiAction::Refresh,
		shop.refresh_enabled,
	));
	if let Some(reason) = &shop.refresh_reason {
		lines.push(format!("  Refresh unavailable: {reason}"));
	}
	lines.push(format!(
		"Reactor: {} of {} points unallocated (unused points grant nothing)",
		shop.unallocated, TOTAL_POINTS
	));
	for (i, slot) in shop.slots.iter().enumerate() {
		let Some(name) = &slot.name else {
			lines.push(format!("Slot {}: empty", i + 1));
			continue;
		};
		let rate = slot
			.rate
			.map_or(" unpowered".to_string(), |r| format!(" {r:.2} attacks/s"));
		lines.push(format!(
			"Slot {}: {} T{} {}{}",
			i + 1,
			name,
			slot.tier,
			slot.pips(),
			rate
		));
		let mut plus = button(
			format!("Slot {} energy +", i + 1),
			UiAction::Energy { slot: i, add: true },
			true,
		);
		plus.same_row_as_next = true;
		buttons.push(plus);
		buttons.push(button(
			format!("Slot {} energy -", i + 1),
			UiAction::Energy {
				slot: i,
				add: false,
			},
			slot.points > 0,
		));
	}
	buttons.push(button(
		"Start next wave",
		UiAction::Start,
		shop.start_enabled,
	));
	if let Some(reason) = &shop.start_reason {
		lines.push(format!("Cannot start: {reason}"));
	}
	ScreenView {
		title: "Shop".into(),
		lines,
		buttons,
	}
}

fn result_screen(run: &Run, state: RunState) -> Option<ScreenView> {
	let result = run.result?;
	let title = match (state, result.outcome) {
		(RunState::Summary, _) => "Summary",
		(_, Outcome::Victory) => "Victory",
		_ => "Defeat",
	};
	Some(ScreenView {
		title: title.into(),
		lines: vec![
			format!("Result: {:?}", result.outcome),
			format!(
				"Waves completed: {} of {}",
				result.waves_completed, run.total_waves
			),
			format!("Scrap held: {}", result.scrap),
		],
		buttons: if state == RunState::Summary {
			Vec::new()
		} else {
			vec![button("Continue", UiAction::Continue, true)]
		},
	})
}

/// The panel to show for the current run state, mode and pause flag; `None` during live combat.
pub fn screen_view(world: &mut World, mode: UiMode, paused: bool) -> Option<ScreenView> {
	if !world.contains_resource::<Run>() {
		return menus::menu_screen(world);
	}
	let run = world.get_resource::<Run>()?.clone();
	if let UiMode::Hint(hint) = mode {
		return Some(ScreenView {
			title: hint.title().into(),
			lines: hint.lines(),
			buttons: vec![
				button("Got it", UiAction::DismissHint, true),
				button("Skip all hints", UiAction::SkipAllHints, true),
			],
		});
	}
	if mode == UiMode::ConfirmAbandon {
		return Some(ScreenView {
			title: "Abandon run?".into(),
			lines: vec!["The build is lost. Earned unlocks are kept.".into()],
			buttons: vec![
				button("Abandon", UiAction::ConfirmAbandon, true),
				button("Keep playing", UiAction::CancelAbandon, true),
			],
		});
	}
	match run.state {
		RunState::Wave if paused => pause_screen(world),
		RunState::Wave => None,
		RunState::Shop => {
			let shop = shop_view(world)?;
			Some(match mode {
				UiMode::Replace(offer) => {
					let details = replacement_details(world, offer);
					replace_screen(&shop, offer, details)
				}
				_ => shop_screen(&shop),
			})
		}
		state => {
			let mut view = result_screen(&run, state)?;
			let (lines, buttons) = menus::result_extras(world, state == RunState::Summary);
			view.lines.extend(lines);
			view.buttons.extend(buttons);
			Some(view)
		}
	}
}

fn replacement_details(world: &mut World, offer: usize) -> Vec<String> {
	let Some(incoming) = world
		.get_resource::<ShopVisit>()
		.and_then(|v| v.offers.get(offer).cloned())
	else {
		return Vec::new();
	};
	let content = world.resource::<ContentRes>().0.clone();
	let Some((loadout, stats)) = world
		.query_filtered::<(&Loadout, &PlayerStats), With<Player>>()
		.iter(world)
		.next()
		.map(|(l, s)| (l.clone(), s.0))
	else {
		return Vec::new();
	};
	compare::replacement_lines(&content, &loadout, &stats, &incoming)
}

/// True when a new-weapon purchase would need a replacement slot (full loadout, weapon not owned).
fn needs_replacement(world: &mut World, offer: usize) -> bool {
	let Some(visit) = world.get_resource::<ShopVisit>().cloned() else {
		return false;
	};
	let Some(Offer::Weapon { id, price, .. }) = visit.offers.get(offer) else {
		return false;
	};
	if world.resource::<Scrap>().0 < *price {
		return false;
	}
	let Some(loadout) = world
		.query_filtered::<&Loadout, With<Player>>()
		.iter(world)
		.next()
		.cloned()
	else {
		return false;
	};
	let owned = loadout.slots.iter().flatten().any(|s| &s.weapon_id == id);
	!owned && loadout.slots.iter().all(Option::is_some)
}

fn shop_message(world: &mut World, command: ShopCommand) -> String {
	match apply(world, command) {
		Ok(ShopOutcome::Bought { .. } | ShopOutcome::Replaced { .. }) => "Purchased.".to_string(),
		Ok(ShopOutcome::Refreshed { .. }) => "Offers refreshed.".to_string(),
		Ok(ShopOutcome::EnergyChanged) => String::new(),
		Err(ShopReject::Insufficient { price, have }) => {
			format!("Need {} more scrap.", price - have)
		}
		Err(ShopReject::RefreshUsed) => "Refresh already used this visit.".to_string(),
		Err(ShopReject::Energy(_)) => {
			"The reactor allows six points and three per powered slot.".to_string()
		}
		Err(other) => format!("Not possible: {other:?}"),
	}
}

/// Runs one widget request against the rules modules and describes the outcome in words.
pub fn apply_action(world: &mut World, action: UiAction) -> String {
	let world_mode = world.get_resource::<UiMode>().copied().unwrap_or_default();
	let revision = world.get_resource::<ShopVisit>().map_or(0, |v| v.revision);
	match action {
		UiAction::Buy(offer) => {
			if needs_replacement(world, offer) {
				world.insert_resource(UiMode::Replace(offer));
				return String::new();
			}
			let command = ShopCommand::Buy {
				offer,
				revision,
				replace_slot: None,
			};
			shop_message(world, command)
		}
		UiAction::ReplaceSlot(slot) => {
			let UiMode::Replace(offer) = *world.resource::<UiMode>() else {
				return String::new();
			};
			world.insert_resource(UiMode::Normal);
			let command = ShopCommand::Buy {
				offer,
				revision,
				replace_slot: Some(slot),
			};
			shop_message(world, command)
		}
		UiAction::CancelReplace | UiAction::CancelAbandon => {
			world.insert_resource(UiMode::Normal);
			String::new()
		}
		UiAction::Refresh => shop_message(world, ShopCommand::Refresh { revision }),
		UiAction::Energy { slot, add } => {
			let command = if add {
				ShopCommand::AddEnergy { slot, revision }
			} else {
				ShopCommand::RemoveEnergy { slot, revision }
			};
			shop_message(world, command)
		}
		UiAction::Start => match start_next_wave(world) {
			Ok(()) => String::new(),
			Err(StartError::PowerRequired) => {
				"Give at least one weapon an energy point first.".to_string()
			}
			Err(other) => format!("Not possible: {other:?}"),
		},
		UiAction::Resume => {
			world.resource_mut::<SimPaused>().0 = false;
			String::new()
		}
		UiAction::AskAbandon => {
			world.insert_resource(UiMode::ConfirmAbandon);
			String::new()
		}
		UiAction::ConfirmAbandon => {
			world.insert_resource(UiMode::Normal);
			world.resource_mut::<SimPaused>().0 = false;
			let _ = abandon(world);
			String::new()
		}
		UiAction::DismissHint | UiAction::SkipAllHints | UiAction::ResetTutorials => {
			if let Some(mut settings) = world.get_resource_mut::<Settings>() {
				match (action, mode_hint(&world_mode)) {
					(UiAction::DismissHint, Some(hint)) => hints::acknowledge(&mut settings, hint),
					(UiAction::SkipAllHints, _) => hints::skip_all(&mut settings),
					(UiAction::ResetTutorials, _) => hints::reset_tutorials(&mut settings),
					_ => {}
				}
				world.insert_resource(SettingsDirty);
			}
			if action != UiAction::ResetTutorials {
				world.insert_resource(UiMode::Normal);
				world.resource_mut::<SimPaused>().0 = false;
			}
			String::new()
		}
		UiAction::Menu(menu) => menus::apply_menu(world, menu),
		UiAction::Continue => {
			let _ = continue_to_summary(world);
			String::new()
		}
	}
}

/// Keyboard and controller focus over the enabled buttons of the current screen.
#[derive(Resource, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct UiFocus {
	pub index: usize,
}

impl UiFocus {
	fn enabled(view: &ScreenView) -> Vec<usize> {
		view.buttons
			.iter()
			.enumerate()
			.filter(|(_, b)| b.enabled)
			.map(|(i, _)| i)
			.collect()
	}

	/// Keeps the focus on an enabled button after the screen changed.
	pub fn clamp(&mut self, view: &ScreenView) {
		let enabled = Self::enabled(view);
		if !enabled.contains(&self.index) {
			self.index = enabled.first().copied().unwrap_or(0);
		}
	}

	/// Moves to the previous or next enabled button, wrapping at the ends.
	pub fn step(&mut self, view: &ScreenView, forward: bool) {
		let enabled = Self::enabled(view);
		if enabled.is_empty() {
			return;
		}
		let at = enabled.iter().position(|&i| i == self.index).unwrap_or(0);
		let next = if forward {
			(at + 1) % enabled.len()
		} else {
			(at + enabled.len() - 1) % enabled.len()
		};
		self.index = enabled[next];
	}

	/// The action under the focus, if that button is enabled.
	pub fn activate(&self, view: &ScreenView) -> Option<UiAction> {
		view.buttons
			.get(self.index)
			.filter(|b| b.enabled)
			.map(|b| b.action)
	}
}

fn mode_hint(mode: &UiMode) -> Option<hints::Hint> {
	match mode {
		UiMode::Hint(hint) => Some(*hint),
		_ => None,
	}
}

/// Set whenever settings changed and a save is owed.
#[derive(Resource, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SettingsDirty;

/// What Back does on the current screen (never buys or starts anything).
pub fn back_action(view: &ScreenView) -> Option<UiAction> {
	view.buttons
		.iter()
		.find(|b| {
			matches!(
				b.action,
				UiAction::CancelReplace
					| UiAction::CancelAbandon
					| UiAction::Resume
					| UiAction::DismissHint
					| UiAction::Menu(menus::MenuAction::Back)
					| UiAction::Menu(menus::MenuAction::CancelReset)
			)
		})
		.map(|b| b.action)
}
