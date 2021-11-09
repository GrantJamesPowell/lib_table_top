use crate::{Token, User};
use lttcore::play::ActionResponse;
use lttcore::pov::{ObserverUpdate, PlayerUpdate};
use lttcore::{GameObserver, GamePlayer, Play, Player, TurnNum};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientHello {
    pub credentials: Token,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerHello<'a> {
    pub user: Cow<'a, User>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum JoinError {
    UnparseableClientHello,
    Unauthorized,
    UnsupportedVersion,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToGameHostMsg<T: Play> {
    RequestObserverState,
    RequestPlayerState {
        player: Player,
    },
    SubmitActionResponse {
        player: Player,
        response: ActionResponse<T>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound = "")]
pub enum FromPlayerMsg<T: Play> {
    RequestPrimary,
    SubmitAction { action: T::Action, turn: TurnNum },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound = "")]
pub enum ToPlayerMsg<T: Play> {
    SyncState(GamePlayer<T>),
    Update(PlayerUpdate<'static, T>),
    SetPrimaryStatus(bool),
    SubmitActionError(SubmitActionErrorKind),
    GameOver,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubmitActionErrorKind {
    NotPrimary,
    Timeout {
        turn_num: TurnNum,
    },
    InvalidTurn {
        attempted: TurnNum,
        correct: Option<TurnNum>,
    },
}

impl<T: Play> From<PlayerUpdate<'static, T>> for ToPlayerMsg<T> {
    fn from(update: PlayerUpdate<'static, T>) -> Self {
        Self::Update(update)
    }
}

impl<T: Play> From<GamePlayer<T>> for ToPlayerMsg<T> {
    fn from(state: GamePlayer<T>) -> Self {
        Self::SyncState(state)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound = "")]
pub enum ToObserverMsg<T: Play> {
    SyncState(GameObserver<T>),
    Update(ObserverUpdate<'static, T>),
    GameOver,
}

impl<T: Play> From<ObserverUpdate<'static, T>> for ToObserverMsg<T> {
    fn from(update: ObserverUpdate<'static, T>) -> Self {
        Self::Update(update)
    }
}

impl<T: Play> From<GameObserver<T>> for ToObserverMsg<T> {
    fn from(state: GameObserver<T>) -> Self {
        Self::SyncState(state)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PingMsg {
    Ping,
    Pong,
}

use PingMsg::*;

impl PingMsg {
    /// Opposite of {Ping/Pong}
    ///
    /// ```
    /// use lttnetworking::messages::PingMsg::*;
    ///
    /// assert_eq!(Ping.opposite(), Pong);
    /// assert_eq!(Pong.opposite(), Ping);
    /// ```
    pub fn opposite(&self) -> Self {
        match self {
            Ping => Pong,
            Pong => Ping,
        }
    }
}
