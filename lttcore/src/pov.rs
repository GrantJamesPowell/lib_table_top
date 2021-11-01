use crate::{Play, Player, PlayerSet, View};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObserverPov<'a, T: Play> {
    pub turn_num: u64,
    pub action_requests: PlayerSet,
    pub settings: &'a <T as Play>::Settings,
    pub public_info: &'a <T as Play>::PublicInfo,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerPov<'a, T: Play> {
    pub turn_num: u64,
    pub action_requests: PlayerSet,
    pub player: Player,
    pub settings: &'a <T as Play>::Settings,
    pub secret_info: &'a <T as Play>::PlayerSecretInfo,
    pub public_info: &'a <T as Play>::PublicInfo,
}

pub struct ObserverUpdate<T: Play> {
    pub turn_num: u64,
    pub action_requests: PlayerSet,
    pub public_info_update: <<T as Play>::PublicInfo as View>::Update,
}

pub trait Observe<T: Play> {
    fn observer_pov(&self) -> ObserverPov<'_, T>;
}
