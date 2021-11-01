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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(bound = "")]
pub struct PlayerUpdate<'a, T: Play> {
    pub observer_update: ObserverUpdate<'a, T>,
    pub secret_info_update: Option<Cow<'a, <<T as Play>::PlayerSecretInfo as View>::Update>>,
}
