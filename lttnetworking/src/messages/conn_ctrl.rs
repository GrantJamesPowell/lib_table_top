use crate::connection::SubConnId;
use crate::messages::closed::Closed;
use bytes::Bytes;
use lttcore::id::GameId;
use lttcore::Player;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ClientConnControlMsg {
    StartSubConn { id: SubConnId, game_type: String },
    SubConnMsg { id: SubConnId, bytes: Bytes },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ServerConnControlMsg {
    SubConnStarted { id: SubConnId },
    SubConnMsg { id: SubConnId, bytes: Bytes },
    SubConnClosed { id: SubConnId, reason: Closed },
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
