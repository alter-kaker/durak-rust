use rand::{thread_rng, Rng};

use crate::card::{Card, Rank, Suit};

pub struct Deck {
    cards: Vec<Card>,
    kozyr: Suit,
}

impl Default for Deck {
    fn default() -> Self {
        let mut cards: Vec<Card> = (0..4)
            .into_iter()
            .flat_map({
                |suit| {
                    (0..9).into_iter().map(move |rank| {
                        let suit = match suit {
                            0 => Suit::Hearts,
                            1 => Suit::Spades,
                            2 => Suit::Diamonds,
                            _ => Suit::Clubs,
                        };
                        let rank = match rank {
                            0 => Rank::Six,
                            1 => Rank::Seven,
                            2 => Rank::Eight,
                            3 => Rank::Nine,
                            4 => Rank::Ten,
                            5 => Rank::Jack,
                            6 => Rank::Queen,
                            7 => Rank::King,
                            _ => Rank::Ace,
                        };
                        Card::new(suit, rank)
                    })
                }
            })
            .collect();

        shuffle(&mut cards);

        Deck {
            kozyr: cards[0].suit(),
            cards,
        }
    }
}

impl Deck {
    pub fn new() -> Self {
        Deck::default()
    }

    pub fn cards(&self) -> &Vec<Card> {
        &self.cards
    }

    pub fn pop(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

fn shuffle(cards: &mut Vec<Card>) {
    let len = cards.len();
    for i in 0..len {
        let r = i + thread_rng().gen_range(0..(len - i));
        cards.swap(i, r);
    }
}
