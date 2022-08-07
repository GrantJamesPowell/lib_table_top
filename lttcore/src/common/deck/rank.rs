use serde_repr::{Deserialize_repr, Serialize_repr};
use std::cmp::Ordering::{self, *};

/// The pips of a standard deck. Important note that the Ace is represented by 1 and not 14
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Rank {
    Ace = 1,
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
}

#[allow(clippy::enum_glob_use)]
use Rank::*;

impl Rank {
    /// Compares using Ace High
    /// ```
    /// use lttcore::common::deck::Rank::*;
    /// use std::cmp::Ordering::{*, self};
    ///
    /// assert_eq!(Ace.cmp_with_ace_high(&King), Greater);
    /// assert_eq!(Ace.cmp_with_ace_high(&Two), Greater);
    /// assert_eq!(Seven.cmp_with_ace_high(&Four), Greater);
    /// assert_eq!(Ace.cmp_with_ace_high(&Ace), Equal);
    /// assert_eq!(King.cmp_with_ace_high(&Ace), Less);
    /// ```
    pub fn cmp_with_ace_high(&self, other: &Self) -> Ordering {
        match (self, other) {
            (x, y) if x == y => Equal,
            (Ace, _) => Greater,
            (_, Ace) => Less,
            (x, y) => x.cmp_with_ace_low(y),
        }
    }

    /// Compares using Ace Low
    /// ```
    /// use lttcore::common::deck::Rank::*;
    /// use std::cmp::Ordering::{*, self};
    ///
    /// assert_eq!(Ace.cmp_with_ace_low(&King), Less);
    /// assert_eq!(Ace.cmp_with_ace_low(&Two), Less);
    /// assert_eq!(Seven.cmp_with_ace_low(&Four), Greater);
    /// assert_eq!(Ace.cmp_with_ace_low(&Ace), Equal);
    /// assert_eq!(King.cmp_with_ace_low(&Ace), Greater);
    /// ```
    pub fn cmp_with_ace_low(&self, other: &Self) -> Ordering {
        (*self as u8).cmp(&(*other as u8))
    }

    /// Returns the next rank, with Ace being high
    /// ```
    /// use lttcore::common::deck::Rank::*;
    ///
    /// assert_eq!(Ace.next_with_ace_high(), None);
    /// assert_eq!(King.next_with_ace_high(), Some(Ace));
    /// ```
    pub fn next_with_ace_high(&self) -> Option<Self> {
        match self {
            Ace => None,
            _ => Some(self.next_with_wrapping()),
        }
    }

    /// Returns the next rank, with Ace being low
    /// ```
    /// use lttcore::common::deck::Rank::*;
    ///
    /// assert_eq!(King.next_with_ace_low(), None);
    /// assert_eq!(Ace.next_with_ace_low(), Some(Two));
    /// ```
    pub fn next_with_ace_low(&self) -> Option<Self> {
        match self {
            King => None,
            _ => Some(self.next_with_wrapping()),
        }
    }

    /// Returns the previous rank, with Ace being high
    /// ```
    /// use lttcore::common::deck::Rank::*;
    ///
    /// assert_eq!(Two.previous_with_ace_high(), None);
    /// assert_eq!(Ace.previous_with_ace_high(), Some(King));
    /// ```
    pub fn previous_with_ace_high(&self) -> Option<Self> {
        match self {
            Two => None,
            _ => Some(self.previous_with_wrapping()),
        }
    }

    /// Returns the previous rank, with Ace being high
    /// ```
    /// use lttcore::common::deck::Rank::*;
    ///
    /// assert_eq!(Two.previous_with_ace_low(), Some(Ace));
    /// assert_eq!(Ace.previous_with_ace_low(), None);
    /// ```
    pub fn previous_with_ace_low(&self) -> Option<Self> {
        match self {
            Ace => None,
            _ => Some(self.previous_with_wrapping()),
        }
    }

