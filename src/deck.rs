use rand::{thread_rng, Rng};

use crate::card::{Card, Rank, Suit};

pub struct Deck {
    cards: Vec<Card>,
    kozyr: Suit,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards: Vec<Card> = [Suit::Hearts, Suit::Spades, Suit::Diamonds, Suit::Clubs]
            .into_iter()
            .enumerate()
            .flat_map({
                |(i, suit)| {
                    (6..=14).into_iter().enumerate().map(move |(j, rank)| {
                        let suit = suit.clone();
                        let rank = match rank {
                            6 => Rank::Six,
                            7 => Rank::Seven,
                            8 => Rank::Eight,
                            9 => Rank::Nine,
                            10 => Rank::Ten,
                            11 => Rank::Jack,
                            12 => Rank::Queen,
                            13 => Rank::King,
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
