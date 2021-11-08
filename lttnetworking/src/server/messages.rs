use lttcore::play::ActionResponse;
use lttcore::pov::{ObserverUpdate, PlayerUpdate};
use lttcore::{GameObserver, GamePlayer, Play, Player};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameHostMsg<T: Play> {
    RequestObserverState,
    RequestPlayerState {
        player: Player,
    },
    SubmitActionResponse {
        player: Player,
        response: ActionResponse<T>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerMsg<T: Play> {
    SyncState(GamePlayer<T>),
    Update(PlayerUpdate<'static, T>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound = "")]
pub enum ObserverMsg<T: Play> {
    SyncState(GameObserver<T>),
    Update(ObserverUpdate<'static, T>),
}

impl<T: Play> From<ObserverUpdate<'static, T>> for ObserverMsg<T> {
    fn from(update: ObserverUpdate<'static, T>) -> Self {
        Self::Update(update)
    }
}

impl<T: Play> From<GameObserver<T>> for ObserverMsg<T> {
    fn from(state: GameObserver<T>) -> Self {
        Self::SyncState(state)
    }
}
