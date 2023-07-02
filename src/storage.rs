use std::sync::Mutex;

use ggez::{graphics::Image, Context};

use crate::error::DurakError;

static CARD_IMAGE: Mutex<Option<Image>> = Mutex::new(None);

pub fn load_card_image(ctx: &Context) -> Result<(), DurakError> {
    let mut image_option = CARD_IMAGE.lock().map_err(|_e| "Lock Error")?;
    if image_option.is_none() {
        *image_option = Some(Image::from_path(ctx, "/cards.png")?);
    }
    Ok(())
}

pub fn card_image() -> Result<Option<Image>, DurakError> {
    Ok(CARD_IMAGE
        .lock()
        .map_err(|_e| "Resource Lock Error")?
        .as_ref()
        .cloned())
}
