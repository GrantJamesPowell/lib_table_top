use crate::messages::Closed;
use crate::{Token, User};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

#[async_trait]
pub trait Auth {
    async fn authorize(&mut self, token: Token) -> Option<User>;
}

#[async_trait]
pub trait ConnectionIO {
    async fn next<T: Send + DeserializeOwned>(&mut self) -> Result<T, Closed>;
    async fn send<T: Send + Serialize>(&mut self, msg: T) -> Result<(), Closed>;
    async fn close(&mut self);
}