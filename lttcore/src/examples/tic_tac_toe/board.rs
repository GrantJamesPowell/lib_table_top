use super::Marker;
use crate::Player;
use itertools::iproduct;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Status {
    /// There are still available positions to be claimed on the board
    InProgress { next_up: Marker },
    /// All positions have been claimed and there is no winner
    Draw,
    /// Win by resignation
    WinByResignation { winner: Marker },
    /// There *is* a winner via connecting three spaces
    Win {
        winner: Marker,
        positions: [Position; 3],
    },
}

impl Status {
    pub fn winner(&self) -> Option<Marker> {
        match self {
            Status::Win { winner, .. } | Status::WinByResignation { winner, .. } => Some(*winner),
            _ => None,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Board([[Option<Marker>; 3]; 3]);

impl From<[[Option<Marker>; 3]; 3]> for Board {
    fn from(markers: [[Option<Marker>; 3]; 3]) -> Self {
        Self(markers)
    }
}

impl Board {
    pub fn rows(&self) -> impl Iterator<Item = [Option<Marker>; 3]> + '_ {
        ROWS.into_iter()
            .map(|row| COLS.map(|col| (self[(col, row)].clone())))
    }

    pub fn cols(&self) -> impl Iterator<Item = [Option<Marker>; 3]> + '_ {
        COLS.into_iter()
            .map(|col| ROWS.map(|row| (self[(col, row)].clone())))
    }

    pub fn diagonals(&self) -> impl Iterator<Item = [Option<Marker>; 3]> + '_ {
        [
            [(COL_0, ROW_0), (COL_1, ROW_1), (COL_2, ROW_2)],
            [(COL_0, ROW_2), (COL_1, ROW_1), (COL_2, ROW_0)],
        ]
        .into_iter()
        .map(|group| group.map(|pos| self[pos].clone()))
    }

