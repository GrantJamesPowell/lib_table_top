use super::GameProgression;
use crate::{
    play::{GameState, Play},
    pov::scenario::Scenario,
};
use std::sync::Arc;

impl<T: Play> From<Scenario<T>> for GameProgression<T> {
    fn from(
        Scenario {
            turn_num,
            settings,
            initial_game_state,
            seed,
        }: Scenario<T>,
    ) -> Self {
        let game_state: GameState<T> = initial_game_state.as_ref().clone();

        Self {
            seed,
            game_state,
            settings,
            turn_num,
            initial_game_state: Some(initial_game_state),
            history: Default::default(),
        }
    }
}

impl<T: Play> GameProgression<T> {
    pub fn scenario(&self) -> Scenario<T> {
        Scenario {
            turn_num: self.turn_num,
            settings: self.settings.clone(),
            initial_game_state: Arc::new(self.game_state.clone()),
            seed: self.seed.clone(),
        }
    }
}
