use crate::play::{Play, SettingsPtr, TurnNum, View};
use crate::pov::{ObserverPov, ObserverUpdate};
use crate::utilities::PlayerSet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct GameObserver<T: Play> {
    pub(crate) turn_num: TurnNum,
    pub(crate) action_requests: PlayerSet,
    pub(crate) settings: SettingsPtr<T::Settings>,
    pub(crate) public_info: T::PublicInfo,
}

impl<T: Play> GameObserver<T> {
    pub fn settings(&self) -> &T::Settings {
        self.settings.settings()
    }
}

impl<T: Play> GameObserver<T> {
    pub fn observer_pov(&self) -> ObserverPov<'_, T> {
        ObserverPov {
            turn_num: self.turn_num,
            action_requests: self.action_requests,
            settings: self.settings(),
            public_info: &self.public_info,
        }
    }

    pub fn update(&mut self, update: ObserverUpdate<'_, T>) {
        self.turn_num = update.turn_num;
        self.action_requests = update.action_requests;
        self.public_info.update(update.public_info_update);
    }
}

use crate::examples::GuessTheNumber;
assert_impl_all!(GameObserver<GuessTheNumber>: Send);
