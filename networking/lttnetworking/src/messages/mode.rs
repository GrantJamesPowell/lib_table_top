use lttcore::id::GameId;
use lttcore::Player;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    JoinInProgressGame(GameId, JoinAs),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JoinAs {
    Player(Player),
    Observer,
}
