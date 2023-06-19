mod game_state;
mod scenes;

use macroquad::prelude::*;
use scenes::{main_menu, Scene};

#[macroquad::main("Durak")]
async fn main() {
    let mut scene = Scene::new(main_menu);
    loop {
        scene = scene.update().await;
        next_frame().await
    }
}
