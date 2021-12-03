//! State machines for representing whan an Observer would see during a game
//!
//! An "Observer" is what someone casually watching a game without playing would to see.  They have
//! no secret information and are never called upon to interact with the game directly

use crate::play::{Play, SettingsPtr, TurnNum, View, Player};
use crate::utilities::PlayerSet;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// A particular observer's view into a game on a particular [`TurnNum`]
///
/// # When would I use this?
///
/// * You're writing a GUI to watch a game as an Observer
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObserverPov<'a, T: Play> {
    /// The current turn number
    pub turn_num: TurnNum,
    /// A [`PlayerSet`] containing the [`Player`](crate::play::Player)s that need to act during this turn
    pub action_requests: PlayerSet,
    /// The [`Settings`](Play::Settings) of the game
    pub settings: &'a T::Settings,
    /// The [`PublicInfo`](Play::PublicInfo) for this [`TurnNum`]
    pub public_info: &'a T::PublicInfo,
}

/// An opaque update to the [`GameObserver`] state machine
///
/// # When would I use this?
///
/// You're using the [`GameObserver`] state machine and need to send it updates, possible across a
/// network connection or other transport mechanism
///
/// # Note:
///
/// The update may contain referenced data. If you need something with a `'static` lifetime
/// (possibly because you want to persist this update or send it across a network connection) use
/// the [`ObserverUpdate::into_owned`] function
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(bound = "")]
pub struct ObserverUpdate<'a, T: Play> {
    pub(crate) turn_num: TurnNum,
    pub(crate) action_requests: PlayerSet,
    pub(crate) public_info_update: Cow<'a, <<T as Play>::PublicInfo as View>::Update>,
}

impl<'a, T: Play> ObserverUpdate<'a, T> {
    /// Return the [`TurnNum`] for the observer update
    pub fn turn_num(&self) -> TurnNum {
        self.turn_num
    }

    /// The public info update that came from the resolution of the previous turn
    pub fn public_info_update(&self) -> &<T::PublicInfo as View>::Update {
        self.public_info_update.as_ref()
    }

    /// Return whether a specific player's input is needed this turn
    pub fn is_player_input_needed_this_turn(&self, player: Player) -> bool {
        self.action_requests.contains(player)
    }

    /// Change the lifetime to 'static making `ObserverUpdate` function like an owned type
    pub fn into_owned(self) -> ObserverUpdate<'static, T> {
        ObserverUpdate {
            turn_num: self.turn_num,
            public_info_update: Cow::Owned(self.public_info_update.into_owned()),
            action_requests: self.action_requests,
        }
    }
}

/// A [state machine](https://en.wikipedia.org/wiki/Finite-state_machine) representing an observer
/// over the course of a game
///
/// # When would I use this?
///
/// If you're writing a new `LibTableTop` compatible runtime and need a state machine to represent
/// an observer over the course of the game. It's unlikely you'll interact directly with this as a
/// Game/Bot author
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct GameObserver<T: Play> {
    pub(crate) turn_num: TurnNum,
    pub(crate) action_requests: PlayerSet,
    pub(crate) settings: SettingsPtr<T::Settings>,
    pub(crate) public_info: T::PublicInfo,
}

impl<T: Play> GameObserver<T> {
    /// Return the [`TurnNum`] for the player update
    pub fn turn_num(&self) -> TurnNum {
        self.turn_num
    }

    /// The [`Settings`](Play::Settings) of the game
    pub fn settings(&self) -> &T::Settings {
        &self.settings
    }
}

impl<T: Play> GameObserver<T> {
    /// Return the [`ObserverPov`] for the current [`TurnNum`]
    pub fn observer_pov(&self) -> ObserverPov<'_, T> {
        ObserverPov {
            turn_num: self.turn_num,
            action_requests: self.action_requests,
            settings: self.settings(),
            public_info: &self.public_info,
        }
    }

    /// Apply an [`ObserverUpdate`] to this state machine, advancing the state machine
    ///
    /// # Panics
    ///
    /// This function will panic if an update is skipped or applied twice
    pub fn update(&mut self, update: ObserverUpdate<'_, T>) {
        self.turn_num = update.turn_num;
        self.action_requests = update.action_requests;
        self.public_info.update(update.public_info_update);
    }
}
