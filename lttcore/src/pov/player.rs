use super::observer::{GameObserver, ObserverPov, ObserverUpdate};
use crate::play::{Play, Player, TurnNum, View};
use crate::utilities::PlayerSet;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerPov<'a, T: Play> {
    pub turn_num: TurnNum,
    pub action_requests: PlayerSet,
    pub player: Player,
    pub settings: &'a T::Settings,
    pub secret_info: &'a T::PlayerSecretInfo,
    pub public_info: &'a T::PublicInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(bound = "")]
pub struct PlayerUpdate<'a, T: Play> {
    pub(crate) player: Player,
    pub(crate) observer_update: ObserverUpdate<'a, T>,
    pub(crate) secret_info_update: Option<Cow<'a, <T::PlayerSecretInfo as View>::Update>>,
}

impl<'a, T: Play> PlayerUpdate<'a, T> {
    /// Return the turn num for the player update
    pub fn turn_num(&self) -> TurnNum {
        self.observer_update.turn_num
    }

    /// Return whether a specific player's input is needed this turn
    pub fn is_player_input_needed_this_turn(&self, player: Player) -> bool {
        self.observer_update.action_requests.contains(player)
    }

    /// Change the lifetime to 'static making `PlayerUpdate` function like an owned type
    pub fn into_owned(self) -> PlayerUpdate<'static, T> {
        PlayerUpdate {
            player: self.player,
            observer_update: self.observer_update.into_owned(),
            secret_info_update: self.secret_info_update.map(|x| Cow::Owned(x.into_owned())),
        }
    }
}

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
