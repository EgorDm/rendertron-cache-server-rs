use std::collections::HashMap;
use hyper::{Uri};
use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, new)]
pub struct Query {
	pub url: Uri,
	pub headers: HashMap<String, String>,
}

impl Query {
	pub fn key(&self) -> String {
		let mut key = self.key_raw();
		key.push_str(&RENDERTRON_CACHE_FILE_SUFFIX);
		key
	}

	pub fn key_raw(&self) -> String {
		let mut key = String::new();
		key.push_str(self.url.host().unwrap_or_default());

		let path = self.url.path().trim_end_matches('/');
		if !path.is_empty() {
			key.push_str(&format!("/{}", path))
		}

		if self.url.query().is_some() {
			key.push_str(&format!("?{}", self.url.query().unwrap()))
		}
		key
	}
}