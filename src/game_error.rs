use std::{fmt::Display, io};

use png::DecodingError;
use winit::error::OsError;

#[derive(Debug)]
pub struct GameError(pub String);

impl GameError {
    pub fn new(msg: &str) -> Self {
        GameError(msg.to_string())
    }
}

impl std::error::Error for GameError {}

impl Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<wgpu::CreateSurfaceError> for GameError {
    fn from(value: wgpu::CreateSurfaceError) -> Self {
        GameError(format!("{}", value))
    }
}

impl From<wgpu::RequestDeviceError> for GameError {
    fn from(value: wgpu::RequestDeviceError) -> Self {
        GameError(format!("{}", value))
    }
}

impl From<io::Error> for GameError {
    fn from(value: io::Error) -> Self {
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
