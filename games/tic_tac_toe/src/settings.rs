use lib_table_top_core::Player;
use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
    players: [Player; 2],
}

impl Settings {
    pub fn players(&self) -> &[Player] {
        &self.players
    }

    pub fn starting_player(&self) -> Player {
        self.players[0]
    }
}

#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum SettingsError {
    /// Returned when both players are the same
    #[error("Players must be different")]
    PlayersCantBeTheSame,
}
