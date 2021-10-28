use crate::game_runner::ActionRequests;
use crate::play::{DebugMsgs, PlayerSecretInfoUpdates};
use crate::{ActionResponse, GameRunner, Play, Player, View};
use std::collections::HashMap;

pub struct GameHost<T: Play> {
    game_runner: GameRunner<T>,
    public_info: <T as Play>::PublicInfo,
    action_requests: ActionRequests<T>,
    player_secret_info: HashMap<Player, <T as Play>::PlayerSecretInfo>,
}

pub struct Updates<T: Play> {
    public_info_update: <<T as Play>::PublicInfo as View>::Update,
    secret_info_updates: PlayerSecretInfoUpdates<T>,
    debug_msgs: DebugMsgs<T>,
}

impl<T: Play> GameHost<T> {
    fn observer(&self) -> Observer<'_, T> {
        Observer {
            turn_num: self.game_runner.turn_num(),
            settings: self.game_runner.settings(),
            public_info: &self.public_info,
        }
    }

    fn action_requests(&self) -> impl Iterator<Item = ActionRequest<'_, T>> + '_ {
        let turn_num = self.game_runner.turn_num();
        let settings = self.game_runner.settings();

        self.action_requests
            .unaccounted_for_players()
            .into_iter()
            .map(move |player| {
                let secret_info = &self.player_secret_info[&player];

                ActionRequest {
                    player,
                    turn_num,
                    settings,
                    secret_info,
                    public_info: &self.public_info,
                }
            })
    }

    pub fn submit_action_response(
        &mut self,
        player: Player,
        action_response: impl Into<ActionResponse<<T as Play>::Action>>,
    ) -> Option<Updates<T>> {
        self.action_requests.add_action(player, action_response);

        todo!()
    }
}

pub struct Observer<'a, T: Play> {
    pub turn_num: u64,
    pub settings: &'a <T as Play>::Settings,
    pub public_info: &'a <T as Play>::PublicInfo,
}

pub struct ActionRequest<'a, T: Play> {
    pub turn_num: u64,
    pub player: Player,
    pub settings: &'a <T as Play>::Settings,
    pub secret_info: &'a <T as Play>::PlayerSecretInfo,
    pub public_info: &'a <T as Play>::PublicInfo,
}
