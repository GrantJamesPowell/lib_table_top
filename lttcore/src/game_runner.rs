use im::Vector;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::collections::HashMap;
use std::sync::Arc;

use crate::play::{ActionResponse, GameAdvance};
use crate::{NumberOfPlayers, Play, Player, Seed, Spectator, Turn};

pub type Actions<T> = SmallVec<[(Player, ActionResponse<<T as Play>::Action>); 2]>;

#[derive(Builder, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[builder(setter(into, strip_option), build_fn(skip))]
// https://stackoverflow.com/questions/61473323/cannot-infer-type-for-type-parameter-when-deriving-deserialize-for-a-type-with-a
// It looks like this is required because serde is trying to provide redundant bounds to T, but
// since T and all of T's assoc'd types are deserialize, I think everything is gravy
#[serde(bound = "")]
pub struct GameRunner<T: Play> {
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

impl<T: Play> GameRunner<T> {
    pub fn game(&self) -> &T {
        &self.state
    }

    pub fn settings(&self) -> &<T as Play>::Settings {
        &self.settings
    }

    pub fn player_views(&self) -> HashMap<Player, <T as Play>::PlayerView> {
        self.state.player_views(&self.settings)
    }

    pub fn number_of_players(&self) -> NumberOfPlayers {
        <T as Play>::number_of_players_for_settings(&self.settings)
    }

    pub fn spectator(&self) -> Spectator<T> {
        Spectator {
            turn_num: self.turn_num,
            settings: self.settings.clone(),
            view: self.state.spectator_view(&self.settings),
        }
    }

    pub fn turn(&self) -> Option<Turn<T>> {
        let action_requests = self.state.action_requests(&self.settings);

        match action_requests.count() {
            0 => None,
            _ => Some(Turn {
                action_requests,
                actions: Default::default(),
                turn_num: self.turn_num,
            }),
        }
    }

    /// Submit a turn to the game runner and advance the game
    ///
    /// Note: this does not mutate the existing game runner, but instead returns a new one
    ///
    /// # Panics
    ///
    /// This will panic if the turn doesn't have all the players accounted for
    ///
    /// ```should_panic
    /// use std::panic::catch_unwind;
    /// use lttcore::examples::GuessTheNumber;
    /// use lttcore::GameRunnerBuilder;
    ///
    /// let game = GameRunnerBuilder::<GuessTheNumber>::default().build().unwrap();
    /// let turn = game.turn().unwrap();
    /// game.submit_turn(turn);
    /// ```
    #[must_use = "advancing the game does not mutate the existing game runner, but instead returns a new one"]
    pub fn submit_turn(&self, turn: Turn<T>) -> (Self, GameAdvance<T>) {
        assert!(
            turn.is_ready_to_submit(),
            "turn {:?} was not ready to submit, it was missing {:?} players actions",
            turn.number(),
            turn.pending_action_requests().count()
        );

        let (new_state, game_advance) = self.state.advance(
            &self.settings,
            turn.actions.clone().into_iter(),
            &mut self.seed.rng_for_turn(self.turn_num),
        );

        let mut history = self.history.clone();
        history.push_back(HistoryEvent {
            actions: turn.actions,
        });

        (
            Self {
                history,
                state: new_state,
                turn_num: self.turn_num + 1,
                ..self.clone()
            },
            game_advance,
        )
    }
}

impl<T: Play> GameRunnerBuilder<T> {
    pub fn build(&self) -> Result<GameRunner<T>, GameRunnerBuilderError> {
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

        Ok(GameRunner {
            seed,
            settings,
            state,
            history,
            initial_state,
            turn_num,
        })
    }
}
