use crate::{GameObserver, GamePlayer, GameProgression, Play};
use std::sync::Arc;

impl<T: Play> GameProgression<T> {
    pub fn game_observer(&self) -> GameObserver<T> {
        GameObserver {
            turn_num: self.turn_num(),
            action_requests: self.which_players_input_needed(),
            settings: Arc::clone(&self.settings),
            public_info: self.public_info(),
        }
    }

    pub fn game_players(&self) -> impl Iterator<Item = GamePlayer<T>> + '_ {
        let mut player_secret_info = self.player_secret_info();

        self.players().into_iter().map(move |player| GamePlayer {
            player,
            turn_num: self.turn_num,
            action_requests: self.which_players_input_needed(),
            settings: Arc::clone(&self.settings),
            public_info: self.public_info(),
            secret_info: player_secret_info
                .remove(&player)
                .expect("game progression did not return secret info for a player"),
        })
    }
}
