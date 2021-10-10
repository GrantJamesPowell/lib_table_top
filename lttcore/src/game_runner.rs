use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use std::sync::Arc;

use crate::play::game_advance::Reset;
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
    history: Vec<<T as Play>::Action>,
    #[builder(setter(skip))]
    pending_action_requests: Vec<(Player, <T as Play>::ActionRequest)>,
    #[builder(setter(skip))]
    game_advance: <T as Play>::GameAdvance,
}

impl<T: Play> GameRunner<T> {
    pub fn game(&self) -> &T {
        &self.state
    }

    pub fn settings(&self) -> &<T as Play>::Settings {
        &self.settings
    }

    pub fn action_request(&self) -> Option<&(Player, <T as Play>::ActionRequest)> {
        self.pending_action_requests.first()
    }

    fn pending_action_requests(
        &self,
    ) -> impl Iterator<Item = &(Player, <T as Play>::ActionRequest)> + '_ {
        self.pending_action_requests.iter()
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
