use macroquad::prelude::*;
use macroquad::ui::{root_ui, Skin};

pub fn load_skin() -> Skin {
    let label_style = root_ui()
            .style_builder()
            .font(include_bytes!("../resources/MinimalPixel v2.ttf"))
            .unwrap()
            .text_color(Color::from_rgba(120, 120, 120, 255))
            .font_size(25)
            .build();

        let window_style = root_ui()
            .style_builder()
            .background(Image::from_file_with_format(
                include_bytes!("../resources/window_background_2.png"),
                None,
            ))
            .background_margin(RectOffset::new(52.0, 52.0, 52.0, 52.0))
            .margin(RectOffset::new(-30.0, 0.0, -30.0, 0.0))
            .build();

        let button_style = root_ui()
            .style_builder()
            .background(Image::from_file_with_format(
                include_bytes!("../resources/button_background_2.png"),
                None,
            ))
            .background_margin(RectOffset::new(8.0, 8.0, 8.0, 8.0))
            .background_hovered(Image::from_file_with_format(
                include_bytes!("../resources/button_hovered_background_2.png"),
                None,
            ))
            .background_clicked(Image::from_file_with_format(
                include_bytes!("../resources/button_clicked_background_2.png"),
                None,
            ))
            .font(include_bytes!("../resources/MinimalPixel v2.ttf"))
            .unwrap()
            .text_color(Color::from_rgba(180, 180, 100, 255))
            .font_size(40)
            .build();

        let checkbox_style = root_ui()
            .style_builder()
            .background(Image::from_file_with_format(
                include_bytes!("../resources/checkbox_background.png"),
                None,
            ))
            .background_hovered(Image::from_file_with_format(
                include_bytes!("../resources/checkbox_hovered_background.png"),
                None,
            ))
            .background_clicked(Image::from_file_with_format(
                include_bytes!("../resources/checkbox_clicked_background.png"),
                None,
            ))
            .build();

        let editbox_style = root_ui()
            .style_builder()
            .background(Image::from_file_with_format(
                include_bytes!("../resources/editbox_background.png"),
                None,
            ))
            .background_margin(RectOffset::new(2., 2., 2., 2.))
            .font(include_bytes!("../resources/MinimalPixel v2.ttf"))
            .unwrap()
            .text_color(Color::from_rgba(120, 120, 120, 255))
            .font_size(25)
            .build();

        let combobox_style = root_ui()
            .style_builder()
            .background(Image::from_file_with_format(
                include_bytes!("../resources/combobox_background.png"),
                None,
            ))
            .background_margin(RectOffset::new(4., 25., 6., 6.))
            .font(include_bytes!("../resources/MinimalPixel v2.ttf"))
            .unwrap()
            .text_color(Color::from_rgba(120, 120, 120, 255))
            .color(Color::from_rgba(210, 210, 210, 255))
            .font_size(25)
            .build();

        Skin {
            window_style,
            button_style,
            label_style,
            checkbox_style,
            editbox_style,
            combobox_style,
            ..root_ui().default_skin()
        }
}
