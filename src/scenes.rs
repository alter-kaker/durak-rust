use ggegui::{egui, Gui};
use ggez::{
    graphics::{Canvas, Color, DrawParam, Drawable},
    Context,
};

pub struct SceneWrapper<T> {
    scene: Option<Box<dyn Scene<T>>>,
}

impl<T> SceneWrapper<T> {
    pub fn new(scene: Box<dyn Scene<T>>) -> Self {
        SceneWrapper { scene: Some(scene) }
    }
    pub fn update(&mut self, gui: &mut Gui, ctx: &mut Context) -> Result<(), DurakError> {
        if let Some(scene) = self.scene.take() {
            self.scene = Some(scene.update(gui, ctx)?);
            Ok(())
        } else {
            Err("No scene".into())
        }
    }

    pub fn draw(&self, gui: &Gui, ctx: &mut Context) -> Result<(), DurakError> {
        if let Some(scene) = self.scene.as_ref() {
            scene.draw(gui, ctx)
        } else {
            Err("No scene".into())
        }
    }
}

use crate::{error::DurakError, game_state::GameState};

pub trait Scene<T> {
    fn update(
        self: Box<Self>,
        gui: &mut Gui,
        _ctx: &mut Context,
    ) -> Result<Box<dyn Scene<T>>, DurakError>;
    fn draw(&self, gui: &Gui, ctx: &mut Context) -> Result<(), DurakError>;
    fn new_boxed(state: T) -> Box<Self>
    where
        Self: Sized;
}

pub struct MainMenu {
    state: GameState,
}

impl Scene<GameState> for MainMenu {
    fn update(
        self: Box<Self>,
        gui: &mut Gui,
        ctx: &mut Context,
    ) -> Result<Box<dyn Scene<GameState>>, DurakError> {
        let next = egui::Area::new("id")
            .show(&gui.ctx(), |ui| {
                ui.label("Main Menu");
                ui.label(format!("{} times played", &self.state.times_played));
                ui.button("Next").clicked()
            })
            .inner;
        gui.update(ctx);
        if next {
            return Ok(Box::new(GamePlay::from(*self)));
        }

        Ok(self)
    }

    fn draw(&self, gui: &Gui, ctx: &mut Context) -> Result<(), DurakError> {
        let mut canvas = Canvas::from_frame(ctx, Color::from([0.1, 0.2, 0.3, 1.0]));
        gui.draw(&mut canvas, DrawParam::new());
        canvas.finish(ctx)?;

        Ok(())
    }

    fn new_boxed(state: GameState) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(Self { state })
    }
}

impl From<GameOver> for MainMenu {
    fn from(value: GameOver) -> Self {
        MainMenu { state: value.state }
    }
}
pub struct GamePlay {
    state: GameState,
}

impl Scene<GameState> for GamePlay {
    fn update(
        mut self: Box<Self>,
        gui: &mut Gui,
        ctx: &mut Context,
    ) -> Result<Box<dyn Scene<GameState>>, DurakError> {
        let next = egui::Area::new("id")
            .show(&gui.ctx(), |ui| {
                ui.label("Game Play");
                ui.label(format!("{} times played", &self.state.times_played));
                ui.button("Next").clicked()
            })
            .inner;
        gui.update(ctx);
        if next {
            self.state.times_played += 1;
            return Ok(Box::new(GameOver::from(*self)));
        }

        Ok(self)
    }

    fn draw(&self, gui: &Gui, ctx: &mut Context) -> Result<(), DurakError> {
        let mut canvas = Canvas::from_frame(ctx, Color::from([0.1, 0.2, 0.3, 1.0]));

        gui.draw(&mut canvas, DrawParam::new());
        canvas.finish(ctx)?;

        Ok(())
    }

    fn new_boxed(state: GameState) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(Self { state })
    }
}

impl From<MainMenu> for GamePlay {
    fn from(value: MainMenu) -> Self {
        GamePlay { state: value.state }
    }
}

pub struct GameOver {
    state: GameState,
}

impl Scene<GameState> for GameOver {
    fn update(
        self: Box<Self>,
        gui: &mut Gui,
        ctx: &mut Context,
    ) -> Result<Box<dyn Scene<GameState>>, DurakError> {
        let next = egui::Area::new("id")
            .show(&gui.ctx(), |ui| {
                ui.label("Game Over");
                ui.label(format!("{} times played", &self.state.times_played));
                ui.button("Next").clicked()
            })
            .inner;
        gui.update(ctx);
        if next {
            return Ok(Box::new(MainMenu::from(*self)));
        }

        Ok(self)
    }

    fn draw(&self, gui: &Gui, ctx: &mut Context) -> Result<(), DurakError> {
        let mut canvas = Canvas::from_frame(ctx, Color::from([0.1, 0.2, 0.3, 1.0]));

        gui.draw(&mut canvas, DrawParam::new());
        canvas.finish(ctx)?;

        Ok(())
    }

    fn new_boxed(state: GameState) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(Self { state })
    }
}

impl From<GamePlay> for GameOver {
    fn from(value: GamePlay) -> Self {
        GameOver { state: value.state }
    }
}
