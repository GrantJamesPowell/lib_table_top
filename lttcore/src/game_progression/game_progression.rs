use im::Vector;
use serde::{Deserialize, Serialize};

use crate::play::{Actions, EnumeratedGameAdvance, PlayerSecretInfos};
use crate::{ActionResponse, NumberOfPlayers, Play, Player, PlayerSet, Seed};
use std::sync::Arc;

#[derive(Builder, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[builder(setter(into, strip_option), build_fn(skip))]
#[serde(bound = "")]
pub struct GameProgression<T: Play> {
    pub(super) seed: Arc<Seed>,
    pub(super) settings: Arc<<T as Play>::Settings>,
    pub(super) initial_state: Option<Arc<T>>,
    pub(super) turn_num: u64,
    #[builder(setter(skip))]
    pub(super) state: T,
    #[builder(setter(skip))]
    pub(super) history: Vector<HistoryEvent<T>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct HistoryEvent<T: Play> {
    actions: Actions<T>,
}

impl<T: Play> GameProgression<T> {
    pub fn from_settings(settings: impl Into<T::Settings>) -> Self {
        let settings = settings.into();
        GameProgressionBuilder::default()
            .settings(settings)
            .build()
            .unwrap()
    }

    pub fn turn_num(&self) -> u64 {
        self.turn_num
    }

    pub fn state(&self) -> &T {
        &self.state
    }

    pub fn settings(&self) -> &<T as Play>::Settings {
        &self.settings
    }

    pub fn settings_arc(&self) -> &Arc<<T as Play>::Settings> {
        &self.settings
    }

    pub fn public_info(&self) -> <T as Play>::PublicInfo {
        self.state.public_info(&self.settings)
    }

    pub fn player_secret_info(&self) -> PlayerSecretInfos<T> {
        self.state.player_secret_info(&self.settings)
    }

    pub fn number_of_players(&self) -> NumberOfPlayers {
        <T as Play>::number_of_players_for_settings(&self.settings)
    }

    pub fn players(&self) -> PlayerSet {
        self.number_of_players().player_set()
    }

    pub fn which_players_input_needed(&self) -> PlayerSet {
        self.state.which_players_input_needed(&self.settings)
    }

    pub fn submit_actions(
        &mut self,
        actions: impl Iterator<Item = (Player, ActionResponse<<T as Play>::Action>)>,
    ) -> EnumeratedGameAdvance<T> {
        let actions: Actions<T> = actions.collect();

        let (new_state, game_advance) = self.state.advance(
            &self.settings,
            actions.clone().into_iter(),
            &mut self.seed.rng_for_turn(self.turn_num),
        );

        self.history.push_back(HistoryEvent { actions });
        self.state = new_state;
        self.turn_num += 1;

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
        let turn_num = self.turn_num.as_ref().cloned().unwrap_or(0);
        let settings = self.settings.as_ref().cloned().unwrap_or_default();
        let state = initial_state
            .as_ref()
            .map(|arc| arc.as_ref())
            .cloned()
            .unwrap_or_else(|| {
                <T as Play>::initial_state_for_settings(&settings, &mut seed.rng_for_init())
            });
        let history = Vector::new();

        Ok(GameProgression {
            seed,
            settings,
            state,
            history,
            initial_state,
            turn_num,
        })
    }
}
