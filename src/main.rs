mod gui;
mod scene;
use macroquad::prelude::*;
use scene::Scene;

#[macroquad::main("Durak")]
async fn main() {
    let mut scene = Scene::MainMenu;
    loop {
        match scene {
            Scene::MainMenu => scene = gui::main_menu().await,
            Scene::Game => scene = gui::game().await,
            Scene::GameOver => scene = gui::game_over().await,
        }
        next_frame().await
    }
}
