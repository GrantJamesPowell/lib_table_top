use crate::{Observe, Observer, Play, PlayerSet};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct GameObserver<T: Play> {
    pub(crate) turn_num: u64,
    pub(crate) action_requests: PlayerSet,
    pub(crate) settings: Arc<<T as Play>::Settings>,
    pub(crate) public_info: <T as Play>::PublicInfo,
}

impl<T: Play> Observe<T> for GameObserver<T> {
    fn observe(&self) -> Observer<'_, T> {
        Observer {
            turn_num: self.turn_num,
            action_requests: self.action_requests,
            settings: &self.settings,
            public_info: &self.public_info,
        }
    }
}
