#[derive(Debug, Copy, Clone)]
pub struct Card(u8);

impl Card {
  pub fn new(rank: u8, suit: u8) -> Self {
    Card((rank << 4) + suit)
  }

  pub fn rank(self) -> u8 { self.0 >> 4 }
  pub fn suit(self) -> u8 { self.0 & 0x0F }
  pub fn index(self) -> u8 { self.suit() * 13 + self.rank() }
}
