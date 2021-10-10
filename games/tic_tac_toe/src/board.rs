use crate::ActionError::{self, *};
use crate::Marker::{self, *};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Col(u8);

impl Default for Col {
    fn default() -> Self {
        Self(0)
    }
}

impl Into<u8> for Col {
    fn into(self) -> u8 {
        self.0
    }
}

impl Into<usize> for Col {
    fn into(self) -> usize {
        self.0 as usize
    }
}

impl Col {
    /// Returns the next column, wrapping back to 0 after 2
    ///
    /// ```
    /// use tic_tac_toe::Col;
    ///
    /// assert_eq!(Col::new(0).next(), Col::new(1));
    /// assert_eq!(Col::new(1).next(), Col::new(2));
    /// assert_eq!(Col::new(2).next(), Col::new(0));
    /// ```
    pub fn next(&self) -> Self {
        Self(match self.0 {
            0 => 1,
            1 => 2,
            2 => 0,
            _ => panic!("invalid index"),
        })
    }

    /// Returns the previous column, wrapping back to 2 after 0
    ///
    /// ```
    /// use tic_tac_toe::Col;
    ///
    /// assert_eq!(Col::new(2).previous(), Col::new(1));
    /// assert_eq!(Col::new(1).previous(), Col::new(0));
    /// assert_eq!(Col::new(0).previous(), Col::new(2));
    /// ```
    pub fn previous(&self) -> Self {
        Self(match self.0 {
            2 => 1,
            1 => 0,
            0 => 2,
            _ => panic!("invalid index"),
        })
    }

    /// Construct a new `Col` (see `Col::try_new` for a non panicking version)
    ///
    /// # Panics
    ///
    /// panics if n is outside of 0..=2
    ///
    /// ```should_panic
    /// use tic_tac_toe::Col;
    /// Col::new(1000);
    /// ```
    pub fn new(n: usize) -> Self {
        Self::try_new(n).expect("Invalid index, n must be within 0..=2")
    }

