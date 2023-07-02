use ggez::{
    glam::{vec2, Vec2},
    graphics::{DrawParam, Drawable, Image, Rect, Transform},
};
use rand::{thread_rng, Rng};

use crate::{
    card::{Card, Rank, Suit, CARD_HEIGHT, CARD_WIDTH},
    error::DurakError,
    sprite::Sprite,
};

pub struct Deck {
    cards: Vec<Card>,
    kozyr: Suit,
}

impl Deck {
    pub fn new(image: &Image) -> Result<Self, DurakError> {
        let w = 1. / 9.;
        let h = 1. / 4.;

        let w = 1. / 9.;
        let h = 1. / 4.;
        let mut cards: Vec<Card> = (0..4)
            .into_iter()
            .flat_map({
                |i| {
                    (0..9).into_iter().map(move |j| {
                        let image = image.clone();
                        let closure = move |j| {
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
                            let src = Rect {
                                x,
                                y,
                                w,
                                h,
                            };
                            let sprite = Sprite { src, image };
                            Card::new(suit, rank, sprite)
                        };
                        closure(j)
                    })
                }
            })
            .collect();

        shuffle(&mut cards);

        Ok(Deck {
            kozyr: cards[0].suit(),
            cards,
        })
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

impl Drawable for Deck {
    fn draw(&self, canvas: &mut ggez::graphics::Canvas, param: impl Into<DrawParam>) {
        let param: DrawParam = param.into();
        if let Transform::Values { dest, .. } = param.transform {
            let dest: Vec2 = dest.into();
            for (i, card) in self.cards.iter().enumerate() {
                let card_dest = dest + vec2(15. * i as f32, 0.);
                card.draw(canvas, param.dest(card_dest))
            }
        }
    }

    fn dimensions(
        &self,
        gfx: &impl ggez::context::Has<ggez::graphics::GraphicsContext>,
    ) -> Option<Rect> {
        Some(Rect {
            x: 0.,
            y: 0.,
            w: (self.cards.len() as f32 * 15.) * CARD_WIDTH,
            h: CARD_HEIGHT,
        })
    }
}
