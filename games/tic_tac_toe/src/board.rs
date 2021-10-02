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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position(pub Col, pub Row);

impl TryFrom<(usize, usize)> for Position {
    type Error = TryFromIntError;

    fn try_from((c, r): (usize, usize)) -> Result<Self, Self::Error> {
        Ok(Position(c.try_into()?, r.try_into()?))
    }
}

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
    pub fn spaces(&self) -> impl Iterator<Item = (Position, Option<Player>)> + '_ {
        self.0.iter().enumerate().flat_map(|(col_num, col)| {
            col.iter()
                .enumerate()
                .map(move |(row_num, &player)| ((col_num, row_num).try_into().unwrap(), player))
        })
    }

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
            .min_by_key(|player| counts[player])
            .copied()
            .unwrap_or(settings.starting_player())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterating_taken_spaces() {
        let b: Board = [[0, 0, 0], [1, 0, 0], [0, 0, 0]].into();

        assert_eq!(b.taken_spaces().collect::<Vec<_>>(), vec![i2pp((1, 0), 1)]);
    }

    #[test]
    fn test_whose_turn() {}

    fn i2pp(pos: (usize, usize), player: u16) -> (Position, Player) {
        (pos.try_into().unwrap(), player.try_into().unwrap())
    }
}
