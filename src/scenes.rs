use std::{future::Future, pin::Pin};

use macroquad::{
    hash,
    prelude::*,
    text::draw_text,
    ui::{root_ui, widgets},
};

use crate::game_state::GameState;

pub type SceneFuture = Pin<Box<dyn Future<Output = Scene>>>;
pub struct Scene {
    func: fn(Self) -> SceneFuture,
    pub state: GameState,
}

impl Scene {
    pub fn new(func: fn(Scene) -> SceneFuture) -> Self {
        Self {
            func,
            state: GameState::new(),
        }
    }
    pub async fn update(self) -> Self {
        (self.func)(self).await
    }
}

pub fn main_menu(scene: Scene) -> SceneFuture {
    Box::pin(async move {
        let mut state = scene.state;
        let name = &mut state.player_name;
        loop {
            let mut pressed = false;
            clear_background(WHITE);
            draw_text("Main Menu", 10., 10., 24., WHITE);

            widgets::Group::new(hash!(), vec2(screen_width() - 20., screen_height() - 20.))
                .position(vec2(10., 10.))
                .ui(&mut root_ui(), |ui| {
                    ui.label(None, "Main Menu");
                    ui.input_text(hash!(), "Name", name);
                    if ui.button(None, "Next") {
                        pressed = true;
                    }
                    ui.label(
                        None,
                        format!("Times Played: {}", state.times_played).as_str(),
                    )
                });
            if pressed && !name.is_empty() {
                return Scene { func: game, state };
            }

            next_frame().await
        }
    })
}

pub fn game(scene: Scene) -> SceneFuture {
    Box::pin(async move {
        let mut state = scene.state;
        loop {
            clear_background(WHITE);
            draw_text(&state.player_name, 10., 10., 24., WHITE);

            let mut pressed = false;
            widgets::Group::new(hash!(), vec2(screen_width() - 20., screen_height() - 20.))
                .position(vec2(10., 10.))
                .ui(&mut root_ui(), |ui| {
                    ui.label(None, &state.player_name);
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
            clear_background(WHITE);
            draw_text("Game Over", 10., 10., 24., WHITE);

            let mut pressed = false;
            widgets::Group::new(hash!(), vec2(screen_width() - 20., screen_height() - 20.))
                .position(vec2(10., 10.))
                .ui(&mut root_ui(), |ui| {
                    ui.label(None, &state.player_name);
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
                    func: main_menu,
                    state,
                };
            }

            next_frame().await
        }
    })
}
