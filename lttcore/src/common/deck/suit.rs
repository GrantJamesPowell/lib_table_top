/// The four suits of a standard deck
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

/// The two colors of a standard deck
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Color {
    Red,
    Black,
}

use Color::*;
use Suit::*;

impl Suit {
    /// Returns the color of a suit
    /// ```
    /// use lttcore::common::deck::{Suit::*, Color::*};
    ///
    /// assert_eq!(Spades.color(), Black);
    /// assert_eq!(Clubs.color(), Black);
    /// assert_eq!(Diamonds.color(), Red);
    /// assert_eq!(Hearts.color(), Red);
    /// ```
    pub fn color(&self) -> Color {
        match self {
            Clubs | Spades => Black,
            Hearts | Diamonds => Red,
        }
    }
}

impl Color {
    /// Returns the suits of a color
    /// ```
    /// use lttcore::common::deck::{Suit::*, Color::*};
    ///
    /// assert_eq!(Red.suits(), [Diamonds, Hearts]);
    /// assert_eq!(Black.suits(), [Clubs, Spades]);
    /// ```
    pub fn suits(&self) -> [Suit; 2] {
        match self {
            Red => [Diamonds, Hearts],
            Black => [Clubs, Spades],
        }
    }
}
