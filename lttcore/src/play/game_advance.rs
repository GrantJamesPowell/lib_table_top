use super::{Play, TurnNum, View};
use crate::pov::{observer::ObserverUpdate, player::PlayerUpdate};
use crate::{
    play::Player,
    utilities::{PlayerIndexedData as PID, PlayerSet},
};
use std::borrow::Cow;

/// The resolution of a turn
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameAdvance<T: Play> {
    /// The next players who's input is needed.
    ///
    /// # How to interpret
    ///
    /// * `None` => This game is over and there will be no more input needed
    /// * `Some(player_set) if player_set.is_empty()` => this turn requires no players input but
    /// the game is not over
    /// * `Some(player_set)` => The players who need to provide input this turn
    pub next_players_input_needed: Option<PlayerSet>,
    /// Changes in the public info from the resolution of the turn. See [`Play::PublicInfo`]
    pub public_info_update: <T::PublicInfo as View>::Update,
    /// Changes in the player's secret info from the resolution of the turn. See [`Play::PlayerSecretInfo`]
    pub player_secret_info_updates: PID<<T::PlayerSecretInfo as View>::Update>,
    /// Debug info for players who did something potentially incorrect. See [`Play::ActionError`]
    /// for more details
    pub debug_msgs: PID<T::ActionError>,
}

/// A [`GameAdvance`] and [`TurnNum`] combo
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumeratedGameAdvance<T: Play> {
    /// The turn number of the turn that was resolved in this update
    pub turn_num: TurnNum,
    /// The resolution of the turn
    pub game_advance: GameAdvance<T>,
}

impl<T: Play> EnumeratedGameAdvance<T> {
    /// Update that advances the [`GameObserver`](crate::pov::observer::GameObserver) state machine
    pub fn observer_update(&self) -> ObserverUpdate<'_, T> {
        ObserverUpdate {
            turn_num: self.turn_num,
            action_requests: self.game_advance.next_players_input_needed,
            public_info_update: Cow::Borrowed(&self.game_advance.public_info_update),
        }
    }

    /// Update that advances the [`GamePlayer`](crate::pov::player::GamePlayer) state machine
    pub fn player_update(&self, player: impl Into<Player>) -> PlayerUpdate<'_, T> {
        let player = player.into();
        let observer_update = self.observer_update();
        let debug_msg = self.game_advance.debug_msgs.get(player).map(Cow::Borrowed);
        let secret_info_update = self
            .game_advance
            .player_secret_info_updates
            .get(player)
            .map(Cow::Borrowed);

        PlayerUpdate {
            player,
            observer_update,
            secret_info_update,
            debug_msg,
        }
    }
}
