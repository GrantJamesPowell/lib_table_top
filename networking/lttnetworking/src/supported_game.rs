use async_trait::async_trait;
use std::fmt::Debug;
use serde::{de::DeserializeOwned, Serialize};
use crate::messages::Closed;
use crate::connection::ConnectionIO;

#[async_trait]
pub trait SupportedGames : Debug + Clone + Copy + PartialEq + Eq + Serialize + DeserializeOwned {
    async fn run_game_observer<T: ConnectionIO>(conn: T) -> Result<(), Closed>;
    async fn run_game_player<T: ConnectionIO>(conn: T) -> Result<(), Closed>;
}
