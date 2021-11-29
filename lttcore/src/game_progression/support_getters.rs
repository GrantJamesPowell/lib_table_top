use crate::play::{settings::NumPlayers, NumberOfPlayers, TurnNum};
use crate::{GameProgression, Play, Player, PlayerSet};
use std::borrow::Cow;

use super::HistoryEvent;

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
        self.settings.settings()
    }

    pub fn history_events(&self) -> impl Iterator<Item = &HistoryEvent<T>> + '_ {
        self.history.iter()
    }

    pub fn public_info(&self) -> Cow<'_, T::PublicInfo> {
        self.state.public_info(self.settings())
    }

    pub fn player_secret_info(&self, player: Player) -> Cow<'_, T::PlayerSecretInfo> {
        self.state.player_secret_info(self.settings(), player)
    }

    pub fn number_of_players(&self) -> NumberOfPlayers {
        self.settings().number_of_players()
    }

    pub fn players(&self) -> PlayerSet {
        self.number_of_players().player_set()
    }

    pub fn which_players_input_needed(&self) -> PlayerSet {
        self.state.which_players_input_needed(self.settings())
    }
}