    /// Provides the next highest rank, wraps from King => Ace => Two
    /// ```
    /// use lttcore::common::deck::Rank::*;
    ///
    /// assert_eq!(King.next_with_wrapping(), Ace);
    /// assert_eq!(Ace.next_with_wrapping(), Two);
    /// assert_eq!(Two.next_with_wrapping(), Three);
    /// // etc ..
    /// ```
    pub fn next_with_wrapping(&self) -> Self {
        match self {
            Ace => Two,
            Two => Three,
            Three => Four,
            Four => Five,
            Five => Six,
            Six => Seven,
            Seven => Eight,
            Eight => Nine,
            Nine => Ten,
            Ten => Jack,
            Jack => Queen,
            Queen => King,
            King => Ace,
        }
    }

    /// Provides the next lowest rank, wraps from Two => Ace => King
    /// ```
    /// use lttcore::common::deck::Rank::*;
    ///
    /// assert_eq!(Two.previous_with_wrapping(), Ace);
    /// assert_eq!(Ace.previous_with_wrapping(), King);
    /// assert_eq!(King.previous_with_wrapping(), Queen);
    /// // etc ..
    /// ```
    pub fn previous_with_wrapping(&self) -> Self {
        match self {
            Ace => King,
            King => Queen,
            Queen => Jack,
            Jack => Ten,
            Ten => Nine,
            Nine => Eight,
            Eight => Seven,
            Seven => Six,
            Six => Five,
            Five => Four,
            Four => Three,
            Three => Two,
            Two => Ace,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_with_ace_high() {
        let test_cases = [
            (Ace, None),
            (King, Some(Ace)),
            (Queen, Some(King)),
            (Jack, Some(Queen)),
            (Ten, Some(Jack)),
            (Nine, Some(Ten)),
            (Eight, Some(Nine)),
            (Seven, Some(Eight)),
            (Six, Some(Seven)),
            (Five, Some(Six)),
            (Four, Some(Five)),
            (Three, Some(Four)),
            (Two, Some(Three)),
        ];

        for (test, expected) in test_cases {
            assert_eq!(test.next_with_ace_high(), expected);
        }
    }

    #[test]
    fn test_next_with_ace_low() {
        let test_cases = [
            (King, None),
            (Queen, Some(King)),
            (Jack, Some(Queen)),
            (Ten, Some(Jack)),
            (Nine, Some(Ten)),
            (Eight, Some(Nine)),
            (Seven, Some(Eight)),
            (Six, Some(Seven)),
            (Five, Some(Six)),
            (Four, Some(Five)),
            (Three, Some(Four)),
            (Two, Some(Three)),
            (Ace, Some(Two)),
        ];

        for (test, expected) in test_cases {
            assert_eq!(test.next_with_ace_low(), expected);
        }
    }

    #[test]
    fn test_previous_with_ace_high() {
        let test_cases = [
            (Ace, Some(King)),
            (King, Some(Queen)),
            (Queen, Some(Jack)),
            (Jack, Some(Ten)),
            (Ten, Some(Nine)),
            (Nine, Some(Eight)),
            (Eight, Some(Seven)),
            (Seven, Some(Six)),
            (Six, Some(Five)),
            (Five, Some(Four)),
            (Four, Some(Three)),
            (Three, Some(Two)),
            (Two, None),
        ];

        for (test, expected) in test_cases {
            assert_eq!(test.previous_with_ace_high(), expected);
        }
    }

    #[test]
    fn test_previous_with_ace_low() {
        let test_cases = [
            (King, Some(Queen)),
            (Queen, Some(Jack)),
            (Jack, Some(Ten)),
            (Ten, Some(Nine)),
            (Nine, Some(Eight)),
            (Eight, Some(Seven)),
            (Seven, Some(Six)),
            (Six, Some(Five)),
            (Five, Some(Four)),
            (Four, Some(Three)),
            (Three, Some(Two)),
            (Two, Some(Ace)),
            (Ace, None),
        ];

        for (test, expected) in test_cases {
            assert_eq!(test.previous_with_ace_low(), expected);
        }
    }
}
