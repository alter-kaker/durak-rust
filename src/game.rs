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
    assets: Assets,
    card: Option<Card>,
    card_loc: (i16, i16),
    card_velocity: (f32, f32),
}

impl Game {
    pub fn new(ctx: Context) -> Result<Self, GameError> {
        let assets = load_assets()?;
        let card = None;

        let game = Game {
            ctx,
            frame: 0,
            fps: 0.,
            assets,
            card,
            card_loc: (24, 16),
            card_velocity: (160., 160.),
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

    fn draw(&mut self) {
        let (screen_w, _screen_h) = self.ctx.screen_dimensions();
        if let Some(card) = self.card.as_ref() {
            if let Some(card_sprite) = self.assets.sprites().get(card) {
                let frame = self.ctx.pixels.frame_mut();
                let mut p = 0;
                let (card_x, card_y) = self.card_loc;
                for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                    let x = (i % screen_w as usize) as i16;
                    let y = (i / screen_w as usize) as i16;

                    let inside_the_box = x >= card_x
                        && x < card_x + card_sprite.w as i16
                        && y >= card_y
                        && y < card_y + card_sprite.h as i16;

                    let rgba = if inside_the_box {
                        // [0x5e, 0x48, 0xe8, 0xff]
                        p += 4;
                        [
                            card_sprite.pixels[p - 4],
                            card_sprite.pixels[p - 3],
                            card_sprite.pixels[p - 2],
                            card_sprite.pixels[p - 1],
                        ]
                    } else {
                        [0x48, 0xb2, 0xe8, 0xff]
                    };

                    pixel.copy_from_slice(&rgba);
                }
            }
        }
    }
    fn update(&mut self) {
        self.frame += 1;
        self.fps = self.frame as f32 / self.ctx.timer.runtime().as_secs_f32();
        println!(
            "frame {} | dt {} | runtime {} | fps {}",
            self.frame,
            self.ctx.timer.dt().as_millis(),
            self.ctx.timer.runtime().as_millis(),
            self.fps
        );

        let card_no = self.ctx.timer.runtime().as_secs() % (4 * 9);

        let rank: Rank = (card_no as usize % 9).into();
        let suit: Suit = (card_no as usize / 9).into();

        self.card = Some(Card { rank, suit });

        if self.card_loc.0 <= 0 || self.card_loc.0 + 71 >= self.ctx.screen_dimensions().0 as i16 {
            self.card_velocity.0 *= -1.;
        }
        if self.card_loc.1 <= 0 || self.card_loc.1 + 96 >= self.ctx.screen_dimensions().1 as i16 {
            self.card_velocity.1 *= -1.;
        }

        self.card_loc.0 += (self.card_velocity.0 * self.ctx.timer.dt().as_secs_f32()) as i16;
        self.card_loc.1 += (self.card_velocity.1 * self.ctx.timer.dt().as_secs_f32()) as i16;

        self.card_loc.0 = self
            .card_loc
            .0
            .clamp(0, (self.ctx.screen_dimensions().0 - 71) as i16);
        self.card_loc.1 = self
            .card_loc
            .1
            .clamp(0, (self.ctx.screen_dimensions().1 - 96) as i16);
    }
    fn keyboard_input(&mut self, key: Option<VirtualKeyCode>) {
        if let Some(VirtualKeyCode::Escape) = key {
            self.ctx.quit = true;
        }
    }
}
