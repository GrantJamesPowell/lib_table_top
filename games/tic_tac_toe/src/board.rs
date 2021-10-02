use crate::settings::Settings;
use lib_table_top_core::Player;
use std::convert::TryFrom;
use std::num::TryFromIntError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Col(pub u8);

impl TryFrom<usize> for Col {
    type Error = TryFromIntError;

    fn try_from(c: usize) -> Result<Self, Self::Error> {
        Ok(Col(c.try_into()?))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Row(pub u8);

impl TryFrom<usize> for Row {
    type Error = TryFromIntError;

    fn try_from(r: usize) -> Result<Self, Self::Error> {
        Ok(Row(r.try_into()?))
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

// impl From<[[u16; 3]; 3]> for Board {
// 
// }

impl Board {
    pub fn iter_taken_spaces(&self) -> impl Iterator<Item=(Position, Player)> + '_ {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(col_num, col)| {
                col
                    .iter()
                    .enumerate()
                    .filter_map(move |(row_num, player)| {
                        player.map(|p| ((col_num, row_num).try_into().unwrap(), p))
                    })
            })
    }

    pub fn whose_turn(&self, settings: &Settings) -> Player {
        let mut space_counts = [0, 0];

        for &player in self.0.iter().flatten().flatten() {
            let p = if player == settings.p1() { 0 } else { 1 };
            space_counts[p] += 1;
        }

        if space_counts[0] > space_counts[1] {
            settings.p1()
        } else {
            settings.p2()
        }
    }
}
