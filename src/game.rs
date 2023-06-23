use winit::{
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
};

use crate::{context::Context, game_error::GameError};

pub struct Game {
    ctx: Context,
    frame: u32,
    fps: f32,
}

impl Game {
    pub fn new(ctx: Context) -> Result<Self, GameError> {
        let game = Game {
            ctx,
            frame: 0,
            fps: 0.,
        };

        Ok(game)
    }

    pub fn run(mut self, event_loop: EventLoop<()>) -> Result<(), GameError> {
        event_loop.run(
            move |event: Event<'_, ()>,
                  _: &EventLoopWindowTarget<()>,
                  control_flow: &mut ControlFlow| {
                if self.ctx.quit {
                    control_flow.set_exit();
                    return;
                }

                match event {
                    Event::RedrawRequested(_) => {
                        self.draw();
                        if let Err(err) = self.ctx.pixels.render() {
                            println!("pixels.render: {}", err);
                            control_flow.set_exit();
                        };
                    }
                    Event::MainEventsCleared => {
                        self.ctx.timer.tick();
                        self.update();
                        self.ctx.window.request_redraw();
                    }
                    Event::WindowEvent { event, .. } => match event {
                        WindowEvent::KeyboardInput { input, .. } => {
                            self.keyboard_input(input.virtual_keycode)
                        }
                        WindowEvent::CloseRequested => control_flow.set_exit(),
                        _ => (),
                    },
                    _ => (),
                }
            },
        );
    }

    fn draw(&mut self) {}
    fn update(&mut self) {
        self.frame += 1;
        self.fps = self.frame as f32 / self.ctx.timer.runtime().as_secs_f32();
        println!(
            "frame {} | dt {} | runtime {} | fps {}",
            self.frame,
            self.ctx.timer.dt().as_millis(),
            self.ctx.timer.runtime().as_millis(),
            self.fps
        )
    }
    fn keyboard_input(&mut self, key: Option<VirtualKeyCode>) {
        if let Some(VirtualKeyCode::Escape) = key {
            self.ctx.quit = true;
        }
    }
}
