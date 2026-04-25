#[allow(dead_code)]
#[derive(Debug)]
pub enum Suit {
    Clubs = 1,
    Diamonds = 2,
    Hearts = 3,
    Spades = 4,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Rank {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl From<u8> for Rank {
    fn from(value: u8) -> Rank {
        match value {
            2 => Rank::Two,
            3 => Rank::Three,
            4 => Rank::Four,
            5 => Rank::Five,
            6 => Rank::Six,
            7 => Rank::Seven,
            8 => Rank::Eight,
            9 => Rank::Nine,
            10 => Rank::Ten,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            14 => Rank::Ace,
            _ => panic!("Invalid rank"),
        }
    }
}

impl From<u8> for Suit {
    fn from(value: u8) -> Suit {
        match value {
            1 => Suit::Clubs,
            2 => Suit::Diamonds,
            3 => Suit::Hearts,
            4 => Suit::Spades,
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

impl ToString for Card {
    fn to_string(&self) -> String {
        format!("{:?} of {:?}", self.rank(), self.suit())
    }
}
