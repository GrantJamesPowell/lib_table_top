use rand::prelude::*;
use smallvec::SmallVec;
use std::collections::HashMap;
use std::sync::Arc;

use crate::play::{ActionResponse, GameAdvance};
use crate::{rng, NumberOfPlayers, Play, Player, PlayerSet, Seed};

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
    turn_num: usize,
}

#[derive(Debug, Clone)]
pub struct Turn<T: Play> {
    turn_num: usize,
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
    #[error("Can't add action for player that was not requested")]
    InvalidPlayer,
    #[error("Can't submit turn with invalid actions")]
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
        todo!()
        // let action_requests = self.state.action_requests(&self.settings);
        // match action_requests.len() {
        //     0 => None,
        //     n => {
        //         let returned_actions = (0..n).map(|_| None).collect();
        //         Some(Turn {
        //             action_requests,
        //             returned_actions,
        //         })
        //     }
        // }
    }

    pub fn submit_turn_mut(&mut self, turn: Turn<T>) -> Result<GameAdvance<T>, SubmitError> {
        todo!()
        // use SubmitError::*;

        // if !turn.is_ready_to_submit()
        //     || turn.action_requests != self.state.action_requests(&self.settings)
        // {
        //     return Err(InvalidTurn);
        // }

        // let actions_iter = turn
        //     .returned_actions
        //     .into_iter()
        //     .enumerate()
        //     .map(|(i, action)| (turn.action_requests[i].clone(), action.unwrap()));

        // let (new_state, game_advance) =
        //     self.state
        //         .advance(&self.settings, actions_iter, &mut rand::thread_rng());
        // self.state = new_state;
        // Ok(game_advance)
    }
}

// impl<T: Play> GameRunnerBuilder<T> {
//     pub fn build(&self) -> Result<GameRunner<T>, GameRunnerBuilderError> {
//         todo!()
//         // let seed = self.seed.as_ref().cloned().unwrap_or_else(|| {
//         //     let seed = rand::thread_rng().gen::<[u8; 32]>().into();
//         //     Arc::new(seed)
//         // });
//
//         // let settings = self.settings.as_ref().cloned().unwrap_or_default();
//
//         // let state = {
//         //     match self.initial_state.as_ref() {
//         //         Some(Some(state)) => {
//         //             if state.is_valid_for_settings(&settings) {
//         //                 Ok((**state).clone())
//         //             } else {
//         //                 Err("Provided initial state is not valid for settings".to_string())
//         //             }
//         //         }
//         //         _ => {
//         //             let mut rng = rng::for_init(*seed);
//         //             Ok(<T as Play>::initial_state_for_settings(&settings, &mut rng))
//         //         }
//         //     }
//         // }?;
//
//         // let initial_state = self.initial_state.as_ref().cloned().unwrap_or_default();
//         // let pending_action_requests = state.action_requests(&settings);
//
//         // Ok(GameRunner {
//         //     seed,
//         //     state,
//         //     settings,
//         //     initial_state,
//         //     pending_action_requests,
//         // })
//     }
// }
