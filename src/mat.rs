use ggez::{glam::vec2, graphics::Canvas, Context};

use crate::{
    card::{Card, CARD_HEIGHT, CARD_WIDTH},
    cards::Cards,
    error::DurakError,
};

pub struct Stack(Card, Option<Card>);

impl From<Stack> for Vec<Card> {
    fn from(value: Stack) -> Self {
        [Some(value.0), value.1].into_iter().flatten().collect()
    }
}

impl<'a> From<&'a Stack> for Vec<&'a Card> {
    fn from(value: &'a Stack) -> Self {
        [Some(&value.0), value.1.as_ref()]
            .into_iter()
            .flatten()
            .collect()
    }
}

pub struct Mat {
    in_play: Vec<Stack>,
    discard_pile: Vec<Card>,
}

impl Mat {
    pub fn attack(&mut self, card: Card) {
        self.in_play.push(Stack(card, None));
        self.set_card_params();
    }

    pub fn defend(&mut self, stack_idx: usize, card: Card) {
        self.in_play[stack_idx].1 = Some(card);
        self.set_card_params();
    }

    pub fn discard_cards(&mut self) {
        let mut cards = self.drain();
        self.discard_pile.append(&mut cards)
    }

    pub fn take_cards(&mut self) -> Cards {
        self.drain().into()
    }

    pub fn set_card_params(&mut self) {
        for (i, stack) in self.in_play.iter_mut().enumerate() {
            let x = (i % 2) as f32 * (CARD_WIDTH + 10.);
            let y = (i / 2) as f32 * (CARD_HEIGHT + 20.);
            stack.0.set_pos(vec2(x, y));
            if let Some(card) = stack.1.as_mut() {
                card.set_pos(vec2(x, y + 15.));
            }
        }
    }

    pub fn draw(&self, canvas: &mut Canvas, ctx: &mut Context) -> Result<(), DurakError> {
        for card in &self
            .in_play
            .iter()
            .flat_map(<&Stack as Into<Vec<&Card>>>::into)
            .collect::<Vec<&Card>>()
        {
            card.draw(canvas)?;
        }

        Ok(())
    }

    fn drain(&mut self) -> Vec<Card> {
        self.in_play
            .drain(..)
            .flat_map(<Stack as Into<Vec<Card>>>::into)
            .collect()
    }
}
