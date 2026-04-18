mod cards;

use cards::Card;

fn main() {
    let card = Card::new(1, 1);
    println!("{}, {}, {}", card.rank(), card.suit(), card.index());
}
