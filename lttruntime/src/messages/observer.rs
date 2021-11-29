use lttcore::play::Play;
use lttcore::pov::{GameObserver, ObserverUpdate};
use serde::{Deserialize, Serialize};

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
