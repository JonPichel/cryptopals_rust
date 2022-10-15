use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum SetOneError {
    InvalidHexError(String),
    IncompatibleSizeError(usize, usize),
}

impl Display for SetOneError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidHexError(invalid_string) => write!(f, "Invalid hex string: {}", invalid_string),
            Self::IncompatibleSizeError(a, b) => write!(f, "Incompatible sizes: {} and {}", a, b),
        }
    }
}

impl Error for SetOneError {}
