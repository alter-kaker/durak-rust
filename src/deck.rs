use ggez::{
    glam::{vec2, Vec2},
    graphics::{DrawParam, Drawable, Image, Rect, Transform},
};
use rand::{thread_rng, Rng};

use crate::{
    card::{Card, CardFactory, Cards, Suit, CARD_HEIGHT, CARD_WIDTH},
    error::DurakError,
};

pub struct Deck {
    cards: Cards,
    kozyr: Suit,
}

impl Deck {
    pub fn new(image: &Image) -> Result<Self, DurakError> {
        let mut cards = CardFactory::new(image.clone()).get_deck();

        let kozyr = cards.get_index(0).unwrap().suit();

        Ok(Deck { kozyr, cards })
    }

    pub fn cards(&self) -> &Cards {
        &self.cards
    }

    pub fn pop(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn shuffle(&mut self) {
        let len = self.cards.len();
        for i in 0..len {
            let r = i + thread_rng().gen_range(0..(len - i));
            self.cards.swap(i, r);
        }
    }
}
impl Drawable for Deck {
    fn draw(&self, canvas: &mut ggez::graphics::Canvas, param: impl Into<DrawParam>) {
        let param: DrawParam = param.into();
        if let Transform::Values { dest, .. } = param.transform {
            let dest: Vec2 = dest.into();
            let mut cards_iter = self.cards.iter();
            if let Some(first_card) = cards_iter.next() {
                first_card.draw(
                    canvas,
                    param
                        .dest(dest + vec2(CARD_HEIGHT, 0.))
                        .rotation(90.0_f32.to_radians())
                        // .offset(vec2(0.5, (CARD_WIDTH / 2.) / CARD_HEIGHT)),
                )
            };
            let dest = dest + vec2(CARD_HEIGHT - CARD_WIDTH, 0.);
            for (i, card) in cards_iter.enumerate() {
                let dest = dest + vec2(2. * i as f32, 0.);
                card.draw(canvas, param.dest(dest))
            }
        }
    }

    fn dimensions(
        &self,
        gfx: &impl ggez::context::Has<ggez::graphics::GraphicsContext>,
    ) -> Option<Rect> {
        Some(Rect {
            x: 0.,
            y: 0.,
            w: (self.cards.len() as f32 * 15.) * CARD_WIDTH,
            h: CARD_HEIGHT,
        })
    }
}
