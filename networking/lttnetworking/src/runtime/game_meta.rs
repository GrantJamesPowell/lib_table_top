use crate::connection::FromConnection;
use crate::messages::player::FromPlayerMsg;
use crate::runtime::error::GameNotFound;
use crate::runtime::{ByteStream, ToByteSink};
use bytes::Bytes;
use lttcore::id::ConnectionId;
use lttcore::utilities::PlayerIndexedData as PID;
use lttcore::{Play, Player};
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};

#[derive(Debug)]
pub struct PlayerConnection<T: Play> {
    sink: UnboundedSender<FromConnection<FromPlayerMsg<T>>>,
    stream: ByteStream,
    connection_id: ConnectionId,
}

impl<T: Play> PlayerConnection<T> {
    pub async fn send(&self, msg: FromPlayerMsg<T>) -> Result<(), GameNotFound> {
        self.sink
            .send(FromConnection {
                from: self.connection_id,
                msg,
            })
            .map_err(|_| GameNotFound)
    }

    pub async fn next_msg(&mut self) -> Option<Bytes> {
        self.stream.recv().await
    }
}

#[derive(Debug)]
pub struct ObserverConnection {
    stream: ByteStream,
    connection_id: ConnectionId,
}

impl ObserverConnection {
    pub async fn next_msg(&mut self) -> Option<Bytes> {
        self.stream.recv().await
    }
}

#[derive(Debug)]
pub struct GameMeta<T: Play> {
    add_observer_chan: UnboundedSender<(ConnectionId, ToByteSink)>,
    add_player_chan: PID<UnboundedSender<(ConnectionId, ToByteSink)>>,
    player_inputs: PID<UnboundedSender<FromConnection<FromPlayerMsg<T>>>>,
}

impl<T: Play> GameMeta<T> {
    pub fn add_observer(&self) -> ObserverConnection {
        let (updates_sender, stream) = unbounded_channel();
        let connection_id = ConnectionId::new();
        self.add_observer_chan
            .send((connection_id, updates_sender))
            .expect("observer connections is alive as long as game meta is");

        ObserverConnection {
            stream,
            connection_id,
        }
    }

    pub fn add_player(&self, player: Player) -> Option<PlayerConnection<T>> {
        let sink = self.player_inputs.get(player)?.clone();

        let connection_id = ConnectionId::new();
        let (updates_sender, stream) = unbounded_channel();

        self.add_player_chan
            .get(player)?
            .send((connection_id, updates_sender))
            .ok()?;

        Some(PlayerConnection {
            connection_id,
            sink,
            stream,
        })
    }
}
