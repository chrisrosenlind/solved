mod cards;

use cards::Card;

fn main() {
    println!("Hello, world!");
    let card = Card::new(1, 1);
    println!("{}, {}, {}", card.rank(), card.suit(), card.index());
}
