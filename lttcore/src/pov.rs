use crate::{Play, Player, PlayerSet, TurnNum, View};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObserverPov<'a, T: Play> {
    pub turn_num: TurnNum,
    pub action_requests: PlayerSet,
    pub settings: &'a <T as Play>::Settings,
    pub public_info: &'a <T as Play>::PublicInfo,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerPov<'a, T: Play> {
    pub turn_num: TurnNum,
    pub action_requests: PlayerSet,
    pub player: Player,
    pub settings: &'a <T as Play>::Settings,
    pub secret_info: &'a <T as Play>::PlayerSecretInfo,
    pub public_info: &'a <T as Play>::PublicInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(bound = "")]
pub struct ObserverUpdate<'a, T: Play> {
    pub turn_num: TurnNum,
    pub action_requests: PlayerSet,
    pub public_info_update: Cow<'a, <<T as Play>::PublicInfo as View>::Update>,
}

impl<'a, T: Play> ObserverUpdate<'a, T> {
    pub fn is_player_input_needed(&self, player: Player) -> bool {
        self.action_requests.contains(player)
    }

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
    pub player: Player,
    pub observer_update: ObserverUpdate<'a, T>,
    pub secret_info_update: Option<Cow<'a, <<T as Play>::PlayerSecretInfo as View>::Update>>,
}

impl<'a, T: Play> PlayerUpdate<'a, T> {
    pub fn is_this_players_input_needed(&self) -> bool {
        self.observer_update.is_player_input_needed(self.player)
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
