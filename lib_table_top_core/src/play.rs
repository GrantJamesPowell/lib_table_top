use crate::{view::NoSecretPlayerInformation, View};

pub enum ActionResponse<T> {
    Response(T),
    Resign,
}

pub trait Play: Sized + Clone {
    type Action;
    type ActionError;
    type ActionRequest;

    type Settings;
    type SettingsError;

    type PlayerView: View = NoSecretPlayerInformation;
    type SpectatorView: View;

    fn advance(
        &mut self,
        settings: &Self::Settings,
        actions: &[(Self::ActionRequest, ActionResponse<Self::Action>)],
        rng: &mut impl rand::Rng,
    ) -> Result<<<Self as Play>::SpectatorView as View>::Update, Self::ActionError>;

    fn action_requests(
        &self,
        settings: &Self::Settings,
        action_requests: &mut Vec<Self::ActionRequest>,
    );
    fn player_view(&self) -> Self::PlayerView;
    fn spectator_view(&self) -> Self::SpectatorView;

    fn initial_state_for_settings(settings: &Self::Settings) -> Self;
}
