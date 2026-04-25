use crate::cards::{Card, Rank, Suit};

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::with_capacity(52);
        for suit in 0..4u8 {
            for rank in 0..13u8 {
                cards.push(Card::new(Rank::from(rank), Suit::from(suit)))
            }
        }
        Deck { cards }
    }
}
