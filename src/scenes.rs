use std::any::Any;

use ggez::{
    glam::Vec2,
    graphics::{Canvas, Color, Text},
    Context,
};

pub struct SceneWrapper {
    scene: Box<dyn Scene>,
}

impl SceneWrapper {
    pub fn new(scene: Box<dyn Scene>) -> Self {
        SceneWrapper { scene }
    }
    pub fn update(&mut self, state: &mut GameState, ctx: &Context) -> Result<(), DurakError> {
        if let Some(new_scene) = self.scene.update(state, ctx)? {
            self.scene = new_scene;
        }
        Ok(())
    }

    pub fn draw(&self, state: &GameState, ctx: &mut Context) -> Result<(), DurakError> {
        self.scene.draw(state, ctx)
    }
}

use crate::{error::DurakError, game_state::GameState};

pub trait SceneTransition<T> {
    fn to(value: T) -> bool;
}

pub trait Scene {
    fn update(
        &self,
        state: &mut GameState,
        _ctx: &Context,
    ) -> Result<Option<Box<dyn Scene>>, DurakError>;
    fn draw(&self, state: &GameState, ctx: &mut Context) -> Result<(), DurakError>;
    fn new_boxed() -> Box<Self>
    where
        Self: Sized;
}

pub trait NewBoxed {
    fn new_boxed() -> Box<Self>;
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
            return Ok(Some(Box::new(GamePlay::from(self))));
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

    fn new_boxed() -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(Self {})
    }
}

impl From<&GameOver> for MainMenu {
    fn from(_value: &GameOver) -> Self {
        MainMenu {}
    }
}
pub struct GamePlay {}

impl Scene for GamePlay {
    fn update(
        &self,
        state: &mut GameState,
        _ctx: &Context,
    ) -> Result<Option<Box<dyn Scene>>, DurakError> {
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

    fn new_boxed() -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(Self {})
    }
}

impl From<&MainMenu> for GamePlay {
    fn from(_value: &MainMenu) -> Self {
        GamePlay {}
    }
}

pub struct GameOver {}

impl Scene for GameOver {
    fn update(
        &self,
        state: &mut GameState,
        _ctx: &Context,
    ) -> Result<Option<Box<dyn Scene>>, DurakError> {
        todo!()
    }

    fn draw(&self, state: &GameState, ctx: &mut Context) -> Result<(), DurakError> {
        todo!()
    }

    fn new_boxed() -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(Self {})
    }
}

impl From<&GamePlay> for GameOver {
    fn from(_value: &GamePlay) -> Self {
        GameOver {}
    }
}
