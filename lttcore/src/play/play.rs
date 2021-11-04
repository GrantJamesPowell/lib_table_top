use super::{
    game_advance::GameAdvance, settings::NoCustomSettings, view::NoSecretPlayerInfo, View,
};
use crate::{utilities::PlayerIndexedData, NumberOfPlayers, Player, PlayerSet};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::Debug;

pub type Actions<T> = PlayerIndexedData<ActionResponse<T>>;
pub type PlayerSecretInfos<T> = PlayerIndexedData<<T as Play>::PlayerSecretInfo>;
pub type DebugMsgs<T> = PlayerIndexedData<<T as Play>::ActionError>;
pub type PlayerSecretInfoUpdates<T> =
    PlayerIndexedData<<<T as Play>::PlayerSecretInfo as View>::Update>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound = "")]
pub enum ActionResponse<T: Play> {
    Response(<T as Play>::Action),
    Resign,
}

pub trait Play: Sized + Clone + Debug + Send + Serialize + DeserializeOwned {
    type Action: Clone + Debug + PartialEq + Eq + Send + Serialize + DeserializeOwned;
    type ActionError: Clone + Debug + PartialEq + Eq + Send + Serialize + DeserializeOwned;

    type Settings: Clone + Debug + PartialEq + Eq + Default + Send + Serialize + DeserializeOwned =
        NoCustomSettings;

    type PublicInfo: View + Send;
    type PlayerSecretInfo: View + Send = NoSecretPlayerInfo;

    fn number_of_players_for_settings(settings: &Self::Settings) -> NumberOfPlayers;
    fn player_secret_info(&self, settings: &Self::Settings) -> PlayerSecretInfos<Self>;
    fn public_info(&self, settings: &Self::Settings) -> Self::PublicInfo;
    fn initial_state_for_settings(settings: &Self::Settings, rng: &mut impl rand::Rng) -> Self;
    fn which_players_input_needed(&self, settings: &Self::Settings) -> PlayerSet;

    fn advance<'a>(
        &'a mut self,
        settings: &Self::Settings,
        actions: impl Iterator<Item = (Player, Cow<'a, ActionResponse<Self>>)>,
        rng: &mut impl rand::Rng,
    ) -> GameAdvance<Self>;
}
