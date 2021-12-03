#![warn(missing_docs)]
//! State machines for representing what a [`Player`] would be able to see during a game
//!
//! A "Player" is someone who is directly taking actions on the game to affect the outcome. Players
//! may have secret information. A player has a superset of the imformation that an Observer would
//! have.
//!

use super::observer::{GameObserver, ObserverPov, ObserverUpdate};
use crate::play::{Play, Player, TurnNum, View};
use crate::utilities::PlayerSet;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// A particular [`Player`]'s view into a game on a particular [`TurnNum`]
///
/// This is most commonly interacted with through the [`Bot`](crate::bot::Bot) interface, where
/// each time a bot is called to act, it is given this view into the game
///
/// # When would I use this?
///
/// * You're writing a GUI to interact with a game as a [`Player`] * [`PlayerPov`] is part of the
/// [`crate::bot::Bot`] interface. If you're a game author and are creating a game specific bot
/// wrapper, you would need to use this as the input to your wrapper.  See
/// [`TicTacToeBot`](crate::examples::tic_tac_toe::TicTacToeBot) and
/// [`TicTacToeBotWrapper`](crate::examples::tic_tac_toe::bot::TicTacToeBotWrapper) as an example
/// of the pattern of turning the general [`PlayerPov`] interface into a game specific one
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct PlayerPov<'a, T: Play> {
    /// The current turn number
    pub turn_num: TurnNum,
    /// A [`PlayerSet`] containing the [`Player`]s that need to act during this turn
    pub action_requests: Option<PlayerSet>,
    /// The [`Player`] who this view is for
    pub player: Player,
    /// The [`Settings`](Play::Settings) of the game
    pub settings: &'a T::Settings,
    /// The [`PlayerSecretInfo`](Play::PlayerSecretInfo) for this particular [`Player`] for this
    /// [`TurnNum`]
    pub secret_info: &'a T::PlayerSecretInfo,
    /// The [`PublicInfo`](Play::PublicInfo) for this [`TurnNum`]
    pub public_info: &'a T::PublicInfo,
}

/// An opaque update to the [`GamePlayer`] state machine
///
/// # When would I use this?
///
/// You're using the [`GamePlayer`] state machine and need to send it updates, possible across a
/// network connection or other transport mechanism
///
/// # Note:
///
/// The update may contain referenced data. If you need something with a `'static` lifetime
/// (possibly because you want to persist this update or send it across a network connection) use
/// the [`PlayerUpdate::into_owned`] function
///
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(bound = "")]
pub struct PlayerUpdate<'a, T: Play> {
    pub(crate) player: Player,
    pub(crate) observer_update: ObserverUpdate<'a, T>,
    pub(crate) secret_info_update: Option<Cow<'a, <T::PlayerSecretInfo as View>::Update>>,
    pub(crate) debug_msg: Option<Cow<'a, T::ActionError>>,
}

impl<'a, T: Play> PlayerUpdate<'a, T> {
    /// Return the [`TurnNum`] for the player update
    pub fn turn_num(&self) -> TurnNum {
        self.observer_update.turn_num()
    }

    /// Return whether this player's input is needed this turn
    pub fn is_this_players_input_needed_this_turn(&self) -> bool {
        self.is_player_input_needed_this_turn(self.player)
    }

    /// Return whether a specific player's input is needed this turn
    pub fn is_player_input_needed_this_turn(&self, player: Player) -> bool {
        self.observer_update
            .is_player_input_needed_this_turn(player)
    }

    /// The secret info update for the player the came from the resolution of the previous turn.
    /// May be [`None`] if there is no change to the player secret info
    pub fn secret_info_update(&self) -> Option<&<T::PlayerSecretInfo as View>::Update> {
        self.secret_info_update.as_ref().map(|cow| cow.as_ref())
    }

    /// The public info update that came from the resolution of the previous turn
    pub fn public_info_update(&self) -> &<T::PublicInfo as View>::Update {
        self.observer_update.public_info_update()
    }

    /// Change the lifetime to 'static making `PlayerUpdate` function like an owned type
    ///
    /// This is useful if you need to send the update over the wire or write it to/from a datastore
    pub fn into_owned(self) -> PlayerUpdate<'static, T> {
        PlayerUpdate {
            player: self.player,
            observer_update: self.observer_update.into_owned(),
            secret_info_update: self.secret_info_update.map(|x| Cow::Owned(x.into_owned())),
            debug_msg: self.debug_msg.map(|x| Cow::Owned(x.into_owned())),
        }
    }
}

/// A [state machine](https://en.wikipedia.org/wiki/Finite-state_machine) representing a [`Player`]
/// over the course of a game
///
/// # When would I use this?
///
/// If you're writing a new `LibTableTop` compatible runtime and need a state machine to represent
/// a particular player over the course of the game. It's unlikely you'll interact directly with
/// this as a Game/Bot author
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct GamePlayer<T: Play> {
    pub(crate) game_observer: GameObserver<T>,
    pub(crate) player: Player,
    pub(crate) secret_info: T::PlayerSecretInfo,
    pub(crate) debug_msgs: Vec<T::ActionError>,
}

impl<T: Play> GamePlayer<T> {
    /// Return the [`Player`] that this state machine is for
    pub fn player(&self) -> Player {
        self.player
    }

    /// Return the current [`TurnNum`] from the perspective of the state machine
    pub fn turn_num(&self) -> TurnNum {
        self.game_observer.turn_num
    }

    /// Create a [`PlayerPov`] from this state machine for the current [`TurnNum`]
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

    /// Create a [`ObserverPov`] from this state machine for the current [`TurnNum`]
    pub fn observer_pov(&self) -> ObserverPov<'_, T> {
        self.game_observer.observer_pov()
    }

    /// Return whether this [`Player`] needs to take an action during this turn
    pub fn is_this_players_input_needed(&self) -> bool {
        self.game_observer
            .action_requests
            .map(|player_set| player_set.contains(self.player))
            .unwrap_or(false)
    }

    /// The [`Settings`](Play::Settings) of the game
    pub fn settings(&self) -> &T::Settings {
        self.game_observer.settings()
    }

    /// An iterator over the debug messages a player has received
    pub fn debug_msgs(&self) -> impl Iterator<Item = &T::ActionError> + '_ {
        self.debug_msgs.iter()
    }

    /// Apply an [`PlayerUpdate`] to this state machine, advancing the state machine
    ///
    /// # Panics
    ///
    /// This function will panic if an update is skipped or applied twice
    pub fn update(&mut self, update: PlayerUpdate<'_, T>) {
        self.game_observer.update(update.observer_update);

        if let Some(update) = update.secret_info_update {
            self.secret_info.update(update);
        }
    }
}
