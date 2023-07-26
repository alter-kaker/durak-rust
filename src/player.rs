use crate::{card::Card, hand::Hand};

#[derive(Debug, Default)]
pub struct Player {
    pub name: String,
    pub hand: Hand,
    pub human: bool,
}

impl Player {
    pub fn new(name: String, human: bool) -> Self {
        Player {
            name,
            hand: Hand::new(),
            human,
        }
    }

    pub fn push_card(&mut self, mut card: Card) {
        if self.human {
            card.flip(true)
        } else {
            card.flip(false)
        }
        self.hand.push(card);
    }
}
