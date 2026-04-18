pub struct Card(u8);

#[allow(dead_code)]
pub enum Suit {
    Clubs = 0,
    Diamonds = 1,
    Hearts = 2,
    Spades = 3,
}
#[allow(dead_code)]
pub enum Rank {
    Two = 0,
    Three = 1,
    Four = 2,
    Five = 3,
    Six = 4,
    Seven = 5,
    Eight = 6,
    Nine = 7,
    Ten = 8,
    Jack = 9,
    Queen = 10,
    King = 11,
    Ace = 12,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Card(((rank as u8) << 4) + (suit as u8))
    }

    pub fn rank(&self) -> u8 {
        self.0 >> 4
    }

    pub fn suit(&self) -> u8 {
        self.0 & 0x0F
    }

    pub fn index(&self) -> u8 {
        self.rank() + (self.suit() * 13)
    }
}
