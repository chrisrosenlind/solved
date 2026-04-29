#[derive(Debug)]
#[repr(u8)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Debug)]
#[repr(u8)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
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
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Self(((rank as u8) << 4) + (suit as u8))
    }

    pub fn rank(&self) -> Rank {
        Rank::from(self.0 >> 4)
    }

    pub fn suit(&self) -> Suit {
        Suit::from(self.0 & 0x0F)
    }

    // pub fn index(&self) -> u8 {
    //     (self.0 >> 4) + ((self.0 & 0x0F) * 13)
    // }
}

impl ToString for Card {
    fn to_string(&self) -> String {
        format!("{:?} of {:?}", self.rank(), self.suit())
    }
}
