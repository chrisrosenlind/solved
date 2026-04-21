mod cards;

use cards::{Card, Rank, Suit};

fn main() {
    let card = Card::new(Rank::Ace, Suit::Hearts);
    println!("{:?}, {}, {}", card.rank(), card.suit(), card.index());
}
