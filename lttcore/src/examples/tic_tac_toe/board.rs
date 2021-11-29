//! A tic-tac-toe board
//!
//! See [`Board`] for more details
//!
//! # Quick Examples
//!
//! ```
//! use lttcore::ttt;
//! use lttcore::examples::tic_tac_toe::{Position, Status, Marker::*};
//!
//! let board = ttt!([
//!   X O X
//!   - - -
//!   O X O
//! ]);
//!
//! assert_eq!(board.at((0, 0)), Ok(Some(O)));
//! assert_eq!(board[Position::new(0, 2)], Some(X));
//! assert_eq!(board.status(), Status::InProgress { next_up: X });
//! ```
//!
//! # Implementation Notes
//!
//! [`Board`] is built on the [bounded](crate::common::cartesian::bounded) primatives. Bounded
//! primatives are cool for the compile time bounds checking, but aren't super erognomic to use,
//! especially for beginners. When writing this I got the opportunity to learn about
//! `macro_rules!` and const generics, but its probably overkill for this use case.

use super::{ActionError, Marker};
use crate::common::cartesian::bounded::{BoundedPoint, BoundedX, BoundedY, BoundsError};
use crate::play::Player;
use itertools::iproduct;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::ops::{Index, IndexMut};

/// A space on the tic-tac-toe board. Built ontop of [bounded](crate::common::cartesian::bounded)
/// primatives which bound the points to be within (0-2, 0-2)
pub type Position = BoundedPoint<2, 2>;

/// A row on the tic-tac-toe board. Built ontop of [bounded](crate::common::cartesian::bounded)
/// primatives which bound the row to [0, 1, 2]
pub type Row = BoundedY<2>;

/// A col on the tic-tac-toe board. Built ontop of [bounded](crate::common::cartesian::bounded)
/// primatives which bound the row to [0, 1, 2]
pub type Col = BoundedX<2>;

#[allow(missing_docs)]
pub mod consts {
    //! Self explanatory constants around tic-tac-toe
    use super::{Col, Position, Row};

    pub const ROW_0: Row = unsafe { Row::new_unchecked(0) };
    pub const ROW_1: Row = unsafe { Row::new_unchecked(1) };
    pub const ROW_2: Row = unsafe { Row::new_unchecked(2) };
    pub const COL_0: Col = unsafe { Col::new_unchecked(0) };
    pub const COL_1: Col = unsafe { Col::new_unchecked(1) };
    pub const COL_2: Col = unsafe { Col::new_unchecked(2) };

    pub const CENTER: Position = unsafe { Position::new_unchecked(1, 1) };

    pub const BOTTOM_LEFT: Position = unsafe { Position::new_unchecked(0, 0) };
    pub const BOTTOM_CENTER: Position = unsafe { Position::new_unchecked(1, 0) };
    pub const BOTTOM_RIGHT: Position = unsafe { Position::new_unchecked(2, 0) };
    pub const MIDDLE_LEFT: Position = unsafe { Position::new_unchecked(0, 1) };
    pub const MIDDLE_CENTER: Position = unsafe { Position::new_unchecked(1, 1) };
    pub const MIDDLE_RIGHT: Position = unsafe { Position::new_unchecked(2, 1) };
    pub const TOP_LEFT: Position = unsafe { Position::new_unchecked(0, 2) };
    pub const TOP_CENTER: Position = unsafe { Position::new_unchecked(1, 2) };
    pub const TOP_RIGHT: Position = unsafe { Position::new_unchecked(2, 2) };

    pub const ROWS: [Row; 3] = [ROW_0, ROW_1, ROW_2];
    pub const COLS: [Col; 3] = [COL_0, COL_1, COL_2];
}

/// The possible statuses of the game
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Status {
    /// There are still available positions to be claimed on the board
    InProgress {
        /// The next marker to move
        next_up: Marker,
    },
    /// All positions have been claimed and there is no winner
    Draw,
    /// Win by resignation.
    ///
    /// # Note
    ///
    /// This can only be produced by the [`status`](super::TicTacToe::status) method on
    /// [`TicTacToe`](super::TicTacToe) and not the [`status`](Board::status) method on [`Board`]
    /// because only [`TicTacToe`](super::TicTacToe) knows about resignations
    WinByResignation {
        /// The marker of the player who has won
        winner: Marker,
    },
    /// There *is* a winner via connecting three spaces
    Win {
        /// The marker of the player who has won
        winner: Marker,
        /// The winning position
        /// Note: If there is more than one, only the first one is shown
        positions: [Position; 3],
    },
}

