use crate::play::PlayerSecretInfos;
use crate::{GameProgression, NumberOfPlayers, Play, PlayerSet, TurnNum};
use std::sync::Arc;

impl<T: Play> GameProgression<T> {
    pub fn is_concluded(&self) -> bool {
        self.which_players_input_needed().is_empty()
    }

    pub fn turn_num(&self) -> TurnNum {
        self.turn_num
    }

    pub fn state(&self) -> &T {
        &self.state
    }

    pub fn settings(&self) -> &T::Settings {
        &self.settings
    }

    pub fn settings_arc(&self) -> Arc<T::Settings> {
        Arc::clone(&self.settings)
    }

    pub fn public_info(&self) -> T::PublicInfo {
        self.state.public_info(&self.settings)
    }

    pub fn player_secret_info(&self) -> PlayerSecretInfos<T> {
        self.state.player_secret_info(&self.settings)
    }

    pub fn number_of_players(&self) -> NumberOfPlayers {
        T::number_of_players_for_settings(&self.settings)
    }

    pub fn players(&self) -> PlayerSet {
        self.number_of_players().player_set()
    }

    pub fn which_players_input_needed(&self) -> PlayerSet {
        self.state.which_players_input_needed(&self.settings)
    }
}
