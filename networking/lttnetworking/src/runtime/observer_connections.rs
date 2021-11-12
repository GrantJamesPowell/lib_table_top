use crate::messages::{
    game_host::ToGameHostMsg::{self, *},
    observer::ToObserverMsg::{self, *},
};
use crate::runtime::{Encoder, ToByteSink};
use lttcore::{id::ConnectionId, Play};
use serde::Serialize;
use smallvec::SmallVec;
use tokio::select;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub struct Inbox<T: Play> {
    pub from_game_host: UnboundedReceiver<ToObserverMsg<T>>,
    pub from_runtime: UnboundedReceiver<(ConnectionId, ToByteSink)>,
}

pub struct Outbox<T: Play> {
    pub to_game_host: UnboundedSender<ToGameHostMsg<T>>,
}

#[derive(Debug)]
struct Conn {
    sink: ToByteSink,
    id: ConnectionId,
    in_sync: bool,
}

#[derive(Debug, Default)]
struct State<E: Encoder> {
    conns: SmallVec<[Conn; 2]>,
    _phantom: std::marker::PhantomData<E>,
}

impl<E: Encoder> State<E> {
    fn send_to<T: Serialize>(&mut self, msg: &T, f: impl Fn(&mut Conn) -> bool) {
        let bytes = E::serialize(msg).expect("All game messages are serializable");
        self.conns.retain(|conn| {
            if f(conn) {
                conn.sink.send(bytes.clone()).is_ok()
            } else {
                !conn.sink.is_closed()
            }
        });
    }

    fn are_all_in_sync(&self) -> bool {
        self.conns.iter().all(|conn| conn.in_sync)
    }
}

pub async fn observer_connections<T: Play, E: Encoder>(
    mut inbox: Inbox<T>,
    outbox: Outbox<T>,
) -> anyhow::Result<()> {
    let mut state: State<E> = State {
        conns: Default::default(),
        _phantom: std::marker::PhantomData,
    };

    loop {
        select! {
            Some((id, sink)) = inbox.from_runtime.recv() => {
                if state.are_all_in_sync() {
                    outbox.to_game_host.send(RequestObserverState)?;
                }
                state.conns.push(Conn {
                    id,
                    sink,
                    in_sync: false
                })
            }
            Some(msg) = inbox.from_game_host.recv() => {
                 match msg {
                     SyncState(_) => {
                         state.send_to(&msg, |conn| {
                             if conn.in_sync {
                                 false
                             } else {
                                 conn.in_sync = true;
                                 true
                             }
                         });
                     }
                     Update(_) => {
                         state.send_to(&msg, |conn| conn.in_sync);
                     }
                     GameOver => {
                         state.send_to(&msg, |_conn| true);
                         break
                     }
                 }

            }
        }
    }

    Ok(())
}

