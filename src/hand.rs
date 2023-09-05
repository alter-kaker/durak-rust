use std::f32::consts::PI;

use ggez::{
    glam::{vec2, Vec2},
    graphics::{Canvas, Color, DrawMode, DrawParam, Mesh},
    Context,
};

use crate::{card::Card, cards::Cards, error::DurakError};

#[derive(Debug, Default)]
pub struct Hand {
    cards: Cards,
    pos: Vec2,
    rotation: f32,
    hovered: Option<usize>,
}

impl Hand {
    pub fn new() -> Self {
        Hand {
            cards: Cards::new(),
            pos: Vec2::ZERO,
            rotation: 0.,
            hovered: None,
        }
    }

    pub fn set_pos(&mut self, pos: Vec2) {
        self.pos = pos
    }

    pub fn get_pos(&self) -> Vec2 {
        self.pos
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation
    }

    pub fn take_hovered(&mut self) -> Option<Card> {
        if let Some(idx) = self.hovered {
            self.cards.take(idx)
        } else {
            None
        }
    }

    pub fn insert(&mut self, card: Card, idx: usize) {
        self.cards.insert(card, idx);
        self.set_card_params();
    }

    pub fn push(&mut self, card: Card) {
        self.cards.push(card);
        self.set_card_params();
    }

    pub fn put_back(&mut self, card: Card) {
        if let Some(idx) = self.hovered {
            self.cards.insert(card, idx)
        } else {
            self.cards.push(card)
        }
        self.set_card_params();
    }

    pub fn take(&mut self, idx: usize) -> Option<Card> {
        if let Some(card) = self.cards.take(idx) {
            self.set_card_params();
            Some(card)
        } else {
            None
        }
    }

    pub fn update_hover(&mut self, mouse_pos: Vec2) {
        self.hovered = self.cards.iter().enumerate().rev().find_map(|(i, card)| {
            if card.intersect(mouse_pos) {
                Some(i)
            } else {
                None
            }
        })
    }

    pub fn remove_hover(&mut self) {
        self.hovered = None;
        self.set_card_params();
    }

    pub fn empty(&mut self) {
        self.cards = Cards::new();
    }

    fn set_card_params(&mut self) {
        let pos = self.pos;
        let rotation = self.rotation;

        let cards_len = self.cards.len() as f32;

        let total_angle = (8. * cards_len).min(90.).max(45.).to_radians();
        let step_angle = total_angle / cards_len;
        let radius = vec2(0., 0. - (180_f32.to_radians() * 7.) / (PI * step_angle));
        let rotation_vec = Vec2::from_angle(rotation);

        for (card, pos, rotation) in self.cards.iter_mut().enumerate().map(|(i, card)| {
            let card_angle = (step_angle * i as f32) - (total_angle / 2.) + rotation;
            let card_angle_vec = Vec2::from_angle(card_angle);

            let card_dest = pos + card_angle_vec.rotate(radius) - rotation_vec.rotate(radius);
            (card, card_dest, card_angle)
        }) {
            card.set_pos(pos);
            card.set_rotation(rotation);
        }
    }

    pub fn draw(&self, canvas: &mut Canvas, ctx: &mut Context) -> Result<(), DurakError> {
        for (i, card) in self.cards.iter().enumerate() {
            card.draw(canvas)?;

            if let Some(idx) = self.hovered {
                if i == idx {
                    let outline = Mesh::new_polygon(
                        ctx,
                        DrawMode::stroke(2.),
                        &card.corners(),
                        Color {
                            r: 1.,
                            g: 1.,
                            b: 0.,
                            a: 0.5,
                        },
                    )?;
                    canvas.draw(&outline, DrawParam::new());
                }
            }
        }

        Ok(())
    }
}
