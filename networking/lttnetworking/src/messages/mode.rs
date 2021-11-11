use crate::SupportedGames;
use lttcore::id::GameId;
use lttcore::Player;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound = "")]
pub enum Mode<SG: SupportedGames> {
    JoinInProgressGame((SG, GameId), JoinAs),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JoinAs {
    Player(Player),
    Observer,
}
