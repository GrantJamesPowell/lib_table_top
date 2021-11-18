#![warn(
    clippy::pedantic,
    missing_debug_implementations,
    //missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::module_name_repetitions)]
#![allow(redundant_closure_for_method_calls)]
#![allow(dead_code)]

#[macro_use]
extern crate derive_builder;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate static_assertions;

mod game_observer;
mod game_player;
mod player;
mod scenario;

pub mod bots;
pub mod common;
#[cfg(feature = "encoder")]
pub mod encoder;
pub mod examples;
pub mod game_progression;
pub mod id;
pub mod play;
pub mod pov;
pub mod seed;
pub mod utilities;

pub use game_observer::GameObserver;
pub use game_player::GamePlayer;
pub use game_progression::{GameProgression, GameProgressionBuilder};
pub use play::{Play, TurnNum, View};
pub use player::Player;
pub use scenario::Scenario;
pub use seed::Seed;
pub use utilities::{NumberOfPlayers, PlayerSet};
