use crate::action_request::ActionRequestSource;
use crate::{ActionRequest, Observe, Observer, Play, Player, PlayerSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GamePlayer<T: Play> {
    turn_num: u64,
    player: Player,
    action_requests: PlayerSet,
    settings: <T as Play>::Settings,
    public_info: <T as Play>::PublicInfo,
    secret_info: <T as Play>::PlayerSecretInfo,
}

impl<T: Play> Observe<T> for GamePlayer<T> {
    fn observe(&self) -> Observer<'_, T> {
        Observer {
            turn_num: self.turn_num,
            action_requests: self.action_requests,
            settings: &self.settings,
            public_info: &self.public_info,
        }
    }
}

impl<T: Play> ActionRequestSource<T> for GamePlayer<T> {
    fn next_action_request(&self) -> Option<ActionRequest<'_, T>> {
        todo!()
    }
}
