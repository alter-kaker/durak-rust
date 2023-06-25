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
        let frame = self.ctx.pixels.frame_mut();
        for pixel in frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[0x48, 0xb2, 0xe8, 0xff]);
        }

        for (card, (card_x, card_y)) in &self.cards {
            if let Some(card_sprite) = self.assets.sprites().get(card) {
                let card_width = card_sprite.w * 4;
                let pixels = card_sprite.pixels.clone();

                let mut p = 0;

                for y in 0..card_sprite.h {
                    let i = *card_x as usize * 4
                        + *card_y as usize * screen_w as usize * 4
                        + y * screen_w as usize * 4;

                    let zipped = frame[i..i + card_width]
                        .iter_mut()
                        .zip(&pixels[p..p + card_width]);

                    for (screen_p, &card_p) in zipped {
                        if card_p > 0 {
                            *screen_p = card_p
                        }
                    }

                    p += card_width;
                }

                // for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                //     let x = (i % screen_w as usize) as i16;
                //     let y = (i / screen_w as usize) as i16;

                //     let inside_the_box = x >= *card_x
                //         && x < card_x + card_sprite.w as i16
                //         && y >= *card_y
                //         && y < card_y + card_sprite.h as i16;

                //     let rgba = if inside_the_box {
                //         // [0x5e, 0x48, 0xe8, 0xff]
                //         p += 4;
                //         [
                //             card_sprite.pixels[p - 4],
                //             card_sprite.pixels[p - 3],
                //             card_sprite.pixels[p - 2],
                //             card_sprite.pixels[p - 1],
                //         ]
                //     } else if pixel == [0, 0, 0, 0] {
                //         [0x48, 0xb2, 0xe8, 0xff]
                //     } else {
                //         [pixel[0], pixel[1], pixel[2], pixel[3]]
                //     };

                //     pixel.copy_from_slice(&rgba);
                // }
            }
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

        if card_x <= 0 || card_x + 71 >= self.ctx.screen_dimensions().0 as i16 {
            self.card_velocity.0 *= -1.;
        }
        if card_y <= 0 || card_y + 96 >= self.ctx.screen_dimensions().1 as i16 {
            self.card_velocity.1 *= -1.;
        }

        card_x += (self.card_velocity.0) as i16;
        card_y += (self.card_velocity.1) as i16;

        card_x = card_x.clamp(0, (self.ctx.screen_dimensions().0 - 71) as i16);
        card_y = card_y.clamp(0, (self.ctx.screen_dimensions().1 - 96) as i16);

        self.cards.push((card, (card_x, card_y)))
    }
    fn keyboard_input(&mut self, key: Option<VirtualKeyCode>) {
        if let Some(VirtualKeyCode::Escape) = key {
            self.ctx.quit = true;
        }
    }
}
