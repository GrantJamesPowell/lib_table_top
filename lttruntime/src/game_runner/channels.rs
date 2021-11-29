use super::id::ConnectionId;
use crate::messages::{FromPlayerMsg, ToGameHostMsg, ToObserverMsg, ToPlayerMsg};
use bytes::Bytes;
use lttcore::{encoder::Encoding, utilities::PlayerIndexedData as PID};
use lttcore::{play::Play, PlayerSet};
use serde::Serialize;
use tokio::sync::mpsc::{
    error::{SendError, TryRecvError},
    unbounded_channel, UnboundedReceiver, UnboundedSender,
};

#[derive(Debug, Clone)]
pub struct BytesSender {
    encoding: Encoding,
    sender: UnboundedSender<Bytes>,
}

impl BytesSender {
    pub fn send_to<'a>(senders: impl Iterator<Item = &'a Self>, msg: impl Serialize) {
        let mut json: Option<Bytes> = None;
        let mut pretty_json: Option<Bytes> = None;
        let mut bincode: Option<Bytes> = None;

        for sender in senders {
            let bytes =
                match sender.encoding() {
                    enc @ Encoding::Json => json
                        .get_or_insert_with(|| enc.serialize(&msg).expect("Could serialize msg")),
                    enc @ Encoding::Bincode => bincode
                        .get_or_insert_with(|| enc.serialize(&msg).expect("Could serialize msg")),
                    enc @ Encoding::PrettyJson => pretty_json
                        .get_or_insert_with(|| enc.serialize(&msg).expect("Could serialize msg")),
                };

            let _ = sender.send_bytes(bytes.clone());
        }
    }

    pub fn is_closed(&self) -> bool {
        self.sender.is_closed()
    }

    pub fn send_bytes(&self, bytes: Bytes) -> Result<(), SendError<Bytes>> {
        self.sender.send(bytes)
    }

    pub fn encoding(&self) -> Encoding {
        self.encoding
    }
}

#[derive(Debug)]
pub struct BytesReceiver {
    encoding: Encoding,
    receiver: UnboundedReceiver<Bytes>,
}

impl BytesReceiver {
    pub fn encoding(&self) -> Encoding {
        self.encoding
    }

    pub async fn next_bytes(&mut self) -> Option<Bytes> {
        self.receiver.recv().await
    }

    pub fn try_next_bytes(&mut self) -> Result<Bytes, TryRecvError> {
        self.receiver.try_recv()
    }
}

pub type ToPlayerMsgSender<T> = UnboundedSender<ToPlayerMsg<T>>;
pub type ToPlayerMsgWithConnectionIdSender<T> = UnboundedSender<(ConnectionId, ToPlayerMsg<T>)>;
pub type ToPlayerMsgReceiver<T> = UnboundedReceiver<ToPlayerMsg<T>>;
pub type ToPlayerMsgWithConnectionIdReceiver<T> = UnboundedReceiver<(ConnectionId, ToPlayerMsg<T>)>;

pub type FromPlayerMsgSender<T> = UnboundedSender<FromPlayerMsg<T>>;
pub type FromPlayerMsgWithConnectionIdSender<T> = UnboundedSender<(ConnectionId, FromPlayerMsg<T>)>;
pub type FromPlayerMsgReceiver<T> = UnboundedReceiver<FromPlayerMsg<T>>;
pub type FromPlayerMsgWithConnectionIdReceiver<T> =
    UnboundedReceiver<(ConnectionId, FromPlayerMsg<T>)>;

pub type AddConnectionSender = UnboundedSender<(ConnectionId, BytesSender)>;
pub type AddConnectionReceiver = UnboundedReceiver<(ConnectionId, BytesSender)>;

pub type ToObserverMsgSender<T> = UnboundedSender<ToObserverMsg<T>>;
pub type ToObserverMsgReceiver<T> = UnboundedReceiver<ToObserverMsg<T>>;

pub type ToGameHostMsgSender<T> = UnboundedSender<ToGameHostMsg<T>>;
pub type ToGameHostMsgReceiver<T> = UnboundedReceiver<ToGameHostMsg<T>>;

pub fn add_connection() -> (AddConnectionSender, AddConnectionReceiver) {
    unbounded_channel()
}

pub fn to_game_host<T: Play>() -> (ToGameHostMsgSender<T>, ToGameHostMsgReceiver<T>) {
    unbounded_channel()
}

pub fn to_observer<T: Play>() -> (ToObserverMsgSender<T>, ToObserverMsgReceiver<T>) {
    unbounded_channel()
}

pub fn from_player_msgs<T: Play>(
    players: PlayerSet,
) -> (
    PID<FromPlayerMsgWithConnectionIdSender<T>>,
    PID<FromPlayerMsgWithConnectionIdReceiver<T>>,
) {
    players
        .into_iter()
        .map(|player| {
            let (sender, receiver) = unbounded_channel();
            ((player, sender), (player, receiver))
        })
        .unzip()
}

pub fn add_player_connections(
    players: PlayerSet,
) -> (PID<AddConnectionSender>, PID<AddConnectionReceiver>) {
    players
        .into_iter()
        .map(|player| {
            let (sender, receiver) = add_connection();
            ((player, sender), (player, receiver))
        })
        .unzip()
}

pub fn to_players<T: Play>(
    players: PlayerSet,
) -> (PID<ToPlayerMsgSender<T>>, PID<ToPlayerMsgReceiver<T>>) {
    players
        .into_iter()
        .map(|player| {
            let (sender, receiver) = unbounded_channel::<ToPlayerMsg<T>>();
            ((player, sender), (player, receiver))
        })
        .unzip()
}

pub fn bytes_channels(encoding: Encoding) -> (BytesSender, BytesReceiver) {
    let (sender, receiver) = unbounded_channel();
    (
        BytesSender { sender, encoding },
        BytesReceiver { receiver, encoding },
    )
}
