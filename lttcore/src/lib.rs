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
mod turn_num;

pub mod bots;
pub mod common;
pub mod examples;
pub mod play;
pub mod pov;
pub mod seed;
pub mod utilities;
pub mod view;

pub use game_observer::GameObserver;
pub use game_player::GamePlayer;
pub use game_progression::{GameProgression, GameProgressionBuilder};
pub use play::Play;
pub use player::Player;
pub use scenario::Scenario;
pub use seed::Seed;
pub use turn_num::TurnNum;
pub use utilities::{NumberOfPlayers, PlayerSet};
pub use view::View;
