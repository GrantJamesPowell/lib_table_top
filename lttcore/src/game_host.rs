use crate::pov::{Observe, ObserverPov, Omniscient, OmniscientPov};
use crate::{GameObserver, GamePlayer, GameProgression, Play, Player, PlayerSet};
use std::collections::HashMap;
use std::sync::Arc;

pub struct GameHost<T: Play> {
    game_progression: GameProgression<T>,
    public_info: <T as Play>::PublicInfo,
    action_requests: PlayerSet,
    player_secret_info: HashMap<Player, <T as Play>::PlayerSecretInfo>,
}

impl<T: Play> Observe<T> for GameHost<T> {
    fn observer_pov(&self) -> ObserverPov<'_, T> {
        ObserverPov {
            action_requests: self.action_requests,
            turn_num: self.game_progression.turn_num(),
            settings: &self.game_progression.settings(),
            public_info: &self.public_info,
        }
    }
}

impl<T: Play> Omniscient<T> for GameHost<T> {
    fn omniscient_pov(&self) -> OmniscientPov<'_, T> {
        OmniscientPov {
            game_state: self.game_progression.state(),
            player_secret_info: &self.player_secret_info,
            public_info: &self.public_info,
            settings: self.game_progression.settings(),
            turn_num: self.game_progression.turn_num(),
        }
    }
}

impl<T: Play> From<GameProgression<T>> for GameHost<T> {
    fn from(game_progression: GameProgression<T>) -> Self {
        let public_info = game_progression.public_info();
        let player_secret_info = game_progression.player_secret_info();
        let action_requests = game_progression.which_players_input_needed();

        Self {
            game_progression,
            public_info,
            player_secret_info,
            action_requests,
        }
    }
}

impl<T: Play> GameHost<T> {
    pub fn new(game_progression: impl Into<GameProgression<T>>) -> Self {
        let game_progression = game_progression.into();
        game_progression.into()
    }

    fn into_game_progression(self) -> GameProgression<T> {
        self.game_progression
    }

    fn game_progression(&self) -> &GameProgression<T> {
        &self.game_progression
    }

    fn game_observer(&self) -> GameObserver<T> {
        GameObserver {
            turn_num: self.game_progression.turn_num(),
            action_requests: self.action_requests,
            settings: Arc::clone(self.game_progression.settings_arc()),
            public_info: self.public_info.clone(),
        }
    }

    fn game_players(&self) -> impl Iterator<Item = GamePlayer<T>> + '_ {
        let turn_num = self.game_progression.turn_num();

        self.game_progression
            .players()
            .into_iter()
            .map(move |player| GamePlayer {
                player,
                turn_num,
                action_requests: self.action_requests,
                settings: Arc::clone(self.game_progression.settings_arc()),
                public_info: self.public_info.clone(),
                secret_info: self.player_secret_info[&player].clone(),
            })
    }

    // fn submit_action_response(
    //     &mut self,
    //     _player: Player,
    //     _action_response: impl Into<ActionResponse<<T as Play>::Action>>,
    // ) -> Option<GameHostUpdates<T>> {
    //     todo!()
    // }
}
