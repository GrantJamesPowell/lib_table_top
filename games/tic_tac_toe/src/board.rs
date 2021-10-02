use crate::settings::Settings;
use lib_table_top_core::Player;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Col(pub u8);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Row(pub u8);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position(pub Col, pub Row);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Board(pub [[Option<Player>; 3]; 3]);

impl Board {
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
