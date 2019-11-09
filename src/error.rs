use std::result;

#[derive(Debug)]
pub enum Error {
	Init(Box<dyn std::error::Error>),
	Other(Box<dyn std::error::Error>),
	InputError(String, Box<dyn std::error::Error>),
	IO(Box<dyn std::error::Error>),
	RequestError(String, Box<dyn std::error::Error>),
}

impl Error {
	pub fn other<E: Into<Box<dyn std::error::Error>>>(e: E) -> Self {
		Error::Other(e.into())
	}
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Error::Init(s) => write!(f, "Init error: {}", s),
			Error::Other(s) => write!(f, "{}", s),
			Error::InputError(s, e) => write!(f, "Input Error ({}): {}", s, e),
			Error::RequestError(s, e) => write!(f, "Request Error ({}): {}", s, e),
			Error::IO( e) => write!(f, "IO Error: {}", e),
		}
	}
}

impl From<std::io::Error> for Error {
	fn from(e: std::io::Error) -> Self { Error::IO(e.into())}
}

impl From<serde_json::error::Error> for Error {
	fn from(e: serde_json::error::Error) -> Self { Error::Other(e.into())}
}

pub type Result<T> = result::Result<T, Error>;