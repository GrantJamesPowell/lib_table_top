use super::channels::{
    bytes_channels, AddConnectionSender, BytesReceiver, FromPlayerMsgWithConnectionIdSender,
};
use super::id::{ConnectionId, ConnectionIdSource};
use crate::error::GameNotFound;
use crate::messages::FromPlayerMsg;
use bytes::Bytes;
use lttcore::play::{Play, Player};
use lttcore::{encoder::Encoding, utilities::PlayerIndexedData as PID};

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

    pub async fn next_bytes(&mut self) -> Option<Bytes> {
        self.receiver.next_bytes().await
    }
}

#[derive(Debug)]
pub struct ObserverConnection {
    receiver: BytesReceiver,
    connection_id: ConnectionId,
}

impl ObserverConnection {
    pub async fn next_msg(&mut self) -> Option<Bytes> {
        self.receiver.next_bytes().await
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

    pub fn add_observer(&self, encoding: Encoding) -> ObserverConnection {
        let connection_id = self.connection_id_source.next();
        let (bytes_sender, bytes_receiver) = bytes_channels(encoding);
        self.add_observer_connection_sender
            .send((connection_id, bytes_sender))
            .expect("observer connections is alive as long as game meta is");

        ObserverConnection {
            receiver: bytes_receiver,
            connection_id,
        }
    }

    pub fn add_player(&self, player: Player, encoding: Encoding) -> Option<PlayerConnection<T>> {
        let sender = self.player_inputs.get(player)?.clone();
        let connection_id = self.connection_id_source.next();
        let (bytes_sender, bytes_receiver) = bytes_channels(encoding);

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
