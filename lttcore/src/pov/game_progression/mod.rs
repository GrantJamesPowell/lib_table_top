#![allow(missing_docs)]
mod support_from_settings;
mod support_getters;
mod support_povs;
mod support_scenarios;

use crate::play::{
    ActionResponse, Actions, EnumeratedGameAdvance, Play, Player, Seed, SettingsPtr, TurnNum,
};
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
    pub(super) initial_state: Option<Arc<T>>,
    pub(super) turn_num: TurnNum,
    #[builder(setter(skip))]
    pub(super) state: T,
    #[builder(setter(skip))]
    pub(super) history: Vector<HistoryEvent<T>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct HistoryEvent<T: Play> {
    turn_num: TurnNum,
    actions: Actions<T>,
}

impl<T: Play> GameProgression<T> {
    pub fn submit_actions(
        &mut self,
        actions: impl IntoIterator<Item = (Player, ActionResponse<T>)>,
    ) -> EnumeratedGameAdvance<T> {
        let actions: Actions<T> = actions.into_iter().collect();
        let game_advance = self.state.advance(
            &self.settings,
            actions
                .iter()
                .map(|(player, action)| (player, Cow::Borrowed(action))),
            &mut self.seed.rng_for_turn(self.turn_num),
        );

        self.history.push_back(HistoryEvent {
            actions,
            turn_num: self.turn_num,
        });
        self.turn_num.increment();

        EnumeratedGameAdvance {
            game_advance,
            turn_num: self.turn_num(),
        }
    }
}

impl<T: Play> GameProgressionBuilder<T> {
    pub fn build(&self) -> Result<GameProgression<T>, GameProgressionBuilderError> {
        let seed = self
            .seed
            .as_ref()
            .cloned()
            .unwrap_or_else(|| Arc::new(Seed::random()));

        let initial_state: Option<Arc<T>> = self.initial_state.as_ref().cloned().unwrap_or(None);
        let turn_num = self.turn_num.as_ref().copied().unwrap_or_default();
        let settings = self.settings.as_ref().cloned().unwrap_or_default();
        let state = initial_state
            .as_ref()
            .map(|arc| arc.as_ref())
            .cloned()
            .unwrap_or_else(|| {
                T::initial_state_for_settings(&settings, &mut seed.rng_for_init())
            });
        let history = Vector::new();

        Ok(GameProgression {
            seed,
            settings,
            initial_state,
            turn_num,
            state,
            history,
        })
    }
}
