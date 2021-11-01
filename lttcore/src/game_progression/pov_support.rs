use crate::pov::{Observe, ObserverPov, Omniscient, OmniscientPov};
use crate::{GameObserver, GamePlayer, GameProgression, Play};
use std::borrow::Cow;
use std::sync::Arc;

impl<T: Play> Observe<T> for GameProgression<T> {
    fn observer_pov(&self) -> ObserverPov<'_, T> {
        ObserverPov {
            turn_num: self.turn_num,
            action_requests: self.which_players_input_needed(),
            settings: Cow::Borrowed(&self.settings()),
            public_info: Cow::Owned(self.public_info()),
        }
    }
}

impl<T: Play> Omniscient<T> for GameProgression<T> {
    fn omniscient_pov(&self) -> OmniscientPov<'_, T> {
        OmniscientPov {
            game_progression: Cow::Borrowed(&self),
        }
    }
}

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
