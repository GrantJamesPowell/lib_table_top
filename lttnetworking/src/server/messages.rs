use lttcore::play::ActionResponse;
use lttcore::pov::{ObserverUpdate, PlayerUpdate};
use lttcore::{GameObserver, GamePlayer, Play, Player};
use serde::{Deserialize, Serialize};

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

pub enum PlayerMsg<T: Play> {
    SyncState(GamePlayer<T>),
    Update(PlayerUpdate<'static, T>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "")]
pub enum ObserverMsg<T: Play> {
    SyncState(GameObserver<T>),
    Update(ObserverUpdate<'static, T>),
}
