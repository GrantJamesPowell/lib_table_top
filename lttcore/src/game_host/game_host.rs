use super::action_collector::ActionCollector;
use crate::play::EnumeratedGameAdvance;
use crate::pov::{Observe, ObserverPov, Omniscient, OmniscientPov};
use crate::{ActionResponse, GameObserver, GamePlayer, GameProgression, Play, Player};
use std::borrow::Cow;
use std::sync::Arc;

pub struct GameHost<T: Play> {
    game_progression: GameProgression<T>,
    action_collector: ActionCollector<T>,
}

impl<T: Play> Observe<T> for GameHost<T> {
    fn observer_pov(&self) -> ObserverPov<'_, T> {
        ObserverPov {
            action_requests: self.action_collector.all_players(),
            turn_num: self.game_progression.turn_num(),
            settings: Cow::Borrowed(&self.game_progression.settings()),
            public_info: Cow::Owned(self.game_progression.public_info()),
        }
    }
}

impl<T: Play> Omniscient<T> for GameHost<T> {
    fn omniscient_pov(&self) -> OmniscientPov<'_, T> {
        OmniscientPov {
            game_progression: Cow::Borrowed(&self.game_progression),
        }
    }
}

impl<T: Play> From<GameProgression<T>> for GameHost<T> {
    fn from(game_progression: GameProgression<T>) -> Self {
        let action_collector: ActionCollector<T> =
            game_progression.which_players_input_needed().into();

        Self {
            game_progression,
            action_collector,
        }
    }
}

impl<T: Play> GameHost<T> {
    pub fn new(game_progression: impl Into<GameProgression<T>>) -> Self {
        let game_progression: GameProgression<T> = game_progression.into();
        game_progression.into()
    }

    pub fn into_game_progression(self) -> GameProgression<T> {
        self.game_progression
    }

    pub fn game_progression(&self) -> &GameProgression<T> {
        &self.game_progression
    }

    pub fn game_observer(&self) -> GameObserver<T> {
        GameObserver {
            turn_num: self.game_progression.turn_num(),
            action_requests: self.action_collector.all_players(),
            settings: Arc::clone(self.game_progression.settings_arc()),
            public_info: self.game_progression().public_info(),
        }
    }

    pub fn game_players(&self) -> impl Iterator<Item = GamePlayer<T>> + '_ {
        let turn_num = self.game_progression.turn_num();
        let public_info = self.game_progression.public_info();
        let mut player_secret_info = self.game_progression.player_secret_info();

        self.game_progression
            .players()
            .into_iter()
            .map(move |player| GamePlayer {
                player,
                turn_num,
                action_requests: self.action_collector.all_players(),
                settings: Arc::clone(self.game_progression.settings_arc()),
                public_info: public_info.clone(),
                secret_info: player_secret_info
                    .remove(&player)
                    .expect("game progression did not return secret info for a player"),
            })
    }

    pub fn submit_action_response(
        &mut self,
        player: Player,
        action_response: impl Into<ActionResponse<<T as Play>::Action>>,
    ) -> Option<EnumeratedGameAdvance<T>> {
        self.action_collector.add_action(player, action_response);

        if self.action_collector.is_ready() {
            let (new_progression, game_advance) = self
                .game_progression
                .submit_actions(self.action_collector.take_actions());

            self.game_progression = new_progression;
            Some(game_advance)
        } else {
            None
        }
    }
}