    /// Iterate over the spaces on the board and the marker in the space (if there is one)
    ///
    /// ```
    /// use lttcore::ttt;
    /// use lttcore::examples::tic_tac_toe::{Row, Col, Marker::*, Position};
    ///
    /// let board = ttt!([
    ///   X O X
    ///   - - -
    ///   - X O
    /// ]);
    /// assert_eq!(
    ///   board.spaces().collect::<Vec<_>>(),
    ///   vec![
    ///     ((Col::new(0), Row::new(0)), None),
    ///     ((Col::new(0), Row::new(1)), None),
    ///     ((Col::new(0), Row::new(2)), Some(X)),
    ///     ((Col::new(1), Row::new(0)), Some(X)),
    ///     ((Col::new(1), Row::new(1)), None),
    ///     ((Col::new(1), Row::new(2)), Some(O)),
    ///     ((Col::new(2), Row::new(0)), Some(O)),
    ///     ((Col::new(2), Row::new(1)), None),
    ///     ((Col::new(2), Row::new(2)), Some(X))
    ///   ]
    /// );
    /// ```
    pub fn spaces(&self) -> impl Iterator<Item = (Position, Option<Marker>)> + '_ {
        iproduct!(COLS, ROWS).map(|position: Position| (position, self[position].clone()))
    }

    /// Iterate over the spaces on the board that are taken
    ///
    /// ```
    /// use lttcore::ttt;
    /// use lttcore::examples::tic_tac_toe::{Row, Col, Marker::*};
    ///
    /// let board = ttt!([
    ///   X O X
    ///   - - -
    ///   - X O
    /// ]);
    /// assert_eq!(
    ///   board.taken_spaces().collect::<Vec<_>>(),
    ///   vec![
    ///     ((Col::new(0), Row::new(2)), X),
    ///     ((Col::new(1), Row::new(0)), X),
    ///     ((Col::new(1), Row::new(2)), O),
    ///     ((Col::new(2), Row::new(0)), O),
    ///     ((Col::new(2), Row::new(2)), X)
    ///   ]
    /// );
    pub fn taken_spaces(&self) -> impl Iterator<Item = (Position, Marker)> + '_ {
        self.spaces()
            .flat_map(|(position, maybe_marker)| maybe_marker.map(|marker| (position, marker)))
    }

    /// Iterator over the empty spaces on the board
    ///
    /// ```
    /// use lttcore::ttt;
    /// use lttcore::examples::tic_tac_toe::{Board, Row, Col, Marker::*, Position};
    ///
    /// let board: Board = Default::default();
    /// assert_eq!(board.empty_spaces().count(), 9);
    ///
    /// let board = ttt!([
    ///   X X X
    ///   X X X
    ///   X X X
    /// ]);
    /// assert_eq!(board.empty_spaces().count(), 0);
    ///
    /// let board = ttt!([
    ///   X O X
    ///   - - -
    ///   - X O
    /// ]);
    /// assert_eq!(board.empty_spaces().count(), 4);
    /// assert_eq!(
    ///   board.empty_spaces().collect::<Vec<_>>(),
    ///   vec![
    ///    (Col::new(0), Row::new(0)),
    ///    (Col::new(0), Row::new(1)),
    ///    (Col::new(1), Row::new(1)),
    ///    (Col::new(2), Row::new(1))
    ///   ]
    /// );
    /// ```
    pub fn empty_spaces(&self) -> impl Iterator<Item = Position> + '_ {
        self.spaces().filter_map(|(pos, player)| match player {
            None => Some(pos),
            Some(_) => None,
        })
    }

    /// Returns a marker at a position, if the row or col is greater than 2, this returns None
    ///
    /// ```
    /// use lttcore::ttt;
    /// use lttcore::examples::tic_tac_toe::{Row, Col, Marker::*};
    ///
    /// let board = ttt!([
    ///   X - -
    ///   - - -
    ///   - X O
    /// ]);
    /// assert_eq!(board.at((2, 0)), Some(O));
    /// assert_eq!(board.at((0, 2)), Some(X));
    /// assert_eq!(board.at((1, 0)), Some(X));
    /// assert_eq!(board.at((0, 0)), None);
    ///
    /// // Out of bounds numbers return None
    /// assert_eq!(board.at((0, 1000)), None);
    /// assert_eq!(board.at((1000, 0)), None);
    /// ```
    pub fn at(&self, (c, r): (usize, usize)) -> Option<Marker> {
        let col = Col::try_new(c)?;
        let row = Row::try_new(r)?;

        self[(col, row)]
    }

    /// Return the marker who's turn it is
    ///
    /// ```
    /// use lttcore::ttt;
    /// use lttcore::examples::tic_tac_toe::{Board, Marker::*};
    ///
    /// // Starts with X
    /// let board: Board = Default::default();
    /// assert_eq!(board.whose_turn(), X);

    /// // Once the first player goes, it's the second player's turn
    /// let board = ttt!([
    ///   - - -
    ///   - - -
    ///   X - -
    /// ]);
    /// assert_eq!(board.whose_turn(), O);

    /// // Once O goes, it's X's turn again
    /// let board = ttt!([
    ///   - - -
    ///   - - -
    ///   X O -
    /// ]);
    /// assert_eq!(board.whose_turn(), X);

    /// // The next player to go is always the one with the fewest spaces
    /// let board = ttt!([
    ///   O O O
    ///   O O O
    ///   - O O
    /// ]);
    /// assert_eq!(board.whose_turn(), X);
    /// ```
    pub fn whose_turn(&self) -> Marker {
        let mut counts = [0, 0];

        for (_, marker) in self.taken_spaces() {
            counts[usize::from(Player::from(marker))] += 1;
        }

        let [xs, os] = counts;
        match xs.cmp(&os) {
            Ordering::Greater => Marker::O,
            Ordering::Equal => Marker::X,
            Ordering::Less => Marker::X,
        }
    }

    /// is the board full?
    ///
    /// ```
    /// use lttcore::ttt;
    ///
    /// let board = ttt!([
    ///   X X X
    ///   X - X
    ///   X X X
    /// ]);
    ///
    /// assert!(board.has_open_spaces());
    ///
    /// let board = ttt!([
    ///   X X X
    ///   X X X
    ///   X X X
    /// ]);
    ///
    /// assert!(!board.has_open_spaces());
    /// ```
    pub fn has_open_spaces(&self) -> bool {
        self.taken_spaces().count() < 9
    }

    /// Returns the status of the current game
    /// ```
    /// use lttcore::{Play, Player};
    /// use lttcore::ttt;
    /// use lttcore::examples::tic_tac_toe::{Board, Row, Col, Status::*, Marker::*};
    ///
    /// // In progress
    /// let board: Board = Default::default();
    /// assert_eq!(board.status(), InProgress{ next_up: X });
    ///
    /// // A draw
    /// let board = ttt!([
    ///   O X O
    ///   X X O
    ///   X O X
    /// ]);
    /// assert_eq!(board.status(), Draw);
    ///
    /// // With a winning position
    /// let board = ttt!([
    ///   - - -
    ///   - - -
    ///   X X X
    /// ]);
    ///
    /// assert_eq!(
    ///   board.status(),
    ///   Win {
    ///     winner: X,
    ///     positions: [
    ///       (Col::new(0), Row::new(0)),
    ///       (Col::new(1), Row::new(0)),
    ///       (Col::new(2), Row::new(0))
    ///     ]
    ///   }
    /// );
    /// ```
    pub fn status(&self) -> Status {
        POSSIBLE_WINS
            .iter()
            .find_map(|&positions| {
                let [a, b, c] = positions.map(|pos| self[pos]);

                if a == b && b == c {
                    a.map(|winner| Status::Win { winner, positions })
                } else {
                    None
                }
            })
            .unwrap_or_else(|| {
                if self.has_open_spaces() {
                    Status::InProgress {
                        next_up: self.whose_turn(),
                    }
                } else {
                    Status::Draw
                }
            })
    }
}

