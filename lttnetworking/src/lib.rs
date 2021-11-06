#![allow(dead_code)]
#![feature(type_alias_impl_trait)]

pub mod client;
pub mod connection;
pub mod hello;
pub mod ping;
pub mod server;

mod user;
pub use user::User;

mod token;
pub use token::Token;
