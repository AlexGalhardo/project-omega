//! Reactor energy allocation (GDD-007): six points, 0–3 per occupied slot.
//! Every change returns a new value or a typed rejection; nothing mutates in place.

pub const TOTAL_POINTS: u8 = 6;
pub const MAX_PER_SLOT: u8 = 3;
pub const SLOTS: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnergyError {
	EmptySlot,
	NoFreePoints,
	SlotFull,
	AlreadyZero,
	BadSlot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct EnergyAllocation([u8; SLOTS]);

impl EnergyAllocation {
	/// Callers pass validated values; the invariants are checked in debug builds.
	pub fn new(slots: [u8; SLOTS]) -> Self {
		debug_assert!(slots.iter().all(|&p| p <= MAX_PER_SLOT));
		debug_assert!(slots.iter().sum::<u8>() <= TOTAL_POINTS);
		Self(slots)
	}

	pub fn slots(&self) -> [u8; SLOTS] {
		self.0
	}

	/// Unallocated points: `sum(slots) + remainder == 6`.
	pub fn remainder(&self) -> u8 {
		TOTAL_POINTS - self.0.iter().sum::<u8>()
	}

	pub fn increment(&self, slot: usize, occupied: [bool; SLOTS]) -> Result<Self, EnergyError> {
		let current = *self.0.get(slot).ok_or(EnergyError::BadSlot)?;
		if !occupied[slot] {
			return Err(EnergyError::EmptySlot);
		}
		if current >= MAX_PER_SLOT {
			return Err(EnergyError::SlotFull);
		}
		if self.remainder() == 0 {
			return Err(EnergyError::NoFreePoints);
		}
		let mut next = self.0;
		next[slot] += 1;
		Ok(Self(next))
	}

	pub fn decrement(&self, slot: usize, occupied: [bool; SLOTS]) -> Result<Self, EnergyError> {
		let current = *self.0.get(slot).ok_or(EnergyError::BadSlot)?;
		if !occupied[slot] {
			return Err(EnergyError::EmptySlot);
		}
		if current == 0 {
			return Err(EnergyError::AlreadyZero);
		}
		let mut next = self.0;
		next[slot] -= 1;
		Ok(Self(next))
	}

	/// Start requires at least one powered equipped weapon.
	pub fn any_powered(&self, occupied: [bool; SLOTS]) -> bool {
		self.0.iter().zip(occupied).any(|(&p, o)| o && p > 0)
	}
}
