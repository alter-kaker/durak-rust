mod card;
mod deck;
mod error;
mod game_state;
mod hand;
mod player;
mod scenes;

use std::{env, path};

use game_state::GameState;
use ggez::{event, ContextBuilder, GameResult};

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ContextBuilder::new("durak_rust", "alter_kaker").add_resource_path(resource_dir);
    let (mut ctx, event_loop) = cb.build()?;
    let state = GameState::new(&mut ctx)?;

    event::run(ctx, event_loop, state)
}
