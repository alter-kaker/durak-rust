use crate::hand::Hand;

#[derive(Debug, Default)]
pub struct Player {
    pub name: String,
    pub hand: Hand,
    pub human: bool,
}

impl Player {
    pub fn new(name: String, human: bool) -> Self {
        Player {
            name,
            hand: Hand::new(),
            human,
        }
    }
}
