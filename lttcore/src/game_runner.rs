// use rand::prelude::*;
// use rand_chacha::ChaCha20Rng;
use std::sync::Arc;

// use crate::play::game_advance::Reset;
use crate::{Play, Player};

#[derive(Builder, Clone, Debug)]
#[builder(setter(into, strip_option), build_fn(skip))]
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
}

#[derive(Debug)]
pub struct Turn<'a, T: Play> {
    pending_action_requests: &'a Vec<(Player, <T as Play>::ActionRequest)>,
    returned_actions: Vec<Option<<T as Play>::Action>>,
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
        self.pending_action_requests.iter().enumerate()
    }

    fn submit_action(
        &mut self,
        action_id: usize,
        action: <T as Play>::Action,
    ) -> Option<<T as Play>::Action> {
        std::mem::replace(&mut self.returned_actions[action_id], Some(action))
    }

    fn is_ready_to_submit(&self) -> bool {
        self.returned_actions.iter().all(|action| action.is_some())
    }
}

impl<T: Play> GameRunner<T> {
    pub fn game(&self) -> &T {
        &self.state
    }

    pub fn settings(&self) -> &<T as Play>::Settings {
        &self.settings
    }

    pub fn turn(&self) -> Turn<T> {
        let returned_actions = (0..self.pending_action_requests.len())
            .map(|_| None)
            .collect();

        Turn {
            pending_action_requests: &self.pending_action_requests,
            returned_actions: returned_actions,
        }
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

    pub fn build(&self) -> Result<GameRunner<T>, GameRunnerBuilderError> {
        todo!()
    }
}
