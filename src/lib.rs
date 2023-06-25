mod assets;
mod card;
mod context;
mod game;
mod game_error;
mod sprite;
mod timer;

pub mod prelude {
    pub use crate::game_error::GameError;
}

use crate::{context::Context, game::Game, game_error::GameError};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub async fn run() -> Result<(), GameError> {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    let (ctx, event_loop) = Context::new().await?;
    let game = Game::new(ctx)?;
    game.run(event_loop)
}
