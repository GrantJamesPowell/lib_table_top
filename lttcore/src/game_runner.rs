use im::Vector;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::collections::HashMap;
use std::sync::Arc;

use crate::play::{ActionResponse, GameAdvance};
use crate::{NumberOfPlayers, Play, Player, PlayerSet, Seed};

use thiserror::Error;

type Actions<T> = SmallVec<[(Player, ActionResponse<<T as Play>::Action>); 2]>;

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

#[derive(Debug, Clone)]
pub struct Turn<T: Play> {
    turn_num: u64,
    action_requests: PlayerSet,
    actions: Actions<T>,
}

impl<T: Play> Turn<T> {
    pub fn number(&self) -> u64 {
        self.turn_num
    }

    pub fn pending_action_requests(&self) -> PlayerSet {
        self.action_requests
            .players()
            .filter(|player| {
                self.actions
                    .binary_search_by_key(&player, |(p, _)| &*p)
                    .is_err()
            })
            .collect()
    }

    /// Add an action to the turn
    ///
    /// # Panics
    ///
    /// This panics if the `Player` isn't in the turn
    /// ```should_panic
    /// use lttcore::examples::{GuessTheNumber, guess_the_number::Guess};
    /// use lttcore::{Player, GameRunnerBuilder};
    ///
    /// let player: Player = 255.into();
    /// let game = GameRunnerBuilder::<GuessTheNumber>::default().build().unwrap();
    /// let mut turn = game.turn().unwrap();
    ///
    /// let guess: Guess = 42.into();
    /// turn.add_action(player, guess);
    /// ```
    pub fn add_action(
        &mut self,
        player: impl Into<Player>,
        action_response: impl Into<ActionResponse<<T as Play>::Action>>,
    ) {
        let player = player.into();
        let action_response = action_response.into();

        assert!(
            self.action_requests.contains(player),
            "{:?} was added to turn {:?}, but player isn't in the turn",
            player,
            self.turn_num
        );

        match self.actions.binary_search_by_key(&player, |(p, _)| *p) {
            Ok(existing_action_index) => {
                self.actions[existing_action_index] = (player, action_response);
            }
            Err(index) => {
                self.actions.insert(index, (player, action_response));
            }
        }
    }

    pub fn is_ready_to_submit(&self) -> bool {
        self.pending_action_requests().is_empty()
    }
}

#[derive(Error, Debug, Clone, Copy)]
pub enum SubmitError {
    #[error("can't add action for player that was not requested")]
    InvalidPlayer,
    #[error("can't submit turn with invalid actions")]
    InvalidTurn,
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

    pub fn spectator_view(&self) -> <T as Play>::SpectatorView {
        self.state.spectator_view(&self.settings)
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

    pub fn submit_turn(&self, turn: Turn<T>) -> Result<(Self, GameAdvance<T>), SubmitError> {
        if !turn.is_ready_to_submit() || (turn.turn_num != self.turn_num) {
            return Err(SubmitError::InvalidTurn);
        }

        let (new_state, game_advance) = self.state.advance(
            &self.settings,
            turn.actions.clone().into_iter(),
            &mut self.seed.rng_for_turn(self.turn_num),
        );

        let mut history = self.history.clone();
        history.push_back(HistoryEvent {
            actions: turn.actions,
        });

        Ok((
            Self {
                history,
                state: new_state,
                turn_num: self.turn_num + 1,
                ..self.clone()
            },
            game_advance,
        ))
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
