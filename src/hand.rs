use ggez::{
    glam::{vec2, Vec2},
    graphics::{DrawParam, Drawable, Transform},
};

use crate::card::Card;

#[derive(Debug, Default)]
pub struct Hand {
    pub cards: Vec<Card>,
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
            for (i, card) in self.cards.iter().enumerate() {
                let card_dest = dest + vec2(15. * i as f32, 0.);
                card.draw(canvas, param.dest(card_dest))
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
