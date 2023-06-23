mod context;
mod game;
mod game_error;
mod timer;

use context::Context;
use game::Game;
use game_error::GameError;

fn main() -> Result<(), GameError> {
    let (ctx, event_loop) = Context::build()?;
    let game = Game::new(ctx)?;
    game.run(event_loop)
}
