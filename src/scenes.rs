use ggez::{
    glam::Vec2,
    graphics::{Canvas, Color, Text},
    Context,
};

use crate::{error::DurakError, game_state::GameState};

pub type UpdateFunc = fn(&mut GameState, &Context) -> Result<(), DurakError>;
pub type DrawFunc = fn(&GameState, &mut Context) -> Result<(), DurakError>;
pub struct Scene {
    update_func: UpdateFunc,
    draw_func: DrawFunc,
}

impl Scene {
    pub fn new(update_func: UpdateFunc, draw_func: DrawFunc) -> Self {
        Self {
            update_func,
            draw_func,
        }
    }
    pub fn update(&self) -> UpdateFunc {
        self.update_func
    }

    pub fn draw(&self) -> DrawFunc {
        self.draw_func
    }
}

pub fn main_menu(state: &mut GameState, _ctx: &Context) -> Result<(), DurakError> {
    state.frames += 1;
    if state.frames > 100 {
        state.scene = Scene::new(game, game_draw)
    }
    Ok(())
}

pub fn main_menu_draw(state: &GameState, ctx: &mut Context) -> Result<(), DurakError> {
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

pub fn game(state: &mut GameState, ctx: &Context) -> Result<(), DurakError> {
    state.frames += 1;
    Ok(())
}

pub fn game_draw(state: &GameState, ctx: &mut Context) -> Result<(), DurakError> {
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

pub fn game_over(state: &mut GameState) -> Result<(), DurakError> {
    todo!()
}
