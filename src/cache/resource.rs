use reqwest::{Client};
use crate::*;
use log::{debug, info};
use hyper::{Uri, HeaderMap};

#[derive(Debug, Clone)]
pub struct Resource {
	client: Client
}

impl Resource {
	pub fn new() -> Self {
		Self {
			client: Client::new()
		}
	}

	pub fn retrieve(&self, doc: &Document, q: &Query) -> Result<Content> {
		let host_uri: &Uri = &RENDERTRON_CACHE_RENDERTRON_URL;
		let path = format!("/render/{}", q.url.to_string());
		let uri: Uri = Uri::builder()
			.scheme(host_uri.scheme_str().unwrap())
			.authority(host_uri.authority_part().unwrap().as_str())
			.path_and_query(path.as_str())
			.build()
			.map_err(|e| Error::InputError(format!("Failed to build url to: {}", path), e.into()))?;

		debug!("Requesting resource: {}", uri.to_string());

		let mut request_headers = HeaderMap::new();
		headers_to_headermap(&q.headers, &mut request_headers);
		let mut result = self.client.get(&uri.to_string())
			.headers(request_headers)
			.send()
			.map_err(|e| Error::RequestError(uri.to_string(), e.into()))?;

		let content = Content::new(
			result.status().as_u16(),
			headers_from_headermap(result.headers()),
			result.text().map_err(|e| Error::RequestError(uri.to_string(), e.into()))?
		);

		if result.status().is_success() {
			doc.write(&content)?;
		}

		Ok(content)
	}
}

