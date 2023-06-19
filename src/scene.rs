use crate::game_state::GameState;

pub enum Scene {
    MainMenu(GameState),
    Game(GameState),
    GameOver(GameState),
}
