use lttcore::common::cartesian::{Point, PointSet};
use lttcore::{
    play::{score::ScoreInterpertation, Score, View},
    utilities::number_of_players::ONE_PLAYER,
    utilities::PlayerIndexedData as PID,
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicInfo {
    hits: PointSet,
    misses: PointSet,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PublicInfoUpdate {
    Hit(Point),
    Miss(Point),
}

use PublicInfoUpdate::*;

impl View for PublicInfo {
    type Update = PublicInfoUpdate;

    fn update(&mut self, update: Cow<'_, Self::Update>) {
        match update.as_ref() {
            Hit(point) => self.hits.insert(*point),
            Miss(point) => self.misses.insert(*point),
        }
    }
}

impl Score for PublicInfo {
    fn score(&self) -> Option<PID<u64>> {
        let num_guesses = self.misses.len() + self.hits.len();
        let scores = ONE_PLAYER.player_indexed_data(|_player| {
            u64::try_from(num_guesses).expect("we're not blowing out u64s...")
        });
        Some(scores)
    }

    fn score_interpertation() -> ScoreInterpertation {
        ScoreInterpertation::LowerIsBetter
    }
}
