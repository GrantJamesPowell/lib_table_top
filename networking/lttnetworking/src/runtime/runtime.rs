use super::game_meta::GameMeta;
use crate::messages::{game_host::ToGameHostMsg, observer::ToObserverMsg, player::ToPlayerMsg};
use crate::runtime::{
    game_host, ObserverConnection, PlayerConnection,
};

use dashmap::DashMap;
use lttcore::id::GameId;
use lttcore::utilities::PlayerIndexedData as PID;
use lttcore::{GameProgression, Play, Player};
use tokio::sync::mpsc::{unbounded_channel};

#[derive(Debug)]
pub struct Runtime<T: Play> {
    games: DashMap<GameId, GameMeta<T>>,
}

impl<T: Play> Runtime<T> {
    pub fn observe_game(&self, game_id: GameId) -> Option<ObserverConnection> {
        self.games.get(&game_id).map(|meta| meta.add_observer())
    }

    pub fn play_game(&self, game_id: GameId, player: Player) -> Option<PlayerConnection<T>> {
        self.games
            .get(&game_id)
            .and_then(|meta| meta.add_player(player))
    }
}
