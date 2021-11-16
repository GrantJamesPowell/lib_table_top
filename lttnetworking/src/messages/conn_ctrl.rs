use crate::connection::SubConnectionId;
use crate::messages::closed::Closed;
use crate::SupportedGames;
use bytes::Bytes;
use lttcore::id::GameId;
use lttcore::Player;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(bound = "")]
pub enum ClientConnControlMsg<S: SupportedGames> {
    StartSubConn { id: SubConnectionId, game_type: S },
    SubConnMsg { id: SubConnectionId, bytes: Bytes },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(bound = "")]
pub enum ServerConnControlMsg<S: SupportedGames> {
    SubConnStarted { id: SubConnectionId, game_type: S },
    SubConnMsg { id: SubConnectionId, bytes: Bytes },
    SubConnClosed { id: SubConnectionId, reason: Closed },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SubConnStartErrorKind {
    TooManySubConns,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SubConnMode {
    JoinGame(GameId, JoinAs),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JoinAs {
    Player(Player),
    Observer,
}
