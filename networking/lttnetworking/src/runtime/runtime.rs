use super::game_meta::GameMeta;
use crate::connection::{FromConnection, ManageConnections, ToConnections};
use crate::messages::player::FromPlayerMsg;
use bytes::Bytes;
use dashmap::DashMap;
use lttcore::id::{ConnectionId, GameId, UserId};
use lttcore::utilities::PlayerIndexedData as PID;
use lttcore::{Play, Player};
use serde::Serialize;
use tokio::sync::mpsc::UnboundedSender;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameNotFound;

pub trait Serializer {
    fn serialize<T>(value: &T) -> Bytes;
}

pub struct PlayerInput<T: Play> {
    chan: UnboundedSender<FromConnection<FromPlayerMsg<T>>>,
    connection_id: ConnectionId,
}

impl<T: Play> PlayerInput<T> {
    fn send(&self, msg: FromPlayerMsg<T>) -> Result<(), GameNotFound> {
        self.chan
            .send(FromConnection {
                from: self.connection_id,
                msg,
            })
            .map_err(|_| GameNotFound)
    }
}

#[derive(Debug)]
struct Runtime<T: Play> {
    games: DashMap<GameId, GameMeta<T>>,
    connections: DashMap<ConnectionId, UnboundedSender<Bytes>>,
}

impl<T: Play> Runtime<T> {
    pub fn observe_game(
        &self,
        game_id: GameId,
        conn: UnboundedSender<Bytes>,
    ) -> Result<ConnectionId, GameNotFound> {
        let game_meta = self.games.get(&game_id).ok_or(GameNotFound)?;
        let connection_id = ConnectionId::new();
        self.connections.insert(connection_id, conn);
        game_meta.add_observers(connection_id);
        Ok(connection_id)
    }

    pub async fn send_to_connections<Msg: Serialize, Ser: Serializer>(
        &self,
        ToConnections { to, msg }: ToConnections<Msg>,
    ) {
        let bytes = Ser::serialize(&msg);

        for id in to {
            if let Some(conn) = self.connections.get(&id) {
                let _ = conn.send(bytes.clone());
            }
        }
    }
}
