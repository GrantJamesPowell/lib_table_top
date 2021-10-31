use im::Vector;
use serde::{Deserialize, Serialize};

use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;

use crate::play::{Actions, EnumeratedGameAdvance};
use crate::pov::{Observe, ObserverPov, Omniscient, OmniscientPov};
use crate::{GameObserver, GamePlayer, NumberOfPlayers, Play, Player, PlayerSet, Scenario, Seed};

#[derive(Builder, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[builder(setter(into, strip_option), build_fn(skip))]
#[serde(bound = "")]
pub struct GameProgression<T: Play> {
    seed: Arc<Seed>,
    settings: Arc<<T as Play>::Settings>,
    initial_state: Option<Arc<T>>,
    turn_num: u64,
    #[builder(setter(skip))]
    state: T,
    #[builder(setter(skip))]
    history: Vector<HistoryEvent<T>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct HistoryEvent<T: Play> {
    actions: Actions<T>,
}

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

impl<T: Play> Observe<T> for GameProgression<T> {
    fn observer_pov(&self) -> ObserverPov<'_, T> {
        ObserverPov {
            turn_num: self.turn_num,
            action_requests: self.which_players_input_needed(),
            settings: Cow::Borrowed(&self.settings()),
            public_info: Cow::Owned(self.public_info()),
        }
    }
}

impl<T: Play> Omniscient<T> for GameProgression<T> {
    fn omniscient_pov(&self) -> OmniscientPov<'_, T> {
        OmniscientPov {
            game_progression: Cow::Borrowed(&self),
        }
    }
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

    pub fn game_observer(&self) -> GameObserver<T> {
        GameObserver {
            turn_num: self.turn_num(),
            action_requests: self.which_players_input_needed(),
            settings: Arc::clone(&self.settings),
            public_info: self.public_info(),
        }
    }

    pub fn game_players(&self) -> impl Iterator<Item = GamePlayer<T>> + '_ {
        let mut player_secret_info = self.player_secret_info();

        self.players().into_iter().map(move |player| GamePlayer {
            player,
            turn_num: self.turn_num,
            action_requests: self.which_players_input_needed(),
            settings: Arc::clone(&self.settings),
            public_info: self.public_info(),
            secret_info: player_secret_info
                .remove(&player)
                .expect("game progression did not return secret info for a player"),
        })
    }

    pub fn scenario(&self) -> Scenario<T> {
        Scenario {
            turn_num: self.turn_num,
            settings: self.settings.clone(),
            initial_state: Arc::new(self.state.clone()),
            seed: self.seed.clone(),
        }
    }

    pub fn player_secret_info(&self) -> HashMap<Player, <T as Play>::PlayerSecretInfo> {
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

    #[must_use = "advancing the game does not mutate the existing game progression, but instead returns a new one"]
    pub fn submit_actions(&self, actions: Actions<T>) -> (Self, EnumeratedGameAdvance<T>) {
        let (new_state, game_advance) = self.state.advance(
            &self.settings,
            actions.clone().into_iter(),
            &mut self.seed.rng_for_turn(self.turn_num),
        );

        let mut history = self.history.clone();

        history.push_back(HistoryEvent { actions });

        let new_game_progression = Self {
            history,
            state: new_state,
            turn_num: self.turn_num + 1,
            ..self.clone()
        };

        let game_advance = EnumeratedGameAdvance {
            game_advance,
            turn_num: new_game_progression.turn_num(),
        };

        (new_game_progression, game_advance)
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
