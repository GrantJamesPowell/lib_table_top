use crate::settings::Settings;
use crate::ActionError::{self, *};
use lib_table_top_core::Player;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::num::TryFromIntError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Col(u8);

impl Col {
    pub fn new(n: u8) -> Self {
        Self::try_new(n).expect(&format!(
            "Invalid index, n must be within 0..=2, and {:?} was supplied",
            n
        ))
    }

    pub fn try_new(n: u8) -> Option<Self> {
        match n {
            0 | 1 | 2 => Some(Self(n)),
            _ => None,
        }
    }
}

impl TryFrom<usize> for Col {
    type Error = TryFromIntError;

    fn try_from(c: usize) -> Result<Self, Self::Error> {
        c.try_into().map(Col)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Row(u8);

impl TryFrom<usize> for Row {
    type Error = TryFromIntError;

    fn try_from(r: usize) -> Result<Self, Self::Error> {
        r.try_into().map(Row)
    }
}

impl Row {
    pub fn new(n: u8) -> Self {
        Self::try_new(n).expect(&format!(
            "Invalid index, n must be within 0..=2, and {:?} was supplied",
            n
        ))
    }

    pub fn try_new(n: u8) -> Option<Self> {
        match n {
            0 | 1 | 2 => Some(Self(n)),
            _ => None,
        }
    }
}

pub type Position = (Col, Row);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Board(pub [[Option<Player>; 3]; 3]);

impl From<[[u16; 3]; 3]> for Board {
    fn from(board: [[u16; 3]; 3]) -> Self {
        let b = board.map(|row| row.map(|i| i.try_into().ok()));
        Board(b)
    }
}

impl Default for Board {
    fn default() -> Self {
        Self([[None; 3]; 3])
    }
}

impl Board {
    pub fn claim_space(&mut self, player: Player, position: Position) -> Result<(), ActionError> {
        if self.at(position).is_some() {
            return Err(SpaceIsTaken {
                attempted: position,
            });
        }

        let (Col(c), Row(r)) = position;
        self.0[c as usize][r as usize] = Some(player);
        Ok(())
    }

    pub fn at(&self, (Col(c), Row(r)): Position) -> Option<Player> {
        self.0[c as usize][r as usize]
    }

    /// Iterate over the spaces on the board and the player in the space (if there is one)
    ///
    /// ```
    /// use tic_tac_toe::{Board, Row, Col, Player, Position, player::p};
    ///
    /// let board: Board = [[0, 1, 2], [0, 0, 0], [1, 2, 1]].into();
    /// assert_eq!(
    ///   board.spaces().collect::<Vec<_>>(),
    ///   vec![
    ///     ((Col::new(0), Row::new(0)), None),
    ///     ((Col::new(0), Row::new(1)), Some(p(1))),
    ///     ((Col::new(0), Row::new(2)), Some(p(2))),
    ///     ((Col::new(1), Row::new(0)), None),
    ///     ((Col::new(1), Row::new(1)), None),
    ///     ((Col::new(1), Row::new(2)), None),
    ///     ((Col::new(2), Row::new(0)), Some(p(1))),
    ///     ((Col::new(2), Row::new(1)), Some(p(2))),
    ///     ((Col::new(2), Row::new(2)), Some(p(1)))
    ///   ]
    /// );
    /// ```
    pub fn spaces(&self) -> impl Iterator<Item = (Position, Option<Player>)> + '_ {
        self.0.iter().enumerate().flat_map(|(col_num, col)| {
            col.iter().enumerate().map(move |(row_num, &player)| {
                (
                    (col_num.try_into().unwrap(), row_num.try_into().unwrap()),
                    player,
                )
            })
        })
    }

    /// Iterate over the spaces on the board that are taken
    ///
    /// ```
    /// use tic_tac_toe::{Board, Row, Col, Player, Position, player::p};
    ///
    /// let board: Board = [[0, 1, 2], [0, 0, 0], [1, 2, 1]].into();
    /// assert_eq!(
    ///   board.taken_spaces().collect::<Vec<_>>(),
    ///   vec![
    ///     ((Col::new(0), Row::new(1)), p(1)),
    ///     ((Col::new(0), Row::new(2)), p(2)),
    ///     ((Col::new(2), Row::new(0)), p(1)),
    ///     ((Col::new(2), Row::new(1)), p(2)),
    ///     ((Col::new(2), Row::new(2)), p(1))
    ///   ]
    /// );
    pub fn taken_spaces(&self) -> impl Iterator<Item = (Position, Player)> + '_ {
        self.spaces()
            .filter_map(|(pos, player)| player.map(|p| (pos, p)))
    }

    /// Return the player who's turn it is
    ///
    /// ```
    /// use tic_tac_toe::{Board, Settings, player::p};
    ///
    /// // Starts with the first player in the settings
    /// let board: Board = Default::default();

    /// assert_eq!(p(1), board.whose_turn(&Settings::new([p(1), p(2)])));
    /// assert_eq!(p(2), board.whose_turn(&Settings::new([p(2), p(1)])));

    /// // The 'starting player' goes first
    /// let settings = Settings::new([p(1), p(2)]);

    /// assert_eq!(settings.starting_player(), board.whose_turn(&settings));

    /// // Once the first player goes, it's the second player's turn
    /// let board: Board = [[1, 0, 0], [0, 0, 0], [0, 0, 0]].into();
    /// assert_eq!(p(2), board.whose_turn(&settings));

    /// // Once the second player goes, it's the first players turn again
    /// // I. E. if all players have an even number of turns, it's the first players turn
    /// let board: Board = [[1, 2, 0], [0, 0, 0], [0, 0, 0]].into();
    /// assert_eq!(p(1), board.whose_turn(&settings));

    /// // The next player to go is always the one with the fewest spaces
    /// let board: Board = [[0, 2, 2], [2, 2, 2], [2, 2, 2]].into();
    /// assert_eq!(p(1), board.whose_turn(&settings));
    /// ```

    pub fn whose_turn(&self, settings: &Settings) -> Player {
        let mut counts: HashMap<Player, usize> = HashMap::new();

        for (_, player) in self.taken_spaces() {
            *counts.entry(player).or_insert(0) += 1;
        }

        settings
            .players()
            .iter()
            .min_by_key(|player| counts.get(player).cloned().unwrap_or(0))
            .copied()
            .unwrap_or(settings.starting_player())
    }
}
