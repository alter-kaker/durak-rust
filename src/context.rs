use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use crate::{game_error::GameError, timer::Timer};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

pub struct Context {
    pub timer: Timer,
    pub pixels: Pixels,
    pub window: Window,
    pub quit: bool,
}

impl Context {
    pub fn build() -> Result<(Self, EventLoop<()>), GameError> {
        let event_loop = EventLoop::new();
        let window = {
            let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);

            WindowBuilder::new()
                .with_title("Durak")
                .with_inner_size(size)
                .build(&event_loop)?
        };

        let pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);

            Pixels::new(WIDTH, HEIGHT, surface_texture)?
        };
        Ok((
            Context {
                timer: Timer::new(),
                pixels,
                window,
                quit: false,
            },
            event_loop,
        ))
    }

    pub fn screen_dimensions(&self) -> (u32, u32) {
        (WIDTH, HEIGHT)
    }
}
