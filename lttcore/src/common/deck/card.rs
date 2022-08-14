use crate::common::deck::{Color, Rank, Suit};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Helper function to have [`Card`] literals
///
/// ```
/// use lttcore::c;
/// use lttcore::common::deck::{Card, Rank::*, Suit::*};
///
/// assert_eq!(c!(a, h), Card::from((Ace, Hearts)));
/// assert_eq!(c!(a, s), Card::from((Ace, Spades)));
/// assert_eq!(c!(a, c), Card::from((Ace, Clubs)));
/// assert_eq!(c!(a, d), Card::from((Ace, Diamonds)));
///
/// assert_eq!(c!(k, h), Card::from((King, Hearts)));
/// assert_eq!(c!(k, s), Card::from((King, Spades)));
/// assert_eq!(c!(k, c), Card::from((King, Clubs)));
/// assert_eq!(c!(k, d), Card::from((King, Diamonds)));
///
/// assert_eq!(c!(q, h), Card::from((Queen, Hearts)));
/// assert_eq!(c!(q, s), Card::from((Queen, Spades)));
/// assert_eq!(c!(q, c), Card::from((Queen, Clubs)));
/// assert_eq!(c!(q, d), Card::from((Queen, Diamonds)));
///
/// assert_eq!(c!(j, h), Card::from((Jack, Hearts)));
/// assert_eq!(c!(j, s), Card::from((Jack, Spades)));
/// assert_eq!(c!(j, c), Card::from((Jack, Clubs)));
/// assert_eq!(c!(j, d), Card::from((Jack, Diamonds)));
///
/// assert_eq!(c!(10, h), Card::from((Ten, Hearts)));
/// assert_eq!(c!(10, s), Card::from((Ten, Spades)));
/// assert_eq!(c!(10, c), Card::from((Ten, Clubs)));
/// assert_eq!(c!(10, d), Card::from((Ten, Diamonds)));
///
/// assert_eq!(c!(9, h), Card::from((Nine, Hearts)));
/// assert_eq!(c!(9, s), Card::from((Nine, Spades)));
/// assert_eq!(c!(9, c), Card::from((Nine, Clubs)));
/// assert_eq!(c!(9, d), Card::from((Nine, Diamonds)));
///
/// // ...
///
/// assert_eq!(c!(2, h), Card::from((Two, Hearts)));
/// assert_eq!(c!(2, s), Card::from((Two, Spades)));
/// assert_eq!(c!(2, c), Card::from((Two, Clubs)));
/// assert_eq!(c!(2, d), Card::from((Two, Diamonds)));
/// ```
#[macro_export]
macro_rules! c {
    ($rank:tt, $suit:tt) => {
       $crate::common::deck::Card::from((c!(@$rank), c!(@$suit)))
    };

    // Ranks

    (@a) => { $crate::common::deck::Rank::Ace };
    (@k) => { $crate::common::deck::Rank::King };
    (@q) => { $crate::common::deck::Rank::Queen };
    (@j) => { $crate::common::deck::Rank::Jack };

    (@10) => { $crate::common::deck::Rank::Ten };
    (@9) => { $crate::common::deck::Rank::Nine };
    (@8) => { $crate::common::deck::Rank::Eight };
    (@7) => { $crate::common::deck::Rank::Seven };
    (@6) => { $crate::common::deck::Rank::Six };
    (@5) => { $crate::common::deck::Rank::Five };
    (@4) => { $crate::common::deck::Rank::Four };
    (@3) => { $crate::common::deck::Rank::Three };
    (@2) => { $crate::common::deck::Rank::Two };

    // Suits

    (@h) => { $crate::common::deck::Suit::Hearts };
    (@c) => { $crate::common::deck::Suit::Clubs };
    (@s) => { $crate::common::deck::Suit::Spades };
    (@d) => { $crate::common::deck::Suit::Diamonds };

    ($_:tt) => {
        compile_error!("Card literals take c!(rank, suit), i.e c!(10, s) or c!(k, h)")
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    #[allow(clippy::enum_glob_use)]
    use crate::common::deck::Rank::*;
    use crate::common::deck::Suit::{Clubs, Diamonds, Hearts, Spades};

    #[test]
    fn test_display() {
        let test_cases = [
            (Card::new(Ace, Spades), "Ace of Spades"),
            (Card::new(King, Hearts), "King of Hearts"),
            (Card::new(Ten, Clubs), "Ten of Clubs"),
            (Card::new(Two, Diamonds), "Two of Diamonds"),
        ];

        for (card, expected) in &test_cases {
            let displayed = format!("{}", card);
            assert_eq!(displayed, *expected);
        }
    }
}
