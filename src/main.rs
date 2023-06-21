mod game_state;
mod scenes;
mod resources;
mod player;
mod card;
mod hand;
mod deck;

use game_state::GameState;
use macroquad::{prelude::{*, collections::storage}, ui::{root_ui, Skin}};
use resources::{load_resources, ResourceError};
use scenes::{main_menu, Scene};


#[macroquad::main("Durak")]
async fn main() -> Result<(), ResourceError>{
    load_resources().await?;
    root_ui().push_skin(&storage::get::<Skin>());

    let state = GameState::new();
    let mut scene = Scene::new(main_menu, state);
    loop {
        scene = scene.update().await;
        next_frame().await
    }
}
