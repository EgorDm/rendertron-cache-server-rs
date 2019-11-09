use hyper::{Request, Body, Uri, Response};
use crate::*;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct CacheServer {
	cache: Cache
}

impl CacheServer {
	pub fn new() -> Self {
		Self { cache: Cache::new() }
	}

	pub fn extract_query(&self, req: &Request<Body>) -> Result<Query> {
		let path = req.uri().path().trim_start_matches("/render").trim_start_matches('/').trim_end_matches('/');
		let url = Uri::from_str(path)
			.map_err(|e| Error::InputError(format!("Failed to parse url: {}", path), e.into()))?;
		let headers = headers_from_headermap(req.headers());

		Ok(Query::new(url, headers))
	}

	fn make_response(&self, content: Content) -> Result<Response<Body>> {
		let mut response = Response::builder();
		response.status(content.status);
		headers_to_headermap(&content.headers, response.headers_mut().unwrap());
		response.body(Body::from(content.content)).map_err(Error::other)
	}

	pub fn retrieve(&self, req: &Request<Body>) -> Result<Response<Body>> {
		let query: Query = self.extract_query(req)?;
		let document = self.cache.query(&query);
		let content = self.cache.retrieve(&document, &query)?;

		self.make_response(content)
	}

	pub fn refresh(&self, req: &Request<Body>) -> Result<Response<Body>> {
		let query: Query = self.extract_query(req)?;
		let document = self.cache.query(&query);
		let content = self.cache.refresh(&document, &query)?;

		self.make_response(content)
	}

	pub fn purge(&self, req: &Request<Body>) -> Result<Response<Body>> {
		let query: Query = self.extract_query(req)?;
		let document = self.cache.query(&query);
		self.cache.purge(&document, &query)?;

		Ok(Response::new(Body::from("Purged cache")))
	}

}