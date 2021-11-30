//! Traits for working with bots
#![allow(missing_docs)]

use bytes::Bytes;
use serde::Serialize;
use crate::{play::{Play, Seed}, encoding::Encoding};
use crate::pov::player::PlayerPov;
use std::borrow::Cow;
use std::sync::Arc;

pub(crate) mod defective;

/// Trait to interact with [`Play`] compatible games as a [`Player`](crate::play::Player)
pub trait Bot: Sync + Send + 'static {
    /// The [`Play`] compatible game that this bot understands
    type Game: Play;

    /// Callback for when it's the bot's [`Player`](crate::play::Player)'s turn to take an action
    fn run(
        &mut self,
        player_pov: &PlayerPov<'_, Self::Game>,
        rng: &Seed,
    ) -> <Self::Game as Play>::Action;
}

pub trait DumpState {
    fn dump_state(&self, encoding: Encoding) -> Bytes; 
}

/// A trait saying that the implemeter can make an instance of a [`Bot`]
pub trait MakeBotInstance<T: Play>: Send + Sync + 'static {
    fn make_bot_instance(&self) -> Box<dyn Bot<Game = T>>;
}

#[allow(missing_debug_implementations)]
#[derive(Clone)]
pub struct Contender<T: Play> {
    name: Cow<'static, str>,
    bot: Arc<dyn MakeBotInstance<T>>,
}

impl<T: Play, B: Bot<Game=T> + Serialize> DumpState for B {
    fn dump_state(&self, encoding: Encoding) -> Bytes {
        encoding.serialize(&self).expect("you can serialize the state of the bot")
    }
}

impl<T: Play> Contender<T> {
    pub fn new(name: impl Into<Cow<'static, str>>, bot: impl MakeBotInstance<T>) -> Self {
        Self {
            name: name.into(),
            bot: Arc::new(bot),
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
    pub fn make_bot_instance(&self) -> Box<dyn Bot<Game = T>> {
        self.bot.make_bot_instance()
    }
}

impl<T: Play, B: Bot<Game = T> + Clone> MakeBotInstance<T> for B {
    fn make_bot_instance(&self) -> Box<dyn Bot<Game = T>> {
        Box::new(self.clone())
    }
}
