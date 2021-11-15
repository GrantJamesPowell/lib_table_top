use super::id::ConnectionId;
use crate::messages::{FromPlayerMsg, ToGameHostMsg, ToObserverMsg, ToPlayerMsg};
use bytes::Bytes;
use lttcore::utilities::PlayerIndexedData as PID;
use lttcore::{Play, PlayerSet};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

pub type BytesSender = UnboundedSender<Bytes>;
pub type BytesReceiver = UnboundedReceiver<Bytes>;

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
