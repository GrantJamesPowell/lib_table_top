use super::{DebugMsgs, Play, PlayerSecretInfoUpdates, View};
use crate::{PlayerSet, TurnNum};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameAdvance<T: Play> {
    pub next_players_input_needed: PlayerSet,
    pub public_info_update: <<T as Play>::PublicInfo as View>::Update,
    pub player_secret_info_updates: PlayerSecretInfoUpdates<T>,
    pub debug_msgs: DebugMsgs<T>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumeratedGameAdvance<T: Play> {
    pub turn_num: TurnNum,
    pub game_advance: GameAdvance<T>,
}
