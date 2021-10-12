use rand::prelude::*;
// use rand_chacha::ChaCha20Rng;
use std::collections::HashMap;
use std::sync::Arc;

use crate::play::{ActionResponse, GameAdvance};
use crate::{Play, Player};

use thiserror::Error;

#[derive(Builder, Clone, Debug)]
#[builder(
    setter(into, strip_option),
    build_fn(name = "build_without_initializing")
)]
pub struct GameRunner<T>
where
    T: Play,
{
    #[builder(default)]
    settings: Arc<<T as Play>::Settings>,
    #[builder(default)]
    initial_state: Option<Arc<T>>,
    #[builder(default = "Arc::new(rand::thread_rng().gen::<[u8; 32]>())")]
    seed: Arc<[u8; 32]>,
    #[builder(setter(skip), default = "self.choose_state()?")]
    state: T,
    #[builder(setter(skip))]
    pending_action_requests: Vec<(Player, <T as Play>::ActionRequest)>,
    #[builder(setter(skip))]
    game_advance: GameAdvance<T>,
}

#[derive(Debug)]
pub struct Turn<T: Play> {
    action_requests: Vec<(Player, <T as Play>::ActionRequest)>,
    returned_actions: Vec<Option<ActionResponse<<T as Play>::Action>>>,
}

impl<T: Play> Turn<T> {
    pub fn action_request(&self) -> Option<(usize, (Player, <T as Play>::ActionRequest))> {
        self.pending_action_requests()
            .next()
            .map(|(id, req)| (id, req.clone()))
    }

    pub fn pending_action_requests(
        &self,
    ) -> impl Iterator<Item = (usize, &(Player, <T as Play>::ActionRequest))> + '_ {
        self.action_requests
            .iter()
            .enumerate()
            .filter(|(id, _)| self.returned_actions[*id].is_none())
    }

    pub fn submit_action(
        &mut self,
        action_id: usize,
        action_response: ActionResponse<<T as Play>::Action>,
    ) -> Option<ActionResponse<<T as Play>::Action>> {
        std::mem::replace(&mut self.returned_actions[action_id], Some(action_response))
    }

    pub fn is_ready_to_submit(&self) -> bool {
        self.returned_actions.iter().all(|action| action.is_some())
    }
}

#[derive(Error, Debug, Clone, Copy)]
pub enum SubmitError {
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

    pub fn players(&self) -> impl Iterator<Item = Player> + '_ {
        (0..<T as Play>::number_of_players_for_settings(&self.settings)).map(|p| Player::new(p))
    }

    pub fn spectator_view(&self) -> <T as Play>::SpectatorView {
        self.state.spectator_view(&self.settings)
    }

    pub fn turn(&self) -> Option<Turn<T>> {
        match self.pending_action_requests.len() {
            0 => None,
            n => {
                let returned_actions = (0..n).map(|_| None).collect();
                Some(Turn {
                    action_requests: self.pending_action_requests.clone(),
                    returned_actions: returned_actions,
                })
            }
        }
    }

    pub fn submit_turn_mut(&mut self, turn: Turn<T>) -> Result<&GameAdvance<T>, SubmitError> {
        use SubmitError::*;

        if !turn.is_ready_to_submit() || turn.action_requests != self.pending_action_requests {
            return Err(InvalidTurn);
        }

        let actions_iter = turn
            .returned_actions
            .into_iter()
            .enumerate()
            .map(|(i, action)| (self.pending_action_requests[i].clone(), action.unwrap()));

        self.game_advance.reset();
        self.state.advance(
            &self.settings,
            actions_iter,
            &mut rand::thread_rng(),
            &mut self.game_advance,
        );

        self.pending_action_requests.clear();
        self.state
            .action_requests_into(&self.settings, &mut self.pending_action_requests);

        Ok(&self.game_advance)
    }
}

impl<T: Play> GameRunnerBuilder<T> {
    fn choose_state(&self) -> Result<T, String> {
        let default_settings = Default::default();
        let settings: &<T as Play>::Settings = self.settings.as_ref().unwrap_or(&default_settings);

        match self.initial_state.as_ref() {
            Some(Some(state)) => {
                if state.is_valid_for_settings(&settings) {
                    Ok((**state).clone())
                } else {
                    Err("Provided initial state is not valid for settings".to_string())
                }
            }
            _ => Ok(<T as Play>::initial_state_for_settings(&settings)),
        }
    }

    // Custom adding of the first set of pending requests into the buffer
    pub fn build(&self) -> Result<GameRunner<T>, GameRunnerBuilderError> {
        let mut runner = self.build_without_initializing()?;
        runner
            .state
            .action_requests_into(&runner.settings, &mut runner.pending_action_requests);
        Ok(runner)
    }
}
