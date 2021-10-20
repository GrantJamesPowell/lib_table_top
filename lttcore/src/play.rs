use crate::{view::NoSecretPlayerInformation, NumberOfPlayers, Player, View};
use smallvec::SmallVec;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NoCustomSettings;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NoCustomActionRequest;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NoCustomSettingsError {}

pub type ActionRequests<T> = SmallVec<[(Player, <T as Play>::ActionRequest); 2]>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugMsg<T: Play> {
    pub attempted_action: <T as Play>::Action,
    pub replaced_action: <T as Play>::Action,
    pub error: <T as Play>::ActionError,
}

pub type DebugMsgs<T> = SmallVec<[(Player, DebugMsg<T>); 2]>;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ActionResponse<T> {
    Response(T),
    Resign,
}

#[derive(Clone, Debug)]
pub struct GameAdvance<T: Play> {
    pub spectator_update: <<T as Play>::SpectatorView as View>::Update,
    pub player_updates: SmallVec<[(Player, <<T as Play>::PlayerView as View>::Update); 2]>,
    pub debug_msgs: DebugMsgs<T>,
}

pub trait Play: Sized + Clone + Debug {
    type Action: Clone + Debug + PartialEq + Eq;
    type ActionError: Clone + Debug + PartialEq + Eq;
    type ActionRequest: Clone + Debug + PartialEq + Eq = NoCustomActionRequest;

    type Settings: Clone + Debug + PartialEq + Eq + Default = NoCustomSettings;
    type SettingsError: Clone + Debug + PartialEq + Eq = NoCustomSettingsError;

    type Status: Clone + Debug + PartialEq + Eq;

    type PlayerView: View = NoSecretPlayerInformation;
    type SpectatorView: View;

    fn number_of_players_for_settings(settings: &Self::Settings) -> NumberOfPlayers;
    fn player_views(&self, settings: &Self::Settings) -> HashMap<Player, Self::PlayerView>;
    fn spectator_view(&self, settings: &Self::Settings) -> Self::SpectatorView;
    fn initial_state_for_settings(settings: &Self::Settings, rng: &mut impl rand::Rng) -> Self;
    fn is_valid_for_settings(&self, settings: &Self::Settings) -> bool;
    fn action_requests(&self, settings: &Self::Settings) -> ActionRequests<Self>;

    fn advance(
        &self,
        settings: &Self::Settings,
        actions: impl Iterator<Item = ((Player, Self::ActionRequest), ActionResponse<Self::Action>)>,
        rng: &mut impl rand::Rng,
    ) -> (Self, GameAdvance<Self>);
}
