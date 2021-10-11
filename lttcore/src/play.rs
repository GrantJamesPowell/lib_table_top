use crate::{view::NoSecretPlayerInformation, Player, View};
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NoCustomSettings {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NoCustomSettingsError {}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ActionResponse<T> {
    Response(T),
    Resign,
}

#[derive(Clone, Debug)]
pub struct GameAdvance<T: Play> {
    pub spectator_view_updates: Vec<<<T as Play>::SpectatorView as View>::Update>,
    pub player_view_updates: Vec<(Player, <<T as Play>::PlayerView as View>::Update)>,
    pub action_errors: Vec<(
        Player,
        (<T as Play>::ActionRequest, <T as Play>::ActionError),
    )>,
}

impl<T: Play> GameAdvance<T> {
    pub fn reset(&mut self) {
        self.spectator_view_updates.clear();
        self.player_view_updates.clear();
        self.action_errors.clear();
    }
}

impl<T: Play> Default for GameAdvance<T> {
    fn default() -> Self {
        Self {
            spectator_view_updates: Vec::new(),
            player_view_updates: Vec::new(),
            action_errors: Vec::new(),
        }
    }
}

pub trait Play: Sized + Clone + Debug {
    type Action: Clone + Debug + PartialEq + Eq;
    type ActionError: Clone + Debug + PartialEq + Eq;
    type ActionRequest: Clone + Debug + PartialEq + Eq;
    type ActionResponse: Clone + Debug = ActionResponse<Self::Action>;

    type Settings: Clone + Debug + PartialEq + Eq = NoCustomSettings;
    type SettingsError: Clone + Debug + PartialEq + Eq = NoCustomSettingsError;

    type PlayerView: View = NoSecretPlayerInformation;
    type SpectatorView: View;

    fn number_of_players_for_settings(settings: &Self::Settings) -> u8;

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

    fn initial_state_for_settings(settings: &Self::Settings) -> Self;

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
        actions: impl Iterator<Item = ((Player, Self::ActionRequest), Self::ActionResponse)>,
        rng: &mut impl rand::Rng,
        game_advance: &mut GameAdvance<Self>,
    );
}
