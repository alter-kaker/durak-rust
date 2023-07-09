use std::hash::Hash;

use ggez::{
    glam::vec2,
    graphics::{DrawParam, Drawable, Image, Rect},
};
use indexmap::{set::Iter, IndexSet};

use crate::sprite::Sprite;

pub const CARD_WIDTH: f32 = 71.;
pub const CARD_HEIGHT: f32 = 96.;

#[derive(Debug)]
pub struct Card {
    suit: Suit,
    rank: Rank,
    sprite: Sprite,
    deck_id: usize,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank, sprite: Sprite, deck_id: usize) -> Self {
        Card {
            suit,
            rank,
            sprite,
            deck_id,
        }
    }

    pub fn suit(&self) -> Suit {
        self.suit
    }

    pub fn rank(&self) -> Rank {
        self.rank
    }
}

impl Hash for Card {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.suit.hash(state);
        self.rank.hash(state);
        self.deck_id.hash(state);
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

impl Drawable for Card {
    fn draw(
        &self,
        canvas: &mut ggez::graphics::Canvas,
        param: impl Into<ggez::graphics::DrawParam>,
    ) {
        canvas.draw(&self.sprite, param)
    }

    fn dimensions(
        &self,
        gfx: &impl ggez::context::Has<ggez::graphics::GraphicsContext>,
    ) -> Option<ggez::graphics::Rect> {
        todo!()
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
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

#[derive(Debug, Default)]
pub struct Cards(IndexSet<Card>);
impl Cards {
    pub fn new() -> Self {
        Cards(IndexSet::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, value: Card) -> bool {
        self.0.insert(value)
    }

    pub fn pop(&mut self) -> Option<Card> {
        self.0.pop()
    }

    pub fn take(&mut self, value: &Card) -> Option<Card> {
        self.0.shift_take(value)
    }

    pub fn get_index(&mut self, index: usize) -> Option<&Card> {
        self.0.get_index(index)
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        self.0.swap_indices(a, b)
    }

    pub fn iter(&self) -> Iter<'_, Card> {
        self.0.iter()
    }
}

impl From<IndexSet<Card>> for Cards {
    fn from(value: IndexSet<Card>) -> Self {
        Cards(value)
    }
}

impl FromIterator<Card> for Cards {
    fn from_iter<T: IntoIterator<Item = Card>>(iter: T) -> Self {
        let mut cards = Cards::new();
        for card in iter {
            cards.insert(card);
        }

        cards
    }
}

impl IntoIterator for Cards {
    type Item = Card;

    type IntoIter = indexmap::set::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Debug)]
pub struct CardFactory {
    decks_made: usize,
    image: Image,
}

impl CardFactory {
    pub fn new(image: Image) -> Self {
        CardFactory {
            decks_made: 0,
            image,
        }
    }

    pub fn get_deck(&mut self) -> Cards {
        let w = 1. / 9.;
        let h = 1. / 4.;

        let deck_i = self.decks_made;
        self.decks_made += 1;

        let suit_range = 0..4;
        let rank_range = 0..9;

        suit_range
            .flat_map({
                |i| {
                    let image = self.image.clone();
                    rank_range.clone().map(move |j| {
                        let closure = |j| {
                            let suit = match i {
                                0 => Suit::Hearts,
                                1 => Suit::Spades,
                                2 => Suit::Diamonds,
                                _ => Suit::Clubs,
                            };
                            let rank = match j {
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
                            let x = w * j as f32;
                            let y = h * i as f32;
                            let src = Rect { x, y, w, h };
                            let sprite = Sprite {
                                src,
                                image: image.clone(),
                            };
                            Card::new(suit, rank, sprite, deck_i)
                        };
                        closure(j)
                    })
                }
            })
            .collect()
    }
}
