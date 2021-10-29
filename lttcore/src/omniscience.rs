use crate::{Play, Player};
use std::collections::HashMap;

pub struct Omniscience<'a, T: Play> {
    pub turn_num: u64,
    pub settings: &'a <T as Play>::Settings,
    pub player_secret_info: HashMap<Player, &'a <T as Play>::PlayerSecretInfo>,
    pub public_info: &'a <T as Play>::PublicInfo,
    pub game_state: &'a T,
}

pub trait Omniscient<'a, T: Play> {
    fn omniscience(&self) -> Omniscience<'_, T>;
}
