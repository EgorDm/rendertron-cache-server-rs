use std::collections::HashMap;
use std::{path::PathBuf, io::Write};
use serde::{Serialize, Deserialize};
use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, new, Serialize, Deserialize)]
pub struct Content {
	pub status: u16,
	pub headers: HashMap<String, String>,
	pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq, new)]
pub struct Document {
	path: PathBuf
}

impl Document {
	pub fn exists(&self) -> bool { self.path.exists() }

	pub fn read(&self) -> Result<Content> {
		shared_read(&self.path, |s| {
			serde_json::from_str(&s).map_err(Error::from)
		})
	}

	pub fn write(&self, content: &Content) -> Result<()> {
		let s = serde_json::to_string(content)?;

		std::fs::create_dir_all(self.path.parent().unwrap())?;
		exclusive_write(&self.path, |f|  {
			write!(f, "{}", s).map_err(Error::from)
		})
	}

	pub fn delete(&self) -> Result<()> {
		if self.exists() {
			if self.path.is_dir() {
				std::fs::remove_dir_all(&self.path).map_err(Error::from)?;
			} else {
				std::fs::remove_file(&self.path).map_err(Error::from)?;
			}
		}
		Ok(())
	}
}