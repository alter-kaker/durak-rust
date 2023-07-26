use std::fmt::Debug;

use ggegui::Gui;
use ggez::{event::EventHandler, graphics::FontData, Context, GameResult};

use crate::{
    card::Card,
    deck::Deck,
    error::DurakError,
    game_scenes::MainMenu,
    hand::Hand,
    player::Player,
    scenes::{Scene, SceneError, SceneWrapper}, mat::Mat,
};

pub struct DurakState {
    pub times_played: u32,
    pub players: Vec<Player>,
    pub deck: Option<Deck>,
    pub mat: Option<Mat>,
    pub discard_pile: Vec<Card>,
    pub held_card: Option<Card>,
    pub gui: Gui,
}

impl DurakState {
    pub fn new(ctx: &Context) -> Result<Self, DurakError> {
        Ok(Self {
            times_played: 0,
            players: vec![
                Player {
                    name: String::new(),
                    hand: Hand::new(),
                    human: true,
                },
                Player {
                    name: String::from("Opponent"),
                    hand: Hand::new(),
                    human: false,
                },
            ],
            deck: None,
            mat: None,
            discard_pile: Vec::new(),
            held_card: None,
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

impl Game<DurakState, DurakError> {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        ctx.gfx
            .add_font("IBM_CGA", FontData::from_path(ctx, "/Px437_IBM_CGA.ttf")?);

        let state = DurakState::new(ctx)?;
        let scene = MainMenu::new_boxed(state, ctx)?;
        Ok(Game {
            scene: SceneWrapper::new(scene),
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

    fn text_input_event(&mut self, _ctx: &mut ggez::Context, character: char) -> Result<(), E> {
        self.gui.input.text_input_event(character);
        Ok(())
    }
    fn mouse_motion_event(
        &mut self,
        ctx: &mut Context,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
    ) -> Result<(), E> {
        self.scene.mouse_motion_event(x, y, dx, dy, ctx)
    }
    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        _button: ggez::event::MouseButton,
        x: f32,
        y: f32,
    ) -> Result<(), E> {
        self.scene.mouse_button_down_event(x, y, ctx)
    }
    fn mouse_button_up_event(
        &mut self,
        ctx: &mut Context,
        _button: ggez::event::MouseButton,
        x: f32,
        y: f32,
    ) -> Result<(), E> {
        self.scene.mouse_button_up_event(x, y, ctx)
    }
}
