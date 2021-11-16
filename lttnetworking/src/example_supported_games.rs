use crate::connection::ConnectionIO;
use crate::messages::closed::Closed;
use crate::server::server_sub_connection::run_server_sub_conn;
use crate::SupportedGames;
use async_trait::async_trait;
use lttcore::encoder::Encoder;
use lttcore::examples::GuessTheNumber;
use lttruntime::Runtime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExampleSupportedGames<E: Encoder> {
    GuessTheNumber(std::marker::PhantomData<E>),
}

pub enum ExampleSupportedRuntimesEnum<E: Encoder> {
    GuessTheNumber(Arc<Runtime<GuessTheNumber, E>>),
}

pub struct ExampleSupportedGamesRuntimes<E: Encoder> {
    runtimes: HashMap<ExampleSupportedGames<E>, ExampleSupportedRuntimesEnum<E>>,
}

impl<E: Encoder> ExampleSupportedGamesRuntimes<E> {
    pub fn init() -> Self {
        Self {
            runtimes: HashMap::from([(
                ExampleSupportedGames::GuessTheNumber(Default::default()),
                ExampleSupportedRuntimesEnum::GuessTheNumber(Arc::new(Runtime::start())),
            )]),
        }
    }

    pub fn get_guess_the_number_run_time(&self) -> Arc<Runtime<GuessTheNumber, E>> {
        if let Some(ExampleSupportedRuntimesEnum::GuessTheNumber(runtime)) = self
            .runtimes
            .get(&ExampleSupportedGames::GuessTheNumber(Default::default()))
        {
            return Arc::clone(runtime);
        } else {
            panic!("The runtime must be built for all games")
        }
    }
}

#[async_trait]
impl<E: Encoder> SupportedGames<E> for ExampleSupportedGames<E> {
    async fn run_server_sub_conn<C: ConnectionIO<E>>(self, conn: C) -> Result<(), Closed> {
        match self {
            ExampleSupportedGames::GuessTheNumber(_) => {
                run_server_sub_conn::<GuessTheNumber, E, C>(conn).await
            }
        }
    }
}
