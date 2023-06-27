use ggez::{event::EventHandler, graphics::FontData, Context, GameResult};

use crate::{
    deck::Deck,
    error::DurakError,
    hand::Hand,
    player::Player,
    scenes::{GameOver, GamePlay, MainMenu, Scene, ToScene},
};

pub enum SceneWrapper {
    MainMenu(MainMenu),
    GamePlay(GamePlay),
    GameOver(GameOver),
}

impl SceneWrapper {
    pub fn update(
        &self,
        state: &mut GameState,
        ctx: &Context,
    ) -> Result<Option<ToScene>, DurakError> {
        match self {
            SceneWrapper::MainMenu(scene) => scene.update(state, ctx),
            SceneWrapper::GamePlay(scene) => scene.update(state, ctx),
            SceneWrapper::GameOver(scene) => scene.update(state, ctx),
        }
    }

    pub fn draw(&self, state: &GameState, ctx: &mut Context) -> Result<(), DurakError> {
        match self {
            SceneWrapper::MainMenu(scene) => scene.draw(state, ctx),
            SceneWrapper::GamePlay(scene) => scene.draw(state, ctx),
            SceneWrapper::GameOver(scene) => scene.draw(state, ctx),
        }
    }

    pub fn to(&self, to_scene: ToScene) -> Option<Self> {
        match (self, to_scene) {
            (SceneWrapper::MainMenu(_), ToScene::GamePlay) => {
                Some(SceneWrapper::GamePlay(GamePlay {}))
            }
            (SceneWrapper::GamePlay(_), ToScene::GameOver) => {
                Some(SceneWrapper::GameOver(GameOver {}))
            }
            (SceneWrapper::GameOver(_), ToScene::MainMenu) => {
                Some(SceneWrapper::MainMenu(MainMenu {}))
            }
            _ => None,
        }
    }
}

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

// impl From<GameState<GamePlay>> for GameState<GameOver> {}
// impl From<GameState<GameOver>> for GameState<MainMenu> {}

pub struct Game {
    pub scene: SceneWrapper,
    pub state: GameState,
}

impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        ctx.gfx
            .add_font("IBM_CGA", FontData::from_path(ctx, "/Px437_IBM_CGA.ttf")?);

        let scene = MainMenu {};

        Ok(Game {
            scene: SceneWrapper::MainMenu(MainMenu {}),
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
