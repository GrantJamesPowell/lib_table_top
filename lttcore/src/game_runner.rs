use rand::prelude::*;
// use rand_chacha::ChaCha20Rng;
use std::sync::Arc;

use crate::play::GameAdvance;
use crate::{Play, Player};

#[derive(Builder, Clone, Debug)]
#[builder(setter(into, strip_option))]
pub struct GameRunner<T>
where
    T: Play,
{
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
pub struct Turn<'a, T: Play> {
    action_requests: &'a Vec<(Player, <T as Play>::ActionRequest)>,
    returned_actions: Vec<Option<<T as Play>::ActionResponse>>,
}

impl<'a, T: Play> Turn<'a, T> {
    fn action_request(&self) -> Option<(usize, &(Player, <T as Play>::ActionRequest))> {
        self.pending_action_requests()
            .filter(|(id, _)| self.returned_actions[*id].is_none())
            .next()
    }

    fn pending_action_requests(
        &self,
    ) -> impl Iterator<Item = (usize, &(Player, <T as Play>::ActionRequest))> + '_ {
        self.action_requests.iter().enumerate()
    }

    fn submit_action(
        &mut self,
        action_id: usize,
        action_response: <T as Play>::ActionResponse,
    ) -> Option<<T as Play>::ActionResponse> {
        std::mem::replace(&mut self.returned_actions[action_id], Some(action_response))
    }

    fn is_ready_to_submit(&self) -> bool {
        self.returned_actions.iter().all(|action| action.is_some())
    }
}

pub enum SubmitError {
    InvalidTurn,
}

impl<T: Play> GameRunner<T> {
    pub fn game(&self) -> &T {
        &self.state
    }

    pub fn settings(&self) -> &<T as Play>::Settings {
        &self.settings
    }

    pub fn turn(&self) -> Option<Turn<T>> {
        match self.pending_action_requests.len() {
            0 => None,
            n => {
                let returned_actions = (0..n).map(|_| None).collect();
                Some(Turn {
                    action_requests: &self.pending_action_requests,
                    returned_actions: returned_actions,
                })
            }
        }
    }

    pub fn submit_turn_mut(&mut self, turn: Turn<T>) -> Result<&GameAdvance<T>, SubmitError> {
        use SubmitError::*;

        if !turn.is_ready_to_submit() || turn.action_requests != &self.pending_action_requests {
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
        let settings = self
            .settings
            .as_ref()
            .ok_or("settings must be set".to_string())?;

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
}
