use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum SetOneError {
    InvalidHex(String),
    IncompatibleSize(usize, usize),
    InvalidBase64(String),
}

impl Display for SetOneError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidHex(invalid_string) => write!(f, "Invalid hex string: {}", invalid_string),
            Self::IncompatibleSize(a, b) => write!(f, "Incompatible sizes: {} and {}", a, b),
            Self::InvalidBase64(invalid_string) => write!(f, "Invalid base64 string: {}", invalid_string),
        }
    }
}

impl Error for SetOneError {}
