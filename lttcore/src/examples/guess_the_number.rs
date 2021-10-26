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
pub struct Action {
    pub guess: u64,
}

type Guesses = SmallVec<[ActionResponse<Action>; 8]>;

#[derive(Error, Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub enum ActionError {
    #[error("Guess of {:?} is out of range {:?}-{:?}", guess, min, max)]
    GuessOutOfRange { min: u64, max: u64, guess: u64 },
}

use ActionError::*;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Settings {
    min: u64,
    max: u64,
    num_players: NumberOfPlayers,
}

impl Settings {
    fn range(&self) -> RangeInclusive<u64> {
        self.min..=self.max
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            min: 0,
            max: u64::MAX,
            num_players: ONE_PLAYER,
        }
    }
}

#[derive(Error, Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum SettingsError {
    #[error("`min` must be less than `max`")]
    MinMustBeLessThanMax,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct SpectatorView(Settings, Option<Guesses>);

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct SpectatorViewUpdate(Guesses);

impl View for SpectatorView {
    type Update = SpectatorViewUpdate;
}

impl Play for GuessTheNumber {
    type Action = Action;
    type ActionError = ActionError;
    type SpectatorView = SpectatorView;
    type Settings = Settings;
    type SettingsError = SettingsError;

    fn number_of_players_for_settings(settings: &Self::Settings) -> NumberOfPlayers {
        settings.num_players
    }
    fn player_views(&self, settings: &Self::Settings) -> HashMap<Player, Self::PlayerView> {
        settings
            .num_players
            .players()
            .map(|player| (player, Default::default()))
            .collect()
    }

    fn spectator_view(&self, settings: &Self::Settings) -> Self::SpectatorView {
        SpectatorView(settings.clone(), self.guesses.clone())
    }

    fn initial_state_for_settings(settings: &Self::Settings, rng: &mut impl rand::Rng) -> Self {
        Self {
            secret_number: rng.gen_range(settings.min..=settings.max),
            guesses: None,
        }
    }

    fn action_requests(&self, settings: &Self::Settings) -> PlayerSet {
        settings.num_players.player_set()
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
            if let Response(attempted @ Action { guess }) = response {
                if !settings.range().contains(&guess) {
                    debug_msgs.push((
                        player,
                        DebugMsg {
                            attempted,
                            error: GuessOutOfRange {
                                guess,
                                min: settings.min,
                                max: settings.max,
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

        (
            Self {
                guesses: Some(guesses.clone()),
                ..self.clone()
            },
            GameAdvance {
                debug_msgs,
                spectator_update: SpectatorViewUpdate(guesses),
                player_updates: Default::default(),
            },
        )
    }
}
