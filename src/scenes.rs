use std::fmt::Debug;

use ggegui::Gui;
use ggez::Context;

pub struct SceneWrapper<S, E>
where
    E: From<SceneError> + Debug,
{
    scene: Option<Box<dyn Scene<State = S, Error = E>>>,
}

impl<S, E> SceneWrapper<S, E>
where
    E: From<SceneError> + Debug,
{
    pub fn new(scene: Box<dyn Scene<State = S, Error = E>>) -> Self {
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
    pub fn mouse_motion_event(&mut self, x: f32, y: f32, ctx: &Context) -> Result<(), E> {
        if let Some(scene) = self.scene.as_mut() {
            scene.mouse_motion_event(x, y, ctx)
        } else {
            Err(SceneError::SceneMissing.into())
        }
    }
}

pub trait Scene {
    type State;
    type Error;
    fn update(
        self: Box<Self>,
        gui: &mut Gui,
        _ctx: &mut Context,
    ) -> Result<Box<dyn Scene<State = Self::State, Error = Self::Error>>, Self::Error>;
    fn draw(&self, gui: &Gui, ctx: &mut Context) -> Result<(), Self::Error>;
    fn mouse_motion_event(&mut self, x: f32, y: f32, ctx: &Context) -> Result<(), Self::Error> {
        Ok(())
    }
    fn new(state: Self::State) -> Result<Self, Self::Error>
    where
        Self: Sized;
    fn new_boxed(state: Self::State) -> Result<Box<Self>, Self::Error>
    where
        Self: Sized,
    {
        Ok(Box::new(Self::new(state)?))
    }

    fn take_state(self) -> Self::State
    where
        Self: Sized;
}

#[derive(Debug)]
pub enum SceneError {
    SceneMissing,
}

pub trait SceneTransition<U, S>
where
    Self: Scene<State = S> + Sized,
    U: Scene<State = S>,
{
    fn transition(self) -> Result<U, U::Error> {
        U::new(self.take_state())
    }
}