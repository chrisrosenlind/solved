mod cards;

use cards::{Card, Deck, Hand, Rank, Suit};

fn main() {
    let deck = Deck::new();
    for card in deck.cards {
        println!("{:?}", card.to_string());
    }
}
