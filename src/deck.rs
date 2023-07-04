use ggez::{
    glam::{vec2, Vec2},
    graphics::{DrawParam, Drawable, Image, Rect, Transform},
};
use rand::{thread_rng, Rng};

use crate::{
    card::{Card, Suit, CARD_HEIGHT, CARD_WIDTH, Cards, CardFactory},
    error::DurakError,
};

pub struct Deck {
    pos: Vec2,
    cards: Cards,
    kozyr: Suit,
}

impl Deck {
    pub fn new(image: &Image, pos: Vec2) -> Result<Self, DurakError> {
        
        let mut cards = CardFactory::new(image.clone()).get_deck();

        shuffle(&mut cards);

        let kozyr = cards.get_index(0).unwrap().suit();

        Ok(Deck {
            pos,
            kozyr,
            cards,
        })
    }

    pub fn cards(&self) -> &Cards {
        &self.cards
    }

    pub fn pop(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn hover(&self, mouse_pos: Vec2) {
        let mut hovered_card = None;
        for (i, card) in self.cards.iter().enumerate().rev() {
            let card_x = self.pos.x + (i as f32 * 15.);
            let hovered = mouse_pos.x > card_x
                && mouse_pos.x < card_x + CARD_WIDTH
                && mouse_pos.y > self.pos.y
                && mouse_pos.y < self.pos.y + CARD_HEIGHT;
            if hovered {
                hovered_card = Some(card);
                break;
            }
        }
        if let Some(card) = hovered_card {
            println!("{:?} {:?}", card.suit(), card.rank())
        }
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
                let card_dest = dest + self.pos + vec2(15. * i as f32, 0.);
                card.draw(canvas, param.dest(card_dest))
            }
        }
    }

    fn dimensions(
        &self,
        gfx: &impl ggez::context::Has<ggez::graphics::GraphicsContext>,
    ) -> Option<Rect> {
        Some(Rect {
            x: self.pos.x,
            y: self.pos.y,
            w: (self.cards.len() as f32 * 15.) * CARD_WIDTH,
            h: CARD_HEIGHT,
        })
    }
}
