use crate::play::{Score, TurnNum, View};
use crate::utilities::PlayerIndexedData as PID;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicInfo {
    pub statuses: PID<PlayerStatus>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PublicInfoUpdate {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayerStatus {
    HitScoreLimit { turn: TurnNum },
    Resigned { turn: TurnNum, chips: u32 },
    InPlay { chips: u32 },
    Busted { turn: TurnNum },
}

impl Score for PublicInfo {
    fn score(&self) -> Option<PID<i64>> {
        todo!()
    }
}

impl View for PublicInfo {
    type Update = PublicInfoUpdate;

    fn update(&mut self, _update: Cow<'_, Self::Update>) {}
}
