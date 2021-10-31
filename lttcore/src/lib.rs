#![allow(dead_code)]
#![feature(associated_type_defaults)]
#![feature(never_type)]
#![feature(derive_default_enum)]
#![feature(bool_to_option)]
#![feature(const_option)]
#![feature(array_zip)]

#[macro_use]
extern crate derive_builder;

mod action_collector;
pub mod bots;
pub mod common;
pub mod examples;
pub mod game_host;
pub mod game_observer;
pub mod game_player;
pub mod game_runner;
pub mod number_of_players;
pub mod play;
pub mod player;
pub mod player_set;
pub mod pov;
pub mod scenario;
pub mod seed;
pub mod view;

pub use game_host::GameHost;
pub use game_observer::GameObserver;
pub use game_player::GamePlayer;
pub use game_runner::{GameRunner, GameRunnerBuilder};
pub use number_of_players::NumberOfPlayers;
pub use play::{ActionResponse, Play};
pub use player::Player;
pub use player_set::PlayerSet;
pub use scenario::Scenario;
pub use seed::Seed;
pub use view::View;
