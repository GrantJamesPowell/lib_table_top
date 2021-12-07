#![allow(missing_docs)]

pub mod bot;
mod public_info;
mod settings;

pub use bot::GuessTheNumberBot;
pub use public_info::{PublicInfo, PublicInfoUpdate};
pub use settings::{Settings, SettingsBuilder, SettingsBuilderError};

use crate::{
    play::{view::NoSecretPlayerInfo, ActionResponse, GameState, GameStateUpdate, Play, View},
    utilities::PlayerIndexedData as PID,
    LibTableTopIdentifier,
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::hash::Hash;
use std::ops::RangeInclusive;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct GuessTheNumber;

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

impl LibTableTopIdentifier for GuessTheNumber {
    fn lib_table_top_identifier() -> &'static str {
        "GuessTheNumber"
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct GameSecretInfo {
    secret_number: u64,
}

impl View for GameSecretInfo {
    // The game secret info never changes
    type Update = ();
}

impl Play for GuessTheNumber {
    type Action = Guess;
    type ActionError = ActionError;
    type PublicInfo = PublicInfo;
    type Settings = Settings;
    type PlayerSecretInfo = NoSecretPlayerInfo;
    type GameSecretInfo = GameSecretInfo;

    fn initial_state_for_settings(
        settings: &Self::Settings,
        rng: &mut impl rand::Rng,
    ) -> GameState<Self> {
        let player_secret_info = settings
            .number_of_players()
            .player_indexed_data(|_player| NoSecretPlayerInfo::default());

        GameState {
            player_secret_info,
            public_info: PublicInfo::InProgress,
            game_secret_info: GameSecretInfo {
                secret_number: rng.gen_range(settings.range()),
            },
            action_requests: Some(settings.number_of_players().player_set()),
        }
    }

    fn resolve(
        game_state: &GameState<Self>,
        settings: &Self::Settings,
        actions: PID<Cow<'_, ActionResponse<Self>>>,
        _rng: &mut impl rand::Rng,
    ) -> GameStateUpdate<Self> {
        use ActionResponse::Response;
        let debug_msgs: PID<ActionError> = actions
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

        GameStateUpdate {
            player_secret_info_updates: PID::empty(),
            game_secret_info_update: (),
            public_info_update: PublicInfoUpdate {
                secret_number: game_state.game_secret_info.secret_number,
                guesses,
            },
            action_requests: None,
            debug_msgs,
        }
    }
}
