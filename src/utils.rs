use hyper::HeaderMap;
use hyper::http::header::HeaderName;
use std::collections::HashMap;
use std::{str::FromStr, path::Path, fs::File, io::{Read, Write}};
use crate::Result;
use fs2::FileExt;

pub fn headers_from_headermap(headers: &HeaderMap) -> HashMap<String, String> {
	headers.iter().map(|(k, v)| (k.to_string(), v.to_str().unwrap().to_string())).collect()
}

pub fn headers_to_headermap(headers: &HashMap<String, String>, result: &mut HeaderMap) {
	result.extend(headers.iter()
		.map(|(k, v)| (HeaderName::from_str(k).unwrap(), v.parse().unwrap())));
}

pub fn shared_read<T, F: FnOnce(String) -> Result<T>>(path: &Path, fun: F) -> Result<T> {
	let mut f = File::open(path)?;
	f.lock_shared()?;
	let mut content = String::new();
	let read_res = f.read_to_string(&mut content);
	f.unlock()?;

	read_res?;
	fun(content)
}

pub fn exclusive_write<F: FnOnce(&mut File) -> Result<()>>(path: &Path, fun: F)  -> Result<()>{
	let mut f = File::create(path)?;
	f.lock_exclusive()?;
	let res = fun(&mut f);
	f.unlock()?;
	res
}