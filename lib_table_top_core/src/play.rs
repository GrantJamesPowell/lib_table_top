use crate::{view::NoSecretPlayerInformation, Player, View};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ActionResponse<T> {
    Response(T),
    Resign,
}

#[derive(Clone, Debug, Default)]
pub struct GameAdvance<ActionRequest, ActionError, PlayerViewUpdate, SpectatorViewUpdate> {
    pub spectator_view_updates: Vec<SpectatorViewUpdate>,
    pub player_view_updates: HashMap<Player, PlayerViewUpdate>,
    pub action_errors: HashMap<ActionRequest, ActionError>,
}

impl<A, B, C, D> GameAdvance<A, B, C, D> {
    pub fn reset(&mut self) {
        self.spectator_view_updates.clear();
        self.player_view_updates.clear();
        self.action_errors.clear()
    }
}

pub trait Play: Sized + Clone {
    type Action;
    type ActionError;
    type ActionRequest;

    type Settings;
    type SettingsError;

    type PlayerView: View = NoSecretPlayerInformation;
    type SpectatorView: View;

    fn player_view(&self) -> Self::PlayerView;
    fn spectator_view(&self) -> Self::SpectatorView;

    fn initial_state_for_settings(settings: &Self::Settings) -> Self;

    fn action_requests(
        &self,
        settings: &Self::Settings,
        action_requests: &mut Vec<Self::ActionRequest>,
    );

    fn advance(
        &mut self,
        settings: &Self::Settings,
        actions: &[(Self::ActionRequest, ActionResponse<Self::Action>)],
        rng: &mut impl rand::Rng,
        game_advance: &mut GameAdvance<
            Self::ActionRequest,
            Self::ActionError,
            <Self::PlayerView as View>::Update,
            <Self::SpectatorView as View>::Update,
        >,
    );
}
