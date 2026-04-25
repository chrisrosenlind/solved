#[allow(dead_code)]
#[derive(Debug)]
pub enum Suit {
    Clubs = 0,
    Diamonds = 1,
    Hearts = 2,
    Spades = 3,
}

#[allow(dead_code)]
#[derive(Debug)]
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

impl From<u8> for Rank {
    fn from(value: u8) -> Rank {
        match value {
            0 => Rank::Two,
            1 => Rank::Three,
            2 => Rank::Four,
            3 => Rank::Five,
            4 => Rank::Six,
            5 => Rank::Seven,
            6 => Rank::Eight,
            7 => Rank::Nine,
            8 => Rank::Ten,
            9 => Rank::Jack,
            10 => Rank::Queen,
            11 => Rank::King,
            12 => Rank::Ace,
            _ => panic!("Invalid rank"),
        }
    }
}

impl From<u8> for Suit {
    fn from(value: u8) -> Suit {
        match value {
            0 => Suit::Clubs,
            1 => Suit::Diamonds,
            2 => Suit::Hearts,
            3 => Suit::Spades,
            _ => panic!("Invalid suit"),
        }
    }
}

#[derive(Debug)]
pub struct Card(u8);

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Card {
        Card(((rank as u8) << 4) + (suit as u8))
    }

    pub fn rank(&self) -> Rank {
        Rank::from(self.0 >> 4)
    }

    pub fn suit(&self) -> Suit {
        Suit::from(self.0 & 0x0F)
    }

    pub fn index(&self) -> u8 {
        (self.0 >> 4) + ((self.0 & 0x0F) * 13)
    }
}
