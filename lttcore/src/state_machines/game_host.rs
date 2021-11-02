use crate::utilities::PlayerIndexedData;
use crate::{GamePlayer, Play};

pub enum DebugInfo<T: Play> {
    GameDebugMsg(<T as Play>::ActionError),
}

pub enum SpectatorMsg<'a, T: Play> {
    Init(GameObserver<T>),
    GameUpdate(ObserverUpdate<'static, T>),
    Meta,
}

pub enum PlayerMsg<'a, T: Play> {
    Init(GamePlayer<T>),
    GameUpdate(PlayerUpdate<'a, T>),
    Debug(DebugInfo<T>),
}

struct GameHost<T: Play> {
    game_progression: GameProgression<T>,
}

struct Effects<'a, T: Play> {
    player_msgs: PlayerIndexedData<PlayerMsg<'a, T>>,
    spectator_msg: Option<SpectatorMsg<'a, T>>,
}

impl<T: Play> GameHost<T> {}
