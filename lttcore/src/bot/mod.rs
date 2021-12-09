//! Traits for working with bots
//!
//! # What is this?
//!
//! Various functionality for working with bots playing `LibTableTop` games.
//!
//! # Who is this for?
//!
//! This module is mostly for people building games and higher level tooling around games. If
//! you're looking to build a bot for a particular game, by convention the game should provide game
//! specific bot interfaces for bot writers to implement against. Those interfaces will likely be
//! built out of functionality from this module.
//!
//! For examples of this see [`TicTacToeBot`](crate::examples::tic_tac_toe::TicTacToeBot) which is
//! a game specific interface built on top of the base [`Bot`] trait
//!
//! # I'm trying to write a game specific bot interface.
//!
//! You've come to the right place.
//!
//! TODO:// Tutorial on how to do this, probably come back after I've implmented a few more games
//!
//! # I'm trying to deal with [`Bot`]s and [`StatefulBot`]s as `Trait Objects`
//!
//! TODO:// Explain Contenders

use crate::pov::player::PlayerPov;
use crate::{
    encoding::SerializeSelf,
    play::{Play, Seed},
    pov::player::PlayerUpdate,
};
use std::panic::RefUnwindSafe;

mod contender;
pub(crate) mod defective;
pub use contender::Contender;

/// Trait to interact with [`Play`] compatible games as a [`Player`](crate::play::Player)
///
/// This trait only allows immutable access to `&self`. Prefer implementing this trait to
/// implementing [`StatefulBot`] if possible.
pub trait Bot: SerializeSelf + RefUnwindSafe + Sync + Send + 'static {
    /// The [`Play`] compatible game that this bot understands
    type Game: Play;

    /// Callback for when it's the bot's [`Player`](crate::play::Player)'s turn to take an action
    fn on_action_request(
        &self,
        player_pov: &PlayerPov<'_, Self::Game>,
        phase: &<Self::Game as Play>::Phase,
        rng: &Seed,
    ) -> <Self::Game as Play>::Action;

    /// Callback for when the turn advances and the [`Player`](crate::play::Player) gets an update.
    /// In the context of [`Bot`] this is only useful as a debuging tool because there is no
    /// mutable state to update
    fn on_turn_advance(
        &self,
        _player_pov: &PlayerPov<'_, Self::Game>,
        _player_update: &PlayerUpdate<'_, Self::Game>,
    ) {
        // by default, don't do anything on player updates
    }
}

/// Trait to interact with [`Play`] compatible games as a [`Player`](crate::play::Player)
///
/// This trait only allows mutable access to `&self` during the turn. Prefer implementing [`Bot`] to
/// [`StatefulBot`] if possible. There is a blanket implementation for [`StatefulBot`] for all
/// structs that implment [`Bot`]
pub trait StatefulBot: SerializeSelf + Sync + Send + 'static {
    /// The [`Play`] compatible game that this bot understands
    type Game: Play;

    /// Callback for when it's the bot's [`Player`](crate::play::Player)'s turn to take an action
    fn on_action_request(
        &mut self,
        player_pov: &PlayerPov<'_, Self::Game>,
        phase: &<Self::Game as Play>::Phase,
        rng: &Seed,
    ) -> <Self::Game as Play>::Action;

    /// Callback for when the turn advances and the [`Player`](crate::play::Player) gets an update. By default, this is
    /// a noop. Use this if you want to update your bot's state outside of times it is called to
    /// act.
    fn on_turn_advance(
        &mut self,
        _player_pov: &PlayerPov<'_, Self::Game>,
        _player_update: &PlayerUpdate<'_, Self::Game>,
    ) {
        // by default, don't do anything on player updates
    }

    /// Special Optimization for times when a bot with immutable state ([`Bot`]) is being used as a
    /// is being used as a [`StatefulBot`].
    ///
    /// Returning [`true`] from this function signals to the higher level tooling that the state of
    /// the bot will not change during the [`StatefulBot::on_action_request`] or
    /// [`StatefulBot::on_turn_advance`] callbacks.
    ///
    /// It's always okay to return `false` from this function even if your bot has immutable state,
    /// if you return `true` and your bot does mutate it's state, unpredictable and wrong things
    /// may happen, but it won't be "`unsafe`" or violate memory or mutability guarantees
    fn has_immutable_state(&self) -> bool {
        false
    }
}

impl<T: Play, B: Bot<Game = T>> StatefulBot for B {
    type Game = T;

    fn on_action_request(
        &mut self,
        player_pov: &PlayerPov<'_, Self::Game>,
        phase: &T::Phase,
        rng: &Seed,
    ) -> <Self::Game as Play>::Action {
        Bot::on_action_request(&*self, player_pov, phase, rng)
    }

    fn on_turn_advance(
        &mut self,
        player_pov: &PlayerPov<'_, Self::Game>,
        player_update: &PlayerUpdate<'_, Self::Game>,
    ) {
        Bot::on_turn_advance(&*self, player_pov, player_update);
    }

    fn has_immutable_state(&self) -> bool {
        true
    }
}
