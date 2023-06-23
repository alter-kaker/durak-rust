use std::time::{Duration, Instant};

use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

fn main() -> Result<(), Error> {
    let mut last_instant = Instant::now();
    let mut runtime = Duration::ZERO;
    let mut frame = 0;
    let event_loop = EventLoop::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Durak")
            .with_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);

        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(_) => {
            draw(pixels.frame_mut());
            if let Err(err) = pixels.render() {
                println!("pixels.render: {}", err);
                control_flow.set_exit();
            };
        }

        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => control_flow.set_exit(),

        Event::WindowEvent {
            event: WindowEvent::KeyboardInput { input, .. },
            ..
        } => {
            if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
                control_flow.set_exit()
            }
        }

        Event::MainEventsCleared => {
            let current_instant = Instant::now();
            runtime += current_instant - last_instant;
            last_instant = current_instant;
            frame +=1;
            
            println!("f= {} t = {}ms", frame, runtime.as_millis());
            
            window.request_redraw()
        }
        _ => (),
    });
}

fn draw(frame: &mut [u8]) {
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        pixel.copy_from_slice(&[0x5e, 0x48, 0xe8, 0xff]);
    }
}
