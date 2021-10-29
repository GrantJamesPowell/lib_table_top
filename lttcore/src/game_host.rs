use crate::{GameRunner, Play, Player, Observer, Observe};
use std::collections::HashMap;

pub struct GameHost<T: Play> {
    game_runner: GameRunner<T>,
    public_info: <T as Play>::PublicInfo,
    // action_requests: ActionRequests<T>,
    player_secret_info: HashMap<Player, <T as Play>::PlayerSecretInfo>,
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
