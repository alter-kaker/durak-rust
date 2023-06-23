#[derive(PartialEq, Eq, Debug, Hash)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.suit != other.suit {
            None
        } else {
            self.rank.partial_cmp(&other.rank)
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

impl From<usize> for Suit {
    fn from(value: usize) -> Self {
        match value {
            0 => Suit::Hearts,
            1 => Suit::Spades,
            2 => Suit::Diamonds,
            _ => Suit::Clubs,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Rank {
    Six = 0,
    Seven = 1,
    Eight = 2,
    Nine = 3,
    Ten = 4,
    Jack = 5,
    Queen = 6,
    King = 7,
    Ace = 8,
}

impl From<usize> for Rank {
    fn from(value: usize) -> Self {
        match value {
            0 => Rank::Six,
            1 => Rank::Seven,
            2 => Rank::Eight,
            3 => Rank::Nine,
            4 => Rank::Ten,
            5 => Rank::Jack,
            6 => Rank::Queen,
            7 => Rank::King,
            _ => Rank::Ace,
        }
    }
}
