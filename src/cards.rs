use std::{
    ops::{Index, IndexMut},
    slice::{Iter, IterMut, SliceIndex},
    vec::IntoIter,
};

use crate::card::Card;

#[derive(Debug, Default)]
pub struct Cards(Vec<Card>);
impl Cards {
    pub fn new() -> Self {
        Cards(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, value: Card) {
        self.0.push(value)
    }

    pub fn pop(&mut self) -> Option<Card> {
        self.0.pop()
    }

    pub fn take(&mut self, value: &Card) -> Option<Card> {
        if let Some(i) =
            self.0.iter().enumerate().find_map(
                |(i, card)| {
                    if card == value {
                        Some(i)
                    } else {
                        None
                    }
                },
            )
        {
            Some(self.0.remove(i))
        } else {
            None
        }
    }

    pub fn get(&self, index: usize) -> Option<&Card> {
        self.0.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Card> {
        self.0.get_mut(index)
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        self.0.swap(a, b)
    }

    pub fn iter(&self) -> Iter<'_, Card> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Card> {
        self.0.iter_mut()
    }

    pub fn append(&mut self, cards: &mut Cards) {
        self.0.append(&mut cards.0)
    }
}

impl<Idx> Index<Idx> for Cards
where
    Idx: SliceIndex<[Card]>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.0[index]
    }
}

impl<Idx> IndexMut<Idx> for Cards
where
    Idx: SliceIndex<[Card]>,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl From<Vec<Card>> for Cards {
    fn from(value: Vec<Card>) -> Self {
        Cards(value)
    }
}

impl FromIterator<Card> for Cards {
    fn from_iter<T: IntoIterator<Item = Card>>(iter: T) -> Self {
        let mut cards = Cards::new();
        for card in iter {
            cards.insert(card);
        }

        cards
    }
}

impl IntoIterator for Cards {
    type Item = Card;

    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Cards {
    type Item = &'a Card;

    type IntoIter = Iter<'a, Card>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
