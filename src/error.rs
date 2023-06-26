use ggez::GameError;

#[derive(Debug)]
pub struct DurakError(String);

impl From<GameError> for DurakError {
    fn from(value: GameError) -> Self {
        DurakError(format!("{:?}", value))
    }
}
