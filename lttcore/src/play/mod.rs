#![allow(missing_docs)]

mod game_advance;
mod player;
mod turn_num;

pub mod number_of_players;
pub mod score;
pub mod seed;
pub mod settings;
pub mod view;

pub use game_advance::{EnumeratedGameAdvance, GameAdvance};
pub use number_of_players::NumberOfPlayers;
pub use player::Player;
pub use score::Score;
pub use seed::Seed;
pub use settings::SettingsPtr;
pub use turn_num::TurnNum;
pub use view::View;

use crate::utilities::PlayerSet;
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

    type PublicInfo: Score + View + RefUnwindSafe;
    type PlayerSecretInfo: View + RefUnwindSafe;

    fn player_secret_info(
        &self,
        settings: &Self::Settings,
        player: Player,
    ) -> Cow<'_, Self::PlayerSecretInfo>;
    fn public_info(&self, settings: &Self::Settings) -> Cow<'_, Self::PublicInfo>;
    fn initial_state_for_settings(settings: &Self::Settings, rng: &mut impl rand::Rng) -> Self;
    fn which_players_input_needed(&self, settings: &Self::Settings) -> Option<PlayerSet>;

    fn advance<'a>(
        &'a mut self,
        settings: &Self::Settings,
        actions: impl Iterator<Item = (Player, Cow<'a, ActionResponse<Self>>)>,
        rng: &mut impl rand::Rng,
    ) -> GameAdvance<Self>;

    fn player_should_act(&self, player: Player, settings: &Self::Settings) -> bool {
        self.which_players_input_needed(settings)
            .map(|player_set| player_set.contains(player))
            .unwrap_or(false)
    }
}
