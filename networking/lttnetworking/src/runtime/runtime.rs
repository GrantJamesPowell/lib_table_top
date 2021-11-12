use super::game_meta::GameMeta;

use crate::runtime::error::GameNotFound;
use bytes::Bytes;
use dashmap::DashMap;
use lttcore::id::{ConnectionId, GameId};

use lttcore::Play;
use serde::Serialize;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub trait Encoder {
    fn serialize<T>(value: &T) -> Bytes;
}

pub type ToByteSink = UnboundedSender<Bytes>;
pub type ByteStream = UnboundedReceiver<Bytes>;

#[derive(Debug)]
pub struct Runtime<T: Play> {
    games: DashMap<GameId, GameMeta<T>>,
}

impl<T: Play> Runtime<T> {
    // pub fn observe_game(
    //     &self,
    //     game_id: GameId,
    //     conn: UnboundedSender<Bytes>,
    // ) -> Result<ConnectionId, GameNotFound> {
    //     let game_meta = self.games.get(&game_id).ok_or(GameNotFound)?;
    //     let connection_id = ConnectionId::new();
    //     game_meta.add_observers(connection_id);
    //     Ok(connection_id)
    // }
}
