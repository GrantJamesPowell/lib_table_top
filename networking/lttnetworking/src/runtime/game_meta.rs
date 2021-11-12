use crate::connection::{
    Connections, FromConnection,
    ManageConnections::{self, *},
};
use crate::messages::player::FromPlayerMsg;
use lttcore::id::{ConnectionId, GameId, UserId};
use lttcore::utilities::PlayerIndexedData as PID;
use lttcore::Play;
use tokio::sync::mpsc::UnboundedSender;

#[derive(Debug)]
pub struct GameMeta<T: Play> {
    manage_observer_connections: UnboundedSender<ManageConnections>,
    manage_player_connections: PID<UnboundedSender<ManageConnections>>,
    player_user_mapping: PID<UserId>,
    to_player_connections: PID<UnboundedSender<FromConnection<FromPlayerMsg<T>>>>,
}

impl<T: Play> GameMeta<T> {
    pub fn add_observers(&self, connections: impl Into<Connections>) {
        let connections = connections.into();
        self.manage_observer_connections
            .send(Add(connections))
            .expect("observer connections is alive as long as game meta is");
    }
}
