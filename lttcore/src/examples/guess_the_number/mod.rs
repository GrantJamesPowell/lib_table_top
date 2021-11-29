mod public_info;
mod settings;
pub use public_info::{PublicInfo, PublicInfoUpdate};
pub use settings::{Settings, SettingsBuilder, SettingsBuilderError};

use crate::{
    play::{
        view::NoSecretPlayerInfo, ActionResponse, DebugMsgs, GameAdvance, LttVersion, Play, Player,
    },
    utilities::{PlayerIndexedData as PID, PlayerSet},
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::hash::Hash;
use std::ops::RangeInclusive;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct GuessTheNumber {
    secret_number: u64,
    guesses: Option<PID<Guess>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Guess(pub u64);

impl From<u64> for Guess {
    fn from(n: u64) -> Self {
        Guess(n)
    }
}

#[derive(Error, Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub enum ActionError {
    #[error("Guess of {:?} is out of range {:?}", guess, range)]
    GuessOutOfRange {
        guess: u64,
        range: RangeInclusive<u64>,
    },
}

use ActionError::GuessOutOfRange;

impl LttVersion for GuessTheNumber {
    fn lib_table_top_identifier() -> &'static str {
        "GuessTheNumber"
    }
}

impl Play for GuessTheNumber {
    type Action = Guess;
    type ActionError = ActionError;
    type PublicInfo = PublicInfo;
    type Settings = Settings;
    type PlayerSecretInfo = NoSecretPlayerInfo;

    fn player_secret_info(&self, _: &Self::Settings, _: Player) -> Cow<'_, Self::PlayerSecretInfo> {
        Cow::Owned(NoSecretPlayerInfo)
    }

    fn public_info(&self, _settings: &Self::Settings) -> Cow<'_, Self::PublicInfo> {
        Cow::Owned(match self.guesses {
            None => PublicInfo::InProgress,
            Some(ref guesses) => PublicInfo::Completed {
                secret_number: self.secret_number,
                guesses: guesses.clone(),
            },
        })
    }

    fn initial_state_for_settings(settings: &Self::Settings, rng: &mut impl rand::Rng) -> Self {
        Self {
            secret_number: rng.gen_range(settings.range()),
            guesses: None,
        }
    }

    fn which_players_input_needed(&self, settings: &Self::Settings) -> PlayerSet {
        match self.guesses {
            Some(_) => PlayerSet::empty(),
            None => settings.number_of_players().player_set(),
        }
    }

    fn advance<'a>(
        &'a mut self,
        settings: &Self::Settings,
        actions: impl Iterator<Item = (Player, Cow<'a, ActionResponse<Self>>)>,
        _rng: &mut impl rand::Rng,
    ) -> GameAdvance<Self> {
        use ActionResponse::Response;
        let actions: PID<Cow<'a, ActionResponse<Self>>> = actions.collect();

        let debug_msgs: DebugMsgs<Self> = actions
            .iter()
            .filter_map(|(player, response)| {
                if let Response(Guess(guess)) = response.as_ref() {
                    (!settings.range().contains(guess)).then(|| {
                        let err = GuessOutOfRange {
                            guess: *guess,
                            range: settings.range(),
                        };
                        (player, err)
                    })
                } else {
                    None
                }
            })
            .collect();

        let guesses: PID<Guess> = actions
            .into_iter()
            .filter_map(|(player, response)| {
                if let Response(guess) = response.as_ref() {
                    Some((player, *guess))
                } else {
                    None
                }
            })
            .collect();

        self.guesses = Some(guesses.clone());

        GameAdvance {
            debug_msgs,
            next_players_input_needed: self.which_players_input_needed(settings),
            player_secret_info_updates: Default::default(),
            public_info_update: PublicInfoUpdate {
                guesses,
                secret_number: self.secret_number,
            },
        }
    }
}
