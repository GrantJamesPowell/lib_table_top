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

#[derive(Debug)]
struct GameMeta<T: Play> {
    manage_observer_connections: UnboundedSender<ManageConnections>,
    manage_player_connections: PID<UnboundedSender<ManageConnections>>,
    player_user_mapping: PID<UserId>,
    from_connection_to_player_connections: PID<UnboundedSender<FromConnection<FromPlayerMsg<T>>>>,
}

pub trait Serializer {
    fn serialize<T>(value: &T) -> Bytes;
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
        game_meta
            .manage_observer_connections
            .send(ManageConnections::Add(connection_id.into()))
            .map(|_| connection_id)
            .map_err(|_| {
                self.connections.remove(&connection_id);
                GameNotFound
            })
    }

    pub fn user_id_for_game_player(&self, game_id: GameId, player: Player) -> Option<UserId> {
        self.games
            .get(&game_id)
            .and_then(|meta| meta.player_user_mapping.get(player).copied())
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
