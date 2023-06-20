use crate::card::Card;

pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Self {
        Hand {
            cards: Vec::new()
        }
    }
}