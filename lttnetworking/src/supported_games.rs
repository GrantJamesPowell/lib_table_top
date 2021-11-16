use crate::connection::ConnectionIO;
use crate::messages::closed::Closed;
use async_trait::async_trait;
use lttcore::encoder::Encoder;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;
use std::hash::Hash;

#[async_trait]
pub trait SupportedGames<E: Encoder>:
    Debug + Clone + Copy + PartialEq + Eq + Send + Sync + Hash + Serialize + DeserializeOwned + 'static
{
    async fn run_server_sub_conn<C: ConnectionIO<E>>(self, conn: C) -> Result<(), Closed>;
}
