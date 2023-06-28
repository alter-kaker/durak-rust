use ggez::GameError;

use crate::scenes::SceneError;

#[derive(Debug)]
pub struct DurakError(String);

impl From<&str> for DurakError {
    fn from(value: &str) -> Self {
        DurakError(value.to_string())
    }
}

impl From<GameError> for DurakError {
    fn from(value: GameError) -> Self {
        DurakError(format!("{:?}", value))
    }
}

impl From<SceneError> for DurakError {
    fn from(value: SceneError) -> Self {
        DurakError(format!("{:?}", value))
    }
}

impl From<DurakError> for GameError {
    fn from(value: DurakError) -> Self {
        GameError::CustomError(format!("{:?}", value))
    }
}
