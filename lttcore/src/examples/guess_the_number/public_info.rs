use super::Guess;
use crate::play::{Score, View};
use crate::utilities::PlayerIndexedData as PID;
use crate::Player;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum PublicInfo {
    InProgress,
    Completed {
        secret_number: u64,
        guesses: PID<Guess>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct PublicInfoUpdate {
    pub secret_number: u64,
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
    fn score(&self) -> Cow<'_, Option<PID<u64>>> {
        match self {
            PublicInfo::InProgress => Cow::Owned(None),
            PublicInfo::Completed {
                secret_number,
                guesses,
            } => {
                let scores = guesses
                    .iter()
                    .map(|(player, guess)| {
                        let diff = guess
                            .0
                            .checked_sub(*secret_number)
                            .or_else(|| secret_number.checked_sub(guess.0))
                            .unwrap();

                        (player, diff)
                    })
                    .collect();

                Cow::Owned(Some(scores))
            }
        }
    }

    fn rank(&self) -> Cow<'_, Option<SmallVec<[SmallVec<[Player; 2]>; 4]>>> {
        let rank = self.score().into_owned().map(|scores| {
            let mut scores: SmallVec<[(Player, u64); 4]> = scores.into_iter().collect();
            scores.sort_by_key(|(_player, score)| *score);
            scores
                .iter()
                .group_by(|(_player, score)| score)
                .into_iter()
                .map(|(_score, group)| {
                    group
                        .map(|(player, _score)| *player)
                        .collect::<SmallVec<[Player; 2]>>()
                })
                .collect()
        });

        Cow::Owned(rank)
    }
}
