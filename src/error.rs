use std::error;
use std::fmt;
use std::result;

pub type Error = Box<dyn error::Error>;

pub type Result<T> = std::result::Result<T, Error>;

// Generic error. Only has an error message.
#[derive(Debug)]
pub struct GenError {
	inner: String,
}

impl GenError {
	pub fn new<T>(inner: String) -> Result<T> {
		Result::Err(Box::new(GenError { inner }))
	}
}

impl fmt::Display for GenError {
	fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
		write!(f, "{}", self.inner)
	}
}

impl error::Error for GenError {}
