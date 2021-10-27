use crate::{
    number_of_players::ONE_PLAYER,
    play::{DebugMsg, DebugMsgs, GameAdvance},
    ActionResponse::{self, *},
    NumberOfPlayers, Play, Player, PlayerSet, View,
};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::collections::HashMap;
use std::ops::RangeInclusive;
use thiserror::Error;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct GuessTheNumber {
    secret_number: u64,
    guesses: Option<Guesses>,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct Guess(u64);

impl From<u64> for Guess {
    fn from(n: u64) -> Self {
        Guess(n)
    }
}

pub type Guesses = SmallVec<[ActionResponse<Guess>; 8]>;

#[derive(Error, Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub enum ActionError {
    #[error("Guess of {:?} is out of range {:?}", guess, range)]
    GuessOutOfRange {
        guess: u64,
        range: RangeInclusive<u64>,
    },
}

use ActionError::*;

#[derive(Builder, Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[builder(derive(Debug), build_fn(validate = "Self::validate"))]
pub struct Settings {
    #[builder(default = "0..=u64::MAX")]
    range: RangeInclusive<u64>,
    #[builder(default = "ONE_PLAYER")]
    num_players: NumberOfPlayers,
}

impl SettingsBuilder {
    fn validate(&self) -> Result<(), String> {
        if let Some(range) = &self.range {
            if range.is_empty() {
                return Err("range must not be empty".into());
            }
        }

        Ok(())
    }
}

impl Settings {
    pub fn range(&self) -> RangeInclusive<u64> {
        self.range.clone()
    }

    pub fn num_players(&self) -> NumberOfPlayers {
        self.num_players
    }
}

impl Default for Settings {
    fn default() -> Self {
        SettingsBuilder::default().build().unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct PublicInfo(Option<GuessTheNumber>);

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct SpectatorUpdate(GuessTheNumber);

impl From<GuessTheNumber> for SpectatorUpdate {
    fn from(game: GuessTheNumber) -> Self {
        Self(game)
    }
}

impl View for PublicInfo {
    type Update = SpectatorUpdate;

    fn update(&mut self, update: &Self::Update) -> Result<(), Box<dyn std::error::Error>> {
        self.0 = Some(update.0.clone());
        Ok(())
    }
}

impl Play for GuessTheNumber {
    type Action = Guess;
    type ActionError = ActionError;
    type PublicInfo = PublicInfo;
    type Settings = Settings;

    fn number_of_players_for_settings(settings: &Self::Settings) -> NumberOfPlayers {
        settings.num_players
    }
    fn player_secret_info(
        &self,
        settings: &Self::Settings,
    ) -> HashMap<Player, Self::PlayerSecretInfo> {
        settings
            .num_players
            .players()
            .map(|player| (player, Default::default()))
            .collect()
    }

    fn public_info(&self, _settings: &Self::Settings) -> Self::PublicInfo {
        let game = self.guesses.is_some().then(|| self.clone());
        PublicInfo(game)
    }

    fn initial_state_for_settings(settings: &Self::Settings, rng: &mut impl rand::Rng) -> Self {
        Self {
            secret_number: rng.gen_range(settings.range.clone()),
            guesses: None,
        }
    }

    fn action_requests(&self, settings: &Self::Settings) -> PlayerSet {
        match self.guesses {
            Some(_) => PlayerSet::empty(),
            None => settings.num_players.player_set(),
        }
    }

    fn advance(
        &self,
        settings: &Self::Settings,
        actions: impl Iterator<Item = (Player, ActionResponse<Self::Action>)>,
        _rng: &mut impl rand::Rng,
    ) -> (Self, GameAdvance<Self>) {
        let mut debug_msgs: DebugMsgs<Self> = Default::default();
        let mut actions_vec = Vec::with_capacity(settings.num_players.get() as usize);

        for action @ (player, response) in actions {
            if let Response(attempted @ Guess(guess)) = response {
                if !settings.range().contains(&guess) {
                    debug_msgs.push((
                        player,
                        DebugMsg {
                            attempted,
                            error: GuessOutOfRange {
                                guess,
                                range: settings.range.clone(),
                            },
                        },
                    ))
                }
            }

            actions_vec.push(action);
        }

        actions_vec.sort_by_key(|(player, _)| *player);
        let guesses: Guesses = actions_vec
            .into_iter()
            .map(|(_, response)| response)
            .collect();

        let new_state = Self {
            guesses: Some(guesses),
            ..self.clone()
        };

        (
            new_state.clone(),
            GameAdvance {
                debug_msgs,
                public_info_update: new_state.into(),
                player_secret_info_updates: Default::default(),
            },
        )
    }
}
