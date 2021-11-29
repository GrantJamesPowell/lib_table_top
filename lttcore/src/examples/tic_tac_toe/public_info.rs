use super::{Marker, Position, TicTacToe};
use crate::{
    play::{Player, Score, View},
    utilities::PlayerIndexedData as PID,
};
use serde::{Deserialize, Serialize};
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

impl From<TicTacToe> for PublicInfo {
    fn from(ttt: TicTacToe) -> Self {
        Self(ttt)
    }
}

impl Score for PublicInfo {
    fn is_score_human_interpertable() -> bool {
        false
    }

    fn score(&self) -> Option<PID<u64>> {
        self.status().winner().map(|winner| {
            [
                (Player::from(winner), 1),
                (Player::from(winner.opponent()), 0),
            ]
            .into_iter()
            .collect()
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PublicInfoUpdate {
    Resign(Marker),
    Claim(Marker, Position),
}

impl View for PublicInfo {
    type Update = PublicInfoUpdate;

    fn update(&mut self, update: Cow<'_, Self::Update>) {
        match update.as_ref() {
            PublicInfoUpdate::Resign(marker) => {
                self.0.resign(*marker);
            }
            PublicInfoUpdate::Claim(marker, position) => {
                self.0
                    .claim_space(*marker, *position)
                    .expect("ttt recieves a valid PublicInfoUpdate");
            }
        }
    }
}
