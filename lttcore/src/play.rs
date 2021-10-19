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

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ActionResponse<T> {
    Response(T),
    Resign,
}

#[derive(Clone, Debug)]
pub enum GameAdvance<T: Play> {
    Unadvanceable {
        request: (Player, <T as Play>::ActionRequest),
        error: <T as Play>::ActionError,
    },
    Advance {
        spectator_update: <<T as Play>::SpectatorView as View>::Update,
        player_updates: SmallVec<[(Player, <<T as Play>::PlayerView as View>::Update); 2]>,
    },
}

pub trait Play: Sized + Clone + Debug {
    type Action: Clone + Debug + PartialEq + Eq;
    type ActionError: Clone + Debug + PartialEq + Eq;
    type ActionRequest: Clone + Debug + PartialEq + Eq = NoCustomActionRequest;

    type Settings: Clone + Debug + PartialEq + Eq + Default = NoCustomSettings;
    type SettingsError: Clone + Debug + PartialEq + Eq = NoCustomSettingsError;

    type PlayerView: View = NoSecretPlayerInformation;
    type SpectatorView: View;

    fn number_of_players_for_settings(settings: &Self::Settings) -> NumberOfPlayers;

    fn player_views(&self, settings: &Self::Settings) -> HashMap<Player, Self::PlayerView> {
        let mut map = HashMap::new();
        self.player_views_into(settings, &mut map);
        map
    }

    fn player_views_into(
        &self,
        settings: &Self::Settings,
        map: &mut HashMap<Player, Self::PlayerView>,
    );

    fn spectator_view(&self, settings: &Self::Settings) -> Self::SpectatorView;

    fn initial_state_for_settings(settings: &Self::Settings, rng: &mut impl rand::Rng) -> Self;

    fn is_valid_for_settings(&self, settings: &Self::Settings) -> bool;

    fn action_requests_into(
        &self,
        settings: &Self::Settings,
        action_requests: &mut Vec<(Player, Self::ActionRequest)>,
    );

    fn action_requests(&self, settings: &Self::Settings) -> Vec<(Player, Self::ActionRequest)> {
        let mut vec = Vec::new();
        self.action_requests_into(settings, &mut vec);

        vec
    }

    fn advance(
        &mut self,
        settings: &Self::Settings,
        actions: impl Iterator<Item = ((Player, Self::ActionRequest), ActionResponse<Self::Action>)>,
        rng: &mut impl rand::Rng,
    ) -> GameAdvance<Self>;
}
