use std::fmt::Debug;

use ggegui::Gui;
use ggez::Context;

pub struct SceneWrapper<T, E>
where
    E: From<SceneError> + Debug,
{
    scene: Option<Box<dyn Scene<T, E>>>,
}

impl<T, E> SceneWrapper<T, E>
where
    E: From<SceneError> + Debug,
{
    pub fn new(scene: Box<dyn Scene<T, E>>) -> Self {
        SceneWrapper { scene: Some(scene) }
    }
    pub fn update(&mut self, gui: &mut Gui, ctx: &mut ggez::Context) -> Result<(), E> {
        if let Some(scene) = self.scene.take() {
            self.scene = Some(scene.update(gui, ctx)?);
            Ok(())
        } else {
            Err(SceneError::SceneMissing.into())
        }
    }

    pub fn draw(&mut self, gui: &Gui, ctx: &mut ggez::Context) -> Result<(), E> {
        if let Some(scene) = self.scene.as_ref() {
            scene.draw(gui, ctx)
        } else {
            Err(SceneError::SceneMissing.into())
        }
    }
}

pub trait Scene<T, E>
where
    E: Debug,
{
    fn update(
        self: Box<Self>,
        gui: &mut Gui,
        _ctx: &mut Context,
    ) -> Result<Box<dyn Scene<T, E>>, E>;
    fn draw(&self, gui: &Gui, ctx: &mut Context) -> Result<(), E>;
    fn new(state: T) -> Result<Self, E>
    where
        Self: Sized;
    fn new_boxed(state: T) -> Result<Box<Self>, E>
    where
        Self: Sized,
    {
        Ok(Box::new(Self::new(state)?))
    }
}

#[derive(Debug)]
pub enum SceneError {
    SceneMissing,
}