impl Status {
    /// Returns the winning marker, if there is one
    pub fn winner(&self) -> Option<Marker> {
        match self {
            Status::Win { winner, .. } | Status::WinByResignation { winner, .. } => Some(*winner),
            _ => None,
        }
    }
}

/// Representation of the tic-tac-toe board. Meant to be indexed via [`Position`]
///
/// # Implementation Notes
///
/// [`Board`] is stored in memory "upside down" to how it's traditionally rendered to humans or how
/// it's interperted by the `ttt!` macro. To display to humans, `(0, 0)` in the bottom left corner.
/// When storing in memory, `(0, 0)` represents the `0th` index of the `0th` array which
/// conceptually puts it in the top left corner.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Board([[Option<Marker>; 3]; 3]);

impl From<[[Option<Marker>; 3]; 3]> for Board {
    fn from(markers: [[Option<Marker>; 3]; 3]) -> Self {
        Self(markers)
    }
}

#[allow(clippy::wildcard_imports)]
use consts::*;

impl Board {
    /// An iterator over the rows of the [`Board`]
    pub fn rows(&self) -> impl Iterator<Item = [(Position, Option<Marker>); 3]> + '_ {
        ROWS.into_iter()
            .map(|row| COLS.map(|col| ((col, row).into(), self[(col, row)])))
    }

    /// An iterator over the cols of the [`Board`]
    pub fn cols(&self) -> impl Iterator<Item = [(Position, Option<Marker>); 3]> + '_ {
        COLS.into_iter()
            .map(|col| ROWS.map(|row| ((col, row).into(), self[(col, row)])))
    }

    /// An iterator over the diagonals of the [`Board`]
    pub fn diagonals(&self) -> impl Iterator<Item = [(Position, Option<Marker>); 3]> + '_ {
        [
            [(COL_0, ROW_0), (COL_1, ROW_1), (COL_2, ROW_2)],
            [(COL_0, ROW_2), (COL_1, ROW_1), (COL_2, ROW_0)],
        ]
        .into_iter()
        .map(|group| group.map(|pos| (pos.into(), self[pos])))
    }

    /// An iterator over all of the connection "triples" of the [`Board`]. This represents all
    /// possible positions that could contain a winning group of three [`Marker`]
    pub fn triples(&self) -> impl Iterator<Item = [(Position, Option<Marker>); 3]> + '_ {
        self.rows().chain(self.cols()).chain(self.diagonals())
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
    ///     (Position::new(0, 0), None),
    ///     (Position::new(0, 1), None),
    ///     (Position::new(0, 2), Some(X)),
    ///     (Position::new(1, 0), Some(X)),
    ///     (Position::new(1, 1), None),
    ///     (Position::new(1, 2), Some(O)),
    ///     (Position::new(2, 0), Some(O)),
    ///     (Position::new(2, 1), None),
    ///     (Position::new(2, 2), Some(X))
    ///   ]
    /// );
    /// ```
    pub fn spaces(&self) -> impl Iterator<Item = (Position, Option<Marker>)> + '_ {
        iproduct!(COLS, ROWS).map(|pos| (pos.into(), self[pos]))
    }

    /// Iterate over the spaces on the board that are taken
    ///
    /// ```
    /// use lttcore::ttt;
    /// use lttcore::examples::tic_tac_toe::{Position, Marker::*};
    ///
    /// let board = ttt!([
    ///   X O X
    ///   - - -
    ///   - X O
    /// ]);
    /// assert_eq!(
    ///   board.taken_spaces().collect::<Vec<_>>(),
    ///   vec![
    ///     (Position::new(0, 2), X),
    ///     (Position::new(1, 0), X),
    ///     (Position::new(1, 2), O),
    ///     (Position::new(2, 0), O),
    ///     (Position::new(2, 2), X)
    ///   ]
    /// );
    pub fn taken_spaces(&self) -> impl Iterator<Item = (Position, Marker)> + '_ {
        self.spaces()
            .filter_map(|(position, maybe_marker)| maybe_marker.map(|marker| (position, marker)))
    }

    /// Iterator over the empty spaces on the board
    ///
    /// ```
    /// use lttcore::ttt;
    /// use lttcore::examples::tic_tac_toe::{Board, Position, Marker::*};
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
    ///    Position::new(0, 0),
    ///    Position::new(0, 1),
    ///    Position::new(1, 1),
    ///    Position::new(2, 1)
    ///   ]
    /// );
    /// ```
    pub fn empty_spaces(&self) -> impl Iterator<Item = Position> + '_ {
        self.spaces().filter_map(|(pos, player)| match player {
            None => Some(pos),
            Some(_) => None,
        })
    }

    /// Whether the board is empty (all spaces are open)
    ///
    /// ```
    /// use lttcore::ttt;
    ///
    /// let board = ttt!([
    ///   - - -
    ///   - - -
    ///   - - -
    /// ]);
    /// assert!(board.is_empty());
    ///
    /// let board = ttt!([
    ///   - - -
    ///   - X -
    ///   - - -
    /// ]);
    ///
    /// assert!(!board.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.empty_spaces().count() == 9
    }

    /// Returns a marker at a position, if the row or col is greater than 2, this returns a bounds
    /// error
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
    /// assert_eq!(board.at((2, 0)), Ok(Some(O)));
    /// assert_eq!(board.at((0, 2)), Ok(Some(X)));
    /// assert_eq!(board.at((1, 0)), Ok(Some(X)));
    /// assert_eq!(board.at((0, 0)), Ok(None));
    ///
    /// // Out of bounds numbers return an error
    /// assert!(board.at((0, 1000)).is_err());
    /// assert!(board.at((1000, 0)).is_err());
    /// ```
    pub fn at(&self, (x, y): (usize, usize)) -> Result<Option<Marker>, BoundsError> {
        let col: Col = x.try_into()?;
        let row: Row = y.try_into()?;
        Ok(self[(col, row)])
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
            Ordering::Equal | Ordering::Less => Marker::X,
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
    /// use lttcore::ttt;
    /// use lttcore::play::{Play, Player};
    /// use lttcore::examples::tic_tac_toe::{Board, Position, Status::*, Marker::*};
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
    ///       Position::new(0, 0),
    ///       Position::new(1, 0),
    ///       Position::new(2, 0)
    ///     ]
    ///   }
    /// );
    /// ```
    pub fn status(&self) -> Status {
        self.triples()
            .find_map(|triple| {
                let [(pos1, a), (pos2, b), (pos3, c)] = triple;

                if a == b && b == c {
                    a.map(|winner| Status::Win {
                        winner,
                        positions: [pos1, pos2, pos3],
                    })
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

    /// Claims a space for a marker, returns an error if that space is taken
    ///
    /// ```
    /// use lttcore::examples::tic_tac_toe::{TicTacToe, Marker::*, Position, ActionError::*};
    ///
    /// let mut game: TicTacToe = Default::default();
    /// let pos = Position::new(0, 0);
    ///
    /// assert_eq!(game.board()[pos], None);
    /// assert!(game.claim_space(X, pos).is_ok());
    /// assert_eq!(game.board()[pos], Some(X));
    ///
    /// // Taking an already claimed space returns an error
    /// assert_eq!(game.claim_space(O, pos), Err(SpaceIsTaken { attempted: pos }));
    /// ```
    pub fn claim_space(
        &mut self,
        marker: Marker,
        position: impl Into<Position>,
    ) -> Result<(), ActionError> {
        let position = position.into();

        match self[position] {
            None => {
                self[position] = Some(marker);
                Ok(())
            }
            Some(_) => Err(ActionError::SpaceIsTaken {
                attempted: position,
            }),
        }
    }
}

impl<T: Into<Position>> Index<T> for Board {
    type Output = Option<Marker>;

    fn index(&self, pos: T) -> &Self::Output {
        let pos = pos.into();
        &self[pos.y()][usize::from(pos.x())]
    }
}

impl<T: Into<Position>> IndexMut<T> for Board {
    fn index_mut(&mut self, pos: T) -> &mut Self::Output {
        let pos = pos.into();
        &mut self[pos.y()][usize::from(pos.x())]
    }
}

impl Index<Row> for Board {
    type Output = [Option<Marker>; 3];

    fn index(&self, row: Row) -> &Self::Output {
        let idx = usize::from(row);
        &self.0[idx]
    }
}

impl IndexMut<Row> for Board {
    fn index_mut(&mut self, row: Row) -> &mut Self::Output {
        let idx = usize::from(row);
        &mut self.0[idx]
    }
}
