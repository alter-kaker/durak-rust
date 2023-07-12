use std::f32::consts::PI;

use ggez::{
    glam::{vec2, Vec2},
    graphics::{Canvas, DrawParam},
};

use crate::card::{Card, Cards, CARD_HEIGHT, CARD_WIDTH};

#[derive(Debug, Default)]
pub struct Hand {
    cards: Cards,
    pos: Option<Vec2>,
    rotation: Option<f32>,
}

impl Hand {
    pub fn new() -> Self {
        Hand {
            cards: Cards::new(),
            pos: None,
            rotation: None,
        }
    }

    pub fn set_pos(&mut self, pos: Vec2) {
        self.pos = Some(pos)
    }

    pub fn get_pos(&self) -> Option<Vec2> {
        self.pos
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = Some(rotation)
    }

    pub fn hover(&self, mouse_pos: Vec2) -> Option<&Card> {
        if let Some(pos) = self.pos {
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
        } else {
            None
        }
    }

    pub fn insert(&mut self, card: Card) -> bool {
        self.cards.insert(card)
    }

    pub fn take(&mut self, card: &Card) -> Option<Card> {
        self.cards.take(card)
    }

    pub fn empty(&mut self) {
        self.cards = Cards::new();
    }

    fn card_params(&self) -> Vec<(&Card, DrawParam)> {
        let pos = self.pos.unwrap_or_default();
        let rotation = self.rotation.unwrap_or_default();
        let param = DrawParam::new();

        let cards_len = self.cards.len() as f32;

        let total_angle = (8. * cards_len).min(90.).max(45.).to_radians();
        let step_angle = total_angle / cards_len;
        let radius = vec2(0., 0. - (180_f32.to_radians() * 7.) / (PI * step_angle));
        let rotation_vec = Vec2::from_angle(rotation);

        self.cards
            .iter()
            .enumerate()
            .map(|(i, card)| {
                let card_angle = (step_angle * i as f32) - (total_angle / 2.) + rotation;
                let card_angle_vec = Vec2::from_angle(card_angle);

                let card_dest = pos + card_angle_vec.rotate(radius) - rotation_vec.rotate(radius);
                let param = param
                    .offset(vec2(0.5, 1.))
                    .dest(card_dest)
                    .rotation(card_angle);
                (card, param)
            })
            .collect()
    }

    pub fn draw_back(&self, canvas: &mut Canvas) {
        for (card, param) in self.card_params() {
            card.draw_back(canvas, param)
        }
    }

    pub fn draw_front(&self, canvas: &mut Canvas) {
        for (card, param) in self.card_params() {
            card.draw_front(canvas, param)
        }
    }
}
