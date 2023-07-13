use std::hash::Hash;

use ggez::{
    glam::Vec2,
    graphics::{Canvas, DrawParam, Image, Rect, Transform},
};
use indexmap::{set::Iter, IndexSet};

use crate::sprite::Sprite;

pub const CARD_WIDTH: f32 = 71.;
pub const CARD_HEIGHT: f32 = 96.;

#[derive(Debug)]
pub struct Card {
    suit: Suit,
    rank: Rank,
    front: Sprite,
    back: Sprite,
    deck_id: usize,
    position: Vec2,
    rotation: f32,
    show_front: bool,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank, front: Sprite, back: Sprite, deck_id: usize) -> Self {
        Card {
            suit,
            rank,
            front,
            back,
            deck_id,
            position: Vec2::ZERO,
            rotation: 0.,
            show_front: false,
        }
    }

    pub fn flip(&mut self) {
        self.show_front = !self.show_front;
    }

    pub fn suit(&self) -> Suit {
        self.suit
    }

    pub fn rank(&self) -> Rank {
        self.rank
    }

    pub fn draw(&self, canvas: &mut Canvas, param: impl Into<DrawParam>) {
        let mut card_dest = self.position;
        let mut card_rotation = self.rotation;
        if let Transform::Values { dest, rotation, .. } = Into::<DrawParam>::into(param).transform {
            card_dest += Into::<Vec2>::into(dest);
            card_rotation += rotation;
        }
        let card_param = DrawParam::new().dest(card_dest).rotation(card_rotation);
        if self.show_front {
            self.draw_front(canvas, card_param)
        } else {
            self.draw_back(canvas, card_param)
        }
    }

    fn draw_front(&self, canvas: &mut Canvas, param: DrawParam) {
        canvas.draw(&self.front, param)
    }

    fn draw_back(&self, canvas: &mut Canvas, param: DrawParam) {
        canvas.draw(&self.back, param)
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

    pub fn flip_index(&mut self, index: usize) {
        if let Some(mut card) = self.0.swap_remove_index(index) {
            card.flip();
            let (end_idx, _) = self.0.insert_full(card);
            self.0.swap_indices(index, end_idx);
        }
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
        let h = 1. / 5.;

        let deck_i = self.decks_made;

        let suit_range = 0..4;
        let rank_range = 0..9;
        let back = {
            let x = w * self.decks_made as f32;
            let y = h * 4.0;
            let src = Rect { x, y, w, h };
            Sprite {
                src,
                image: self.image.clone(),
            }
        };

        self.decks_made += 1;
        suit_range
            .flat_map({
                |i| {
                    let image = self.image.clone();
                    let back = back.clone();
                    rank_range.clone().map(move |j| {
                        let closure = |j, back| {
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
                            let front = Sprite {
                                src,
                                image: image.clone(),
                            };
                            Card::new(suit, rank, front, back, deck_i)
                        };
                        closure(j, back.clone())
                    })
                }
            })
            .collect()
    }
}
