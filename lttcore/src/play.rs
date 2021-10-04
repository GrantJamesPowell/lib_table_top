use crate::{view::NoSecretPlayerInformation, Player, View};
use std::collections::HashMap;
use std::fmt::Debug;
// use std::hash::Hash;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ActionResponse<T> {
    Response(T),
    Resign,
}

#[derive(Clone, Debug)]
pub struct GameAdvance<ActionRequest, ActionError, PlayerViewUpdate, SpectatorViewUpdate> {
    pub spectator_view_updates: Vec<SpectatorViewUpdate>,
    pub player_view_updates: HashMap<Player, PlayerViewUpdate>,
    pub action_errors: HashMap<ActionRequest, ActionError>,
}

pub mod game_advance {
    pub trait Reset {
        fn reset(&mut self);
    }
}

// This feels like it should be what happens if you derive(Default) on `GameAdvance`
// Deriving default on Game Advance doesn't currently work though :(
impl<A, B, C, D> Default for GameAdvance<A, B, C, D> {
    fn default() -> Self {
        Self {
            spectator_view_updates: Vec::new(),
            player_view_updates: HashMap::new(),
            action_errors: HashMap::new(),
        }
    }
}

impl<A, B, C, D> game_advance::Reset for GameAdvance<A, B, C, D> {
    fn reset(&mut self) {
        self.spectator_view_updates.clear();
        self.player_view_updates.clear();
        self.action_errors.clear()
    }
}

pub trait Play: Sized + Clone + Debug {
    type Action: Clone + Debug + PartialEq + Eq;
    type ActionError: Clone + Debug + PartialEq + Eq;
    type ActionRequest: Clone + Debug + PartialEq + Eq;
    type ActionResponse = ActionResponse<Self::Action>;

    type Settings: Clone + Debug + PartialEq + Eq;
    type SettingsError: Clone + Debug + PartialEq + Eq;

    type PlayerView: View = NoSecretPlayerInformation;
    type SpectatorView: View;

    type GameAdvance: Clone + Debug + Default + game_advance::Reset = GameAdvance<
        Self::ActionRequest,
        Self::ActionError,
        <Self::PlayerView as View>::Update,
        <Self::SpectatorView as View>::Update,
    >;

    fn player_view(&self) -> Self::PlayerView;
    fn spectator_view(&self) -> Self::SpectatorView;

    fn initial_state_for_settings(settings: &Self::Settings) -> Self;

    fn is_valid_for_settings(&self, settings: &Self::Settings) -> bool;

    fn action_requests(
        &self,
        settings: &Self::Settings,
        action_requests: &mut Vec<(Player, Self::ActionRequest)>,
    );

    fn advance(
        &mut self,
        settings: &Self::Settings,
        actions: &[((Player, Self::ActionRequest), Self::ActionResponse)],
        rng: &mut impl rand::Rng,
        game_advance: &mut Self::GameAdvance,
    );
}
