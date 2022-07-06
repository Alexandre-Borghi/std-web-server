use std::{error::Error, fmt::Display, io};

#[derive(Debug)]
pub enum MyError {
    InvalidMethodToken(String),
    IoError,
}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::InvalidMethodToken(token) => write!(f, "Invalid method token {}", token),
            Self::IoError => write!(f, "IO error"),
        }
    }
}

impl Error for MyError {}

impl From<io::Error> for MyError {
    fn from(_: io::Error) -> Self {
        Self::IoError
    }
}
