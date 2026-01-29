// Error types will be implemented in Task 2

use std::fmt;

#[derive(Debug)]
pub struct Error;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error")
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
