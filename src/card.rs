use std::hash::Hash;

use ggez::{
    glam::{vec2, Vec2},
    graphics::{Canvas, Color, DrawMode, DrawParam, Image, Mesh, Rect},
    Context,
};

use crate::{cards::Cards, error::DurakError, sprite::Sprite};

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
    pub hovered: bool,
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
            hovered: false,
        }
    }

    pub fn intersect(&self, pos: Vec2) -> bool {
        let [a, b, c, _d] = self.corners();

        let ab = b - a;
        let am = pos - a;
        let bc = c - b;
        let bm = pos - b;

        ab.dot(ab) >= ab.dot(am) && ab.dot(am) >= 0. && bc.dot(bc) >= bc.dot(bm) && bc.dot(bm) >= 0.
    }

    pub fn flip(&mut self, show_face: bool) {
        self.show_front = show_face;
    }

    pub fn set_pos(&mut self, pos: Vec2) {
        self.position = pos;
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    pub fn suit(&self) -> Suit {
        self.suit
    }

    pub fn rank(&self) -> Rank {
        self.rank
    }

    pub fn corners(&self) -> [Vec2; 4] {
        let rotation_vec = Vec2::from_angle(self.rotation);

        let a = self.position - rotation_vec.rotate(vec2(CARD_WIDTH / 2., 0.));
        let b = a - rotation_vec.rotate(vec2(0., CARD_HEIGHT));
        let c = b + rotation_vec.rotate(vec2(CARD_WIDTH, 0.));
        let d = self.position + rotation_vec.rotate(vec2(CARD_WIDTH / 2., 0.));

        [a, b, c, d]
    }

    pub fn draw(&self, canvas: &mut Canvas, ctx: &mut Context) -> Result<(), DurakError> {
        let card_param = DrawParam::new()
            .dest(self.position)
            .rotation(self.rotation)
            .offset(vec2(0.5, 1.));
        if self.show_front {
            canvas.draw(&self.front, card_param)
        } else {
            canvas.draw(&self.back, card_param)
        }

        if self.hovered {
            let corners = self.corners();
            let outline = Mesh::new_polygon(
                ctx,
                DrawMode::stroke(2.),
                &corners,
                Color {
                    r: 1.,
                    g: 1.,
                    b: 0.,
                    a: 0.5,
                },
            )?;
            canvas.draw(&outline, DrawParam::new());
        }
        Ok(())
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
