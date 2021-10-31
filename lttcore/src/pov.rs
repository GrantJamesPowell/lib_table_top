use crate::{Play, Player, PlayerSet, View};
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObserverPov<'a, T: Play> {
    pub turn_num: u64,
    pub action_requests: PlayerSet,
    pub settings: Cow<'a, <T as Play>::Settings>,
    pub public_info: Cow<'a, <T as Play>::PublicInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerPov<'a, T: Play> {
    pub turn_num: u64,
    pub player: Player,
    pub settings: Cow<'a, <T as Play>::Settings>,
    pub secret_info: Cow<'a, <T as Play>::PlayerSecretInfo>,
    pub public_info: Cow<'a, <T as Play>::PublicInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OmniscientPov<'a, T: Play> {
    pub turn_num: u64,
    pub settings: Cow<'a, <T as Play>::Settings>,
    pub player_secret_info: Cow<'a, HashMap<Player, <T as Play>::PlayerSecretInfo>>,
    pub public_info: Cow<'a, <T as Play>::PublicInfo>,
    pub game_state: Cow<'a, T>,
}

pub struct ObserverUpdate<T: Play> {
    pub turn_num: u64,
    pub action_requests: PlayerSet,
    pub public_info_update: <<T as Play>::PublicInfo as View>::Update,
}

pub trait Observe<T: Play> {
    fn observer_pov(&self) -> ObserverPov<'_, T>;
}

pub trait Omniscient<T: Play> {
    fn omniscient_pov(&self) -> OmniscientPov<'_, T>;
}
