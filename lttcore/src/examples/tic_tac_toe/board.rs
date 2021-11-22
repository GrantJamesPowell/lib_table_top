use super::Marker;
use crate::common::cartesian::{BoundedCol, BoundedPosition, BoundedRow};
use crate::Player;
use itertools::iproduct;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::ops::{Index, IndexMut};

pub type Position = BoundedPosition<2, 2>;
pub type Row = BoundedRow<2>;
pub type Col = BoundedCol<2>;

const ROW_0: Row = unsafe { Row::new_unchecked(0) };
const ROW_1: Row = unsafe { Row::new_unchecked(1) };
const ROW_2: Row = unsafe { Row::new_unchecked(2) };

const ROWS: [Row; 3] = [ROW_0, ROW_1, ROW_2];

const COL_0: Col = unsafe { Col::new_unchecked(0) };
const COL_1: Col = unsafe { Col::new_unchecked(1) };
const COL_2: Col = unsafe { Col::new_unchecked(2) };

const COLS: [Col; 3] = [COL_0, COL_1, COL_2];

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
    pub fn rows(&self) -> impl Iterator<Item = [(Position, Option<Marker>); 3]> + '_ {
        ROWS.into_iter()
            .map(|row| COLS.map(|col| ((col, row), self[(col, row)].clone())))
    }

    pub fn cols(&self) -> impl Iterator<Item = [(Position, Option<Marker>); 3]> + '_ {
        COLS.into_iter()
            .map(|col| ROWS.map(|row| ((col, row), self[(col, row)].clone())))
    }

    pub fn diagonals(&self) -> impl Iterator<Item = [(Position, Option<Marker>); 3]> + '_ {
        [
            [(COL_0, ROW_0), (COL_1, ROW_1), (COL_2, ROW_2)],
            [(COL_0, ROW_2), (COL_1, ROW_1), (COL_2, ROW_0)],
        ]
        .into_iter()
        .map(|group| group.map(|pos| (pos, self[pos].clone())))
    }

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
