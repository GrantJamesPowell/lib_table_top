use crate::connection::ConnectionIO;
use crate::messages::closed::Closed;
use crate::server::server_connection::run_server_sub_conn;
use async_trait::async_trait;
use lttcore::encoder::Encoder;
use lttcore::{examples::GuessTheNumber, Play};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt::Debug;

#[async_trait]
pub trait SupportedGames:
    Debug + Clone + Copy + PartialEq + Eq + Send + Sync + Serialize + DeserializeOwned + 'static
{
    async fn run_server_sub_conn<E: Encoder, C: ConnectionIO<E>>(self, conn: C);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExampleSupportedGames {
    GuessTheNumber,
}

#[async_trait]
impl SupportedGames for ExampleSupportedGames {
    async fn run_server_sub_conn<E: Encoder, C: ConnectionIO<E>>(self, conn: C) {
        match self {
            ExampleSupportedGames::GuessTheNumber => {
                run_server_sub_conn::<GuessTheNumber, E, C>(conn).await
            }
        }
    }
}