    /// Try to construct a `Col`, returning None if n is out of bounds
    ///
    /// ```
    /// use tic_tac_toe::Col;
    /// assert!(Col::try_new(1).is_some());
    /// assert!(Col::try_new(1000).is_none());
    /// ```
    pub fn try_new(n: usize) -> Option<Self> {
        match n {
            0 | 1 | 2 => Some(Self(n.try_into().unwrap())),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Row(u8);

impl Default for Row {
    fn default() -> Self {
        Self(0)
    }
}

impl Into<u8> for Row {
    fn into(self) -> u8 {
        self.0
    }
}

impl Into<usize> for Row {
    fn into(self) -> usize {
        self.0 as usize
    }
}

impl Row {
    /// Returns the next column, wrapping back to 0 after 2
    ///
    /// ```
    /// use tic_tac_toe::Row;
    ///
    /// assert_eq!(Row::new(0).next(), Row::new(1));
    /// assert_eq!(Row::new(1).next(), Row::new(2));
    /// assert_eq!(Row::new(2).next(), Row::new(0));
    /// ```
    pub fn next(&self) -> Self {
        Self(match self.0 {
            0 => 1,
            1 => 2,
            2 => 0,
            _ => panic!("invalid index"),
        })
    }

    /// Returns the previous column, wrapping back to 2 after 0
    ///
    /// ```
    /// use tic_tac_toe::Row;
    ///
    /// assert_eq!(Row::new(2).previous(), Row::new(1));
    /// assert_eq!(Row::new(1).previous(), Row::new(0));
    /// assert_eq!(Row::new(0).previous(), Row::new(2));
    /// ```
    pub fn previous(&self) -> Self {
        Self(match self.0 {
            2 => 1,
            1 => 0,
            0 => 2,
            _ => panic!("invalid index"),
        })
    }

    /// Construct a new Row (see `Row::try_new` for a non panicking version)
    ///
    /// # Panics
    ///
    /// panics if n is outside of 0..=2
    ///
    /// ```should_panic
    /// use tic_tac_toe::Row;
    /// Row::new(1000);
    /// ```
    pub fn new(n: usize) -> Self {
        Self::try_new(n).expect("Invalid index, n must be within 0..=2")
    }

    /// Try to construct a `Row`, returning None if n is out of bounds
    ///
    /// ```
    /// use tic_tac_toe::Row;
    /// assert!(Row::try_new(1).is_some());
    /// assert!(Row::try_new(1000).is_none());
    /// ```
    pub fn try_new(n: usize) -> Option<Self> {
        match n {
            0 | 1 | 2 => Some(Self(n.try_into().unwrap())),
            _ => None,
        }
    }
}

pub type Position = (Col, Row);

pub const POSSIBLE_WINS: [[(Col, Row); 3]; 8] = [
    // Fill up a row
    [(Col(0), Row(0)), (Col(0), Row(1)), (Col(0), Row(2))],
    [(Col(1), Row(0)), (Col(1), Row(1)), (Col(1), Row(2))],
    [(Col(2), Row(0)), (Col(2), Row(1)), (Col(2), Row(2))],
    // Fill up a col
    [(Col(0), Row(0)), (Col(1), Row(0)), (Col(2), Row(0))],
    [(Col(0), Row(1)), (Col(1), Row(1)), (Col(2), Row(1))],
    [(Col(0), Row(2)), (Col(1), Row(2)), (Col(2), Row(2))],
    // Diagonal
    [(Col(0), Row(0)), (Col(1), Row(1)), (Col(2), Row(2))],
    [(Col(2), Row(0)), (Col(1), Row(1)), (Col(0), Row(2))],
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Board(pub [[Option<Marker>; 3]; 3]);

impl Default for Board {
    fn default() -> Self {
        Self([[None; 3]; 3])
    }
}

impl Board {
    /// Claims a space for a marker, returns an error if that space is taken
    ///
    /// ```
    /// use tic_tac_toe::{Board, Marker::*, Col, Row, ActionError::*};
    ///
    /// let mut board: Board = Default::default();
    /// let pos = (Col::new(0), Row::new(0));
    ///
    /// assert_eq!(board.at_position(pos), None);
    /// assert!(board.claim_space(X, pos).is_ok());
    /// assert_eq!(board.at_position(pos), Some(X));
    ///
    /// // Taking an already claimed space returns an error
    /// assert_eq!(board.claim_space(O, pos), Err(SpaceIsTaken { attempted: pos }));
    /// ```
    pub fn claim_space(&mut self, marker: Marker, position: Position) -> Result<(), ActionError> {
        if self.at_position(position).is_some() {
            return Err(SpaceIsTaken {
                attempted: position,
            });
        }

        let (Col(c), Row(r)) = position;
        self.0[c as usize][r as usize] = Some(marker);
        Ok(())
    }

    /// Returns the marker at a position, since this requires [`Row`] and [`Col`] structs
    /// the indexing will always be inbound
    ///
    /// ```
    /// use tic_tac_toe::{Board, Row, Col, Marker::*};
    ///
    /// let board: Board = Board::from_ints([[0, 1, 2], [0, 0, 0], [1, 0, 0]]);
    /// assert_eq!(board.at_position((Col::new(2), Row::new(0))), Some(X));
    /// assert_eq!(board.at_position((Col::new(0), Row::new(2))), Some(O));
    /// assert_eq!(board.at_position((Col::new(0), Row::new(0))), None);
    /// ```
    pub fn at_position(&self, (Col(c), Row(r)): Position) -> Option<Marker> {
        self.0[c as usize][r as usize]
    }

    /// Returns a marker at a position, if the row or col is greater than 2, this returns None
    ///
    /// ```
    /// use tic_tac_toe::{Board, Row, Col, Marker::*};
    ///
    /// let board: Board = Board::from_ints([[0, 1, 2], [0, 0, 0], [1, 0, 0]]);
    /// assert_eq!(board.at((2, 0)), Some(X));
    /// assert_eq!(board.at((0, 2)), Some(O));
    /// assert_eq!(board.at((0, 0)), None);
    ///
    /// // Out of bounds numbers return None
    /// assert_eq!(board.at((0, 1000)), None);
    /// assert_eq!(board.at((1000, 0)), None);
    /// ```
    pub fn at(&self, (c, r): (usize, usize)) -> Option<Marker> {
        let col = Col::try_new(c.try_into().ok()?)?;
        let row = Row::try_new(r.try_into().ok()?)?;

        self.at_position((col, row))
    }

    /// Iterate over the spaces on the board and the marker in the space (if there is one)
    ///
    /// ```
    /// use tic_tac_toe::{Board, Row, Col, Marker::*, Position};
    ///
    /// let board: Board = Board::from_ints([[0, 1, 2], [0, 0, 0], [1, 2, 1]]);
    /// assert_eq!(
    ///   board.spaces().collect::<Vec<_>>(),
    ///   vec![
    ///     ((Col::new(0), Row::new(0)), None),
    ///     ((Col::new(0), Row::new(1)), Some(X)),
    ///     ((Col::new(0), Row::new(2)), Some(O)),
    ///     ((Col::new(1), Row::new(0)), None),
    ///     ((Col::new(1), Row::new(1)), None),
    ///     ((Col::new(1), Row::new(2)), None),
    ///     ((Col::new(2), Row::new(0)), Some(X)),
    ///     ((Col::new(2), Row::new(1)), Some(O)),
    ///     ((Col::new(2), Row::new(2)), Some(X))
    ///   ]
    /// );
    /// ```
    pub fn spaces(&self) -> impl Iterator<Item = (Position, Option<Marker>)> + '_ {
        self.0.iter().enumerate().flat_map(|(col_num, col)| {
            col.iter().enumerate().map(move |(row_num, &marker)| {
                (
                    (
                        Col::new(col_num.try_into().unwrap()),
                        Row::new(row_num.try_into().unwrap()),
                    ),
                    marker,
                )
            })
        })
    }

    /// Iterate over the spaces on the board that are taken
    ///
    /// ```
    /// use tic_tac_toe::{Board, Row, Col, Marker::*, Position};
    ///
    /// let board: Board = Board::from_ints([[0, 1, 2], [0, 0, 0], [1, 2, 1]]);
    /// assert_eq!(
    ///   board.taken_spaces().collect::<Vec<_>>(),
    ///   vec![
    ///     ((Col::new(0), Row::new(1)), X),
    ///     ((Col::new(0), Row::new(2)), O),
    ///     ((Col::new(2), Row::new(0)), X),
    ///     ((Col::new(2), Row::new(1)), O),
    ///     ((Col::new(2), Row::new(2)), X)
    ///   ]
    /// );
    pub fn taken_spaces(&self) -> impl Iterator<Item = (Position, Marker)> + '_ {
        self.spaces()
            .filter_map(|(pos, marker)| marker.map(|p| (pos, p)))
    }

    /// Return the marker who's turn it is
    ///
    /// ```
    /// use tic_tac_toe::{Board, Marker::*};
    ///
    /// // Starts with X
    /// let board: Board = Default::default();
    /// assert_eq!(X, board.whose_turn());

    /// // Once the first player goes, it's the second player's turn
    /// let board = Board::from_ints([[1, 0, 0], [0, 0, 0], [0, 0, 0]]);
    /// assert_eq!(O, board.whose_turn());

    /// // Once O goes, it's X's turn again
    /// let board = Board::from_ints([[1, 2, 0], [0, 0, 0], [0, 0, 0]]);
    /// assert_eq!(X, board.whose_turn());

    /// // The next player to go is always the one with the fewest spaces
    /// let board = Board::from_ints([[0, 2, 2], [2, 2, 2], [2, 2, 2]]);
    /// assert_eq!(X, board.whose_turn());
    /// ```
    pub fn whose_turn(&self) -> Marker {
        let mut counts: HashMap<Marker, usize> = HashMap::new();

        for (_, marker) in self.taken_spaces() {
            *counts.entry(marker).or_insert(0) += 1;
        }

        [X, O]
            .iter()
            .min_by_key(|marker| counts.get(marker).cloned().unwrap_or(0))
            .copied()
            .unwrap_or(X)
    }

    /// Convenience method to construct a board from arrays of ints, nice for literals in specs
    /// 0 => None
    /// 1 => Some(X)
    /// 2 => Some(O)
    ///
    /// ```
    /// // An empty board
    /// use tic_tac_toe::{Board, Col, Row, Marker::*};
    /// let board = Board::from_ints(
    ///   [
    ///     [0, 0, 0],
    ///     [0, 0, 0],
    ///     [0, 0, 0]
    ///   ]
    /// );
    ///
    /// assert_eq!(board, Default::default());
    ///
    /// // With some things on the board
    ///
    /// let board = Board::from_ints(
    ///   [
    ///     [1, 0, 0],
    ///     [2, 1, 0],
    ///     [0, 0, 0]
    ///   ]
    /// );
    ///
    /// assert_eq!(
    ///   board.taken_spaces().collect::<Vec<_>>(),
    ///   vec![
    ///     ((Col::new(0), Row::new(0)), X),
    ///     ((Col::new(1), Row::new(0)), O),
    ///     ((Col::new(1), Row::new(1)), X)
    ///   ]
    /// )
    /// ```
    ///
    /// # Panics
    ///
    /// Will panic if the number is outside of 0..=2
    ///
    /// ```should_panic
    /// use tic_tac_toe::Board;
    ///
    /// Board::from_ints(
    ///   [
    ///     [0, 0, 0],
    ///     [0, 3, 0],
    ///     [0, 0, 0]
    ///   ]
    /// );
    /// ```
    pub fn from_ints(board: [[u16; 3]; 3]) -> Self {
        let b = board.map(|col| {
            col.map(|n| match n {
                0 => None,
                1 => Some(X),
                2 => Some(O),
                _ => panic!("Invalid number, must ints must be within 0..=2"),
            })
        });
        Board(b)
    }

    /// is the board full?
    ///
    /// ```
    /// use tic_tac_toe::Board;
    ///
    /// let board = Board::from_ints(
    ///   [
    ///     [1, 1, 1],
    ///     [1, 0, 1],
    ///     [1, 1, 1]
    ///   ]
    /// );
    ///
    /// assert!(!board.is_full());
    ///
    /// let board = Board::from_ints(
    ///   [
    ///     [1, 1, 1],
    ///     [1, 1, 1],
    ///     [1, 1, 1]
    ///   ]
    /// );
    ///
    /// assert!(board.is_full());
    ///
    /// ```
    pub fn is_full(&self) -> bool {
        self.taken_spaces().count() == 9
    }
}
