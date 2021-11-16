
use crate::connection::ConnectionIO;
use crate::SupportedGames;
use crate::messages::closed::Closed;
use crate::server::server_sub_connection::run_server_sub_conn;
use async_trait::async_trait;
use lttcore::encoder::Encoder;
use lttcore::{examples::GuessTheNumber};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExampleSupportedGames {
    GuessTheNumber,
}

pub enum ExampleSupportedRuntimesEnum {
    GuessTheNumber
}

#[async_trait]
impl SupportedGames for ExampleSupportedGames {
    async fn run_server_sub_conn<E: Encoder, C: ConnectionIO<E>>(self, conn: C) -> Result<(), Closed> {
        match self {
            ExampleSupportedGames::GuessTheNumber => {
                run_server_sub_conn::<GuessTheNumber, E, C>(conn).await
            }
        }
    }
}
