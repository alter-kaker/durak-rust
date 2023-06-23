use std::rc::Rc;

#[derive(Clone)]
pub struct Sprite {
    pub w: usize,
    pub h: usize,
    pub pixels: Rc<Vec<u8>>,
}
