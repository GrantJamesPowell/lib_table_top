use crate::connection::SubConnectionId;
use crate::messages::closed::Closed;
use crate::SupportedGames;
use bytes::Bytes;
use lttcore::encoder::Encoder;
use lttcore::id::GameId;
use lttcore::Player;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(bound = "")]
pub enum ClientConnControlMsg<S: SupportedGames<E>, E: Encoder> {
    StartSubConn {
        id: SubConnectionId,
        game_type: S,
        #[serde(skip)]
        _encoder: std::marker::PhantomData<E>,
    },
    SubConnMsg {
        id: SubConnectionId,
        bytes: Bytes,
        #[serde(skip)]
        _encoder: std::marker::PhantomData<E>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(bound = "")]
pub enum ServerConnControlMsg<S: SupportedGames<E>, E: Encoder> {
    SubConnStarted {
        id: SubConnectionId,
        game_type: S,
        #[serde(skip)]
        _encoder: std::marker::PhantomData<E>,
    },
    SubConnMsg {
        id: SubConnectionId,
        bytes: Bytes,
        #[serde(skip)]
        _encoder: std::marker::PhantomData<E>,
    },
    SubConnClosed {
        id: SubConnectionId,
        reason: Closed,
        #[serde(skip)]
        _encoder: std::marker::PhantomData<E>,
    },
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
