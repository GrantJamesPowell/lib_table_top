use super::{ActionResponse, Play, Player, TurnNum, View};
use crate::{
    pov::{observer::ObserverUpdate, player::PlayerUpdate},
    utilities::PlayerIndexedData as PID,
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameState<T: Play> {
    pub player_secret_info: PID<T::PlayerSecretInfo>,
    pub game_secret_info: T::GameSecretInfo,
    pub public_info: T::PublicInfo,
    pub action_requests: Option<PID<T::Phase>>,
}

impl<T: Play> GameState<T> {
    pub fn phase_for_player(&self, player: impl Into<Player>) -> Option<&T::Phase> {
        self.action_requests
            .as_ref()
            .and_then(|pid| pid.get(player.into()))
    }

    pub fn update(&mut self, update: GameStateUpdate<T>) {
        for (player, update) in update.player_secret_info_updates {
            self.player_secret_info[player].update(Cow::Owned(update));
        }

        self.game_secret_info
            .update(Cow::Owned(update.game_secret_info_update));
        self.public_info
            .update(Cow::Owned(update.public_info_update));
        self.action_requests = update.action_requests;
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameStateUpdate<T: Play> {
    /// Changes in the player's secret info from the resolution of the turn. See [`Play::PlayerSecretInfo`]
    pub player_secret_info_updates: PID<<T::PlayerSecretInfo as View>::Update>,
    /// Changes in the game's secret info from the resolution of the turn. See [`Play::GameSecretInfo`]
    pub game_secret_info_update: <T::GameSecretInfo as View>::Update,
    /// Changes in the public info from the resolution of the turn. See [`Play::PublicInfo`]
    pub public_info_update: <T::PublicInfo as View>::Update,
    /// The next players who's input is needed.
    ///
    /// # How to interpret
    ///
    /// * `None` => This game is over and there will be no more input needed *
    /// `Some(player_indexed_data) if player_indexed_data.is_empty()` => this turn requires no
    /// players input but the game is not over
    /// * `Some(player_indexed_data)` => The players who need to provide input this turn
    pub action_requests: Option<PID<T::Phase>>,
    /// Debug info for players who did something potentially incorrect. See [`Play::ActionError`]
    /// for more details
    pub debug_msgs: PID<T::ActionError>,
}

impl<T: Play> GameStateUpdate<T> {
    pub fn phase_for_player(&self, player: impl Into<Player>) -> Option<&T::Phase> {
        self.action_requests
            .as_ref()
            .and_then(|phases| phases.get(player.into()))
    }
}

/// A [`GameStateUpdate`] and [`TurnNum`] combo
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnumeratedGameStateUpdate<T: Play> {
    /// The turn number of the turn that was resolved in this update
    pub(crate) turn_num: TurnNum,
    /// The resolution of the turn
    pub(crate) game_state_update: GameStateUpdate<T>,
    /// Actions taken during the turn
    pub(crate) actions: Option<PID<ActionResponse<T>>>,
}

impl<T: Play> EnumeratedGameStateUpdate<T> {
    pub fn game_state_update(&self) -> &GameStateUpdate<T> {
        &self.game_state_update
    }

    /// Update that advances the [`GameObserver`](crate::pov::observer::GameObserver) state machine
    pub fn observer_update(&self) -> ObserverUpdate<'_, T> {
        ObserverUpdate {
            turn_num: self.turn_num,
            public_info_update: Cow::Borrowed(&self.game_state_update.public_info_update),
        }
    }

    pub fn current_turn_num(&self) -> TurnNum {
        self.turn_num
    }

    pub fn next_turn_num(&self) -> TurnNum {
        self.turn_num.next()
    }

    /// Update that advances the [`GamePlayer`](crate::pov::player::GamePlayer) state machine
    pub fn player_update(&self, player: impl Into<Player>) -> PlayerUpdate<'_, T> {
        let player = player.into();

        let phase = self
            .game_state_update
            .phase_for_player(player)
            .map(Cow::Borrowed);

        let observer_update = self.observer_update();

        let debug_msg = self
            .game_state_update
            .debug_msgs
            .get(player)
            .map(Cow::Borrowed);

        let secret_info_update = self
            .game_state_update
            .player_secret_info_updates
            .get(player)
            .map(Cow::Borrowed);

        PlayerUpdate {
            player,
            phase,
            observer_update,
            secret_info_update,
            debug_msg,
        }
    }
}
