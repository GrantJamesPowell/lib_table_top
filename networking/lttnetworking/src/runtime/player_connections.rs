use crate::messages::{
    game_host::ToGameHostMsg::{self, *},
    player::{
        FromPlayerMsg::{self, *},
        SubmitActionErrorKind::*,
        ToPlayerMsg::{self, *},
    },
};
use crate::runtime::ToByteSink;
use lttcore::{encoder::Encoder, id::ConnectionId, play::ActionResponse, Play, Player, TurnNum};
use serde::Serialize;
use smallvec::SmallVec;
use std::time::Duration;
use tokio::select;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

#[derive(Debug)]
struct Conn {
    sink: ToByteSink,
    id: ConnectionId,
    primary: bool,
    in_sync: bool,
}

#[derive(Debug)]
struct State<E: Encoder> {
    conns: SmallVec<[Conn; 1]>,
    awaiting_turn: Option<TurnNum>,
    player: Player,
    timeout: Duration,
    timeout_tx: UnboundedSender<TurnNum>,
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

    fn primary(&self) -> Option<ConnectionId> {
        self.conns
            .iter()
            .filter(|conn| conn.primary)
            .map(|conn| conn.id.clone())
            .next()
    }

    fn are_all_in_sync(&self) -> bool {
        self.conns.iter().all(|conn| conn.in_sync)
    }
}

pub struct Inbox<T: Play> {
    pub from_connections: UnboundedReceiver<(ConnectionId, FromPlayerMsg<T>)>,
    pub from_game_host: UnboundedReceiver<ToPlayerMsg<T>>,
    pub from_runtime: UnboundedReceiver<(ConnectionId, ToByteSink)>,
}

pub struct Outbox<T: Play> {
    pub to_game_host: UnboundedSender<ToGameHostMsg<T>>,
}

pub async fn player_connections<T: Play, E: Encoder>(
    player: Player,
    timeout: Duration,
    mut inbox: Inbox<T>,
    outbox: Outbox<T>,
) -> anyhow::Result<()> {
    let (timeout_tx, mut timeout_rx) = unbounded_channel::<TurnNum>();

    let mut state = State {
        player,
        timeout,
        timeout_tx,
        awaiting_turn: None,
        conns: Default::default(),
        _phantom: std::marker::PhantomData,
    };

    loop {
        select! {
            // Adding a new connection
            Some((id, sink)) = inbox.from_runtime.recv() => {
                // Request state if we haven't already
                if state.are_all_in_sync() {
                    outbox.to_game_host.send(RequestPlayerState {
                        player: state.player,
                    })?;
                }
                state.conns.push(Conn { id, sink, primary: false, in_sync: false });
            }

            // Messages hot off the wire from clients
            Some(msg) = inbox.from_connections.recv() => {
                process_from_connection::<T, E>(msg, &mut state, &outbox)?;
            }

            // Messages from the game host
            Some(msg) = inbox.from_game_host.recv() => {
                let is_game_over = process_from_game_host(msg, &mut state)?;

                if is_game_over {
                    break
                }
            }

            // Timeout for a turn
            // Note: Since we hold a sender for this channel it will never return `None`
            // so this `select!` block will never yield to an `else` clause
            Some(turn_num) = timeout_rx.recv() => {
                if state.awaiting_turn == Some(turn_num) {
                    state.awaiting_turn = None;

                    let msg: ToPlayerMsg<T> = SubmitActionError(Timeout { turn_num });
                    state.send_to(&msg, |conn| conn.in_sync);

                    outbox.to_game_host.send(SubmitActionResponse {
                        player: state.player,
                        response: ActionResponse::Timeout,
                    })?;
                }
            }
        }
    }

    Ok(())
}

fn process_from_connection<T: Play, E: Encoder>(
    (from, msg): (ConnectionId, FromPlayerMsg<T>),
    state: &mut State<E>,
    outbox: &Outbox<T>,
) -> anyhow::Result<()> {
    match msg {
        RequestPrimary => {
            let msg: ToPlayerMsg<T> = SetPrimaryStatus(false);
            state.send_to(&msg, |conn| std::mem::replace(&mut conn.primary, false));

            let msg: ToPlayerMsg<T> = SetPrimaryStatus(true);
            state.send_to(&msg, |conn| conn.id == from);
        }
        SubmitAction { action, turn } => {
            let is_correct_turn = state.awaiting_turn == Some(turn);
            let is_connection_primary = state.primary() == Some(from);

            if is_correct_turn && is_connection_primary {
                outbox.to_game_host.send(SubmitActionResponse {
                    player: state.player,
                    response: ActionResponse::Response(action),
                })?;

                state.awaiting_turn = None;
            }

            if !is_connection_primary {
                let msg: ToPlayerMsg<T> = SubmitActionError(NotPrimary);
                state.send_to(&msg, |conn| conn.id == from);
            }

            if !is_correct_turn {
                let msg: ToPlayerMsg<T> = SubmitActionError(InvalidTurn {
                    attempted: turn,
                    correct: state.awaiting_turn,
                });
                state.send_to(&msg, |conn| conn.id == from);
            }
        }
    }

    Ok(())
}

