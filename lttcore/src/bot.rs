//! Traits for working with bots

use crate::play::{Play, Seed};
use crate::pov::player::PlayerPov;

/// Trait to interact with [`Play`] compatible games as a [`Player`](crate::play::Player)
pub trait Bot: Sync + Send + 'static {
    /// The [`Play`] compatible game that this bot understands
    type Game: Play;

    /// Callback for when it's the bot's [`Player`](crate::play::Player)'s turn to take an action
    fn run(
        &self,
        player_pov: &PlayerPov<'_, Self::Game>,
        rng: &Seed,
    ) -> <Self::Game as Play>::Action;
}
