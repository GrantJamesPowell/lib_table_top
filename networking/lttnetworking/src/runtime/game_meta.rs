use crate::connection::{
    Connections, FromConnection,
    ManageConnections::{self, *},
};
use crate::messages::player::FromPlayerMsg;
use crate::runtime::error::{GameNotFound, PlayerNotFound};
use lttcore::id::{ConnectionId};
use lttcore::utilities::PlayerIndexedData as PID;
use lttcore::{Play, Player};
use tokio::sync::mpsc::UnboundedSender;

#[derive(Debug)]
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
pub struct GameMeta<T: Play> {
    manage_observer_connections: UnboundedSender<ManageConnections>,
    manage_player_connections: PID<UnboundedSender<ManageConnections>>,
    player_inputs: PID<UnboundedSender<FromConnection<FromPlayerMsg<T>>>>,
}

impl<T: Play> GameMeta<T> {
    pub fn add_observers(&self, connections: impl Into<Connections>) {
        let connections = connections.into();
        self.manage_observer_connections
            .send(Add(connections))
            .expect("observer connections is alive as long as game meta is");
    }

    pub fn add_player(
        &self,
        player: Player,
        connection_id: ConnectionId,
    ) -> Result<PlayerInput<T>, PlayerNotFound> {
        let chan = self
            .player_inputs
            .get(player)
            .ok_or(PlayerNotFound)?
            .clone();

        Ok(PlayerInput {
            connection_id,
            chan,
        })
    }
}
