use crate::{GameProgression, Play, Scenario};
use std::sync::Arc;

impl<T: Play> From<Scenario<T>> for GameProgression<T> {
    fn from(
        Scenario {
            turn_num,
            settings,
            initial_state,
            seed,
        }: Scenario<T>,
    ) -> Self {
        let state: T = initial_state.as_ref().clone();

        Self {
            seed,
            state,
            settings,
            turn_num,
            initial_state: Some(initial_state),
            history: Default::default(),
        }
    }
}

impl<T: Play> GameProgression<T> {
    pub fn scenario(&self) -> Scenario<T> {
        Scenario {
            turn_num: self.turn_num,
            settings: self.settings.clone(),
            initial_state: Arc::new(self.state.clone()),
            seed: self.seed.clone(),
        }
    }
}
