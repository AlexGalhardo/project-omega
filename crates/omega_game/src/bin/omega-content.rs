//! Content validation CLI (SP-003): `omega-content validate PATH --profile prototype|full`.
//! Exits 0 only when the whole bundle validates.

use omega_game::content::{Profile, load_dir};
use std::path::PathBuf;
use std::process::ExitCode;

const USAGE: &str = "usage: omega-content validate PATH --profile prototype|full";

fn main() -> ExitCode {
	let args: Vec<String> = std::env::args().skip(1).collect();
	let (path, profile) = match args.as_slice() {
		[cmd, path, flag, profile] if cmd == "validate" && flag == "--profile" => {
			match profile.parse::<Profile>() {
				Ok(p) => (PathBuf::from(path), p),
				Err(e) => {
					eprintln!("{e}\n{USAGE}");
					return ExitCode::from(2);
				}
			}
		}
		_ => {
			eprintln!("{USAGE}");
			return ExitCode::from(2);
		}
	};
	match load_dir(&path, profile) {
		Ok(s) => {
			let c = &s.catalogs;
			println!(
				"valid {:?} content revision {}: {} characters, {} weapons, {} items, {} enemies, {} bosses, {} waves, {} unlocks",
				s.profile,
				s.revision_hex(),
				c.characters.len(),
				c.weapons.len(),
				c.items.len(),
				c.enemies.len(),
				c.bosses.len(),
				c.waves.len(),
				c.unlocks.len()
			);
			ExitCode::SUCCESS
		}
		Err(errors) => {
			eprint!("{errors}");
			eprintln!("{} content error(s)", errors.0.len());
			ExitCode::FAILURE
		}
	}
}
