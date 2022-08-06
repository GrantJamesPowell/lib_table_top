#![allow(missing_docs)]

mod settings;
mod public_info;
mod player_secret_info;
mod game_secret_info;
pub use settings::Settings;
pub use public_info::{PublicInfo, PublicInfoUpdate};
pub use player_secret_info::{PlayerSecretInfo, PlayerSecretInfoUpdate};
pub use game_secret_info::{GameSecretInfo, GameSecretInfoUpdate};

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::{
    play::{GameState, GameStateUpdate, Play, ActionResponse},
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

    type GameSecretInfo = GameSecretInfo;

    fn initial_state_for_settings(
        _settings: &Self::Settings,
        _rng: &mut impl rand::Rng,
    ) -> GameState<Self> {
        todo!()
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
