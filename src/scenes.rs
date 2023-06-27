use ggez::{
    glam::Vec2,
    graphics::{Canvas, Color, Text},
    Context,
};

pub enum Scene {
    MainMenu(MainMenu),
    GamePlay(GamePlay),
    GameOver(GameOver),
}

impl Scene {
    pub fn update(
        &self,
        state: &mut GameState,
        ctx: &Context,
    ) -> Result<Option<ToScene>, DurakError> {
        match self {
            Scene::MainMenu(scene) => scene.update(state, ctx),
            Scene::GamePlay(scene) => scene.update(state, ctx),
            Scene::GameOver(scene) => scene.update(state, ctx),
        }
    }

    pub fn draw(&self, state: &GameState, ctx: &mut Context) -> Result<(), DurakError> {
        match self {
            Scene::MainMenu(scene) => scene.draw(state, ctx),
            Scene::GamePlay(scene) => scene.draw(state, ctx),
            Scene::GameOver(scene) => scene.draw(state, ctx),
        }
    }

    pub fn to(&self, to_scene: ToScene) -> Option<Self> {
        match (self, to_scene) {
            (Scene::MainMenu(scene), ToScene::GamePlay) => Some(Scene::GamePlay(GamePlay {})),
            (Scene::GamePlay(scene), ToScene::GameOver) => Some(Scene::GameOver(GameOver {})),
            (Scene::GameOver(scene), ToScene::MainMenu) => Some(Scene::MainMenu(scene.into())),
            _ => None,
        }
    }
}

pub enum ToScene {
    MainMenu,
    GamePlay,
    GameOver,
}

use crate::{error::DurakError, game_state::GameState};

pub type SceneResult = Result<Option<ToScene>, DurakError>;

pub struct MainMenu {}

impl MainMenu {
    fn new() -> Self {
        Self {}
    }

    fn update(&self, state: &mut GameState, _ctx: &Context) -> SceneResult {
        state.frames += 1;
        if state.frames > 100 {
            return Ok(Some(ToScene::GamePlay));
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

impl From<&GameOver> for MainMenu {
    fn from(_value: &GameOver) -> Self {
        MainMenu {}
    }
}

pub struct GamePlay {}

impl GamePlay {
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

impl From<&MainMenu> for GamePlay {
    fn from(_value: &MainMenu) -> Self {
        GamePlay {}
    }
}

pub struct GameOver {}

impl GameOver {
    fn update(&self, state: &mut GameState, _ctx: &Context) -> SceneResult {
        todo!()
    }

    fn draw(&self, state: &GameState, ctx: &mut Context) -> Result<(), DurakError> {
        todo!()
    }
}

impl From<&GamePlay> for GameOver {
    fn from(_value: &GamePlay) -> Self {
        GameOver {}
    }
}
