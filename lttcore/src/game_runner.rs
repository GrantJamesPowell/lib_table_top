use rand::prelude::*;
// use rand_chacha::ChaCha20Rng;
use std::sync::Arc;

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
    action_requests: Vec<(Player, <T as Play>::ActionRequest)>,
    #[builder(setter(skip))]
    game_advance: <T as Play>::GameAdvance,
}

impl<T: Play> GameRunner<T> {
    fn action_requests(&self) -> &[(Player, <T as Play>::ActionRequest)] {
        &self.action_requests
    }

    fn advance_mut(&mut self, _actions: &[(Player, <T as Play>::Action)]) {
        // let mut rng = ChaCha20Rng::from_seed(*self.seed);
        // let stream_num = self.history.len().try_into().unwrap();
        // rng.set_stream(stream_num);
        // self.state.advance()
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
