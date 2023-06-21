use macroquad::{
    prelude::*,
};


#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
    pub texture: Texture2D,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank, image: &Image, rect: Rect) -> Self {
        let image_data = image.get_image_data();
        let mut texture_bytes = Vec::new();

        for j in rect.y as usize..(rect.y + rect.h) as usize {
            for i in rect.x as usize..(rect.x + rect.w) as usize {
                for byte in image_data[i + j * image.width()] {
                    texture_bytes.push(byte)
                }
            }
        }

        let texture = Texture2D::from_rgba8(rect.w as u16, rect.h as u16, &texture_bytes);

        Card {
            suit,
            rank,
            texture,
        }
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.suit == other.suit && self.rank == other.rank
    }
}

impl Eq for Card {}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.suit != other.suit {
            None
        } else {
            self.rank.partial_cmp(&other.rank)
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Rank {
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}
