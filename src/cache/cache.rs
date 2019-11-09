use crate::*;
use std::path::PathBuf;
use log::{debug, info};

#[derive(Debug, Clone)]
pub struct Cache {
	storage_path: PathBuf,
	resource: Resource,
}

impl Cache {
	pub fn new() -> Self {
		Self {
			storage_path: RENDERTRON_CACHE_ROOT.clone(),
			resource: Resource::new()
		}
	}

	pub fn query(&self, q: &Query) -> Document {
		Document::new(self.storage_path.join(q.key()))
	}

	pub fn retrieve(&self, doc: &Document, q: &Query) -> Result<Content> {
		if doc.exists() {
			debug!("Cache Hit: {}", q.url.to_string());

			let mut content = doc.read()?;
			content.headers.insert("Rendertron-Cached".to_string(), "1".to_string());
			Ok(content)
		} else {
			debug!("Cache Miss: {}", q.url.to_string());

			let mut content = self.resource.retrieve(doc, q)?;
			content.headers.insert("Rendertron-Cached".to_string(), "0".to_string());
			Ok(content)
		}
	}

	pub fn refresh(&self, doc: &Document, q: &Query) -> Result<Content> {
		debug!("Refreshing cache: {}", q.url.to_string());

		self.delete(doc)?;
		self.retrieve(doc, q)
	}

	pub fn delete(&self, doc: &Document) -> Result<()> { doc.delete() }

	pub fn purge(&self, doc: &Document, q: &Query) -> Result<()> {
		let dir = Document::new(self.storage_path.join(q.key_raw()));
		let file = doc;

		info!("Purging Cache");

		dir.delete()?;
		file.delete()
	}
}