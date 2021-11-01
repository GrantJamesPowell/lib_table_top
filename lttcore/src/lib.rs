#![allow(dead_code)]
#![feature(associated_type_defaults)]
#![feature(never_type)]
#![feature(derive_default_enum)]
#![feature(bool_to_option)]
#![feature(const_option)]
#![feature(array_zip)]

#[macro_use]
extern crate derive_builder;

pub mod bots;
pub mod common;
pub mod examples;
pub mod game_observer;
pub mod game_player;
pub mod game_progression;
pub mod number_of_players;
pub mod play;
pub mod player;
pub mod pov;
pub mod scenario;
pub mod seed;
pub mod utilities;
pub mod view;

pub use game_observer::GameObserver;
pub use game_player::GamePlayer;
pub use game_progression::{GameProgression, GameProgressionBuilder};
pub use number_of_players::NumberOfPlayers;
pub use play::{ActionResponse, Play};
pub use player::Player;
pub use scenario::Scenario;
pub use seed::Seed;
pub use utilities::PlayerSet;
pub use view::View;
