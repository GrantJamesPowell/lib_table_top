#![allow(dead_code)]
#![feature(associated_type_defaults)]
#![feature(never_type)]
#![feature(derive_default_enum)]
#![feature(bool_to_option)]

#[macro_use]
extern crate derive_builder;

pub mod player;
pub use player::Player;

pub mod number_of_players;
pub use number_of_players::NumberOfPlayers;

pub mod view;
pub use view::View;

pub mod play;
pub use play::Play;

pub mod game_runner;
pub use game_runner::{GameRunner, GameRunnerBuilder};

pub mod rng;
pub use rng::Seed;

pub mod common;
