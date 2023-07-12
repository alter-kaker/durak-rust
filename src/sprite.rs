use ggez::graphics::{DrawParam, Drawable, Image, Rect};

#[derive(Debug, Clone)]
pub struct Sprite {
    pub src: Rect,
    pub image: Image,
}

impl Drawable for Sprite {
    fn draw(&self, canvas: &mut ggez::graphics::Canvas, param: impl Into<DrawParam>) {
        canvas.draw(&self.image, param.into().src(self.src))
    }

    fn dimensions(
        &self,
        _gfx: &impl ggez::context::Has<ggez::graphics::GraphicsContext>,
    ) -> Option<ggez::graphics::Rect> {
        todo!()
    }
}