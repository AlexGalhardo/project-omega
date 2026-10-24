//! Remappable action bindings (GDD section 15.2): conflict detection, a Back action that
//! always stays reachable, and restore-defaults. Inputs are stable names such as `KeyW`
//! or `GamepadSouth`, so the map persists in settings as plain strings.

use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Action {
	MoveUp,
	MoveDown,
	MoveLeft,
	MoveRight,
	ManualAim,
	Back,
	Confirm,
	OpenEnergy,
}

impl Action {
	pub const ALL: [Action; 8] = [
		Action::MoveUp,
		Action::MoveDown,
		Action::MoveLeft,
		Action::MoveRight,
		Action::ManualAim,
		Action::Back,
		Action::Confirm,
		Action::OpenEnergy,
	];

	pub fn name(self) -> &'static str {
		match self {
			Action::MoveUp => "move_up",
			Action::MoveDown => "move_down",
			Action::MoveLeft => "move_left",
			Action::MoveRight => "move_right",
			Action::ManualAim => "manual_aim",
			Action::Back => "back",
			Action::Confirm => "confirm",
			Action::OpenEnergy => "open_energy",
		}
	}

	pub fn from_name(name: &str) -> Option<Self> {
		Self::ALL.into_iter().find(|a| a.name() == name)
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BindError {
	/// The input already triggers another action.
	Conflict {
		with: Action,
	},
	/// Back must keep at least one binding.
	BackUnreachable,
	UnknownAction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bindings {
	map: BTreeMap<Action, Vec<String>>,
}

impl Default for Bindings {
	fn default() -> Self {
		let table: [(Action, &[&str]); 8] = [
			(Action::MoveUp, &["KeyW", "ArrowUp"]),
			(Action::MoveDown, &["KeyS", "ArrowDown"]),
			(Action::MoveLeft, &["KeyA", "ArrowLeft"]),
			(Action::MoveRight, &["KeyD", "ArrowRight"]),
			(Action::ManualAim, &["MouseRight"]),
			(Action::Back, &["Escape", "GamepadEast", "GamepadMenu"]),
			(Action::Confirm, &["Enter", "Space", "GamepadSouth"]),
			(Action::OpenEnergy, &["KeyE", "GamepadRightShoulder"]),
		];
		Self {
			map: table
				.into_iter()
				.map(|(a, inputs)| (a, inputs.iter().map(|s| (*s).to_string()).collect()))
				.collect(),
		}
	}
}

impl Bindings {
	pub fn inputs(&self, action: Action) -> &[String] {
		self.map.get(&action).map_or(&[], Vec::as_slice)
	}

	pub fn action_for(&self, input: &str) -> Option<Action> {
		self.map
			.iter()
			.find(|(_, inputs)| inputs.iter().any(|i| i == input))
			.map(|(a, _)| *a)
	}

	/// Replaces every binding of `action` with `input`, unless another action owns it.
	pub fn rebind(&mut self, action: Action, input: &str) -> Result<(), BindError> {
		if let Some(owner) = self.action_for(input)
			&& owner != action
		{
			return Err(BindError::Conflict { with: owner });
		}
		self.map.insert(action, vec![input.to_string()]);
		Ok(())
	}

	/// Removes one input; Back keeps at least one.
	pub fn unbind(&mut self, action: Action, input: &str) -> Result<(), BindError> {
		let inputs = self.map.entry(action).or_default();
		if action == Action::Back && inputs.len() <= 1 && inputs.iter().any(|i| i == input) {
			return Err(BindError::BackUnreachable);
		}
		inputs.retain(|i| i != input);
		Ok(())
	}

	pub fn restore_defaults(&mut self) {
		*self = Self::default();
	}

	/// Only the differences from the defaults, as stored in settings.
	pub fn to_settings(&self) -> BTreeMap<String, String> {
		let defaults = Self::default();
		self.map
			.iter()
			.filter(|(a, inputs)| defaults.map.get(a) != Some(inputs))
			.map(|(a, inputs)| (a.name().to_string(), inputs.join(",")))
			.collect()
	}

	/// Applies stored overrides; unknown actions are reported and left out.
	pub fn from_settings(stored: &BTreeMap<String, String>) -> (Self, Vec<String>) {
		let mut bindings = Self::default();
		let mut unknown = Vec::new();
		for (name, inputs) in stored {
			match Action::from_name(name) {
				Some(action) => {
					bindings.map.insert(
						action,
						inputs
							.split(',')
							.filter(|s| !s.is_empty())
							.map(str::to_string)
							.collect(),
					);
				}
				None => unknown.push(name.clone()),
			}
		}
		if bindings.inputs(Action::Back).is_empty() {
			bindings
				.map
				.insert(Action::Back, Self::default().map[&Action::Back].clone());
		}
		(bindings, unknown)
	}
}
