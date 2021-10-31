use crate::action_request::ActionRequestSource;
use crate::{ActionRequest, Observe, Observer, Play, Player, PlayerSet};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct GamePlayer<T: Play> {
    pub(crate) turn_num: u64,
    pub(crate) player: Player,
    pub(crate) action_requests: PlayerSet,
    pub(crate) settings: Arc<<T as Play>::Settings>,
    pub(crate) public_info: <T as Play>::PublicInfo,
    pub(crate) secret_info: <T as Play>::PlayerSecretInfo,
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
