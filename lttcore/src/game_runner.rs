use rand::prelude::*;
use smallvec::SmallVec;
use std::collections::HashMap;
use std::sync::Arc;

use crate::play::{ActionResponse, GameAdvance};
use crate::{NumberOfPlayers, Play, Player, PlayerSet, Seed};

use thiserror::Error;

#[derive(Builder, Clone, Debug)]
#[builder(setter(into, strip_option), build_fn(skip))]
pub struct GameRunner<T>
where
    T: Play,
{
    seed: Arc<Seed>,
    #[builder(default)]
    settings: Arc<<T as Play>::Settings>,
    #[builder(setter(skip))]
    state: T,
    #[builder(setter(skip))]
    turn_num: u64,
}

#[derive(Debug, Clone)]
pub struct Turn<T: Play> {
    turn_num: u64,
    action_requests: PlayerSet,
    actions: SmallVec<[(Player, ActionResponse<<T as Play>::Action>); 2]>,
}

impl<T: Play> Turn<T> {
    pub fn pending_action_requests(&self) -> impl Iterator<Item = Player> + '_ {
        self.action_requests.players().filter(|player| {
            self.actions
                .binary_search_by_key(&player, |(p, _)| &*p)
                .is_err()
        })
    }

    pub fn add_action(
        &mut self,
        player: Player,
        action_response: ActionResponse<<T as Play>::Action>,
    ) -> Result<(), SubmitError> {
        if !self.action_requests.contains(player) {
            return Err(SubmitError::InvalidPlayer);
        }

        match self.actions.binary_search_by_key(&player, |(p, _)| *p) {
            Ok(existing_action_index) => {
                self.actions[existing_action_index] = (player, action_response);
            }
            Err(index) => {
                self.actions.insert(index, (player, action_response));
            }
        }

        return Ok(());
    }

    pub fn is_ready_to_submit(&self) -> bool {
        self.pending_action_requests().next().is_none()
    }
}

#[derive(Error, Debug, Clone, Copy)]
pub enum SubmitError {
    #[error("can't add action for player that was not requested")]
    InvalidPlayer,
    #[error("can't submit turn with invalid actions")]
    InvalidTurn,
}

impl<T: Play> GameRunner<T> {
    pub fn game(&self) -> &T {
        &self.state
    }

    pub fn settings(&self) -> &<T as Play>::Settings {
        &self.settings
    }

    pub fn player_views(&self) -> HashMap<Player, <T as Play>::PlayerView> {
        self.state.player_views(&self.settings)
    }

    pub fn number_of_players(&self) -> NumberOfPlayers {
        <T as Play>::number_of_players_for_settings(&self.settings)
    }

    pub fn spectator_view(&self) -> <T as Play>::SpectatorView {
        self.state.spectator_view(&self.settings)
    }

    pub fn turn(&self) -> Option<Turn<T>> {
        let action_requests = self.state.action_requests(&self.settings);

        match action_requests.count() {
            0 => None,
            _ => Some(Turn {
                action_requests,
                actions: Default::default(),
                turn_num: self.turn_num,
            }),
        }
    }

    pub fn submit_turn(&self, turn: Turn<T>) -> Result<(Self, GameAdvance<T>), SubmitError> {
        if !turn.is_ready_to_submit() || (turn.turn_num != self.turn_num) {
            return Err(SubmitError::InvalidTurn);
        }

        let (new_state, game_advance) = self.state.advance(
            &self.settings,
            turn.actions.into_iter(),
            &mut self.seed.rng_for_turn(self.turn_num),
        );

        Ok((
            Self {
                state: new_state,
                turn_num: self.turn_num + 1,
                ..self.clone()
            },
            game_advance,
        ))
    }
}

impl<T: Play> GameRunnerBuilder<T> {
    pub fn build(&self) -> Result<GameRunner<T>, GameRunnerBuilderError> {
        let seed = self.seed.as_ref().cloned().unwrap_or_else(|| {
            let seed: Seed = rand::thread_rng().gen::<[u8; 32]>().into();
            Arc::new(seed)
        });

        let settings = self.settings.as_ref().cloned().unwrap_or_default();
        let state = <T as Play>::initial_state_for_settings(&settings, &mut seed.rng_for_init());

        Ok(GameRunner {
            seed,
            settings,
            state,
            turn_num: 0,
        })
    }
}
