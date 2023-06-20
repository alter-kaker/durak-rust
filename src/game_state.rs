use crate::{deck::Deck, hand::Hand, player::Player};

pub struct GameState {
    pub times_played: u32,
    pub players: Vec<Player>,
    pub deck: Option<Deck>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            times_played: 0,
            players: vec![
                Player {
                    name: String::new(),
                    hand: Hand { cards: Vec::new() },
                    human: true,
                },
                Player {
                    name: String::from("Opponent"),
                    hand: Hand { cards: Vec::new() },
                    human: false,
                },
            ],
            deck: None,
        }
    }
}
