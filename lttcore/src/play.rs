use crate::{view::NoSecretPlayerInfo, NumberOfPlayers, Player, PlayerSet, TurnNum, View};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use smallvec::SmallVec;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct NoCustomSettings;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugMsg<T: Play> {
    pub attempted: <T as Play>::Action,
    pub error: <T as Play>::ActionError,
}

pub type Actions<T> = SmallVec<[(Player, ActionResponse<<T as Play>::Action>); 2]>;
pub type PlayerSecretInfos<T> = HashMap<Player, <T as Play>::PlayerSecretInfo>;
pub type DebugMsgs<T> = SmallVec<[(Player, DebugMsg<T>); 2]>;
pub type PlayerSecretInfoUpdates<T> =
    SmallVec<[(Player, <<T as Play>::PlayerSecretInfo as View>::Update); 2]>;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionResponse<T> {
    Response(T),
    Resign,
}

impl<T> From<T> for ActionResponse<T> {
    fn from(t: T) -> Self {
        Self::Response(t)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameAdvance<T: Play> {
    pub next_players_input_needed: PlayerSet,
    pub public_info_update: <<T as Play>::PublicInfo as View>::Update,
    pub player_secret_info_updates: PlayerSecretInfoUpdates<T>,
    pub debug_msgs: DebugMsgs<T>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumeratedGameAdvance<T: Play> {
    pub turn_num: TurnNum,
    pub game_advance: GameAdvance<T>,
}

pub trait Play: Sized + Clone + Debug + Serialize + DeserializeOwned {
    type Action: Clone + Debug + PartialEq + Eq + Serialize + DeserializeOwned;
    type ActionError: Clone + Debug + PartialEq + Eq + Serialize + DeserializeOwned;

    type Settings: Clone + Debug + PartialEq + Eq + Default + Serialize + DeserializeOwned =
        NoCustomSettings;

    type PublicInfo: View;
    type PlayerSecretInfo: View = NoSecretPlayerInfo;

    fn number_of_players_for_settings(settings: &Self::Settings) -> NumberOfPlayers;
    fn player_secret_info(
        &self,
        settings: &Self::Settings,
    ) -> HashMap<Player, Self::PlayerSecretInfo>;
    fn public_info(&self, settings: &Self::Settings) -> Self::PublicInfo;
    fn initial_state_for_settings(settings: &Self::Settings, rng: &mut impl rand::Rng) -> Self;
    fn which_players_input_needed(&self, settings: &Self::Settings) -> PlayerSet;

    fn advance(
        &self,
        settings: &Self::Settings,
        actions: impl Iterator<Item = (Player, ActionResponse<Self::Action>)>,
        rng: &mut impl rand::Rng,
    ) -> (Self, GameAdvance<Self>);
}
