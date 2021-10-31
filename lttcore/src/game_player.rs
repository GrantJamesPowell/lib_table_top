use crate::pov::{Observe, ObserverPov};
use crate::{Play, Player, PlayerSet};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
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
    fn observer_pov(&self) -> ObserverPov<'_, T> {
        ObserverPov {
            turn_num: self.turn_num,
            action_requests: self.action_requests,
            settings: Cow::Borrowed(&self.settings),
            public_info: Cow::Borrowed(&self.public_info),
        }
    }
}
