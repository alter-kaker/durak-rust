use macroquad::{
    prelude::*,
    text::draw_text,
    ui::{root_ui, widgets},
};

use crate::scene::Scene;

pub async fn main_menu() -> Scene {
    loop {
        draw_text("Main Menu", 10., 10., 24., WHITE);

        if widgets::Button::new("Next")
            .size(vec2(300., 300.))
            .position(vec2(
                screen_width() / 2. - 150.,
                screen_height() / 2. - 150.,
            ))
            .ui(&mut root_ui())
        {
            return Scene::Game;
        }

        next_frame().await
    }
}

pub async fn game() -> Scene {
    loop {
        draw_text("Game", 10., 10., 24., WHITE);

        if widgets::Button::new("Next")
            .size(vec2(300., 300.))
            .position(vec2(
                screen_width() / 2. - 150.,
                screen_height() / 2. - 150.,
            ))
            .ui(&mut root_ui())
        {
            return Scene::GameOver;
        }

        next_frame().await
    }
}

pub async fn game_over() -> Scene {
    loop {
        draw_text("Game Over", 10., 10., 24., WHITE);

        if widgets::Button::new("Next")
            .size(vec2(300., 300.))
            .position(vec2(
                screen_width() / 2. - 150.,
                screen_height() / 2. - 150.,
            ))
            .ui(&mut root_ui())
        {
            return Scene::MainMenu;
        }

        next_frame().await
    }
}
