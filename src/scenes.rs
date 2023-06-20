use std::{future::Future, pin::Pin};

use macroquad::{
    hash,
    prelude::*,
    text::draw_text,
    ui::{root_ui, widgets},
};

use crate::{
    deck::Deck,
    game_state::GameState,
    hand::Hand,
    player::{self, Player},
};

pub type SceneFuture = Pin<Box<dyn Future<Output = Scene>>>;
pub struct Scene {
    func: fn(Self) -> SceneFuture,
    pub state: GameState,
}

impl Scene {
    pub fn new(func: fn(Scene) -> SceneFuture, state: GameState) -> Self {
        Self { func, state }
    }
    pub async fn update(self) -> Self {
        (self.func)(self).await
    }
}

pub fn main_menu(scene: Scene) -> SceneFuture {
    Box::pin(async move {
        let mut state = scene.state;
        let mut no_of_players = state.players.len();
        let mut player_names: Vec<String> = state
            .players
            .iter()
            .map(|player| player.name.clone())
            .collect();
        root_ui().set_input_focus(hash!(format!("player_0_input_box")));

        loop {
            let mut pressed = is_key_pressed(KeyCode::Enter);
            clear_background(BLACK);
            draw_text("Main Menu", 10., 10., 24., WHITE);

            widgets::Window::new(
                hash!(),
                vec2(10., 10.),
                vec2(screen_width() - 20., screen_height() - 20.),
            )
            .movable(false)
            .ui(&mut root_ui(), |ui| {
                ui.label(None, "Main Menu");
                for (i, name) in &mut player_names.iter_mut().enumerate() {
                    if i < no_of_players {
                        ui.input_text(
                            hash!(format!("player_{i}_input_box")),
                            &format!("Player {} Name", i),
                            name,
                        );
                    }
                }
                if ui.button(None, "Remove Player") && no_of_players > 2 {
                    no_of_players -= 1;
                }
                if ui.button(None, "Add Player") && no_of_players < 4 {
                    no_of_players += 1;
                }
                if ui.button(None, "Next") {
                    pressed = true;
                }
                ui.label(None, &format!("Times Played: {}", state.times_played));
            });
            while player_names.len() < no_of_players {
                player_names.push(format!("Opponent {}", player_names.len()));
            }
            if pressed
                && !player_names
                    .iter()
                    .enumerate()
                    .any(|(i, name)| i < no_of_players && name.is_empty())
            {
                player_names.truncate(no_of_players);
                state.players = player_names
                    .into_iter()
                    .enumerate()
                    .map(|(i, name)| Player::new(name, i == 0))
                    .collect();
                return Scene { func: game, state };
            }

            next_frame().await
        }
    })
}

pub fn game(scene: Scene) -> SceneFuture {
    Box::pin(async move {
        let mut state = scene.state;
        state.deck = Some(Deck::new());
        for player in &mut state.players {
            player.hand.cards = Vec::new();
        }

        for _ in 0..7 {
            for player in &mut state.players {
                let card = state.deck.as_mut().unwrap().pop().unwrap();
                player.hand.cards.push(card);
            }
        }

        loop {
            clear_background(BLACK);
            draw_text(
                &format!("Player Name: {}", &state.players[0].name),
                10.,
                10.,
                24.,
                WHITE,
            );

            let mut pressed = is_key_pressed(KeyCode::Enter);
            widgets::Window::new(
                hash!(),
                vec2(10., 10.),
                vec2(screen_width() - 20., screen_height() - 20.),
            )
            .movable(false)
            .ui(&mut root_ui(), |ui| {
                ui.label(None, &state.players[0].name);
                for player in &state.players {
                    ui.label(None, &format!("{}'s cards:", player.name));
                    for card in &player.hand.cards {
                        ui.label(None, &format!("{:?} of {:?}", card.rank, card.suit))
                    }
                }

                if ui.button(None, "Next") {
                    pressed = true;
                }

                ui.label(
                    None,
                    format!("Times Played: {}", state.times_played).as_str(),
                )
            });
            if pressed {
                return Scene {
                    func: game_over,
                    state,
                };
            }

            next_frame().await
        }
    })
}

pub fn game_over(scene: Scene) -> SceneFuture {
    Box::pin(async move {
        let mut state = scene.state;
        state.times_played += 1;

        loop {
            clear_background(BLACK);
            draw_text("Game Over", 10., 10., 24., WHITE);

            let mut pressed = is_key_pressed(KeyCode::Enter);
            widgets::Window::new(
                hash!(),
                vec2(10., 10.),
                vec2(screen_width() - 20., screen_height() - 20.),
            )
            .movable(false)
            .ui(&mut root_ui(), |ui| {
                ui.label(None, &state.players[0].name);
                if ui.button(None, "Next") {
                    pressed = true;
                }
                ui.label(None, &format!("Times Played: {}", state.times_played))
            });
            if pressed {
                return Scene {
                    func: main_menu,
                    state,
                };
            }

            next_frame().await
        }
    })
}
