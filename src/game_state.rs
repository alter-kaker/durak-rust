use ggez::{event::EventHandler, graphics::FontData, Context, GameResult};

use crate::{
    deck::Deck,
    error::DurakError,
    hand::Hand,
    player::Player,
    scenes::{main_menu, main_menu_draw, Scene},
};

pub struct GameState {
    pub times_played: u32,
    pub players: Vec<Player>,
    pub deck: Option<Deck>,
    pub scene: Scene,
    pub frames: usize,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        ctx.gfx
            .add_font("IBM_CGA", FontData::from_path(ctx, "/Px437_IBM_CGA.ttf")?);

        let scene = Scene::new(main_menu, main_menu_draw);

        Ok(GameState {
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
            scene,
            frames: 0,
        })
    }
}

impl EventHandler<DurakError> for GameState {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), DurakError> {
        (self.scene.update())(self, ctx)
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), DurakError> {
        (self.scene.draw())(self, ctx)
    }
}
