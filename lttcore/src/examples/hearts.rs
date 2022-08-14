#![allow(missing_docs)]
#![allow(dead_code)]

mod player_secret_info;
mod public_info;
mod settings;
mod trick;
pub use player_secret_info::{PlayerSecretInfo, PlayerSecretInfoUpdate};
pub use public_info::{PublicInfo, PublicInfoUpdate};
use rand::seq::SliceRandom;
pub use settings::Settings;
pub use trick::{Round, Trick};

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::{
    common::deck::STANDARD_DECK,
    play::{
        settings::NumPlayers, view::NoSecretGameInfo, ActionResponse, GameState, GameStateUpdate,
        Play,
    },
    player_set,
    utilities::PlayerIndexedData as PID,
    LibTableTopIdentifier,
};

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hearts;

impl LibTableTopIdentifier for Hearts {
    fn lib_table_top_identifier() -> &'static str {
        "Hearts"
    }
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Action;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionError;

impl Play for Hearts {
    type Action = Action;

    type ActionError = ActionError;

    type Settings = Settings;

    type PublicInfo = PublicInfo;

    type PlayerSecretInfo = PlayerSecretInfo;

    type GameSecretInfo = NoSecretGameInfo;

    fn initial_state_for_settings(
        settings: &Self::Settings,
        rng: &mut impl rand::Rng,
    ) -> GameState<Self> {
        let mut deck = STANDARD_DECK.clone();
        deck.shuffle(rng);
        // let mut dealer = deck.chunks_exact(deck.len() / (usize::from(settings.number_of_players())));

        let player_secret_info = settings
            .number_of_players()
            .player_indexed_data(|_player| PlayerSecretInfo {});

        GameState {
            player_secret_info,
            game_secret_info: NoSecretGameInfo,
            public_info: PublicInfo::default(),
            action_requests: Some(player_set![]),
        }
    }

    fn resolve(
        _game_state: &GameState<Self>,
        _settings: &Self::Settings,
        _actions: Cow<'_, PID<ActionResponse<Self>>>,
        _rng: &mut impl rand::Rng,
    ) -> GameStateUpdate<Self> {
        todo!()
    }
}
