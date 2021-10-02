#![allow(dead_code)]
#![feature(never_type)]
#![feature(associated_type_defaults)]

use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use std::sync::Arc;

mod player;
pub use player::Player;

pub trait View {
    type Update;

    fn update(&mut self, _update: Self::Update) {}
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct NoSecretPlayerInformation {}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct NoSecretPlayerInformationUpdate {}

impl View for NoSecretPlayerInformation {
    type Update = NoSecretPlayerInformationUpdate;
}

pub trait Play: Sized + Clone {
    type Action;
    type ActionError;

    type Settings;
    type SettingsError;

    type PlayerView: View = NoSecretPlayerInformation;
    type SpectatorView: View;

    fn advance(
        &mut self,
        settings: &Self::Settings,
        actions: &[(Player, Self::Action)],
        rng: &mut impl rand::Rng,
    ) -> Result<<<Self as Play>::SpectatorView as View>::Update, Self::ActionError>;

    fn action_requests(&self, settings: &Self::Settings) -> Vec<Player>;
    fn player_view(&self) -> Self::PlayerView;
    fn spectator_view(&self) -> Self::SpectatorView;

    fn initial_state_for_settings(settings: &Self::Settings) -> Self;
}

struct GameRunner<T>
where
    T: Play,
{
    state: Arc<T>,
    seed: Arc<[u8; 32]>,
    initial_state: Option<Arc<T>>,
    settings: Arc<<T as Play>::Settings>,
    history: Vec<<T as Play>::Action>,
}

impl<T: Play> GameRunner<T> {
    fn advance_mut(&mut self, actions: &[(Player, <T as Play>::Action)]) -> ! {
        let new_state = Arc::make_mut(&mut self.state);

        let mut rng = ChaCha20Rng::from_seed(*self.seed);
        let stream_num = self.history.len().try_into().unwrap();
        rng.set_stream(stream_num);

        let _ = new_state.advance(&self.settings, actions, &mut rng);

        todo!()
    }
}
