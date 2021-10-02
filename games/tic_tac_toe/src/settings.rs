use lib_table_top_core::Player;
use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
    players: [Player; 2],
}

impl Settings {
    pub fn p1(&self) -> Player {
        self.players[0]
    }
    pub fn p2(&self) -> Player {
        self.players[1]
    }
}

#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum SettingsError {
    /// Returned when both players are the same
    #[error("Players must be different")]
    PlayersCantBeTheSame,
}
