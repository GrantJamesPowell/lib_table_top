use lttcore::pov::PlayerUpdate;
use lttcore::{GamePlayer, Play, TurnNum};
use serde::{Deserialize, Serialize};

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
