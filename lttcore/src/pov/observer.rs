use crate::play::{Play, SettingsPtr, TurnNum, View};
use crate::utilities::PlayerSet;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObserverPov<'a, T: Play> {
    pub turn_num: TurnNum,
    pub action_requests: PlayerSet,
    pub settings: &'a T::Settings,
    pub public_info: &'a T::PublicInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(bound = "")]
pub struct ObserverUpdate<'a, T: Play> {
    pub(crate) turn_num: TurnNum,
    pub(crate) action_requests: PlayerSet,
    pub(crate) public_info_update: Cow<'a, <<T as Play>::PublicInfo as View>::Update>,
}

impl<'a, T: Play> ObserverUpdate<'a, T> {
    /// Change the lifetime to 'static making `ObserverUpdate` function like an owned type
    pub fn into_owned(self) -> ObserverUpdate<'static, T> {
        ObserverUpdate {
            turn_num: self.turn_num,
            public_info_update: Cow::Owned(self.public_info_update.into_owned()),
            action_requests: self.action_requests,
        }
    }
}

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
