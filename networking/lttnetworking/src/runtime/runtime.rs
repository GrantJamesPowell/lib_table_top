use super::game_meta::GameMeta;
use crate::runtime::{ObserverConnection, PlayerConnection};
use bytes::Bytes;
use dashmap::DashMap;
use lttcore::id::GameId;
use lttcore::{Play, Player};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub type ToByteSink = UnboundedSender<Bytes>;
pub type ByteStream = UnboundedReceiver<Bytes>;

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
