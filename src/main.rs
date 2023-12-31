use ggez::{event, ContextBuilder, GameResult};
use std::{env, path};

use durak_rust::{game::Game, storage};

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
    storage::load_card_image(&ctx)?;

    let state = Game::new(&mut ctx)?;

    event::run(ctx, event_loop, state)
}
