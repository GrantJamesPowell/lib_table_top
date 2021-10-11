use crate::Marker::{self, *};
use lttcore::Player;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
    players: [Player; 2],
}

impl Settings {
    pub fn new(players: [Player; 2]) -> Self {
        Self { players }
    }

    pub fn players(&self) -> &[Player] {
        &self.players
    }

    pub fn player_for_marker(&self, marker: Marker) -> Player {
        match marker {
            X => self.players[0],
            O => self.players[1],
        }
    }
}
