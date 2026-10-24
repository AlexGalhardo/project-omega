//! Strict JSON record reader. Every accessor records the field path so errors name
//! `record_id.field[index].field`, and `finish` rejects unknown fields.

use super::ContentError;
use serde_json::{Map, Value};

/// The only field a record may carry beyond its schema (GDD-012).
pub const NOTES_FIELD: &str = "authoring_notes";

pub struct Rec<'v> {
	file: &'v str,
	path: String,
	map: &'v Map<String, Value>,
	seen: Vec<&'v str>,
}

pub type R<T> = Result<T, ContentError>;

impl<'v> Rec<'v> {
	pub fn new(file: &'v str, path: String, value: &'v Value) -> R<Self> {
		match value {
			Value::Object(map) => Ok(Self {
				file,
				path,
				map,
				seen: Vec::new(),
			}),
			other => Err(ContentError::new(
				file,
				&path,
				format!("expected object, found {}", kind(other)),
			)),
		}
	}

	pub fn path(&self) -> &str {
		&self.path
	}

	pub fn file(&self) -> &'v str {
		self.file
	}

	pub fn field_path(&self, key: &str) -> String {
		if self.path.is_empty() {
			key.to_string()
		} else {
			format!("{}.{key}", self.path)
		}
	}

	pub fn error(&self, key: &str, message: impl Into<String>) -> ContentError {
		ContentError::new(self.file, &self.field_path(key), message.into())
	}

	pub fn opt(&mut self, key: &str) -> Option<&'v Value> {
		let (k, v) = self.map.get_key_value(key)?;
		self.seen.push(k.as_str());
		Some(v)
	}

	pub fn req(&mut self, key: &str) -> R<&'v Value> {
		self.opt(key)
			.ok_or_else(|| self.error(key, "missing required field"))
	}

	pub fn string(&mut self, key: &str) -> R<String> {
		let v = self.req(key)?;
		v.as_str()
			.filter(|s| !s.trim().is_empty())
			.map(str::to_string)
			.ok_or_else(|| self.error(key, format!("expected nonempty string, found {}", kind(v))))
	}

	pub fn id(&mut self, key: &str) -> R<String> {
		let s = self.string(key)?;
		valid_id(&s)
			.then_some(s.clone())
			.ok_or_else(|| self.error(key, format!("invalid id `{s}`: expected [a-z][a-z0-9_]*")))
	}

	pub fn opt_id(&mut self, key: &str) -> R<Option<String>> {
		match self.map.get(key) {
			None | Some(Value::Null) => {
				self.opt(key);
				Ok(None)
			}
			Some(_) => self.id(key).map(Some),
		}
	}

	pub fn bool(&mut self, key: &str) -> R<bool> {
		let v = self.req(key)?;
		v.as_bool()
			.ok_or_else(|| self.error(key, format!("expected boolean, found {}", kind(v))))
	}

	/// Integer within `min..=max`; decimals, strings and `"NaN"` text are rejected.
	pub fn int(&mut self, key: &str, min: i64, max: i64) -> R<i64> {
		let v = self.req(key)?;
		int_value(v, min, max).map_err(|m| self.error(key, m))
	}

	pub fn uint(&mut self, key: &str, min: u32, max: u32) -> R<u32> {
		self.int(key, i64::from(min), i64::from(max))
			.map(|v| v as u32)
	}

	/// Finite number within `min..=max` (JSON cannot encode NaN or infinity).
	pub fn number(&mut self, key: &str, min: f64, max: f64) -> R<f64> {
		let v = self.req(key)?;
		number_value(v, min, max).map_err(|m| self.error(key, m))
	}

	/// Strictly positive finite number.
	pub fn positive(&mut self, key: &str) -> R<f64> {
		let v = self.number(key, 0.0, f64::MAX)?;
		if v > 0.0 {
			Ok(v)
		} else {
			Err(self.error(key, "must be positive"))
		}
	}

	pub fn array(&mut self, key: &str) -> R<&'v Vec<Value>> {
		let v = self.req(key)?;
		v.as_array()
			.ok_or_else(|| self.error(key, format!("expected array, found {}", kind(v))))
	}

	pub fn numbers(&mut self, key: &str, min: f64, max: f64) -> R<Vec<f64>> {
		let path = self.field_path(key);
		let file = self.file;
		let items = self.array(key)?;
		if items.is_empty() {
			return Err(self.error(key, "must not be empty"));
		}
		items
			.iter()
			.enumerate()
			.map(|(i, v)| {
				number_value(v, min, max)
					.map_err(|m| ContentError::new(file, &format!("{path}[{i}]"), m))
			})
			.collect()
	}

	pub fn object(&mut self, key: &str) -> R<Rec<'v>> {
		let path = self.field_path(key);
		let v = self.req(key)?;
		Rec::new(self.file, path, v)
	}

	pub fn objects(&mut self, key: &str) -> R<Vec<Rec<'v>>> {
		let path = self.field_path(key);
		let file = self.file;
		self.array(key)?
			.iter()
			.enumerate()
			.map(|(i, v)| Rec::new(file, format!("{path}[{i}]"), v))
			.collect()
	}

	pub fn choice(&mut self, key: &str, allowed: &[&str]) -> R<String> {
		let s = self.string(key)?;
		if allowed.contains(&s.as_str()) {
			Ok(s)
		} else {
			Err(self.error(
				key,
				format!(
					"unsupported value `{s}`, expected one of {}",
					allowed.join(", ")
				),
			))
		}
	}

	/// Rejects any field that no accessor read, except authoring notes.
	pub fn finish(mut self) -> R<()> {
		self.opt(NOTES_FIELD);
		for key in self.map.keys() {
			if !self.seen.contains(&key.as_str()) {
				return Err(self.error(key, "unknown field"));
			}
		}
		Ok(())
	}
}

pub fn valid_id(s: &str) -> bool {
	let mut chars = s.chars();
	chars.next().is_some_and(|c| c.is_ascii_lowercase())
		&& chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

fn int_value(v: &Value, min: i64, max: i64) -> Result<i64, String> {
	let n = v
		.as_i64()
		.ok_or_else(|| format!("expected integer, found {}", kind(v)))?;
	if (min..=max).contains(&n) {
		Ok(n)
	} else {
		Err(format!("{n} is outside {min}..={max}"))
	}
}

fn number_value(v: &Value, min: f64, max: f64) -> Result<f64, String> {
	let n = v
		.as_f64()
		.filter(|n| n.is_finite())
		.ok_or_else(|| format!("expected finite number, found {}", kind(v)))?;
	if (min..=max).contains(&n) {
		Ok(n)
	} else {
		Err(format!("{n} is outside {min}..={max}"))
	}
}

fn kind(v: &Value) -> String {
	match v {
		Value::Null => "null".into(),
		Value::Bool(_) => "boolean".into(),
		Value::Number(n) if n.is_i64() || n.is_u64() => "integer".into(),
		Value::Number(_) => "decimal number".into(),
		Value::String(s) => format!("string \"{s}\""),
		Value::Array(_) => "array".into(),
		Value::Object(_) => "object".into(),
	}
}
