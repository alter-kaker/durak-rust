use macroquad::{prelude::*, ui::Ui};

use crate::card::{Card, Rank, Suit};

#[derive(Debug)]
pub struct CardsTexture(pub Image);

pub struct Deck {
    cards: Vec<Card>,
    kozyr: Suit,
}

impl Deck {
    pub fn new(cards_image: &Image) -> Self {
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
                        let w = 71;
                        let h = 96;
                        let x = j * w;
                        let y = i * h;
                        let rect = Rect::new(x as f32, y as f32, w as f32, h as f32);
                        Card::new(suit, rank, cards_image, rect)
                    })
                }
            })
            .collect();

        shuffle(&mut cards);

        Deck {
            kozyr: cards[0].suit.clone(),
            cards,
        }
    }

    pub fn pop(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn draw(&self, ui: &mut Ui) {
        for (i, card) in self.cards.iter().enumerate() {
            if ui.texture(card.texture, card.texture.width(), card.texture.height()) {
                println!("card clicked");
            }
        }
    }
}

fn shuffle(cards: &mut Vec<Card>) {
    let len = cards.len();
    for i in 0..len {
        let r = i + rand::gen_range(0, len - i);
        cards.swap(i, r);
    }
}
