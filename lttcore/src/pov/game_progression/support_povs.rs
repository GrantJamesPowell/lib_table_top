use super::GameProgression;
use crate::pov::{observer::GameObserver, player::GamePlayer};
use crate::{
    play::{Play, Player, SettingsPtr},
    pov::{observer::ObserverPov, player::PlayerPov},
};

impl<T: Play> GameProgression<T> {
    pub fn observer_pov(&self) -> ObserverPov<'_, T> {
        ObserverPov {
            turn_num: self.turn_num,
            settings: self.settings(),
            public_info: self.public_info(),
        }
    }

    pub fn player_pov(&self, player: impl Into<Player>) -> PlayerPov<'_, T> {
        let player = player.into();

        PlayerPov {
            player,
            turn_num: self.turn_num,
            settings: self.settings(),
            secret_info: self.player_secret_info(player),
            public_info: self.public_info(),
        }
    }

    pub fn game_observer(&self) -> GameObserver<T> {
        GameObserver {
            turn_num: self.turn_num(),
            settings: SettingsPtr::clone(&self.settings),
            public_info: self.public_info().clone(),
        }
    }

    pub fn game_player(&self, player: impl Into<Player>) -> GamePlayer<T> {
        let player = player.into();
        let game_observer = self.game_observer();
        let secret_info = self.player_secret_info(player).clone();
        let phase = self.game_state.phase_for_player(player).cloned();

        GamePlayer {
            game_observer,
            phase,
            player,
            secret_info,
            debug_msgs: Vec::new(),
        }
    }
}
