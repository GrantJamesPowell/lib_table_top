use super::messages::{GameHostMsg, ObserverMsg};
use crate::connection::{ConnectionId, ConnectionMsg, ManageConnections};
use futures_util::{Sink, SinkExt, Stream, StreamExt};
use lttcore::Play;
use smallvec::SmallVec;

struct Conn {
    id: ConnectionId,
    needs_state: bool,
}

pub enum Mail<T: Play> {
    OM(ObserverMsg<T>),
    MC(ManageConnections),
}

use GameHostMsg::*;
use Mail::*;
use ManageConnections::*;
use ObserverMsg::*;

pub async fn observer_connections<T: Play>(
    mut mailbox: impl Stream<Item = Mail<T>> + Unpin,
    mut to_client: impl Sink<ConnectionMsg<ObserverMsg<T>>> + Unpin,
    mut to_game_host: impl Sink<GameHostMsg<T>> + Unpin,
) {
    let mut connections: SmallVec<[Conn; 4]> = Default::default();
    let mut state_requested = false;

    while let Some(mail) = mailbox.next().await {
        match mail {
            OM(msg @ SyncState(_)) => {
                state_requested = false;

                let to = connections
                    .iter()
                    .filter(|conn| conn.needs_state)
                    .map(|conn| conn.id)
                    .collect();

                send(&mut to_client, ConnectionMsg { to, msg }).await;

                for conn in connections.iter_mut() {
                    conn.needs_state = false;
                }
            }
            OM(msg @ Update(_)) => {
                let to = connections
                    .iter()
                    .filter(|conn| !conn.needs_state)
                    .map(|conn| conn.id)
                    .collect();

                send(&mut to_client, ConnectionMsg { to, msg }).await;
            }
            MC(Add(new_conns)) => {
                connections.extend(new_conns.into_iter().map(|id| Conn {
                    id,
                    needs_state: true,
                }));

                if !state_requested {
                    send(&mut to_game_host, RequestObserverState).await;
                    state_requested = true;
                }
            }
            MC(Remove(remove_conns)) => connections.retain(|conn| !remove_conns.contains(conn.id)),
        }
    }
}

async fn send<T>(sink: &mut (impl Sink<T> + Unpin), msg: T) {
    match sink.send(msg).await {
        Ok(_) => {},
        Err(_) => panic!("ObserverConnections can't send on a channel, and I can't figure out how to force Stream::Error to be debug")
    }
}
