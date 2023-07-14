use std::f32::consts::PI;

use ggez::{
    glam::{vec2, Vec2},
    graphics::Canvas,
};

use crate::{
    card::{Card, CARD_HEIGHT, CARD_WIDTH},
    cards::Cards,
};

#[derive(Debug, Default)]
pub struct Hand {
    cards: Cards,
    pos: Vec2,
    rotation: f32,
}

impl Hand {
    pub fn new() -> Self {
        Hand {
            cards: Cards::new(),
            pos: Vec2::ZERO,
            rotation: 0.,
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

    pub fn hover(&self, mouse_pos: Vec2) -> Option<&Card> {
        let pos = self.pos;
        let mut hovered_card = None;
        for (i, card) in self.cards.iter().enumerate().rev() {
            let card_x = pos.x + (i as f32 * 15.);
            let hovered = mouse_pos.x > card_x
                && mouse_pos.x < card_x + CARD_WIDTH
                && mouse_pos.y > pos.y
                && mouse_pos.y < pos.y + CARD_HEIGHT;
            if hovered {
                hovered_card = Some(card);
                break;
            }
        }
        hovered_card
    }

    pub fn insert(&mut self, card: Card) {
        self.cards.insert(card);
        self.set_card_params();
    }

    pub fn take(&mut self, card: &Card) -> Option<Card> {
        if let Some(card) = self.cards.take(card) {
            self.set_card_params();
            Some(card)
        } else {
            None
        }
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

    pub fn draw(&self, canvas: &mut Canvas) {
        for card in &self.cards {
            card.draw(canvas)
        }
    }
}
