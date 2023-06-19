mod gui;
mod scene;
mod game_state;

use macroquad::prelude::*;
use scene::Scene;
use game_state::GameState;

#[macroquad::main("Durak")]
async fn main() {
    let mut scene = Scene::MainMenu(GameState::default());
    loop {
        match scene {
            Scene::MainMenu(state) => scene = Scene::Game(gui::main_menu(state).await),
            Scene::Game(state) => scene = Scene::GameOver(gui::game(state).await),
            Scene::GameOver(state) => scene = Scene::MainMenu(gui::game_over(state).await),
        }
        next_frame().await
    }
}
