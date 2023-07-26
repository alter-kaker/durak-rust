use ggez::{
    glam::vec2,
    graphics::{DrawParam, Image},
};
use rand::{thread_rng, Rng};

use crate::cards::Cards;
use crate::{
    card::{Card, CardFactory, Suit, CARD_HEIGHT, CARD_WIDTH},
    error::DurakError,
};

pub struct Deck {
    cards: Cards,
    kozyr: Suit,
}

impl Deck {
    pub fn new(image: &Image) -> Result<Self, DurakError> {
        let cards = CardFactory::new(image.clone()).get_deck();
        let kozyr = cards.get(0).unwrap().suit();

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
        self.cards[0].flip(true);

        let pos = vec2(CARD_HEIGHT, CARD_WIDTH / 2.);
        let rotation = 270.0_f32.to_radians();

        let mut cards_iter = self.cards.iter_mut();
        let first_card = cards_iter.next().unwrap();

        first_card.set_pos(pos);
        first_card.set_rotation(rotation);

        for (i, card) in cards_iter.enumerate() {
            card.set_pos(vec2((CARD_WIDTH * 7. / 8.) + (2. * i as f32), CARD_HEIGHT))
        }
    }
    pub fn draw(
        &self,
        canvas: &mut ggez::graphics::Canvas,
        _param: impl Into<DrawParam>,
    ) -> Result<(), DurakError> {
        for card in &self.cards {
            card.draw(canvas)?;
        }

        Ok(())
    }
}
