use crate::connection::ConnectionIO;
use crate::messages::Closed;
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

#[async_trait]
pub trait SupportedGames:
    Debug + Clone + Copy + PartialEq + Eq + Send + Sync + Serialize + DeserializeOwned
{
    async fn run_game_observer<T: ConnectionIO>(conn: T) -> Result<(), Closed>;
    async fn run_game_player<T: ConnectionIO>(conn: T) -> Result<(), Closed>;
}
