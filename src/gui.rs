use macroquad::{
    hash,
    prelude::*,
    text::draw_text,
    ui::{root_ui, widgets},
};

use crate::game_state::GameState;

pub async fn main_menu(mut state: GameState) -> GameState {
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
            return state;
        }

        next_frame().await
    }
}

pub async fn game(mut state: GameState) -> GameState {
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
            return state;
        }

        next_frame().await
    }
}

pub async fn game_over(mut state: GameState) -> GameState {
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
            return state;
        }

        next_frame().await
    }
}
