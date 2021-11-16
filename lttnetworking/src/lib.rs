#![allow(dead_code)]

pub mod auth;
pub mod client;
pub mod connection;
pub mod messages;
pub mod server;

mod example_supported_games;
mod supported_games;
pub use supported_games::SupportedGames;

#[cfg(feature = "ws")]
pub mod ws;

mod user;
pub use user::User;

mod token;
pub use token::Token;
