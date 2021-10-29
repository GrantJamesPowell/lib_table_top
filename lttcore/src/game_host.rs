use crate::action_request::ActionRequestSource;
use crate::omniscience::Omniscient;
use crate::play::{DebugMsgs, PlayerSecretInfoUpdates};
use crate::{
    ActionRequest, ActionResponse, GameObserver, GamePlayer, GameRunner, Observe, Observer,
    Omniscience, Play, Player, View,
};
use std::collections::HashMap;

pub struct GameHost<T: Play> {
    game_runner: GameRunner<T>,
    public_info: <T as Play>::PublicInfo,
    // action_requests: ActionRequests<T>,
    player_secret_info: HashMap<Player, <T as Play>::PlayerSecretInfo>,
}

pub struct GameHostUpdates<T: Play> {
    public_info_update: <<T as Play>::PublicInfo as View>::Update,
    secret_info_updates: PlayerSecretInfoUpdates<T>,
    debug_msgs: DebugMsgs<T>,
}

impl<T: Play> Observe<T> for GameHost<T> {
    fn observe(&self) -> Observer<'_, T> {
        Observer {
            turn_num: self.game_runner.turn_num(),
            settings: &self.game_runner.settings(),
            public_info: &self.public_info,
        }
    }
}

impl<T: Play> ActionRequestSource<T> for GameHost<T> {
    fn next_action_request(&self) -> Option<ActionRequest<'_, T>> {
        todo!()
    }
}

impl<T: Play> Omniscient<T> for GameHost<T> {
    fn omniscience(&self) -> Omniscience<'_, T> {
        todo!()
    }
}

impl<T: Play> From<GameRunner<T>> for GameHost<T> {
    fn from(_game_runner: GameRunner<T>) -> Self {
        todo!()
    }
}

impl<T: Play> GameHost<T> {
    fn into_game_runner(self) -> GameRunner<T> {
        todo!()
    }

    fn game_runner(&self) -> GameRunner<T> {
        todo!()
    }

    fn game_observer(&self) -> GameObserver<T> {
        todo!()
    }

    fn game_players(&self) -> impl Iterator<Item = GamePlayer<T>> + '_ {
        None.iter().cloned()
    }

    fn submit_action_response(
        &mut self,
        _player: Player,
        _action_response: impl Into<ActionResponse<<T as Play>::Action>>,
    ) -> Option<GameHostUpdates<T>> {
        todo!()
    }
}
