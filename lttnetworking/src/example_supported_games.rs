use crate::connection::ConnectionIO;
use crate::messages::closed::Closed;
use crate::server::server_sub_connection::run_server_sub_conn;
use crate::SupportedGames;
use async_trait::async_trait;
use lttcore::encoder::{bincode::BincodeEncoder, Encoder};
use lttcore::examples::GuessTheNumber;
use lttruntime::Runtime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

/// This will eventually be the output from a macro
/// Something like `supported_games!(GuessTheNumber, TicTacToe)` or
/// `supported_games!(./path/to/config.toml)`, I'm writing it out by hand here to figure out how
/// the macro should work

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExampleSupportedGames<E: Encoder = BincodeEncoder> {
    GuessTheNumber(std::marker::PhantomData<E>),
}

pub enum ExampleSupportedRuntimesEnum<E: Encoder = BincodeEncoder> {
    GuessTheNumber(Arc<Runtime<GuessTheNumber, E>>),
}

pub struct ExampleSupportedGamesRuntimes<E: Encoder = BincodeEncoder> {
    runtimes: HashMap<ExampleSupportedGames<E>, ExampleSupportedRuntimesEnum<E>>,
}

impl<E: Encoder> ExampleSupportedGamesRuntimes<E> {
    pub fn init() -> Arc<Self> {
        Arc::new(Self {
            runtimes: HashMap::from([(
                ExampleSupportedGames::GuessTheNumber(Default::default()),
                ExampleSupportedRuntimesEnum::GuessTheNumber(Arc::new(Runtime::start())),
            )]),
        })
    }

    pub fn get_guess_the_number_run_time(&self) -> Arc<Runtime<GuessTheNumber, E>> {
        if let Some(ExampleSupportedRuntimesEnum::GuessTheNumber(runtime)) = self
            .runtimes
            .get(&ExampleSupportedGames::GuessTheNumber(Default::default()))
        {
            Arc::clone(runtime)
        } else {
            panic!("The runtime must be built for all games")
        }
    }
}

#[async_trait]
impl<E: Encoder> SupportedGames<E> for ExampleSupportedGames<E> {
    type Runtimes = ExampleSupportedGamesRuntimes<E>;

    async fn run_server_sub_conn<C: ConnectionIO<E>>(
        self,
        conn: C,
        runtimes: Arc<Self::Runtimes>,
    ) -> Result<(), Closed> {
        match self {
            ExampleSupportedGames::GuessTheNumber(_) => {
                let runtime = runtimes.get_guess_the_number_run_time();
                run_server_sub_conn::<GuessTheNumber, E, C>(conn, runtime).await
            }
        }
    }

    fn try_from_str(s: &str) -> Option<Self> {
        match s {
            "GuessTheNumber" => Some(ExampleSupportedGames::GuessTheNumber(Default::default())),
            _ => None,
        }
    }
}
