use crate::cards::Card;

#[derive(Debug)]
pub struct Hand([Card; 2]);

impl Hand {
    pub fn new(card1: Card, card2: Card) -> Self {
        Hand([card1, card2])
    }

    pub fn cards(&self) -> &[Card; 2] {
        &self.0
    }
}
