use crate::{GamePlayer, Play, TurnNum};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound = "")]
pub enum ClientInGameMsg<T: Play> {
    RequestStateSync,
    SubmitAction {
        turn: TurnNum,
        action: <T as Play>::Action,
    },
    Resign,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound = "")]
pub enum ServerInGameMsg<T: Play> {
    StateSync(GamePlayer<T>),
}
