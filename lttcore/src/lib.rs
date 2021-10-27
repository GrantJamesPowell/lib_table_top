#![allow(dead_code)]
#![feature(associated_type_defaults)]
#![feature(never_type)]
#![feature(derive_default_enum)]
#![feature(bool_to_option)]
#![feature(const_option)]

#[macro_use]
extern crate derive_builder;

pub mod player;
pub use player::Player;

pub mod player_set;
pub use player_set::PlayerSet;

pub mod number_of_players;
pub use number_of_players::NumberOfPlayers;

pub mod view;
pub use view::View;

pub mod play;
pub use play::{ActionResponse, Play};

pub mod game_runner;
pub use game_runner::{GameRunner, GameRunnerBuilder};

pub mod turn;
pub use turn::Turn;

pub mod seed;
pub use seed::Seed;

pub mod spectator;
pub use spectator::Spectator;

pub mod bots;
pub mod common;
pub mod examples;
