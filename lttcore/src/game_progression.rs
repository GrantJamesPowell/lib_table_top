use im::Vector;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::collections::HashMap;
use std::sync::Arc;

use crate::play::{ActionResponse, DebugMsgs, GameAdvance, PlayerSecretInfoUpdates};
use crate::pov::ObserverUpdate;
use crate::{NumberOfPlayers, Play, Player, PlayerSet, Scenario, Seed};

pub type Actions<T> = SmallVec<[(Player, ActionResponse<<T as Play>::Action>); 2]>;
pub type PlayerSercretInfos<T> = HashMap<Player, <T as Play>::PlayerSecretInfo>;

#[derive(Builder, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[builder(setter(into, strip_option), build_fn(skip))]
// https://stackoverflow.com/questions/61473323/cannot-infer-type-for-type-parameter-when-deriving-deserialize-for-a-type-with-a
// It looks like this is required because serde is trying to provide redundant bounds to T, but
// since T and all of T's assoc'd types are deserialize, I think everything is gravy
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

pub struct GameProgressionAdvance<T: Play> {
    pub observer_update: ObserverUpdate<T>,
    pub player_secret_info_updates: PlayerSecretInfoUpdates<T>,
    pub debug_msgs: DebugMsgs<T>,
}

impl<T: Play> GameProgressionAdvance<T> {
    fn from(game_advance: GameAdvance<T>, turn_num: u64, action_requests: PlayerSet) -> Self {
        Self {
            debug_msgs: game_advance.debug_msgs,
            player_secret_info_updates: game_advance.player_secret_info_updates,
            observer_update: ObserverUpdate {
                turn_num,
                action_requests,
                public_info_update: game_advance.public_info_update,
            },
        }
    }
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

impl<T: Play> GameProgression<T> {
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
    pub fn submit_actions(&self, actions: Actions<T>) -> (Self, GameProgressionAdvance<T>) {
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

        let game_advance = GameProgressionAdvance::from(
            game_advance,
            self.turn_num + 1,
            new_game_progression.which_players_input_needed(),
        );

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
