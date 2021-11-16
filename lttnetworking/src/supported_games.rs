use crate::connection::ConnectionIO;
use crate::messages::closed::Closed;
use async_trait::async_trait;
use lttcore::encoder::Encoder;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

#[async_trait]
pub trait SupportedGames:
    Debug + Clone + Copy + PartialEq + Eq + Send + Sync + Serialize + DeserializeOwned
{
    async fn run_sub_conn<E: Encoder>(conn: impl ConnectionIO<E>) -> Result<(), Closed>;
}
