use ggez::{
    glam::Vec2,
    graphics::{Canvas, Color, Text},
    Context,
};

use crate::{error::DurakError, game_state::GameState};

pub type SceneResult = Result<Option<Box<dyn Scene>>, DurakError>;

pub trait Scene {
    fn update(&self, state: &mut GameState, _ctx: &Context) -> SceneResult;
    fn draw(&self, state: &GameState, ctx: &mut Context) -> Result<(), DurakError>;
}

pub struct MainMenu {}

impl Scene for MainMenu {
    fn update(
        &self,
        state: &mut GameState,
        _ctx: &Context,
    ) -> Result<Option<Box<dyn Scene>>, DurakError> {
        state.frames += 1;
        if state.frames > 100 {
            return Ok(Some(Box::new(GamePlay {})));
        }
        Ok(None)
    }

    fn draw(&self, state: &GameState, ctx: &mut Context) -> Result<(), DurakError> {
        let mut canvas = Canvas::from_frame(ctx, Color::from([0.1, 0.2, 0.3, 1.0]));

        // Text is drawn from the top-left corner.
        let offset = state.frames as f32 / 10.0;
        let dest_point = Vec2::new(offset, offset);
        canvas.draw(
            Text::new(format!("Main Menu! Frame {}", state.frames))
                .set_font("IBM_CGA")
                .set_scale(24.),
            dest_point,
        );

        canvas.finish(ctx)?;

        if (state.frames % 100) == 0 {
            println!("FPS: {}", ctx.time.fps());
        }

        Ok(())
    }
}

struct GamePlay {}

impl Scene for GamePlay {
    fn update(&self, state: &mut GameState, _ctx: &Context) -> SceneResult {
        state.frames += 1;
        Ok(None)
    }

    fn draw(&self, state: &GameState, ctx: &mut Context) -> Result<(), DurakError> {
        let mut canvas = Canvas::from_frame(ctx, Color::from([0.1, 0.2, 0.3, 1.0]));

        // Text is drawn from the top-left corner.
        let offset = state.frames as f32 / 10.0;
        let dest_point = Vec2::new(offset, offset);
        canvas.draw(
            Text::new(format!("Gameplay! Frame {}", state.frames))
                .set_font("IBM_CGA")
                .set_scale(24.),
            dest_point,
        );

        canvas.finish(ctx)?;

        Ok(())
    }
}
