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

        shuffle(&mut cards);

        let kozyr = cards.get_index(0).unwrap().suit();

        Ok(Deck {  kozyr, cards })
    }

    pub fn cards(&self) -> &Cards {
        &self.cards
    }

    pub fn pop(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

fn shuffle(cards: &mut Cards) {
    let len = cards.len();
    for i in 0..len {
        let r = i + thread_rng().gen_range(0..(len - i));
        cards.swap(i, r);
    }
}

impl Drawable for Deck {
    fn draw(&self, canvas: &mut ggez::graphics::Canvas, param: impl Into<DrawParam>) {
        let param: DrawParam = param.into();
        if let Transform::Values { dest, .. } = param.transform {
            let dest: Vec2 = dest.into();
            for (i, card) in self.cards.iter().enumerate() {
                let card_dest = dest + vec2(15. * i as f32, 0.);
                card.draw(canvas, param.dest(card_dest))
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
