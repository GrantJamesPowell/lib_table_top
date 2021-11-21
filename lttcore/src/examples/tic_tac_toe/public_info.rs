use super::{helpers::opponent, Position, TicTacToe};
use crate::Player;
use crate::{
    play::{Score, View},
    utilities::PlayerIndexedData as PID,
};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::borrow::Cow;
use std::ops::Deref;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicInfo(TicTacToe);

impl Deref for PublicInfo {
    type Target = TicTacToe;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PublicInfo {
    pub fn from_ttt(ttt: TicTacToe) -> Self {
        Self(ttt)
    }
}

impl Score for PublicInfo {
    fn is_score_human_interpertable() -> bool {
        false
    }

    fn score(&self) -> Option<PID<u64>> {
        self.status()
            .winner()
            .map(|winner| [(winner, 1), (opponent(winner), 0)].into_iter().collect())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PublicInfoUpdate {
    Resign(Player),
    Claim(Player, Position),
}

impl View for PublicInfo {
    type Update = PublicInfoUpdate;

    fn update(&mut self, update: Cow<'_, Self::Update>) {
        match update.as_ref() {
            PublicInfoUpdate::Resign(player) => {
                self.0.resign(*player);
            }
            PublicInfoUpdate::Claim(player, position) => {
                self.0
                    .claim_space(*player, *position)
                    .expect("ttt recieves a valid PublicInfoUpdate");
            }
        }
    }
}
