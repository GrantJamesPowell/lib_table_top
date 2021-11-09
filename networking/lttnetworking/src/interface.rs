use crate::messages::JoinError;
use crate::{Token, User};
use async_trait::async_trait;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerConnectionError {
    Closed,
}

#[async_trait]
pub trait Auth {
    async fn authorize(&mut self, token: Token) -> Result<User, JoinError>;
}

#[async_trait]
pub trait ServerConnection {
    async fn next<T: Send + DeserializeOwned>(&mut self) -> Result<T, ServerConnectionError>;
    async fn send<T: Send + Serialize>(&mut self, msg: T) -> Result<(), ServerConnectionError>;
    async fn close(&mut self);
}
