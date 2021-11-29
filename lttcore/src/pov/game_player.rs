use crate::play::{Play, Player, TurnNum, View};
use crate::pov::{GameObserver, ObserverPov, PlayerPov, PlayerUpdate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct GamePlayer<T: Play> {
    pub(crate) game_observer: GameObserver<T>,
    pub(crate) player: Player,
    pub(crate) secret_info: T::PlayerSecretInfo,
}

impl<T: Play> GamePlayer<T> {
    pub fn player(&self) -> Player {
        self.player
    }

    pub fn turn_num(&self) -> TurnNum {
        self.game_observer.turn_num
    }

    pub fn player_pov(&self) -> PlayerPov<'_, T> {
        PlayerPov {
            player: self.player,
            secret_info: &self.secret_info,
            turn_num: self.game_observer.turn_num,
            action_requests: self.game_observer.action_requests,
            settings: self.game_observer.settings(),
            public_info: &self.game_observer.public_info,
        }
    }

    pub fn observer_pov(&self) -> ObserverPov<'_, T> {
        self.game_observer.observer_pov()
    }

    pub fn is_player_input_needed(&self) -> bool {
        self.game_observer.action_requests.contains(self.player)
    }

    pub fn update(&mut self, update: PlayerUpdate<'_, T>) {
        self.game_observer.update(update.observer_update);

        if let Some(update) = update.secret_info_update {
            self.secret_info.update(update);
        }
    }
}

use crate::examples::GuessTheNumber;
assert_impl_all!(GamePlayer<GuessTheNumber>: Send);
