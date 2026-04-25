use crate::cards::Card;

#[derive(Debug)]
pub struct Hand {
    pub card1: Card,
    pub card2: Card,
}

impl Hand {
    pub fn new(card1: Card, card2: Card) -> Self {
        Hand { card1, card2 }
    }
}
