use std::str::FromStr;
use std::collections::HashSet;
use std::net::{SocketAddr};
use std::{path::PathBuf};
use hyper::Uri;
use crate::Error;

fn read_env<T: FromStr, D: Into<T>>(key: &str, default: D) -> T {
	std::env::var(key).ok()
		.and_then(|s| s.parse::<T>().ok())
		.unwrap_or(default.into())
}

fn env_str<D: Into<String>>(key: &str, default: D) -> String {
	read_env(key, default)
}

fn env_ostr(key: &str) -> Option<String> {
	match read_env::<String, _>(key, "") {
		ref s if s.is_empty()  => None,
		s => Some(s)
	}
}

pub fn init_vars() {
	lazy_static::initialize(&RENDERTRON_CACHE_DEBUG);
	lazy_static::initialize(&RENDERTRON_CACHE_SOCKET);
	lazy_static::initialize(&RENDERTRON_CACHE_LOCK_TIMEOUT);
	lazy_static::initialize(&RENDERTRON_CACHE_ROOT);
	lazy_static::initialize(&RENDERTRON_CACHE_FILE_SUFFIX);
	lazy_static::initialize(&RENDERTRON_CACHE_RENDERTRON_URL);
	lazy_static::initialize(&RENDERTRON_CACHE_HEADER_REQUEST_BLACKLIST);
	lazy_static::initialize(&RENDERTRON_CACHE_HEADER_RESPONSE_BLACKLIST);
}

lazy_static! {
pub static ref RENDERTRON_CACHE_DEBUG: bool = read_env("RENDERTRON_CACHE_DEBUG", false);
pub static ref RENDERTRON_CACHE_SOCKET: SocketAddr = env_str("RENDERTRON_CACHE_SOCKET", "127.0.0.1:5000").as_str().parse().unwrap();
pub static ref RENDERTRON_CACHE_LOCK_TIMEOUT: i32 = read_env("RENDERTRON_CACHE_LOCK_TIMEOUT", 1000);
pub static ref RENDERTRON_CACHE_ROOT: PathBuf = PathBuf::from_str(&env_str("RENDERTRON_CACHE_ROOT", "./cache")).unwrap();
pub static ref RENDERTRON_CACHE_FILE_SUFFIX: String = env_str("RENDERTRON_CACHE_FILE_SUFFIX", ".json");
pub static ref RENDERTRON_CACHE_RENDERTRON_URL: Uri = Uri::from_str(&env_str("RENDERTRON_CACHE_RESOURCE_URL", "http://127.0.0.1")).unwrap();
pub static ref RENDERTRON_CACHE_HEADER_REQUEST_BLACKLIST: HashSet<String>
	= env_str("RENDERTRON_CACHE_HEADER_REQUEST_BLACKLIST", "")
	.split(',').map(str::to_string).collect();
pub static ref RENDERTRON_CACHE_HEADER_RESPONSE_BLACKLIST: HashSet<String>
	= env_str("RENDERTRON_CACHE_HEADER_RESPONSE_BLACKLIST", "Set-Cookie,Content-Encoding,Transfer-Encoding")
		.split(',').map(str::to_string).collect();
}