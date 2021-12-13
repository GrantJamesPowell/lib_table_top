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
//! # I'm trying to deal with [`Bot`]s as `Trait Objects`
//!
//! TODO:// Explain Contenders

use crate::pov::player::PlayerPov;
use crate::{encoding::SerializeSelf, play::Play, pov::player::PlayerUpdate};
use std::panic::RefUnwindSafe;

mod contender;
mod context;
pub(crate) mod defective;
pub use contender::Contender;
pub use context::{BotContext, BotContextBuilder};

/// Various errors associated with [`Bot`] execution
#[derive(Debug)]
pub enum BotError<T: Play> {
    /// Triggered from the [`BotContext::checkpoint`] method
    TimeExceeded(Option<T::Action>),
    /// Any other error
    Custom(Box<dyn std::error::Error>),
}

/// Trait to interact with [`Play`] compatible games as a [`Player`](crate::play::Player)
pub trait Bot: SerializeSelf + RefUnwindSafe + Sync + Send + 'static {
    /// The [`Play`] compatible game that this bot understands
    type Game: Play;

    /// Callback for when it's the bot's [`Player`](crate::play::Player)'s turn to take an action
    fn on_action_request(
        &mut self,
        player_pov: &PlayerPov<'_, Self::Game>,
        bot_context: &BotContext<'_, Self::Game>,
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

    /// Special Optimization for times when a bot with immutable state is being used
    ///
    /// Returning [`true`] from this function signals to the higher level tooling that the state of
    /// the bot will not change during the [`Bot::on_action_request`] or [`Bot::on_turn_advance`]
    /// callbacks.
    ///
    /// It's always okay to return `false` from this function even if your bot has immutable state,
    /// if you return `true` and your bot does mutate it's state, unpredictable and wrong things
    /// may happen, but it won't be "`unsafe`" or violate memory or mutability guarantees
    fn has_immutable_state(&self) -> bool {
        false
    }
}
