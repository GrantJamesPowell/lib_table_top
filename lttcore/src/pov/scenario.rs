#![allow(missing_docs)]
use crate::play::{GameState, Play, Seed, SettingsPtr, TurnNum};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct Scenario<T: Play> {
    pub(crate) turn_num: TurnNum,
    pub(crate) settings: SettingsPtr<<T as Play>::Settings>,
    pub(crate) initial_game_state: Arc<GameState<T>>,
    pub(crate) seed: Arc<Seed>,
}
