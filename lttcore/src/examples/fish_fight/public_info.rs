use super::{Board, Position};
use crate::play::Score;
use crate::play::View;
use crate::utilities::PlayerIndexedData as PID;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PublicInfo {
    Setup,
    Playing { boards: PID<Board> },
}

impl Score for PublicInfo {
    fn score(&self) -> Option<PID<u64>> {
        match self {
            PublicInfo::Setup => None,
            PublicInfo::Playing { boards } => {
                let scores = boards
                    .iter()
                    .map(|(player, board)| (player, board.hits.count_ones()))
                    .collect();

                Some(scores)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PublicInfoUpdate {
    pub guesses: PID<Position>,
    pub hits: PID<Position>,
}

impl View for PublicInfo {
    type Update = PublicInfoUpdate;

    fn update(&mut self, _update: Cow<'_, Self::Update>) {}
}
