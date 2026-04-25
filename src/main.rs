mod cards;

use cards::{Card, Hand, Rank, Suit};

fn main() {
    let hand = Hand::new(
        Card::new(Rank::Ace, Suit::Diamonds),
        Card::new(Rank::Ace, Suit::Hearts),
    );
    print!("{:?}", hand)
}
