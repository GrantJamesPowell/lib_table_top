use super::{DebugMsgs, Play, PlayerSecretInfoUpdates, TurnNum, View};
use crate::pov::{ObserverUpdate, PlayerUpdate};
use crate::{play::Player, utilities::PlayerSet};
use std::borrow::Cow;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameAdvance<T: Play> {
    pub next_players_input_needed: PlayerSet,
    pub public_info_update: <<T as Play>::PublicInfo as View>::Update,
    pub player_secret_info_updates: PlayerSecretInfoUpdates<T>,
    pub debug_msgs: DebugMsgs<T>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumeratedGameAdvance<T: Play> {
    pub turn_num: TurnNum,
    pub game_advance: GameAdvance<T>,
}

impl<T: Play> EnumeratedGameAdvance<T> {
    pub fn observer_update(&self) -> ObserverUpdate<'_, T> {
        ObserverUpdate {
            turn_num: self.turn_num,
            action_requests: self.game_advance.next_players_input_needed,
            public_info_update: Cow::Borrowed(&self.game_advance.public_info_update),
        }
    }

    pub fn player_update(&self, player: impl Into<Player>) -> PlayerUpdate<'_, T> {
        let player = player.into();
        let observer_update = self.observer_update();
        let secret_info_update = self
            .game_advance
            .player_secret_info_updates
            .iter()
            .find(|(p, _)| *p == player)
            .map(|(_, update)| Cow::Borrowed(update));

        PlayerUpdate {
            player,
            observer_update,
            secret_info_update,
        }
    }
}
