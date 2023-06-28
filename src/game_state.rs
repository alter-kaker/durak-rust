use ggegui::{egui, Gui};
use ggez::{event::EventHandler, graphics::FontData, Context, GameResult};

use crate::{
    deck::Deck,
    error::DurakError,
    hand::Hand,
    player::Player,
    scenes::{MainMenu, Scene, SceneWrapper},
};

pub struct GameState {
    pub times_played: u32,
    pub players: Vec<Player>,
    pub deck: Option<Deck>,
    pub frames: usize,
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
            frames: 0,
            gui: Gui::new(ctx),
        })
    }
}

pub struct Game {
    pub scene: SceneWrapper<GameState>,
    pub state: GameState,
}

impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        ctx.gfx
            .add_font("IBM_CGA", FontData::from_path(ctx, "/Px437_IBM_CGA.ttf")?);

        Ok(Game {
            scene: SceneWrapper::new(MainMenu::new_boxed()),
            state: GameState::new(ctx)?,
        })
    }
}

impl EventHandler<DurakError> for Game {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), DurakError> {
        self.scene.update(&mut self.state, ctx)?;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), DurakError> {
        self.scene.draw(&mut self.state, ctx)
    }
}
