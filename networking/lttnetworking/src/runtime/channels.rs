use crate::messages::{
    game_host::ToGameHostMsg,
    observer::ToObserverMsg,
    player::{FromPlayerMsg, ToPlayerMsg},
};
use crate::runtime::id::ConnectionId;
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

fn to_players<T: Play>(
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
