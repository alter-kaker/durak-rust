use std::fmt::Debug;

use ggegui::Gui;
use ggez::{event::EventHandler, graphics::FontData, Context, GameResult};

use crate::{
    deck::Deck,
    error::DurakError,
    hand::Hand,
    player::Player,
    scenes::{MainMenu, Scene, SceneError},
};

pub struct GameState {
    pub times_played: u32,
    pub players: Vec<Player>,
    pub deck: Option<Deck>,
    pub gui: Gui,
}

impl GameState {
    pub fn new(ctx: &Context) -> Result<Self, DurakError> {
        Ok(Self {
            times_played: 0,
            players: vec![
                Player {
                    name: String::new(),
                    hand: Hand { cards: Vec::new() },
                    human: true,
                },
                Player {
                    name: String::from("Opponent"),
                    hand: Hand { cards: Vec::new() },
                    human: false,
                },
            ],
            deck: None,
            gui: Gui::new(ctx),
        })
    }
}

pub struct Game<T, E: Debug> {
    scene: Option<Box<dyn Scene<T, E>>>,
    gui: Gui,
}

impl Game<GameState, DurakError> {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        ctx.gfx
            .add_font("IBM_CGA", FontData::from_path(ctx, "/Px437_IBM_CGA.ttf")?);

        Ok(Game {
            scene: Some(MainMenu::new_boxed(GameState::new(ctx)?)),
            gui: Gui::new(ctx),
        })
    }
}

impl<T, E> EventHandler<E> for Game<T, E>
where
    E: From<SceneError> + Debug,
{
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), E> {
        if let Some(scene) = self.scene.take() {
            self.scene = Some(scene.update(&mut self.gui, ctx)?);
            Ok(())
        } else {
            Err(SceneError::SceneMissing.into())
        }
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), E> {
        if let Some(scene) = self.scene.as_ref() {
            scene.draw(&self.gui, ctx)
        } else {
            Err(SceneError::SceneMissing.into())
        }
    }
}
