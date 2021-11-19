#![warn(
    missing_debug_implementations,
    //missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]
#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::redundant_closure_for_method_calls)]
#![allow(clippy::default_trait_access)]
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
pub use play::{Play, TurnNum};
pub use player::Player;
pub use scenario::Scenario;
pub use seed::Seed;
pub use utilities::{NumberOfPlayers, PlayerSet};
