mod game_state;
mod scenes;
mod ui;
mod player;
mod card;
mod hand;
mod deck;

use game_state::GameState;
use macroquad::{prelude::*, ui::root_ui};
use scenes::{main_menu, Scene};
use ui::load_skin;


#[macroquad::main("Durak")]
async fn main() {
    let skin = load_skin();
    root_ui().push_skin(&skin);

    let state = GameState::new();
    let mut scene = Scene::new(main_menu, state);
    loop {
        scene = scene.update().await;
        next_frame().await
    }
}
