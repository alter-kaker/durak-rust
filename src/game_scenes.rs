use std::fmt::Debug;

use ggegui::{egui::Area, Gui};
use ggez::{
    glam::vec2,
    graphics::{Canvas, Color, DrawParam, Drawable},
    Context,
};

use crate::{
    deck::Deck,
    error::DurakError,
    game::GameState,
    hand::Hand,
    player::Player,
    scenes::{Scene, SceneTransition},
    storage,
};

pub struct MainMenu {
    state: GameState,
    no_of_players: usize,
}

impl Scene<GameState, DurakError> for MainMenu {
    fn update(
        mut self: Box<Self>,
        gui: &mut Gui,
        ctx: &mut Context,
    ) -> Result<Box<dyn Scene<GameState, DurakError>>, DurakError> {
        let next = Area::new("id")
            .show(&gui.ctx(), |ui| {
                ui.label("Main Menu");
                ui.label(format!("{} times played", &self.state.times_played));
                for player in &mut self.state.players[0..self.no_of_players] {
                    let name = &mut player.name;
                    if ui.text_edit_singleline(name).changed() {};
                }
                if ui.button("Add player").clicked() && self.no_of_players < 4 {
                    self.no_of_players += 1;
                    if self.no_of_players > self.state.players.len() {
                        self.state.players.push(Player {
                            name: String::new(),
                            hand: Hand::new(),
                            human: false,
                        });
                    }
                }
                if ui.button("Remove player").clicked() && self.no_of_players > 2 {
                    self.no_of_players -= 1;
                }
                Ok::<bool, DurakError>(ui.button("Next").clicked())
            })
            .inner?;
        gui.update(ctx);

        if next
            && !self
                .state
                .players
                .iter()
                .take(self.no_of_players)
                .any(|player| player.name.is_empty())
        {
            self.state.players.truncate(self.no_of_players);
            let result = self.transition()?;
            return Ok(Box::new(result));
        }

        Ok(self)
    }

    fn draw(&self, gui: &Gui, ctx: &mut Context) -> Result<(), DurakError> {
        let mut canvas = Canvas::from_frame(ctx, Color::from([0.1, 0.2, 0.3, 1.0]));

        gui.draw(&mut canvas, DrawParam::new());
        canvas.finish(ctx)?;

        Ok(())
    }

    fn new(state: GameState) -> Result<MainMenu, DurakError> {
        Ok(MainMenu {
            no_of_players: state.players.len(),
            state,
        })
    }

    fn take_state(self) -> GameState
    where
        Self: Sized,
    {
        self.state
    }
}

impl SceneTransition<GamePlay, GameState, DurakError> for MainMenu {}

pub struct GamePlay {
    state: GameState,
}

impl Scene<GameState, DurakError> for GamePlay {
    fn update(
        mut self: Box<Self>,
        gui: &mut Gui,
        ctx: &mut Context,
    ) -> Result<Box<dyn Scene<GameState, DurakError>>, DurakError> {
        let next = Area::new("id")
            .show(&gui.ctx(), |ui| {
                ui.label(format!("{} times played", &self.state.times_played));
                ui.button("Next").clicked()
            })
            .inner;
        gui.update(ctx);
        if next {
            self.state.times_played += 1;
            let result = <Self as SceneTransition<GameOver, GameState, DurakError>>::transition(*self)?;
            return Ok(Box::new(result));
        }
        Ok(self)
    }

    fn draw(&self, gui: &Gui, ctx: &mut Context) -> Result<(), DurakError> {
        let mut canvas = Canvas::from_frame(ctx, Color::from([0.1, 0.2, 0.3, 1.0]));
        for (i, player) in self.state.players.iter().enumerate() {
            canvas.draw(
                &player.hand,
                DrawParam::new().dest(vec2(100., 100. * (i + 1) as f32)),
            );
        }
        if let Some(deck) = &self.state.deck {
            canvas.draw(deck, DrawParam::new());
        }
        gui.draw(&mut canvas, DrawParam::new());
        canvas.finish(ctx)?;

        Ok(())
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32, ctx: &Context) -> Result<(), DurakError> {
        Ok(())
    }

    fn new(mut state: GameState) -> Result<GamePlay, DurakError> {
        let image = storage::card_image()?.ok_or("Cannot load card image")?;

        state.deck = Some(Deck::new(&image)?);
        for _ in 0..7 {
            for player in &mut state.players {
                let card = state
                    .deck
                    .as_mut()
                    .ok_or(DurakError::from("Deck Error"))?
                    .pop()
                    .ok_or(DurakError::from("Insufficient Cards"))?;
                player.hand.insert(card);
            }
        }
        let result = GamePlay { state };
        Ok(result)
    }

    fn take_state(self) -> GameState
    where
        Self: Sized,
    {
        self.state
    }
}
impl SceneTransition<GameOver, GameState, DurakError> for GamePlay {}
impl SceneTransition<MainMenu, GameState, DurakError> for GamePlay {}

pub struct GameOver {
    state: GameState,
}

impl Scene<GameState, DurakError> for GameOver {
    fn update(
        self: Box<Self>,
        gui: &mut Gui,
        ctx: &mut Context,
    ) -> Result<Box<dyn Scene<GameState, DurakError>>, DurakError> {
        let next = Area::new("id")
            .show(&gui.ctx(), |ui| {
                ui.label("Game Over");
                ui.label(format!("{} times played", &self.state.times_played));
                ui.button("Next").clicked()
            })
            .inner;
        gui.update(ctx);
        if next {
            let result = self.transition()?;
            return Ok(Box::new(result));
        }

        Ok(self)
    }

    fn draw(&self, gui: &Gui, ctx: &mut Context) -> Result<(), DurakError> {
        let mut canvas = Canvas::from_frame(ctx, Color::from([0.1, 0.2, 0.3, 1.0]));

        gui.draw(&mut canvas, DrawParam::new());
        canvas.finish(ctx)?;

        Ok(())
    }

    fn new(state: GameState) -> Result<GameOver, DurakError> {
        Ok(GameOver { state })
    }

    fn take_state(self) -> GameState
    where
        Self: Sized,
    {
        self.state
    }
}

impl SceneTransition<MainMenu, GameState, DurakError> for GameOver {}
