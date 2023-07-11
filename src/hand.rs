use std::f32::consts::PI;

use ggez::{
    glam::{vec2, Vec2},
    graphics::{DrawParam, Drawable, Mesh, Transform},
};

use crate::card::{Card, Cards, CARD_HEIGHT, CARD_WIDTH};

#[derive(Debug, Default)]
pub struct Hand {
    cards: Cards,
    pos: Option<Vec2>,
}

impl Hand {
    pub fn new() -> Self {
        Hand {
            cards: Cards::new(),
            pos: None,
        }
    }

    pub fn set_pos(&mut self, pos: Vec2) {
        self.pos = Some(pos)
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
}

impl Drawable for Hand {
    fn draw(
        &self,
        canvas: &mut ggez::graphics::Canvas,
        param: impl Into<ggez::graphics::DrawParam>,
    ) {
        let param: DrawParam = param.into();
        if let Transform::Values { dest, rotation, .. } = param.transform {
            let dest: Vec2 = dest.into();

            let total_angle = (8. * self.cards.len() as f32)
                .min(90.)
                .max(45.)
                .to_radians();
            let step_angle = total_angle / self.cards.len() as f32;

            let radius_len = 0. - (180_f32.to_radians() * 7.) / (PI * step_angle);

            let radius = vec2(0., radius_len);

            let rotation_vec = Vec2::from_angle(rotation);

            for (i, card) in self.cards.iter().enumerate() {
                let card_angle = step_angle * i as f32 - (total_angle / 2.) + rotation;
                let card_angle_vec = Vec2::from_angle(card_angle);

                let card_dest = dest + card_angle_vec.rotate(radius)
                    - rotation_vec.rotate(radius);
                card.draw(
                    canvas,
                    param
                        .offset(vec2(0.5, 1.))
                        .dest(card_dest)
                        .rotation(card_angle),
                )
            }
        }
    }

    fn dimensions(
        &self,
        gfx: &impl ggez::context::Has<ggez::graphics::GraphicsContext>,
    ) -> Option<ggez::graphics::Rect> {
        todo!()
    }
}
