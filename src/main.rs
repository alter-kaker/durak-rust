use macroquad::{prelude::*, window};

#[macroquad::main("Durak")]
async fn main() {
    let text: &str = "Hello World";
    let text_measurements = measure_text(text, None, 40, 1.);

    loop {
        clear_background(BLACK);

        draw_text(
            text,
            (window::screen_width() - text_measurements.width) / 2.,
            10. + text_measurements.height,
            40.,
            WHITE,
        );

        next_frame().await
    }
}
