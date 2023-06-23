use std::{fmt::Display, io};

use png::DecodingError;
use winit::error::OsError;

#[derive(Debug)]
pub struct GameError(pub String);

impl std::error::Error for GameError {}

impl Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<io::Error> for GameError {
    fn from(value: io::Error) -> Self {
        GameError(format!("{}", value))
    }
}

impl From<pixels::Error> for GameError {
    fn from(value: pixels::Error) -> Self {
        GameError(format!("{}", value))
    }
}

impl From<OsError> for GameError {
    fn from(value: OsError) -> Self {
        GameError(format!("{}", value))
    }
}

impl From<DecodingError> for GameError {
    fn from(value: DecodingError) -> Self {
        GameError(format!("{}", value))
    }
}
