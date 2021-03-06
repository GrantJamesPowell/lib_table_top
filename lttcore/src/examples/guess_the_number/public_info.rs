use super::Guess;
use crate::play::{score::ScoreInterpertation, Score, View};
use crate::utilities::PlayerIndexedData as PID;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum PublicInfo {
    InProgress,
    Completed {
        secret_number: u32,
        guesses: PID<Guess>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct PublicInfoUpdate {
    pub secret_number: u32,
    pub guesses: PID<Guess>,
}

impl From<PublicInfoUpdate> for PublicInfo {
    fn from(
        PublicInfoUpdate {
            secret_number,
            guesses,
        }: PublicInfoUpdate,
    ) -> Self {
        PublicInfo::Completed {
            secret_number,
            guesses,
        }
    }
}

impl View for PublicInfo {
    type Update = PublicInfoUpdate;

    fn update(&mut self, update: Cow<'_, Self::Update>) {
        let new: PublicInfo = update.into_owned().into();
        let _old = std::mem::replace(self, new);
    }
}

impl Score for PublicInfo {
    fn score_interpertation() -> ScoreInterpertation {
        ScoreInterpertation::LowerIsBetter
    }

    fn score(&self) -> Option<PID<i64>> {
        match self {
            PublicInfo::InProgress => None,
            PublicInfo::Completed {
                secret_number,
                guesses,
            } => Some(
                guesses
                    .iter()
                    .map(|(player, Guess(guess))| {
                        let diff = ((*guess as i64) - (*secret_number as i64)).abs();
                        (player, diff)
                    })
                    .collect(),
            ),
        }
    }
}
