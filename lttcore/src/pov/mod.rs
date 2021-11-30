//! Various state machines for interacting with in progress [`Play`](crate::play::Play) games
//!
//! # When should I use these?
//!
//! * Writing a new runtime for `LibTableTop` (i.e. supporting embedding in a new language)
//! * Writing a bot wrapper you will need to use [`player::PlayerPov`] and translate it for your
//! specific game

pub mod game_progression;
pub mod observer;
pub mod player;
pub mod scenario;
