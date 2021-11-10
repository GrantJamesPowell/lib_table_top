#![allow(dead_code)]
#![feature(type_alias_impl_trait)]

pub mod client;
pub mod connection;
pub mod messages;
pub mod server;

mod user;
pub use user::User;

mod token;
pub use token::Token;

mod auth {
    use super::*;
    use async_trait::async_trait;

    #[async_trait]
    pub trait Auth {
        async fn authorize(&mut self, token: Token) -> Option<User>;
    }
}
