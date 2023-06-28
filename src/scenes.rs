use ggegui::egui;
use ggez::{
    glam::Vec2,
    graphics::{Canvas, Color, Text, Drawable, DrawParam},
    Context,
};

pub struct SceneWrapper<T> {
    scene: Box<dyn Scene<T>>,
}

impl<T> SceneWrapper<T> {
    pub fn new(scene: Box<dyn Scene<T>>) -> Self {
        SceneWrapper { scene }
    }
    pub fn update(&mut self, state: &mut T, ctx: &mut Context) -> Result<(), DurakError> {
        if let Some(new_scene) = self.scene.update(state, ctx)? {
            self.scene = new_scene;
        }
        Ok(())
    }

    pub fn draw(&self, state: &mut GameState, ctx: &mut Context) -> Result<(), DurakError> {
        self.scene.draw(state, ctx)
    }
}

use crate::{error::DurakError, game_state::GameState};

pub trait Scene<T> {
    fn update(
        &self,
        state: &mut T,
        _ctx: &mut Context,
    ) -> Result<Option<Box<dyn Scene<T>>>, DurakError>;
    fn draw(&self, state: &mut GameState, ctx: &mut Context) -> Result<(), DurakError>;
    fn new_boxed() -> Box<Self>
    where
        Self: Sized;
}

pub struct MainMenu {}

impl Scene<GameState> for MainMenu {
    fn update(
        &self,
        state: &mut GameState,
        ctx: &mut Context,
    ) -> Result<Option<Box<dyn Scene<GameState>>>, DurakError> {
        state.frames += 1;
        let mut result: Option<Box<dyn Scene<GameState>>> = None;
        egui::Area::new("id").show(&state.gui.ctx(), |ui| {
            if ui.button("Next").clicked() {
                result = Some(Box::new(GamePlay::from(self)));
            }
        });
        state.gui.update(ctx);
        Ok(result)
    }

    fn draw(&self, state: &mut GameState, ctx: &mut Context) -> Result<(), DurakError> {
        let mut canvas = Canvas::from_frame(ctx, Color::from([0.1, 0.2, 0.3, 1.0]));

        // Text is drawn from the top-left corner.
        let offset = 10.;
        let dest_point = Vec2::new(offset, offset);
        canvas.draw(
            Text::new(format!("Main Menu! Frame {}", state.frames))
                .set_font("IBM_CGA")
                .set_scale(24.),
            dest_point,
        );

        state.gui.draw(&mut canvas, DrawParam::new());

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

impl Scene<GameState> for GamePlay {
    fn update(
        &self,
        state: &mut GameState,
        _ctx: &mut Context,
    ) -> Result<Option<Box<dyn Scene<GameState>>>, DurakError> {
        state.frames += 1;
        Ok(None)
    }

    fn draw(&self, state: &mut GameState, ctx: &mut Context) -> Result<(), DurakError> {
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

impl Scene<GameState> for GameOver {
    fn update(
        &self,
        _state: &mut GameState,
        _ctx: &mut Context,
    ) -> Result<Option<Box<dyn Scene<GameState>>>, DurakError> {
        todo!()
    }

    fn draw(&self, _state: &mut GameState, _ctx: &mut Context) -> Result<(), DurakError> {
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