// #[cfg(test)]
// // mod tests {
// //     use super::*;
// //     use lttcore::examples::{
// //         guess_the_number::{Guess, Settings},
// //         GuessTheNumber,
// //     };
// //     use lttcore::{play::ActionResponse, pov::ObserverUpdate};
// //     use lttcore::{GameObserver, GameProgression};
// //
// //     use tokio::sync::mpsc::error::TryRecvError;
// //     use tokio::sync::mpsc::unbounded_channel;
// //     use tokio::time::{sleep, Duration};
// //
// //     #[tokio::test]
// //     async fn test_observer_connections() {
// //         let (inbox, outbox, mut mailbox_handles) = setup_test_infra::<GuessTheNumber>();
// //         let (_game_progression, game_observer, observer_update) = setup_guess_the_number();
// //
// //         let connection_id_1 = ConnectionId::new();
// //         let connection_id_2 = ConnectionId::new();
// //         let connection_id_3 = ConnectionId::new();
// //
// //         let _handle = tokio::spawn(observer_connections(inbox, outbox));
// //
// //         // On the first connections added, it sends a request for the game state
// //         mailbox_handles
// //             .to_from_runtime
// //             .send(Add(connection_id_1.into()))
// //             .unwrap();
// //         assert_eq!(
// //             mailbox_handles.from_to_game_host.recv().await,
// //             Some(RequestObserverState)
// //         );
// //
// //         // On the second, it does not re-request the game state
// //         mailbox_handles
// //             .to_from_runtime
// //             .send(Add(connection_id_2.into()))
// //             .unwrap();
// //         sleep(Duration::from_millis(50)).await;
// //         assert_eq!(
// //             mailbox_handles.from_to_game_host.try_recv(),
// //             Err(TryRecvError::Empty)
// //         );
// //
// //         // If an update arrives it doesn't get sent out to connections awaiting the full sync
// //         mailbox_handles
// //             .to_from_game_host
// //             .send(observer_update.clone().into())
// //             .unwrap();
// //         assert_eq!(
// //             mailbox_handles.from_to_connections.recv().await.unwrap(),
// //             ToConnections {
// //                 // Empty
// //                 to: Default::default(),
// //                 msg: observer_update.clone().into()
// //             }
// //         );
// //
// //         // If the state arrives it gets sent to awaiting connections
// //         mailbox_handles
// //             .to_from_game_host
// //             .send(game_observer.clone().into())
// //             .unwrap();
// //         assert_eq!(
// //             mailbox_handles.from_to_connections.recv().await.unwrap(),
// //             ToConnections {
// //                 to: [connection_id_1, connection_id_2].into_iter().collect(),
// //                 msg: game_observer.clone().into()
// //             }
// //         );
// //
// //         // Add connection id 3 which doesn't have the state yet
// //         mailbox_handles
// //             .to_from_runtime
// //             .send(Add(connection_id_3.into()))
// //             .unwrap();
// //
// //         // Get an update (only 1 & 2 get it because 3 is waiting on the state)
// //         mailbox_handles
// //             .to_from_game_host
// //             .send(observer_update.clone().into())
// //             .unwrap();
// //         assert_eq!(
// //             mailbox_handles.from_to_connections.recv().await.unwrap(),
// //             ToConnections {
// //                 // Empty
// //                 to: [connection_id_1, connection_id_2].into_iter().collect(),
// //                 msg: observer_update.clone().into()
// //             }
// //         );
// //
// //         // Once the state is sent, only connections waiting on it get it
// //         mailbox_handles
// //             .to_from_game_host
// //             .send(game_observer.clone().into())
// //             .unwrap();
// //         assert_eq!(
// //             mailbox_handles.from_to_connections.recv().await.unwrap(),
// //             ToConnections {
// //                 to: connection_id_3.into(),
// //                 msg: game_observer.clone().into()
// //             }
// //         );
// //     }
// //
// //     struct MailboxHandles<T: Play> {
// //         to_from_game_host: UnboundedSender<ToObserverMsg<T>>,
// //         to_from_runtime: UnboundedSender<ManageConnections>,
// //         from_to_connections: UnboundedReceiver<ToConnections<ToObserverMsg<T>>>,
// //         from_to_game_host: UnboundedReceiver<ToGameHostMsg<T>>,
// //     }
// //
// //     fn setup_guess_the_number() -> (
// //         GameProgression<GuessTheNumber>,
// //         GameObserver<GuessTheNumber>,
// //         ObserverUpdate<'static, GuessTheNumber>,
// //     ) {
// //         let settings: Settings = (0..=10).try_into().unwrap();
// //         let mut game_progression = GameProgression::from_settings(settings);
// //         let guess: Guess = 4.into();
// //         let update = game_progression.submit_actions([(0.into(), ActionResponse::Response(guess))]);
// //         let observer_update = update.observer_update().into_owned();
// //         let game_observer = game_progression.game_observer();
// //
// //         (game_progression, game_observer, observer_update)
// //     }
// //
// //     fn setup_test_infra<T: Play>() -> (Inbox<T>, Outbox<T>, MailboxHandles<T>) {
// //         let (to_from_game_host, from_game_host) = unbounded_channel();
// //         let (to_from_runtime, from_runtime) = unbounded_channel();
// //
// //         let (to_connections, from_to_connections) = unbounded_channel();
// //         let (to_game_host, from_to_game_host) = unbounded_channel();
// //
// //         let inbox = Inbox {
// //             from_game_host,
// //             from_runtime,
// //         };
// //
// //         let outbox = Outbox {
// //             to_connections,
// //             to_game_host,
// //         };
// //
// //         let handles = MailboxHandles {
// //             to_from_game_host,
// //             to_from_runtime,
// //             from_to_connections,
// //             from_to_game_host,
// //         };
// //
// //         (inbox, outbox, handles)
// //     }
// // }
