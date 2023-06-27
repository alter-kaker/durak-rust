use ggez::{event::EventHandler, graphics::FontData, Context, GameResult};

use crate::{
    deck::Deck,
    error::DurakError,
    hand::Hand,
    player::Player,
    scenes::{MainMenu, Scene},
};

pub struct GameState {
    pub times_played: u32,
    pub players: Vec<Player>,
    pub deck: Option<Deck>,
    pub frames: usize,
}

impl GameState {
    pub fn new() -> Result<Self, DurakError> {
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
        })
    }
}

pub struct Game {
    pub scene: Scene,
    pub state: GameState,
}

impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        ctx.gfx
            .add_font("IBM_CGA", FontData::from_path(ctx, "/Px437_IBM_CGA.ttf")?);

        Ok(Game {
            scene: Scene::MainMenu(MainMenu {}),
            state: GameState::new()?,
        })
    }
}

impl EventHandler<DurakError> for Game {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), DurakError> {
        if let Some(to_scene) = self.scene.update(&mut self.state, ctx)? {
            if let Some(scene) = self.scene.to(to_scene) {
                self.scene = scene;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), DurakError> {
        self.scene.draw(&self.state, ctx)
    }
}