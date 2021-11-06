use crate::connection::{ConnectionId, ConnectionMsg, ManageConnections};
use crate::server::messages::{GameHostMsg, ObserverMsg};
use lttcore::Play;
use smallvec::SmallVec;
use tokio::sync::mpsc::{Receiver, Sender};

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

pub async fn observer_connections<T: Play, M, TC, TGH>(
    mut mailbox: Receiver<Mail<T>>,
    to_client: Sender<ConnectionMsg<ObserverMsg<T>>>,
    to_game_host: Sender<GameHostMsg<T>>,
) -> anyhow::Result<()> {
    let mut connections: SmallVec<[Conn; 4]> = Default::default();
    let mut state_requested = false;

    while let Some(mail) = mailbox.recv().await {
        match mail {
            OM(msg @ SyncState(_)) => {
                state_requested = false;

                let to = connections
                    .iter()
                    .filter(|conn| conn.needs_state)
                    .map(|conn| conn.id)
                    .collect();

                to_client.send(ConnectionMsg { to, msg }).await?;

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

                to_client.send(ConnectionMsg { to, msg }).await?;
            }
            MC(Add(new_conns)) => {
                connections.extend(new_conns.into_iter().map(|id| Conn {
                    id,
                    needs_state: true,
                }));

                if !state_requested {
                    to_game_host.send(RequestObserverState).await?;
                    state_requested = true;
                }
            }
            MC(Remove(remove_conns)) => connections.retain(|conn| !remove_conns.contains(conn.id)),
        }
    }

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::{observer_connections, Mail};
//     use lttcore::examples::GuessTheNumber;
//     use tokio::sync::mpsc::unbounded_channel;
//
//     #[tokio::test]
//     async fn test_observer_connections_work() {
//         let (_to_mailbox, mut mailbox) = unbounded_channel::<Mail<GuessTheNumber>>();
//         let (to_client, _client) = unbounded_channel();
//         let (to_game_host, _game_host) = unbounded_channel();
//
//         let handle = tokio::spawn(observer_connections(
//             || async move { mailbox.recv().await },
//             move |msg| async {
//                 let _ = to_client.clone().send(msg);
//                 return ();
//             },
//             move |msg| async {
//                 let _ = to_game_host.clone().send(msg);
//                 return ();
//             },
//         ));
//
//         handle.await;
//     }
// }
