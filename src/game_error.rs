use std::fmt::Display;

use pixels::Error;
use winit::error::OsError;

#[derive(Debug)]
pub struct GameError(pub String);

impl std::error::Error for GameError {}

impl Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Error> for GameError {
    fn from(value: Error) -> Self {
        GameError(format!("{}", value))
    }
}

impl From<OsError> for GameError {
    fn from(value: OsError) -> Self {
        GameError(format!("{}", value))
    }
}
