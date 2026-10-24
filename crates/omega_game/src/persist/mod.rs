//! Two-slot JSON save envelopes with checked SHA-256 (SP-010, ADR-007).
//!
//! A save writes the next generation into the slot that does not hold the newest valid
//! generation: temp write, sync, reopen and validate, then same-directory rename. A failed
//! step leaves the previous generation untouched. Storage is injectable so tests can fault
//! every protocol boundary.

use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha256};
use std::io;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

pub const FORMAT: &str = "omega-save";

/// Filesystem operations the protocol needs; every one can fail independently.
pub trait Storage {
	fn read(&self, path: &Path) -> io::Result<Option<Vec<u8>>>;
	/// Creates or truncates `path`, writes all bytes and syncs them to disk.
	fn write_synced(&self, path: &Path, bytes: &[u8]) -> io::Result<()>;
	fn rename(&self, from: &Path, to: &Path) -> io::Result<()>;
	fn remove(&self, path: &Path) -> io::Result<()>;
	fn create_dir_all(&self, path: &Path) -> io::Result<()>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct DiskStorage;

impl Storage for DiskStorage {
	fn read(&self, path: &Path) -> io::Result<Option<Vec<u8>>> {
		match std::fs::read(path) {
			Ok(bytes) => Ok(Some(bytes)),
			Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
			Err(e) => Err(e),
		}
	}

	fn write_synced(&self, path: &Path, bytes: &[u8]) -> io::Result<()> {
		use std::io::Write;
		let mut file = std::fs::File::create(path)?;
		file.write_all(bytes)?;
		file.sync_all()
	}

	fn rename(&self, from: &Path, to: &Path) -> io::Result<()> {
		std::fs::rename(from, to)
	}

	fn remove(&self, path: &Path) -> io::Result<()> {
		match std::fs::remove_file(path) {
			Err(e) if e.kind() != io::ErrorKind::NotFound => Err(e),
			_ => Ok(()),
		}
	}

