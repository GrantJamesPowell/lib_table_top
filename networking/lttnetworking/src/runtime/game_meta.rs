use crate::messages::player::FromPlayerMsg;
use crate::runtime::channels::{
    AddConnectionSender, BytesReceiver, FromPlayerMsgWithConnectionIdSender,
};
use crate::runtime::error::GameNotFound;
use crate::runtime::id::{ConnectionId, ConnectionIdSource};
use bytes::Bytes;
use lttcore::utilities::PlayerIndexedData as PID;
use lttcore::{Play, Player};
use tokio::sync::mpsc::unbounded_channel;

#[derive(Debug)]
pub struct PlayerConnection<T: Play> {
    sender: FromPlayerMsgWithConnectionIdSender<T>,
    receiver: BytesReceiver,
    connection_id: ConnectionId,
}

impl<T: Play> PlayerConnection<T> {
    pub async fn send(&self, msg: FromPlayerMsg<T>) -> Result<(), GameNotFound> {
        self.sender
            .send((self.connection_id, msg))
            .map_err(|_| GameNotFound)
    }

    pub async fn next_msg(&mut self) -> Option<Bytes> {
        self.receiver.recv().await
    }
}

#[derive(Debug)]
pub struct ObserverConnection {
    receiver: BytesReceiver,
    connection_id: ConnectionId,
}

impl ObserverConnection {
    pub async fn next_msg(&mut self) -> Option<Bytes> {
        self.receiver.recv().await
    }
}

#[derive(Debug)]
pub struct GameMeta<T: Play> {
    connection_id_source: ConnectionIdSource,
    add_observer_connection_sender: AddConnectionSender,
    add_player_connections_senders: PID<AddConnectionSender>,
    player_inputs: PID<FromPlayerMsgWithConnectionIdSender<T>>,
}

impl<T: Play> GameMeta<T> {
    pub fn new(
        add_observer_connection_sender: AddConnectionSender,
        add_player_connections_senders: PID<AddConnectionSender>,
        player_inputs: PID<FromPlayerMsgWithConnectionIdSender<T>>,
    ) -> Self {
        Self {
            add_observer_connection_sender,
            add_player_connections_senders,
            player_inputs,
            connection_id_source: Default::default(),
        }
    }

    pub fn add_observer(&self) -> ObserverConnection {
        let (sender, receiver) = unbounded_channel();
        let connection_id = self.connection_id_source.next();
        self.add_observer_connection_sender
            .send((connection_id, sender))
            .expect("observer connections is alive as long as game meta is");

        ObserverConnection {
            receiver,
            connection_id,
        }
    }

    pub fn add_player(&self, player: Player) -> Option<PlayerConnection<T>> {
        let sender = self.player_inputs.get(player)?.clone();
        let connection_id = self.connection_id_source.next();
        let (bytes_sender, bytes_receiver) = unbounded_channel::<Bytes>();

        self.add_player_connections_senders
            .get(player)?
            .send((connection_id, bytes_sender))
            .ok()?;

        Some(PlayerConnection {
            connection_id,
            sender,
            receiver: bytes_receiver,
        })
    }
}
