use wgpu::{SurfaceError, TextureViewDescriptor};
use winit::{
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
};

use crate::{
    assets::{load_assets, Assets},
    card::{Card, Rank, Suit},
    context::Context,
    game_error::GameError,
};

pub struct Game {
    ctx: Context,
    frame: u32,
    fps: f32,
    card_timer: f32,
    card_no: usize,
    assets: Assets,
    cards: Vec<(Card, (i16, i16))>,
    card_velocity: (f32, f32),
}

impl Game {
    pub fn new(ctx: Context) -> Result<Self, GameError> {
        let assets = load_assets()?;
        let cards = Vec::new();

        let game = Game {
            ctx,
            frame: 0,
            fps: 0.,
            assets,
            cards,
            card_timer: 0.,
            card_no: 0,
            card_velocity: (5., 5.),
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
                    }
                    Event::MainEventsCleared => {
                        self.ctx.timer.tick();
                        self.update();
                        self.ctx.window().request_redraw();
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

    fn draw(&mut self) {
        match self.ctx.render() {
            Ok(_) => {},
            Err(SurfaceError::Lost) => self.ctx.reconfigure(),
            Err(SurfaceError::OutOfMemory) => self.ctx.quit = true,
            Err(e) => eprint!("{:?}", e),
        }
    }
    fn update(&mut self) {
        self.frame += 1;
        self.fps = self.frame as f32 / self.ctx.timer.runtime().as_secs_f32();
        self.card_timer += self.ctx.timer.dt().as_secs_f32();
        println!(
            "frame {} | dt {} | runtime {} | fps {}",
            self.frame,
            self.ctx.timer.dt().as_millis(),
            self.ctx.timer.runtime().as_millis(),
            self.fps
        );

        self.card_timer = 0.;
        self.card_no += 1;

        let rank: Rank = (self.card_no % 9).into();
        let suit: Suit = (self.card_no / 9).into();

        let (mut card_x, mut card_y) = *self.cards.last().map(|(_, loc)| loc).unwrap_or(&(1, 1));

        let card = Card { rank, suit };

        if card_x <= 0 || card_x + 71 >= self.ctx.size().width as i16 {
            self.card_velocity.0 *= -1.;
        }
        if card_y <= 0 || card_y + 96 >= self.ctx.size().height as i16 {
            self.card_velocity.1 *= -1.;
        }

        card_x += (self.card_velocity.0) as i16;
        card_y += (self.card_velocity.1) as i16;

        card_x = card_x.clamp(0, (self.ctx.size().width - 71) as i16);
        card_y = card_y.clamp(0, (self.ctx.size().height - 96) as i16);

        self.cards.push((card, (card_x, card_y)))
    }
    fn keyboard_input(&mut self, key: Option<VirtualKeyCode>) {
        if let Some(VirtualKeyCode::Escape) = key {
            self.ctx.quit = true;
        }
    }
}
