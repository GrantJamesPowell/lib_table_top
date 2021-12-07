#![allow(missing_docs)]

mod game_state;
mod player;
mod turn_num;

pub mod number_of_players;
pub mod score;
pub mod seed;
pub mod settings;
pub mod view;

pub use game_state::{EnumeratedGameStateUpdate, GameState, GameStateUpdate};
pub use number_of_players::NumberOfPlayers;
pub use player::Player;
pub use score::Score;
pub use seed::Seed;
pub use settings::SettingsPtr;
pub use turn_num::TurnNum;
pub use view::View;

use crate::utilities::PlayerIndexedData as PID;
use crate::LibTableTopIdentifier;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt::Debug;
use std::hash::Hash;
use std::{borrow::Cow, panic::RefUnwindSafe};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound = "")]
pub enum ActionResponse<T: Play> {
    Response(T::Action),
    Timeout,
    Resign,
}

pub trait Play:
    LibTableTopIdentifier
    + RefUnwindSafe
    + Sized
    + Clone
    + Debug
    + Send
    + Sync
    + PartialEq
    + Eq
    + Hash
    + Serialize
    + DeserializeOwned
    + 'static
{
    type Action: Clone
        + RefUnwindSafe
        + Debug
        + PartialEq
        + Eq
        + Send
        + Sync
        + Serialize
        + DeserializeOwned
        + 'static;

    type ActionError: Clone
        + RefUnwindSafe
        + Debug
        + PartialEq
        + Eq
        + Send
        + Sync
        + Serialize
        + DeserializeOwned
        + 'static;

    type Settings: Clone
        + settings::NumPlayers
        + settings::BuiltinGameModes
        + RefUnwindSafe
        + Debug
        + Default
        + PartialEq
        + Eq
        + Sync
        + Send
        + Serialize
        + DeserializeOwned
        + 'static;

    type PublicInfo: View + Score + RefUnwindSafe;
    type PlayerSecretInfo: View + RefUnwindSafe;
    type GameSecretInfo: View + RefUnwindSafe;

    fn initial_state_for_settings(
        settings: &Self::Settings,
        rng: &mut impl rand::Rng,
    ) -> GameState<Self>;

    fn resolve(
        game_state: &GameState<Self>,
        settings: &Self::Settings,
        actions: PID<Cow<'_, ActionResponse<Self>>>,
        rng: &mut impl rand::Rng,
    ) -> GameStateUpdate<Self>;
}
