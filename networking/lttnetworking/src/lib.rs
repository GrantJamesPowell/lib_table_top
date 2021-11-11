#![allow(dead_code)]
#![feature(type_alias_impl_trait)]

pub mod auth;
pub mod client;
pub mod connection;
pub mod messages;
pub mod server;

mod supported_game;
pub use supported_game::SupportedGames;

mod user;
pub use user::User;

mod token;
pub use token::Token;
