use super::{game_advance::GameAdvance, view::NoSecretPlayerInfo, LttSettings, View};
use crate::{Player, PlayerSet};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use smallvec::SmallVec;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Debug;

pub type Actions<T> = SmallVec<[(Player, ActionResponse<<T as Play>::Action>); 2]>;
pub type PlayerSecretInfos<T> = HashMap<Player, <T as Play>::PlayerSecretInfo>;
pub type DebugMsgs<T> = SmallVec<[(Player, <T as Play>::ActionError); 2]>;
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

pub trait Play: Sized + Clone + Debug + Serialize + DeserializeOwned {
    type Action: Clone + Debug + PartialEq + Eq + Serialize + DeserializeOwned;
    type ActionError: Clone + Debug + PartialEq + Eq + Serialize + DeserializeOwned;
    type Settings: LttSettings;
    type PublicInfo: View;
    type PlayerSecretInfo: View = NoSecretPlayerInfo;

    fn player_secret_info(&self, settings: &Self::Settings) -> PlayerSecretInfos<Self>;
    fn public_info(&self, settings: &Self::Settings) -> Self::PublicInfo;
    fn initial_state_for_settings(settings: &Self::Settings, rng: &mut impl rand::Rng) -> Self;
    fn which_players_input_needed(&self, settings: &Self::Settings) -> PlayerSet;

    fn advance<'a>(
        &'a mut self,
        settings: &Self::Settings,
        actions: impl Iterator<Item = (Player, Cow<'a, ActionResponse<Self::Action>>)>,
        rng: &mut impl rand::Rng,
    ) -> GameAdvance<Self>;
}
