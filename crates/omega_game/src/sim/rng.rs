//! Run seed and named RNG streams (architecture section 6, ADR-011).
//!
//! Streams are domain-separated by XORing the seed's final byte with a fixed tag.
//! This is reproducibility separation for one build, not a cryptographic protocol.

use bevy::prelude::*;
use rand_chacha::ChaCha8Rng;
use rand_chacha::rand_core::{Rng, SeedableRng};
use std::fmt;

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunSeed(pub [u8; 32]);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeedParseError(pub String);

impl fmt::Display for SeedParseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(&self.0)
	}
}

impl std::error::Error for SeedParseError {}

impl RunSeed {
	/// Parses exactly 64 hexadecimal characters.
	pub fn from_hex(text: &str) -> Result<Self, SeedParseError> {
		if text.len() != 64 || !text.is_ascii() {
			return Err(SeedParseError(format!(
				"seed must be 64 hex characters, got {} characters",
				text.chars().count()
			)));
		}
		let mut bytes = [0u8; 32];
		for (i, byte) in bytes.iter_mut().enumerate() {
			*byte = u8::from_str_radix(&text[i * 2..i * 2 + 2], 16).map_err(|_| {
				SeedParseError(format!("seed has a non-hex digit near position {}", i * 2))
			})?;
		}
		Ok(Self(bytes))
	}

	pub fn to_hex(&self) -> String {
		self.0.iter().map(|b| format!("{b:02x}")).collect()
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StreamTag {
	SpawnTypes = 1,
	SpawnAnchors = 2,
	Shop = 3,
	Crit = 4,
}

/// One authoritative random stream.
#[derive(Debug, Clone)]
pub struct SimRng(ChaCha8Rng);

impl SimRng {
	pub fn new(seed: &RunSeed, tag: StreamTag) -> Self {
		let mut bytes = seed.0;
		bytes[31] ^= tag as u8;
		Self(ChaCha8Rng::from_seed(bytes))
	}

	pub fn next_u32(&mut self) -> u32 {
		self.0.next_u32()
	}

	/// Unbiased integer in `0..n` by rejection sampling; never plain `% n`.
	pub fn below(&mut self, n: u32) -> u32 {
		assert!(n > 0, "empty range");
		let threshold = n.wrapping_neg() % n;
		loop {
			let r = self.next_u32();
			if r >= threshold {
				return r % n;
			}
		}
	}

	/// Words consumed so far; part of the authoritative trace.
	pub fn position(&self) -> u128 {
		self.0.get_word_pos()
	}
}

#[derive(Resource, Debug, Clone)]
pub struct RngStreams {
	pub spawn_types: SimRng,
	pub spawn_anchors: SimRng,
	pub shop: SimRng,
	pub crit: SimRng,
}

impl RngStreams {
	pub fn new(seed: &RunSeed) -> Self {
		Self {
			spawn_types: SimRng::new(seed, StreamTag::SpawnTypes),
			spawn_anchors: SimRng::new(seed, StreamTag::SpawnAnchors),
			shop: SimRng::new(seed, StreamTag::Shop),
			crit: SimRng::new(seed, StreamTag::Crit),
		}
	}

	pub fn positions(&self) -> [u128; 4] {
		[
			self.spawn_types.position(),
			self.spawn_anchors.position(),
			self.shop.position(),
			self.crit.position(),
		]
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn hex_round_trip_and_rejection() {
		let hex = "00ff".repeat(16);
		assert_eq!(RunSeed::from_hex(&hex).unwrap().to_hex(), hex);
		assert!(RunSeed::from_hex("abc").is_err());
		assert!(RunSeed::from_hex(&"zz".repeat(32)).is_err());
	}

	#[test]
	fn streams_are_separated_and_reproducible() {
		let seed = RunSeed([7; 32]);
		let mut a = RngStreams::new(&seed);
		let mut b = RngStreams::new(&seed);
		let first = a.shop.next_u32();
		assert_eq!(first, b.shop.next_u32());
		assert_ne!(
			first,
			a.crit.next_u32(),
			"different tags give different streams"
		);
		for _ in 0..1000 {
			assert!(a.spawn_types.below(3) < 3);
		}
	}
}
