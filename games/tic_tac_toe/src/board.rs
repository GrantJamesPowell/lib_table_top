use crate::settings::Settings;
use lib_table_top_core::Player;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::num::TryFromIntError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Col(pub u8);

impl TryFrom<usize> for Col {
    type Error = TryFromIntError;

    fn try_from(c: usize) -> Result<Self, Self::Error> {
        c.try_into().map(Col)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Row(pub u8);

impl TryFrom<usize> for Row {
    type Error = TryFromIntError;

    fn try_from(r: usize) -> Result<Self, Self::Error> {
        r.try_into().map(Row)
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
    /// Iterate over the spaces on the board and the player in the space (if there is one)
    ///
    /// ```
    /// use tic_tac_toe::{Board, Row, Col, Player, Position, player::p};
    ///
    /// let board: Board = [[0, 1, 2], [0, 0, 0], [1, 2, 1]].into();
    /// assert_eq!(
    ///   board.spaces().collect::<Vec<_>>(),
    ///   vec![
    ///     ((Col(0), Row(0)), None),
    ///     ((Col(0), Row(1)), Some(p(1))),
    ///     ((Col(0), Row(2)), Some(p(2))),
    ///     ((Col(1), Row(0)), None),
    ///     ((Col(1), Row(1)), None),
    ///     ((Col(1), Row(2)), None),
    ///     ((Col(2), Row(0)), Some(p(1))),
    ///     ((Col(2), Row(1)), Some(p(2))),
    ///     ((Col(2), Row(2)), Some(p(1)))
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
    ///     ((Col(0), Row(1)), p(1)),
    ///     ((Col(0), Row(2)), p(2)),
    ///     ((Col(2), Row(0)), p(1)),
    ///     ((Col(2), Row(1)), p(2)),
    ///     ((Col(2), Row(2)), p(1))
    ///   ]
    /// );
    pub fn taken_spaces(&self) -> impl Iterator<Item = (Position, Player)> + '_ {
        self.spaces()
            .filter_map(|(pos, player)| player.map(|p| (pos, p)))
    }

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

#[cfg(test)]
mod tests {
    use lib_table_top_core::player::p;

    use super::*;

    #[test]
    fn test_iterating_taken_spaces() {
        let b: Board = [[0, 0, 0], [1, 0, 0], [0, 0, 0]].into();

        assert_eq!(b.taken_spaces().collect::<Vec<_>>(), vec![i2pp((1, 0), 1)]);
    }

    #[test]
    fn test_whose_turn() {
        // Starts with the first player in the settings
        let board: Board = Default::default();

        assert_eq!(p(1), board.whose_turn(&Settings::new([p(1), p(2)])));
        assert_eq!(p(2), board.whose_turn(&Settings::new([p(2), p(1)])));

        // The 'starting player' goes first
        let settings = Settings::new([p(1), p(2)]);

        assert_eq!(settings.starting_player(), board.whose_turn(&settings));

        // Once the first player goes, it's the second player's turn
        let board: Board = [[1, 0, 0], [0, 0, 0], [0, 0, 0]].into();
        assert_eq!(p(2), board.whose_turn(&settings));

        // Once the second player goes, it's the first players turn again
        // I. E. if all players have an even number of turns, it's the first players turn
        let board: Board = [[1, 2, 0], [0, 0, 0], [0, 0, 0]].into();
        assert_eq!(p(1), board.whose_turn(&settings));

        // The next player to go is always the one with the fewest spaces
        let board: Board = [[0, 2, 2], [2, 2, 2], [2, 2, 2]].into();
        assert_eq!(p(1), board.whose_turn(&settings));
    }

    fn i2pp((c, r): (usize, usize), player: u16) -> (Position, Player) {
        ((c.try_into().unwrap(), r.try_into().unwrap()), p(player))
    }
}
