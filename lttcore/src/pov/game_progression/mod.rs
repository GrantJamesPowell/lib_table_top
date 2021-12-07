#![allow(missing_docs)]
mod support_from_settings;
mod support_getters;
mod support_povs;
mod support_scenarios;

use crate::play::{
    ActionResponse, EnumeratedGameStateUpdate, GameState, Play, Seed, SettingsPtr, TurnNum,
};
use crate::utilities::PlayerIndexedData as PID;
use im::Vector;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::sync::Arc;

#[derive(Builder, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[builder(setter(into, strip_option), build_fn(skip))]
#[serde(bound = "")]
pub struct GameProgression<T: Play> {
    pub(super) seed: Arc<Seed>,
    pub(super) settings: SettingsPtr<T::Settings>,
    pub(super) initial_game_state: Option<Arc<GameState<T>>>,
    pub(super) turn_num: TurnNum,
    #[builder(setter(skip))]
    pub(super) game_state: GameState<T>,
    #[builder(setter(skip))]
    pub(super) history: Vector<HistoryEvent<T>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct HistoryEvent<T: Play> {
    turn_num: TurnNum,
    actions: PID<ActionResponse<T>>,
}

impl<T: Play> GameProgression<T> {
    #[must_use = "resolve only figures out the update, but does not apply it"]
    pub fn resolve(&self, actions: PID<ActionResponse<T>>) -> EnumeratedGameStateUpdate<T> {
        debug_assert_eq!(
            Some(actions.players().collect()),
            self.game_state.action_requests,
            "correct actions submitted"
        );

        let game_state_update = T::resolve(
            &self.game_state,
            &self.settings,
            actions
                .iter()
                .map(|(player, action)| (player, Cow::Borrowed(action)))
                .collect(),
            &mut self.seed.rng_for_turn(self.turn_num),
        );

        EnumeratedGameStateUpdate {
            game_state_update,
            actions: Some(actions),
            turn_num: self.turn_num,
        }
    }

    pub fn update(&mut self, update: EnumeratedGameStateUpdate<T>) {
        debug_assert_eq!(
            self.turn_num,
            update.current_turn_num(),
            "tried to apply an update for the wrong turn"
        );
        update.actions.map(|actions| {
            self.history.push_back(HistoryEvent {
                turn_num: self.turn_num,
                actions,
            })
        });
        self.game_state.update(update.game_state_update);
        self.turn_num.increment();
    }
}

impl<T: Play> GameProgressionBuilder<T> {
    pub fn build(&self) -> Result<GameProgression<T>, GameProgressionBuilderError> {
        let seed = self
            .seed
            .as_ref()
            .cloned()
            .unwrap_or_else(|| Arc::new(Seed::random()));

        let initial_game_state: Option<Arc<GameState<T>>> =
            self.initial_game_state.as_ref().cloned().unwrap_or(None);
        let turn_num = self.turn_num.as_ref().copied().unwrap_or_default();
        let settings = self.settings.as_ref().cloned().unwrap_or_default();
        let game_state = initial_game_state
            .as_ref()
            .map(|arc| arc.as_ref())
            .cloned()
            .unwrap_or_else(|| T::initial_state_for_settings(&settings, &mut seed.rng_for_init()));
        let history = Vector::new();

        Ok(GameProgression {
            seed,
            settings,
            initial_game_state,
            turn_num,
            game_state,
            history,
        })
    }
}
