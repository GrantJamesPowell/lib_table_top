use crate::play::{Play, Player, TurnNum, View};
use crate::PlayerSet;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObserverPov<'a, T: Play> {
    pub turn_num: TurnNum,
    pub action_requests: PlayerSet,
    pub settings: &'a T::Settings,
    pub public_info: &'a T::PublicInfo,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerPov<'a, T: Play> {
    pub turn_num: TurnNum,
    pub action_requests: PlayerSet,
    pub player: Player,
    pub settings: &'a T::Settings,
    pub secret_info: &'a T::PlayerSecretInfo,
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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(bound = "")]
pub struct PlayerUpdate<'a, T: Play> {
    pub(crate) player: Player,
    pub(crate) observer_update: ObserverUpdate<'a, T>,
    pub(crate) secret_info_update: Option<Cow<'a, <T::PlayerSecretInfo as View>::Update>>,
}

impl<'a, T: Play> PlayerUpdate<'a, T> {
    /// Return the turn num for the player update
    pub fn turn_num(&self) -> TurnNum {
        self.observer_update.turn_num
    }

    /// Return whether a specific player's input is needed this turn
    pub fn is_player_input_needed_this_turn(&self, player: Player) -> bool {
        self.observer_update.action_requests.contains(player)
    }

    /// Change the lifetime to 'static making `PlayerUpdate` function like an owned type
    pub fn into_owned(self) -> PlayerUpdate<'static, T> {
        PlayerUpdate {
            player: self.player,
            observer_update: self.observer_update.into_owned(),
            secret_info_update: self.secret_info_update.map(|x| Cow::Owned(x.into_owned())),
        }
    }
}
