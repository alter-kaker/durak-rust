use ggez::{
    glam::{vec2, Vec2},
    graphics::{DrawParam, Drawable, Transform},
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
}

impl Drawable for Hand {
    fn draw(
        &self,
        canvas: &mut ggez::graphics::Canvas,
        param: impl Into<ggez::graphics::DrawParam>,
    ) {
        let param: DrawParam = param.into();
        if let Transform::Values { dest, .. } = param.transform {
            let dest: Vec2 = dest.into();
            let total = 1.;
            let step = total / (self.cards.len() - 1) as f32;
            let gap = step.sin() * CARD_HEIGHT;
            let shift = gap.max(12. - gap);
            for (i, card) in self.cards.iter().enumerate() {
                let i = i as f32;
                let card_dest = dest + vec2(shift * i, 0.);
                card.draw(
                    canvas,
                    param
                        .dest(card_dest + vec2(CARD_WIDTH / 2., CARD_HEIGHT))
                        .offset(vec2(0.5, 1.))
                        .rotation((i * step) - (total / 2.)),
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
