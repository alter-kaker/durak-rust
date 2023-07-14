use ggegui::{egui::Area, Gui};
use ggez::{
    glam::{vec2, Vec2},
    graphics::{Canvas, Color, DrawMode, DrawParam, Drawable, Mesh},
    winit::dpi::PhysicalSize,
    Context,
};

use crate::{
    deck::Deck,
    error::DurakError,
    game::DurakState,
    hand::Hand,
    player::Player,
    scenes::{Scene, SceneResult, SceneTransition},
    storage,
};

pub trait DurakSceneTransition<U: Scene<State = DurakState, Error = DurakError>>:
    SceneTransition<U, DurakState>
{
}
impl<S, U> SceneTransition<U, DurakState> for S
where
    S: Scene<State = DurakState, Error = DurakError> + DurakSceneTransition<U>,
    U: Scene<State = DurakState, Error = DurakError>,
{
}

pub struct MainMenu {
    state: DurakState,
    no_of_players: usize,
}

impl Scene for MainMenu {
    type State = DurakState;
    type Error = DurakError;

    fn update(mut self: Box<Self>, gui: &mut Gui, ctx: &mut Context) -> SceneResult<Self> {
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
            let result = self.transition(ctx)?;
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

    fn new(mut state: DurakState, _ctx: &Context) -> Result<MainMenu, DurakError> {
        for player in &mut state.players {
            player.hand.empty();
        }
        Ok(MainMenu {
            no_of_players: state.players.len(),
            state,
        })
    }

    fn take_state(self) -> DurakState
    where
        Self: Sized,
    {
        self.state
    }
}

impl DurakSceneTransition<GamePlay> for MainMenu {}

pub struct GamePlay {
    state: DurakState,
}

impl Scene for GamePlay {
    type State = DurakState;

    type Error = DurakError;
    fn update(mut self: Box<Self>, gui: &mut Gui, ctx: &mut Context) -> SceneResult<Self> {
        let (next, pop_card) = Area::new("id")
            .show(&gui.ctx(), |ui| {
                ui.label(format!("{} times played", &self.state.times_played));
                (ui.button("Next").clicked(), ui.button("pop").clicked())
            })
            .inner;
        gui.update(ctx);
        if pop_card {
            if let Some(card) = self.state.deck.as_mut().unwrap().pop() {
                self.state.players[0].insert_card(card);
            }
        }
        if next {
            self.state.times_played += 1;
            let result = <Self as SceneTransition<GameOver, DurakState>>::transition(*self, ctx)?;
            return Ok(Box::new(result));
        }
        Ok(self)
    }

    fn draw(&self, gui: &Gui, ctx: &mut Context) -> Result<(), DurakError> {
        let mut canvas = Canvas::from_frame(ctx, Color::from([0.1, 0.2, 0.3, 1.0]));
        for player in self.state.players.iter() {
            let circle = Mesh::new_circle(ctx, DrawMode::fill(), Vec2::ZERO, 5., 1., Color::RED)?;

            player.hand.draw(&mut canvas);
            canvas.draw(&circle, DrawParam::new().dest(player.hand.get_pos()));
        }

        if let Some(deck) = &self.state.deck {
            canvas.draw(deck, DrawParam::new());
        }
        gui.draw(&mut canvas, DrawParam::new());
        canvas.finish(ctx)?;

        Ok(())
    }

    fn mouse_motion_event(&mut self, _x: f32, _y: f32, _ctx: &Context) -> Result<(), DurakError> {
        Ok(())
    }

    fn new(mut state: DurakState, ctx: &Context) -> Result<GamePlay, DurakError> {
        let image = storage::card_image()?.ok_or("Cannot load card image")?;

        let mut deck = Deck::new(&image)?;
        deck.shuffle();

        let rotation_step = (360. / state.players.len() as f32).to_radians();

        for (i, player) in state.players.iter_mut().enumerate() {
            let rotation = rotation_step * i as f32;
            let PhysicalSize { height, width, .. } = ctx.gfx.window().inner_size();
            let table_size = (height.min(width) / 2) as f32;
            let radius = vec2(0., table_size * 3. / 4.);
            let center = vec2(table_size, table_size);
            let pos = center + Vec2::from_angle(rotation).rotate(radius);

            player.hand.set_pos(pos);
            player.hand.set_rotation(rotation);
        }

        for _ in 0..7 {
            for player in &mut state.players {
                let mut card = deck.pop().ok_or(DurakError::from("Insufficient Cards"))?;
                player.insert_card(card);
            }
        }

        state.deck = Some(deck);
        let result = GamePlay { state };
        Ok(result)
    }

    fn take_state(self) -> DurakState
    where
        Self: Sized,
    {
        self.state
    }
}
impl SceneTransition<GameOver, DurakState> for GamePlay {}
impl SceneTransition<MainMenu, DurakState> for GamePlay {}

pub struct GameOver {
    state: DurakState,
}

impl Scene for GameOver {
    type State = DurakState;

    type Error = DurakError;
    fn update(self: Box<Self>, gui: &mut Gui, ctx: &mut Context) -> SceneResult<Self> {
        let next = Area::new("id")
            .show(&gui.ctx(), |ui| {
                ui.label("Game Over");
                ui.label(format!("{} times played", &self.state.times_played));
                ui.button("Next").clicked()
            })
            .inner;
        gui.update(ctx);
        if next {
            let result = self.transition(ctx)?;
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

    fn new(state: DurakState, _ctx: &Context) -> Result<GameOver, DurakError> {
        Ok(GameOver { state })
    }

    fn take_state(self) -> DurakState
    where
        Self: Sized,
    {
        self.state
    }
}

impl SceneTransition<MainMenu, DurakState> for GameOver {}
