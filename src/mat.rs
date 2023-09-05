use ggez::{
    glam::{vec2, Vec2},
    graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect},
    Context,
};

use crate::{
    card::{Card, CARD_HEIGHT, CARD_WIDTH},
    cards::Cards,
    error::DurakError,
};

pub struct Stack(Card, Option<Card>);

impl From<Stack> for Vec<Card> {
    fn from(value: Stack) -> Self {
        [Some(value.0), value.1].into_iter().flatten().collect()
    }
}

impl<'a> From<&'a Stack> for Vec<&'a Card> {
    fn from(value: &'a Stack) -> Self {
        [Some(&value.0), value.1.as_ref()]
            .into_iter()
            .flatten()
            .collect()
    }
}

#[derive(Default)]
pub struct Mat {
    in_play: Vec<Stack>,
    rect: Rect,
    intersect: bool,
}

impl Mat {
    pub fn new(rect: Rect) -> Self {
        Mat {
            rect,
            ..Default::default()
        }
    }
    pub fn attack(&mut self, card: Card) {
        self.in_play.push(Stack(card, None));
        self.set_card_params();
    }

    pub fn defend(&mut self, stack_idx: usize, card: Card) {
        self.in_play[stack_idx].1 = Some(card);
        self.set_card_params();
    }

    pub fn take_cards(&mut self) -> Cards {
        self.drain().into()
    }

    pub fn update_intersect(&mut self, mouse_pos: Vec2) {
        self.intersect = self.rect.contains(mouse_pos);
    }

    pub fn intersect(&self) -> bool {
        self.intersect
    }

    pub fn set_card_params(&mut self) {
        for (i, stack) in self.in_play.iter_mut().enumerate() {
            let x = (i % 2) as f32 * (CARD_WIDTH + 10.) + self.rect.x;
            let y = ((i / 2) + 1) as f32 * (CARD_HEIGHT + 20.) + self.rect.y;
            stack.0.set_pos(vec2(x, y));
            stack.0.set_rotation(0.);
            if let Some(card) = stack.1.as_mut() {
                card.set_pos(vec2(x, y + 15.));
                card.set_rotation(0.)
            }
        }
    }

    pub fn draw(&self, canvas: &mut Canvas, ctx: &mut Context) -> Result<(), DurakError> {
        for card in &self
            .in_play
            .iter()
            .flat_map(<&Stack as Into<Vec<&Card>>>::into)
            .collect::<Vec<&Card>>()
        {
            card.draw(canvas)?;
        }
        if self.intersect {
            let outline = Mesh::new_rectangle(
                ctx,
                DrawMode::stroke(2.),
                self.rect,
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

    fn drain(&mut self) -> Vec<Card> {
        self.in_play
            .drain(..)
            .flat_map(<Stack as Into<Vec<Card>>>::into)
            .collect()
    }
}
