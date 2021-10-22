use crate::common::deck::{Color, Rank, Suit};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Serialize, Deserialize)]
pub struct Card(Rank, Suit);

impl From<(Rank, Suit)> for Card {
    fn from((rank, suit): (Rank, Suit)) -> Self {
        Card::new(rank, suit)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} of {:?}", self.rank(), self.suit())
    }
}

impl Card {
    pub const fn new(rank: Rank, suit: Suit) -> Self {
        Self(rank, suit)
    }

    pub fn color(&self) -> Color {
        self.1.color()
    }
    pub fn suit(&self) -> Suit {
        self.1
    }

    pub fn rank(&self) -> Rank {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::deck::Rank::*;
    use crate::common::deck::Suit::*;

    #[test]
    fn test_display() {
        let test_cases = [
            (Card::new(Ace, Spades), "Ace of Spades"),
            (Card::new(King, Hearts), "King of Hearts"),
            (Card::new(Ten, Clubs), "Ten of Clubs"),
            (Card::new(Two, Diamonds), "Two of Diamonds"),
        ];

        for (card, expected) in test_cases.iter() {
            let displayed = format!("{}", card);
            assert_eq!(displayed, *expected);
        }
    }
}
