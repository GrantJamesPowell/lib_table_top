use crate::pov::{Observe, ObserverPov, Omniscient, OmniscientPov};
use crate::{GameObserver, GamePlayer, GameRunner, Play, Player, PlayerSet};
use std::collections::HashMap;
use std::sync::Arc;

pub struct GameHost<T: Play> {
    game_runner: GameRunner<T>,
    public_info: <T as Play>::PublicInfo,
    action_requests: PlayerSet,
    player_secret_info: HashMap<Player, <T as Play>::PlayerSecretInfo>,
}

impl<T: Play> Observe<T> for GameHost<T> {
    fn observer_pov(&self) -> ObserverPov<'_, T> {
        ObserverPov {
            action_requests: self.action_requests,
            turn_num: self.game_runner.turn_num(),
            settings: &self.game_runner.settings(),
            public_info: &self.public_info,
        }
    }
}

impl<T: Play> Omniscient<T> for GameHost<T> {
    fn omniscient_pov(&self) -> OmniscientPov<'_, T> {
        OmniscientPov {
            game_state: self.game_runner.state(),
            player_secret_info: &self.player_secret_info,
            public_info: &self.public_info,
            settings: self.game_runner.settings(),
            turn_num: self.game_runner.turn_num(),
        }
    }
}

impl<T: Play> From<GameRunner<T>> for GameHost<T> {
    fn from(_game_runner: GameRunner<T>) -> Self {
        todo!()
    }
}

impl<T: Play> GameHost<T> {
    fn into_game_runner(self) -> GameRunner<T> {
        self.game_runner
    }

    fn game_runner(&self) -> &GameRunner<T> {
        &self.game_runner
    }

    fn game_observer(&self) -> GameObserver<T> {
        GameObserver {
            turn_num: self.game_runner.turn_num(),
            action_requests: self.action_requests,
            settings: Arc::clone(self.game_runner.settings_arc()),
            public_info: self.public_info.clone(),
        }
    }

    fn game_players(&self) -> impl Iterator<Item = GamePlayer<T>> + '_ {
        let turn_num = self.game_runner.turn_num();

        self.game_runner
            .players()
            .into_iter()
            .map(move |player| GamePlayer {
                player,
                turn_num,
                action_requests: self.action_requests,
                settings: Arc::clone(self.game_runner.settings_arc()),
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
