#![allow(dead_code)]
#![feature(associated_type_defaults)]
#![feature(never_type)]
#![feature(derive_default_enum)]
#![feature(bool_to_option)]
#![feature(const_option)]
#![feature(array_zip)]

#[macro_use]
extern crate derive_builder;

mod game_observer;
mod game_player;
mod game_progression;
mod player;
mod scenario;

pub mod bots;
pub mod common;
pub mod examples;
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