fn process_from_game_host<T: Play, E: Encoder>(
    msg: ToPlayerMsg<T>,
    state: &mut State<E>,
) -> anyhow::Result<bool> {
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
        Update(ref player_update) => {
            if player_update.is_player_input_needed_this_turn(state.player) {
                let turn_num = player_update.turn_num();
                state.awaiting_turn = Some(turn_num);
                let sender = state.timeout_tx.clone();
                let timeout = state.timeout;

                tokio::spawn(async move {
                    tokio::time::sleep(timeout).await;
                    let _ = sender.send(turn_num);
                });
            }

            state.send_to(&msg, |conn| conn.in_sync);
        }
        GameOver => {
            state.send_to(&msg, |_conn| true);
            return Ok(true);
        }
        SetPrimaryStatus(_) | SubmitActionError(_) => {
            panic!("The game host generated a player message it shouldn't have")
        }
    }

    Ok(false)
}

//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use lttcore::examples::GuessTheNumber;
//
//     struct MailboxHandles<T: Play> {
//         to_from_connections: UnboundedSender<FromConnection<FromPlayerMsg<T>>>,
//         to_from_game_host: UnboundedSender<ToPlayerMsg<T>>,
//         to_from_runtime: UnboundedSender<ManageConnections>,
//         from_to_connections: UnboundedReceiver<ToConnections<ToPlayerMsg<T>>>,
//         from_to_game_host: UnboundedReceiver<ToGameHostMsg<T>>,
//         timeout_rx: UnboundedReceiver<TurnNum>,
//     }
//
//     #[tokio::test]
//     async fn test_managing_connections() {
//         let (_inbox, outbox, mut state, mut handles) = setup_test_infra::<GuessTheNumber>();
//
//         let new_conn_1 = ConnectionId::new();
//         let new_conn_2 = ConnectionId::new();
//
//         // It sends a request to game host if it's the first connection being added
//         // since the last sync state
//         state.needs_sync_conns.clear();
//         process_manage_connections(Add(new_conn_1.into()), &mut state, &outbox).unwrap();
//
//         assert_eq!(
//             handles.from_to_game_host.recv().await.unwrap(),
//             RequestPlayerState {
//                 player: state.player
//             }
//         );
//
//         // It doesn't do it the second time if it's before the game host has responded
//         process_manage_connections(Add(new_conn_2.into()), &mut state, &outbox).unwrap();
//
//         tokio::time::sleep(Duration::from_millis(50)).await;
//         assert!(handles.from_to_game_host.try_recv().is_err());
//
//         // You can remove connections
//         assert!(state.needs_sync_conns.contains(&new_conn_1));
//         process_manage_connections(Remove(new_conn_1.into()), &mut state, &outbox).unwrap();
//         assert!(!state.needs_sync_conns.contains(&new_conn_1));
//     }
//
//     #[tokio::test]
//     async fn test_processing_timeouts_for_awaited_turn() {
//         let (_inbox, outbox, mut state, mut handles) = setup_test_infra::<GuessTheNumber>();
//
//         // When turn matches the one you're awaiting it sends timeout info out
//         let turn_num: TurnNum = 0.into();
//         state.awaiting_turn = Some(turn_num);
//         process_timeout(turn_num, &mut state, &outbox).unwrap();
//
//         assert_eq!(
//             handles.from_to_connections.recv().await.unwrap(),
//             ToConnections {
//                 to: state.in_sync_conns.into_iter().collect(),
//                 msg: SubmitActionError(Timeout { turn_num })
//             }
//         );
//
//         assert_eq!(
//             handles.from_to_game_host.recv().await.unwrap(),
//             SubmitActionResponse {
//                 player: state.player,
//                 response: ActionResponse::Timeout
//             }
//         );
//     }
//
//     #[tokio::test]
//     async fn test_processing_timeouts_when_turn_is_not_awaited() {
//         let (_inbox, outbox, mut state, mut handles) = setup_test_infra::<GuessTheNumber>();
//
//         // When turn doesn't match the one you're awaiting it does nothing
//         let turn_num: TurnNum = 0.into();
//         state.awaiting_turn = Some(1.into());
//         process_timeout(turn_num, &mut state, &outbox).unwrap();
//
//         tokio::time::sleep(Duration::from_millis(50)).await;
//         assert!(handles.from_to_connections.try_recv().is_err());
//         assert!(handles.from_to_game_host.try_recv().is_err());
//     }
//
//     fn setup_test_infra<T: Play>() -> (Inbox<T>, Outbox<T>, State, MailboxHandles<T>) {
//         let (to_from_connections, from_connections) = unbounded_channel();
//         let (to_from_game_host, from_game_host) = unbounded_channel();
//         let (to_from_runtime, from_runtime) = unbounded_channel();
//
//         let (to_connections, from_to_connections) = unbounded_channel();
//         let (to_game_host, from_to_game_host) = unbounded_channel();
//
//         let (timeout_tx, timeout_rx) = unbounded_channel::<TurnNum>();
//
//         let inbox = Inbox {
//             from_connections,
//             from_game_host,
//             from_runtime,
//         };
//
//         let outbox = Outbox {
//             to_connections,
//             to_game_host,
//         };
//
//         let handles = MailboxHandles {
//             to_from_connections,
//             to_from_game_host,
//             to_from_runtime,
//             from_to_connections,
//             from_to_game_host,
//             timeout_rx,
//         };
//
//         let state = State {
//             awaiting_turn: None,
//             in_sync_conns: [ConnectionId::new()].into_iter().collect(),
//             needs_sync_conns: [ConnectionId::new()].into_iter().collect(),
//             player: 0.into(),
//             primary: None,
//             timeout: Duration::from_millis(1000),
//             timeout_tx,
//         };
//
//         (inbox, outbox, state, handles)
//     }
// }
