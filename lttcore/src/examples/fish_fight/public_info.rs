use super::{PlayerBoards, Settings};
use crate::common::cartesian::Point;
use crate::play::View;
use crate::play::{LttSettings, Score};
use crate::utilities::PlayerIndexedData as PID;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PublicInfo {
    boards: PID<PlayerBoards>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Phase {
    Setup,
    Playing,
}

impl PublicInfo {
    pub fn init_from_settings(settings: &Settings) -> Self {
        Self {
            boards: settings
                .number_of_players()
                .player_indexed_data(|_player| settings.dimensions().into()),
        }
    }

    pub fn player_boards(&self) -> &PID<PlayerBoards> {
        &self.boards
    }

    pub fn phase(&self) -> Phase {
        let any_guesses = self
            .boards
            .iter()
            .any(|(_player, board)| board.hits.count() > 0 || board.misses.count() > 0);

        if any_guesses {
            Phase::Playing
        } else {
            Phase::Setup
        }
    }
}

impl Score for PublicInfo {
    fn score(&self) -> Option<PID<u64>> {
        let scores = self
            .boards
            .iter()
            .map(|(player, board)| (player, board.hits.count() as u64))
            .collect();

        Some(scores)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GuessResult {
    Hit(Point),
    Miss(Point),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PublicInfoUpdate {
    pub guesses: PID<GuessResult>,
}

impl View for PublicInfo {
    type Update = PublicInfoUpdate;

    fn update(&mut self, update: Cow<'_, Self::Update>) {
        update
            .guesses
            .iter()
            .for_each(|(player, result)| match result {
                GuessResult::Hit(position) => self.boards[player].hits.insert(*position),
                GuessResult::Miss(position) => self.boards[player].misses.insert(*position),
            });
    }
}
