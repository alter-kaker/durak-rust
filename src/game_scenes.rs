use ggegui::{egui::Area, Gui};
use ggez::{
    graphics::{Canvas, Color, DrawParam, Drawable},
    Context,
};

use crate::{error::DurakError, game::GameState, hand::Hand, player::Player, scenes::Scene};

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
                            hand: Hand { cards: Vec::new() },
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
            return Ok(Box::new(GamePlay::from(*self)));
        }

        Ok(self)
    }

    fn draw(&self, gui: &Gui, ctx: &mut Context) -> Result<(), DurakError> {
        let mut canvas = Canvas::from_frame(ctx, Color::from([0.1, 0.2, 0.3, 1.0]));
        gui.draw(&mut canvas, DrawParam::new());
        canvas.finish(ctx)?;

        Ok(())
    }

    fn new(state: GameState) -> MainMenu {
        MainMenu {
            no_of_players: state.players.len(),
            state,
        }
    }
}

impl From<GameOver> for MainMenu {
    fn from(value: GameOver) -> Self {
        MainMenu::new(value.state)
    }
}
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
                ui.label("Game Play");
                for name in self.state.players.iter().map(|player| &player.name) {
                    ui.label(name);
                }
                ui.label(format!("{} times played", &self.state.times_played));
                ui.button("Next").clicked()
            })
            .inner;
        gui.update(ctx);
        if next {
            self.state.times_played += 1;
            return Ok(Box::new(GameOver::from(*self)));
        }

        Ok(self)
    }

    fn draw(&self, gui: &Gui, ctx: &mut Context) -> Result<(), DurakError> {
        let mut canvas = Canvas::from_frame(ctx, Color::from([0.1, 0.2, 0.3, 1.0]));

        gui.draw(&mut canvas, DrawParam::new());
        canvas.finish(ctx)?;

        Ok(())
    }

    fn new(state: GameState) -> GamePlay {
        GamePlay { state }
    }
}

impl From<MainMenu> for GamePlay {
    fn from(value: MainMenu) -> Self {
        GamePlay { state: value.state }
    }
}

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
            return Ok(Box::new(MainMenu::from(*self)));
        }

        Ok(self)
    }

    fn draw(&self, gui: &Gui, ctx: &mut Context) -> Result<(), DurakError> {
        let mut canvas = Canvas::from_frame(ctx, Color::from([0.1, 0.2, 0.3, 1.0]));

        gui.draw(&mut canvas, DrawParam::new());
        canvas.finish(ctx)?;

        Ok(())
    }

    fn new(state: GameState) -> GameOver {
        GameOver { state }
    }
}

impl From<GamePlay> for GameOver {
    fn from(value: GamePlay) -> Self {
        GameOver { state: value.state }
    }
}
