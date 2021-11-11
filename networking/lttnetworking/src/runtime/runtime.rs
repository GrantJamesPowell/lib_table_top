use crate::connection::{ManageConnections, ToConnections};
use bytes::Bytes;
use dashmap::DashMap;
use lttcore::id::{ConnectionId, GameId, UserId};
use lttcore::utilities::PlayerIndexedData;
use lttcore::Player;
use serde::Serialize;
use tokio::sync::mpsc::UnboundedSender;

#[derive(Debug)]
struct GameMeta {
    manage_observer_connections: UnboundedSender<ManageConnections>,
    manage_player_connections: UnboundedSender<ManageConnections>,
    player_user_mapping: PlayerIndexedData<UserId>,
}

pub trait Serializer {
    fn serialize<T>(value: &T) -> Bytes;
}

#[derive(Debug)]
struct Runtime {
    games: DashMap<GameId, GameMeta>,
    connections: DashMap<ConnectionId, UnboundedSender<Bytes>>,
}

impl Runtime {
    pub fn user_id_for_game_player(&self, game_id: GameId, player: Player) -> Option<UserId> {
        self.games
            .get(&game_id)
            .and_then(|meta| meta.player_user_mapping.get(player).copied())
    }

    pub async fn send_to_connections<T: Serialize, S: Serializer>(
        &self,
        ToConnections { to, msg }: ToConnections<T>,
    ) {
        let bytes = S::serialize(&msg);

        for connection_id in to {
            if let Some(sender) = self.connections.get(&connection_id) {
                let _ = sender.send(bytes.clone());
            }
        }
    }
}
