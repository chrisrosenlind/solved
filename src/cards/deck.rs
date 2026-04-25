use crate::cards::{Card, Rank, Suit};

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::with_capacity(52);
        for suit in 1..5u8 {
            for rank in 2..15u8 {
                cards.push(Card::new(Rank::from(rank), Suit::from(suit)))
            }
        }
        Deck { cards }
    }
}
