use crate::connection::SubConnection;
use crate::messages::closed::Closed;
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

#[async_trait]
pub trait SupportedGames:
    Debug + Clone + Copy + PartialEq + Eq + Send + Sync + Serialize + DeserializeOwned
{
    async fn run_sub_conn(conn: SubConnection) -> Result<(), Closed>;
}