	fn create_dir_all(&self, path: &Path) -> io::Result<()> {
		std::fs::create_dir_all(path)
	}
}

/// The per-user directory for saves; there is deliberately no fallback to the install
/// directory.
pub fn user_dir() -> Option<PathBuf> {
	std::env::var_os("LOCALAPPDATA")
		.filter(|v| !v.is_empty())
		.map(|base| PathBuf::from(base).join("ProjectOmega"))
}

#[derive(Serialize, Deserialize)]
struct Envelope {
	format: String,
	schema: u32,
	generation: u64,
	sha256: String,
	payload: serde_json::Value,
}

fn payload_hash(payload: &serde_json::Value) -> String {
	let bytes = serde_json::to_vec(payload).unwrap_or_default();
	Sha256::digest(&bytes)
		.iter()
		.map(|b| format!("{b:02x}"))
		.collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadProblem {
	/// Slots exist but none holds a valid current-schema generation; files are untouched.
	Corrupt {
		files: Vec<String>,
	},
	/// A slot was written by a newer build; files are untouched.
	FutureSchema {
		found: u32,
	},
	Unreadable(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadStatus {
	/// No save exists yet: the caller starts from its initial value.
	Fresh,
	Loaded {
		generation: u64,
	},
	/// The newest slot was unreadable; the previous valid generation was used.
	RecoveredPrevious {
		generation: u64,
	},
	Problem(LoadProblem),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SaveError {
	/// The request carried a generation that is not newer than the committed one.
	Stale {
		committed: u64,
	},
	/// Load found a problem; saving needs an explicit recovery first.
	NeedsRecovery,
	Io(String),
	/// The temp file did not read back identical.
	VerifyFailed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Slot {
	A,
	B,
}

impl Slot {
	fn other(self) -> Self {
		match self {
			Slot::A => Slot::B,
			Slot::B => Slot::A,
		}
	}
}

/// One logical save (profile or settings) stored as slot A, slot B and a temp file.
pub struct SlotStore<T> {
	dir: PathBuf,
	name: &'static str,
	schema: u32,
	committed: u64,
	newest: Option<Slot>,
	blocked: bool,
	marker: PhantomData<T>,
}

impl<T: Serialize + DeserializeOwned> SlotStore<T> {
	pub fn new(dir: &Path, name: &'static str, schema: u32) -> Self {
		Self {
			dir: dir.to_path_buf(),
			name,
			schema,
			committed: 0,
			newest: None,
			blocked: false,
			marker: PhantomData,
		}
	}

	/// True after a load problem: saving is refused until `quarantine` runs.
	pub fn needs_recovery(&self) -> bool {
		self.blocked
	}

	pub fn committed_generation(&self) -> u64 {
		self.committed
	}

	fn slot_path(&self, slot: Slot) -> PathBuf {
		let tag = if slot == Slot::A { "a" } else { "b" };
		self.dir.join(format!("{}.{tag}.json", self.name))
	}

	fn temp_path(&self) -> PathBuf {
		self.dir.join(format!("{}.tmp", self.name))
	}

	fn parse(&self, bytes: &[u8]) -> Result<(u64, T), SlotFault> {
		let envelope: Envelope = serde_json::from_slice(bytes).map_err(|_| SlotFault::Corrupt)?;
		if envelope.format != FORMAT || envelope.sha256 != payload_hash(&envelope.payload) {
			return Err(SlotFault::Corrupt);
		}
		if envelope.schema > self.schema {
			return Err(SlotFault::Future(envelope.schema));
		}
		let value = serde_json::from_value(envelope.payload).map_err(|_| SlotFault::Corrupt)?;
		Ok((envelope.generation, value))
	}

	/// Loads the newest valid generation. Never writes, never replaces corrupt bytes.
	pub fn load(&mut self, storage: &dyn Storage) -> (Option<T>, LoadStatus) {
		self.committed = 0;
		self.newest = None;
		self.blocked = false;
		let mut valid: Vec<(u64, Slot, T)> = Vec::new();
		let mut corrupt = Vec::new();
		let mut existing = 0;
		for slot in [Slot::A, Slot::B] {
			let path = self.slot_path(slot);
			let bytes = match storage.read(&path) {
				Ok(Some(b)) => b,
				Ok(None) => continue,
				Err(e) => {
					self.blocked = true;
					return (
						None,
						LoadStatus::Problem(LoadProblem::Unreadable(e.to_string())),
					);
				}
			};
			existing += 1;
			match self.parse(&bytes) {
				Ok((generation, value)) => valid.push((generation, slot, value)),
				Err(SlotFault::Future(found)) => {
					self.blocked = true;
					return (
						None,
						LoadStatus::Problem(LoadProblem::FutureSchema { found }),
					);
				}
				Err(SlotFault::Corrupt) => corrupt.push(path.display().to_string()),
			}
		}
		if existing == 0 {
			return (None, LoadStatus::Fresh);
		}
		valid.sort_by_key(|(generation, ..)| *generation);
		let Some((generation, slot, value)) = valid.pop() else {
			self.blocked = true;
			return (
				None,
				LoadStatus::Problem(LoadProblem::Corrupt { files: corrupt }),
			);
		};
		self.committed = generation;
		self.newest = Some(slot);
		let status = if corrupt.is_empty() {
			LoadStatus::Loaded { generation }
		} else {
			LoadStatus::RecoveredPrevious { generation }
		};
		(Some(value), status)
	}

	/// Publishes `value` as `generation` (must exceed the committed one).
	pub fn save(
		&mut self,
		storage: &dyn Storage,
		value: &T,
		generation: u64,
	) -> Result<(), SaveError> {
		if self.blocked {
			return Err(SaveError::NeedsRecovery);
		}
		if generation <= self.committed {
			return Err(SaveError::Stale {
				committed: self.committed,
			});
		}
		let payload = serde_json::to_value(value).map_err(|e| SaveError::Io(e.to_string()))?;
		let envelope = Envelope {
			format: FORMAT.to_string(),
			schema: self.schema,
			generation,
			sha256: payload_hash(&payload),
			payload,
		};
		let bytes =
			serde_json::to_vec_pretty(&envelope).map_err(|e| SaveError::Io(e.to_string()))?;
		let io = |e: io::Error| SaveError::Io(e.to_string());
		storage.create_dir_all(&self.dir).map_err(io)?;
		let temp = self.temp_path();
		let target = self.slot_path(self.newest.map_or(Slot::A, Slot::other));
		let result = (|| {
			storage.write_synced(&temp, &bytes).map_err(io)?;
			let back = storage
				.read(&temp)
				.map_err(io)?
				.ok_or(SaveError::VerifyFailed)?;
			if back != bytes || self.parse(&back).is_err() {
				return Err(SaveError::VerifyFailed);
			}
			storage.rename(&temp, &target).map_err(io)
		})();
		if result.is_err() {
			let _ = storage.remove(&temp);
			return result;
		}
		self.committed = generation;
		self.newest = Some(self.newest.map_or(Slot::A, Slot::other));
		Ok(())
	}

	/// Explicit recovery after a load problem: moves the unusable files aside (never deletes
	/// them) and unblocks saving, so the caller can start a fresh value.
	pub fn quarantine(&mut self, storage: &dyn Storage) -> Result<Vec<PathBuf>, SaveError> {
		let mut moved = Vec::new();
		for slot in [Slot::A, Slot::B] {
			let from = self.slot_path(slot);
			if storage
				.read(&from)
				.map_err(|e| SaveError::Io(e.to_string()))?
				.is_none()
			{
				continue;
			}
			let to = self.dir.join(format!(
				"{}.{}.quarantine-{}",
				self.name,
				if slot == Slot::A { "a" } else { "b" },
				self.committed.max(1)
			));
			let to = unique(storage, to);
			storage
				.rename(&from, &to)
				.map_err(|e| SaveError::Io(e.to_string()))?;
			moved.push(to);
		}
		self.newest = None;
		self.blocked = false;
		Ok(moved)
	}
}

fn unique(storage: &dyn Storage, base: PathBuf) -> PathBuf {
	let mut candidate = base.clone();
	let mut n = 1;
	while storage.read(&candidate).ok().flatten().is_some() {
		candidate = PathBuf::from(format!("{}.{n}", base.display()));
		n += 1;
	}
	candidate
}

enum SlotFault {
	Corrupt,
	Future(u32),
}
