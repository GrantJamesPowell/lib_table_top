use serde::{Deserialize, Serialize};

macro_rules! board_index {
    ($id:ident) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct $id(BoardIndex);

        impl From<$id> for u8 {
            fn from(n: $id) -> Self {
                n.0 .0
            }
        }

        impl From<$id> for usize {
            fn from(n: $id) -> Self {
                n.0 .0 as usize
            }
        }

        impl $id {
            /// See `BoardIndex::next`
            pub fn next(&self) -> Self {
                Self(self.0.next())
            }

            /// See `BoardIndex::previous`
            pub fn previous(&self) -> Self {
                Self(self.0.previous())
            }

            /// See `BoardIndex::new`
            pub fn new(n: usize) -> Self {
                Self(BoardIndex::new(n))
            }

            /// See `BoardIndex::try_new`
            pub fn try_new(n: usize) -> Option<Self> {
                BoardIndex::try_new(n).map(Self)
            }
        }
    };
}

board_index!(Row);
board_index!(Col);

/// Limiter for the indexes (rows, cols) to [0, 1, 2]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BoardIndex(u8);

impl BoardIndex {
    /// Returns the next `BoardIndex`, wrapping back to 0 after 2
    ///
    /// ```
    /// use tic_tac_toe::BoardIndex;
    ///
    /// assert_eq!(BoardIndex::new(0).next(), BoardIndex::new(1));
    /// assert_eq!(BoardIndex::new(1).next(), BoardIndex::new(2));
    /// assert_eq!(BoardIndex::new(2).next(), BoardIndex::new(0));
    /// ```
    pub fn next(&self) -> Self {
        Self(match self.0 {
            0 => 1,
            1 => 2,
            2 => 0,
            _ => panic!("invalid index"),
        })
    }

    /// Returns the previous `BoardIndex`, wrapping back to 2 after 0
    ///
    /// ```
    /// use tic_tac_toe::BoardIndex;
    ///
    /// assert_eq!(BoardIndex::new(2).previous(), BoardIndex::new(1));
    /// assert_eq!(BoardIndex::new(1).previous(), BoardIndex::new(0));
    /// assert_eq!(BoardIndex::new(0).previous(), BoardIndex::new(2));
    /// ```
    pub fn previous(&self) -> Self {
        Self(match self.0 {
            2 => 1,
            1 => 0,
            0 => 2,
            _ => panic!("invalid index"),
        })
    }

    /// Construct a new BoardIndex (see `Row::try_new` for a non panicking version)
    ///
    /// # Panics
    ///
    /// panics if n is outside of 0..=2
    ///
    /// ```should_panic
    /// use tic_tac_toe::BoardIndex;
    /// BoardIndex::new(1000);
    /// ```
    pub fn new(n: usize) -> Self {
        Self::try_new(n).expect("Invalid index, n must be within 0..=2")
    }

    /// Try to construct a `BoardIndex`, returning None if n is out of bounds
    ///
    /// ```
    /// use tic_tac_toe::BoardIndex;
    /// assert!(BoardIndex::try_new(1).is_some());
    /// assert!(BoardIndex::try_new(1000).is_none());
    /// ```
    pub fn try_new(n: usize) -> Option<Self> {
        match n {
            0 | 1 | 2 => Some(Self(n.try_into().unwrap())),
            _ => None,
        }
    }
}

pub type Position = (Col, Row);
use BoardIndex as BI;

pub const POSSIBLE_WINS: [[(Col, Row); 3]; 8] = [
    // Fill up a row
    [
        (Col(BI(0)), Row(BI(0))),
        (Col(BI(0)), Row(BI(1))),
        (Col(BI(0)), Row(BI(2))),
    ],
    [
        (Col(BI(1)), Row(BI(0))),
        (Col(BI(1)), Row(BI(1))),
        (Col(BI(1)), Row(BI(2))),
    ],
    [
        (Col(BI(2)), Row(BI(0))),
        (Col(BI(2)), Row(BI(1))),
        (Col(BI(2)), Row(BI(2))),
    ],
    // Fill up a col
    [
        (Col(BI(0)), Row(BI(0))),
        (Col(BI(1)), Row(BI(0))),
        (Col(BI(2)), Row(BI(0))),
    ],
    [
        (Col(BI(0)), Row(BI(1))),
        (Col(BI(1)), Row(BI(1))),
        (Col(BI(2)), Row(BI(1))),
    ],
    [
        (Col(BI(0)), Row(BI(2))),
        (Col(BI(1)), Row(BI(2))),
        (Col(BI(2)), Row(BI(2))),
    ],
    // Diagonal
    [
        (Col(BI(0)), Row(BI(0))),
        (Col(BI(1)), Row(BI(1))),
        (Col(BI(2)), Row(BI(2))),
    ],
    [
        (Col(BI(2)), Row(BI(0))),
        (Col(BI(1)), Row(BI(1))),
        (Col(BI(0)), Row(BI(2))),
    ],
];
