use crate::{GameObserver, GamePlayer, GameProgression, Play, Player};
use std::sync::Arc;

impl<T: Play> GameProgression<T> {
    pub fn game_observer(&self) -> GameObserver<T> {
        GameObserver {
            turn_num: self.turn_num(),
            action_requests: self.which_players_input_needed(),
            settings: Arc::clone(&self.settings),
            public_info: self.public_info().into_owned(),
        }
    }

    pub fn game_player(&self, player: impl Into<Player>) -> GamePlayer<T> {
        let player = player.into();
        let game_observer = self.game_observer();
        let secret_info = self.player_secret_info(player).into_owned();
        GamePlayer {
            player,
            secret_info,
            game_observer,
        }
    }
}
