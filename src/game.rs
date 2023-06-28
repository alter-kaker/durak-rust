use std::fmt::Debug;

use ggegui::Gui;
use ggez::{event::EventHandler, graphics::FontData, Context, GameResult};

use crate::{
    deck::Deck,
    error::DurakError,
    hand::Hand,
    player::Player,
    scenes::{SceneError, SceneWrapper, Scene}, game_scenes::MainMenu,
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

pub struct Game<T, E: Debug>
where
    E: From<SceneError> + Debug,
{
    scene: SceneWrapper<T, E>,
    gui: Gui,
}

impl Game<GameState, DurakError>
where
{
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        ctx.gfx
            .add_font("IBM_CGA", FontData::from_path(ctx, "/Px437_IBM_CGA.ttf")?);

        Ok(Game {
            scene: SceneWrapper::new(MainMenu::new_boxed(GameState::new(ctx)?)),
            gui: Gui::new(ctx),
        })
    }
}

impl<T, E> EventHandler<E> for Game<T, E>
where
    E: From<SceneError> + Debug,
{
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), E> {
        self.scene.update(&mut self.gui, ctx)
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), E> {
        self.scene.draw(&self.gui, ctx)
    }
}
