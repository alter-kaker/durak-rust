use std::{collections::HashMap, fs::File, rc::Rc};

use png::Decoder;

use crate::{card::Card, game_error::GameError, sprite::Sprite};

const CARD_W: usize = 71;
const CARD_H: usize = 96;

pub struct Assets {
    card_sprites: HashMap<Card, Sprite>,
}

impl Assets {
    pub fn sprites(&self) -> &HashMap<Card, Sprite> {
        &self.card_sprites
    }
}

pub fn load_assets() -> Result<Assets, GameError> {
    let (bytes, width) = load_png("assets/cards.png")?;
    let card_sprites: HashMap<Card, Sprite> = read_cards(&bytes, width as usize)?;

    Ok(Assets {
        card_sprites,
    })
}

fn read_cards(bytes: &[u8], width: usize) -> Result<HashMap<Card, Sprite>, GameError> {
    let result = (0..9)
        .flat_map(|rank| {
            (0..4).map(move |suit| {
                let card_bytes: Vec<u8> = ((suit * CARD_H)..(suit * CARD_H) + CARD_H)
                    .flat_map(|y| {
                        ((rank * CARD_W)..(rank * CARD_W) + CARD_W).flat_map(move |x| {
                            let idx = (x + (y * width)) * 3;
                            let mut pixel = bytes[idx..idx + 3].to_vec();
                            pixel.push(0xff);
                            pixel
                        })
                    })
                    .collect();
                (
                    Card {
                        suit: suit.into(),
                        rank: rank.into(),
                    },
                    Sprite {
                        w: CARD_W,
                        h: CARD_H,
                        pixels: Rc::new(card_bytes),
                    },
                )
            })
        })
        .collect();

    Ok(result)
}

fn load_png(path: &str) -> Result<(Vec<u8>, u32), GameError> {
    let decoder = Decoder::new(File::open(path)?);
    let mut reader = decoder.read_info()?;
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf)?;
    let palette = reader.info().palette.as_ref().unwrap().as_ref();

    let width = info.width;
    let bytes: Vec<u8> = buf[..info.buffer_size()]
        .iter()
        .map(|i| *i as usize)
        .flat_map(|i| palette[i * 3..(i * 3) + 3].to_vec())
        .collect();

    Ok((bytes, width))
}