impl Index<Position> for Board {
    type Output = Option<Marker>;

    fn index(&self, (col, row): Position) -> &Self::Output {
        &self[row][usize::from(col)]
    }
}

impl IndexMut<Position> for Board {
    fn index_mut(&mut self, (col, row): Position) -> &mut Self::Output {
        &mut self[row][usize::from(col)]
    }
}

impl Index<Row> for Board {
    type Output = [Option<Marker>; 3];

    fn index(&self, row: Row) -> &Self::Output {
        &self.0[usize::from(row)]
    }
}

impl IndexMut<Row> for Board {
    fn index_mut(&mut self, row: Row) -> &mut Self::Output {
        &mut self.0[usize::from(row)]
    }
}

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
    /// use lttcore::examples::tic_tac_toe::BoardIndex;
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
    /// use lttcore::examples::tic_tac_toe::BoardIndex;
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

    /// Construct a new `BoardIndex` (see `BoardIndex::try_new` for a non panicking version)
    ///
    /// # Panics
    ///
    /// panics if n is outside of 0..=2
    ///
    /// ```should_panic
    /// use lttcore::examples::tic_tac_toe::BoardIndex;
    /// BoardIndex::new(1000);
    /// ```
    pub fn new(n: usize) -> Self {
        Self::try_new(n).expect("Invalid index, n must be within 0..=2")
    }

    /// Try to construct a `BoardIndex`, returning None if n is out of bounds
    ///
    /// ```
    /// use lttcore::examples::tic_tac_toe::BoardIndex;
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

pub const ROW_0: Row = Row(BI(0));
pub const ROW_1: Row = Row(BI(1));
pub const ROW_2: Row = Row(BI(2));

pub const ROWS: [Row; 3] = [ROW_0, ROW_1, ROW_2];
pub const COLS: [Col; 3] = [COL_0, COL_1, COL_2];

pub const COL_0: Col = Col(BI(0));
pub const COL_1: Col = Col(BI(1));
pub const COL_2: Col = Col(BI(2));

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
