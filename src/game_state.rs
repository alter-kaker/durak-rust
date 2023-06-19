

pub struct GameState {
    pub player_name: String,
    pub times_played: u32,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            player_name: "".to_string(),
            times_played: 0,
        }
    }
}
