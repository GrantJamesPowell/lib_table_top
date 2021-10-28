use crate::bots::BotContext;
use crate::{GameRunner, Play, Player, Seed, Spectator, Turn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct Scenario<T: Play> {
    pub(crate) turn_num: u64,
    pub(crate) settings: Arc<<T as Play>::Settings>,
    pub(crate) initial_state: Arc<T>,
    pub(crate) seed: Arc<Seed>,
}

pub struct ActionContexts<T: Play> {
    turn: Turn<T>,
    spectator: Spectator<T>,
    player_secret_info: HashMap<Player, <T as Play>::PlayerSecretInfo>,
}

pub struct ActionContext<'a, T: Play> {
    pub player: Player,
    pub secret_info: &'a <T as Play>::PlayerSecretInfo,
    pub spectator: &'a Spectator<T>,
}

impl<T: Play> Scenario<T> {
    fn action_contexts(&self) -> ActionContexts<T> {
        let game_runner: GameRunner<T> = self.clone().into();
        let spectator = game_runner.spectator();
        let player_secret_info = game_runner.player_secret_info();
        let turn = game_runner.turn().unwrap();

        ActionContexts {
            turn,
            spectator,
            player_secret_info,
        }
    }
}
