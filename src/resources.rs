use macroquad::prelude::collections::storage;
use macroquad::prelude::*;
use macroquad::ui::{root_ui, Skin};

use crate::deck::CardsTexture;

#[derive(Debug, Clone)]
pub struct ResourceError(pub String);

impl From<&'static str> for ResourceError {
    fn from(s: &'static str) -> Self {
        Self(s.to_string())
    }
}

impl From<FontError> for ResourceError {
    fn from(value: FontError) -> Self {
        Self(format!("FontError: {}", value.0))
    }
}

impl From<FileError> for ResourceError {
    fn from(value: FileError) -> Self {
        Self(format!("FileError: {} {}", value.kind, value.path))
    }
}

pub async fn load_resources() -> Result<(), ResourceError> {
    let skin = load_skin();
    let cards_texture = load_cards_image();
    storage::store(skin.await?);
    storage::store(cards_texture.await?);
    Ok(())
}

async fn load_cards_image() -> Result<CardsTexture, ResourceError> {
    let cards_texture = load_image("resources/cards.png").await?;

    Ok(CardsTexture(cards_texture))
}

async fn load_skin() -> Result<Skin, ResourceError> {
    let font = load_file("./resources/MinimalPixel v2.ttf");
    let window_bg = load_file("./resources/window_background_2.png");
    let button_bg = load_file("./resources/button_background_2.png");
    let button_hovered_bg = load_file("./resources/button_hovered_background_2.png");
    let button_clicked_bg = load_file("./resources/button_clicked_background_2.png");
    let checkbox_bg = load_file("./resources/checkbox_background.png");
    let checkbox_hovered_bg = load_file("./resources/checkbox_hovered_background.png");
    let checkbox_clicked_bg = load_file("./resources/checkbox_clicked_background.png");
    let editbox_bg = load_file("./resources/editbox_background.png");

    let font = font.await?;

    let label_style = root_ui()
        .style_builder()
        .font(&font)?
        .text_color(Color::from_rgba(120, 120, 120, 255))
        .font_size(25)
        .build();

    let window_style = root_ui()
        .style_builder()
        .background(Image::from_file_with_format(&window_bg.await?, None))
        .background_margin(RectOffset::new(52.0, 52.0, 52.0, 52.0))
        .margin(RectOffset::new(-30.0, 0.0, -30.0, 0.0))
        .build();

    let button_style = root_ui()
        .style_builder()
        .background(Image::from_file_with_format(&button_bg.await?, None))
        .background_margin(RectOffset::new(8.0, 8.0, 8.0, 8.0))
        .background_hovered(Image::from_file_with_format(
            &button_hovered_bg.await?,
            None,
        ))
        .background_clicked(Image::from_file_with_format(
            &button_clicked_bg.await?,
            None,
        ))
        .font(&font)?
        .text_color(Color::from_rgba(180, 180, 100, 255))
        .font_size(40)
        .build();

    let checkbox_style = root_ui()
        .style_builder()
        .background(Image::from_file_with_format(&checkbox_bg.await?, None))
        .background_hovered(Image::from_file_with_format(
            &checkbox_hovered_bg.await?,
            None,
        ))
        .background_clicked(Image::from_file_with_format(
            &checkbox_clicked_bg.await?,
            None,
        ))
        .build();

    let editbox_style = root_ui()
        .style_builder()
        .background(Image::from_file_with_format(&editbox_bg.await?, None))
        .background_margin(RectOffset::new(2., 2., 2., 2.))
        .font(&font)?
        .text_color(Color::from_rgba(120, 120, 120, 255))
        .font_size(25)
        .build();

    Ok(Skin {
        window_style,
        button_style,
        label_style,
        checkbox_style,
        editbox_style,
        ..root_ui().default_skin()
    })
}
