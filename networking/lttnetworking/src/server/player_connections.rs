use crate::connection::{
    ConnectionId, FromConnection,
    ManageConnections::{self, *},
    ToConnections,
};
use crate::messages::{
    FromPlayerMsg::{self, *},
    SubmitActionErrorKind::*,
    ToGameHostMsg::{self, *},
    ToPlayerMsg::{self, *},
};
use lttcore::{play::ActionResponse, Play, Player, TurnNum};
use smallvec::SmallVec;
use std::time::Duration;
use tokio::select;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

#[derive(Debug)]
struct State {
    awaiting_turn: Option<TurnNum>,
    in_sync_conns: SmallVec<[ConnectionId; 4]>,
    needs_sync_conns: SmallVec<[ConnectionId; 1]>,
    player: Player,
    primary: Option<ConnectionId>,
    timeout: Duration,
    timeout_tx: UnboundedSender<TurnNum>,
}

impl State {
    fn all_connections(&self) -> impl Iterator<Item = ConnectionId> + '_ {
        self.needs_sync_conns
            .iter()
            .chain(self.in_sync_conns.iter())
            .copied()
    }
}

pub struct Inbox<T: Play> {
    pub from_connections: UnboundedReceiver<FromConnection<FromPlayerMsg<T>>>,
    pub from_game_host: UnboundedReceiver<ToPlayerMsg<T>>,
    pub from_runtime: UnboundedReceiver<ManageConnections>,
}

pub struct Outbox<T: Play> {
    pub to_connections: UnboundedSender<ToConnections<ToPlayerMsg<T>>>,
    pub to_game_host: UnboundedSender<ToGameHostMsg<T>>,
}

pub async fn player_connections<T: Play>(
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
        primary: None,
        in_sync_conns: Default::default(),
        needs_sync_conns: Default::default(),
    };

    loop {
        select! {
            // Connection management messages from the runtime
            Some(msg) = inbox.from_runtime.recv() => {
                process_manage_connections(msg, &mut state, &outbox)?;
            }

            // Messages hot off the wire from clients
            Some(msg) = inbox.from_connections.recv() => {
                process_from_connection(msg, &mut state, &outbox)?;
            }

            // Messages from the game host
            Some(msg) = inbox.from_game_host.recv() => {
                let is_game_over = process_from_game_host(msg, &mut state, &outbox)?;

                if is_game_over {
                    break
                }
            }

            // Timeout for a turn
            // Note: Since we hold a sender for this channel it will never return `None`
            // so this `select!` block will never yield to an `else` clause
            Some(turn_num) = timeout_rx.recv() => {
                process_timeout(turn_num, &mut state, &outbox)?;
            }
        }
    }

    Ok(())
}

fn process_manage_connections<T: Play>(
    msg: ManageConnections,
    state: &mut State,
    outbox: &Outbox<T>,
) -> anyhow::Result<()> {
    match msg {
        Add(conns) => {
            if state.needs_sync_conns.is_empty() {
                outbox.to_game_host.send(RequestPlayerState {
                    player: state.player,
                })?;
            }
            state.needs_sync_conns.extend(conns);
        }
        Remove(remove) => {
            state.in_sync_conns.retain(|id| !remove.contains(*id));
            state.needs_sync_conns.retain(|id| !remove.contains(*id));
        }
    }

    Ok(())
}

fn process_from_connection<T: Play>(
    FromConnection { from, msg }: FromConnection<FromPlayerMsg<T>>,
    state: &mut State,
    outbox: &Outbox<T>,
) -> anyhow::Result<()> {
    match msg {
        RequestPrimary => {
            outbox.to_connections.send(ToConnections {
                to: from.into(),
                msg: SetPrimaryStatus(true),
            })?;

            if let Some(old_primary) = state.primary.replace(from) {
                outbox.to_connections.send(ToConnections {
                    to: old_primary.into(),
                    msg: SetPrimaryStatus(false),
                })?;
            }
        }
        SubmitAction { action, turn } => {
            let is_correct_turn = state.awaiting_turn == Some(turn);
            let is_connection_primary = state.primary == Some(from);

            if is_correct_turn && is_connection_primary {
                outbox.to_game_host.send(SubmitActionResponse {
                    player: state.player,
                    response: ActionResponse::Response(action),
                })?;

                state.awaiting_turn = None;
            }

            if !is_connection_primary {
                outbox.to_connections.send(ToConnections {
                    to: from.into(),
                    msg: SubmitActionError(NotPrimary),
                })?;
            }

            if !is_correct_turn {
                outbox.to_connections.send(ToConnections {
                    to: from.into(),
                    msg: SubmitActionError(InvalidTurn {
                        attempted: turn,
                        correct: state.awaiting_turn,
                    }),
                })?;
            }
        }
    }

    Ok(())
}

fn process_from_game_host<T: Play>(
    msg: ToPlayerMsg<T>,
    state: &mut State,
    outbox: &Outbox<T>,
) -> anyhow::Result<bool> {
    match msg {
        SyncState(_) => {
            let needs_sync = std::mem::take(&mut state.needs_sync_conns);

            outbox.to_connections.send(ToConnections {
                to: needs_sync.iter().copied().collect(),
                msg,
            })?;

            state.in_sync_conns.extend(needs_sync);
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

            outbox.to_connections.send(ToConnections {
                to: state.in_sync_conns.iter().copied().collect(),
                msg,
            })?;
        }
        GameOver => {
            outbox.to_connections.send(ToConnections {
                to: state.all_connections().collect(),
                msg,
            })?;

            return Ok(true);
        }
        SetPrimaryStatus(_) | SubmitActionError(_) => {
            panic!("The game host generated a player message it shouldn't have")
        }
    }

    Ok(false)
}

fn process_timeout<T: Play>(
    turn_num: TurnNum,
    state: &mut State,
    outbox: &Outbox<T>,
) -> anyhow::Result<()> {
    if state.awaiting_turn == Some(turn_num) {
        state.awaiting_turn = None;

        outbox.to_connections.send(ToConnections {
            to: state.in_sync_conns.iter().copied().collect(),
            msg: SubmitActionError(Timeout { turn_num }),
        })?;

        outbox.to_game_host.send(SubmitActionResponse {
            player: state.player,
            response: ActionResponse::Timeout,
        })?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use lttcore::examples::GuessTheNumber;

    struct MailboxHandles<T: Play> {
        to_from_connections: UnboundedSender<FromConnection<FromPlayerMsg<T>>>,
        to_from_game_host: UnboundedSender<ToPlayerMsg<T>>,
        to_from_runtime: UnboundedSender<ManageConnections>,
        from_to_connections: UnboundedReceiver<ToConnections<ToPlayerMsg<T>>>,
        from_to_game_host: UnboundedReceiver<ToGameHostMsg<T>>,
        timeout_rx: UnboundedReceiver<TurnNum>,
    }

    #[tokio::test]
    async fn test_managing_connections() {
        let (_inbox, outbox, mut state, mut handles) = setup_test_infra::<GuessTheNumber>();

        let new_conn_1 = ConnectionId::new();
        let new_conn_2 = ConnectionId::new();

        // It sends a request to game host if it's the first connection being added
        // since the last sync state
        state.needs_sync_conns.clear();
        process_manage_connections(Add(new_conn_1.into()), &mut state, &outbox).unwrap();

        assert_eq!(
            handles.from_to_game_host.recv().await.unwrap(),
            RequestPlayerState {
                player: state.player
            }
        );

        // It doesn't do it the second time if it's before the game host has responded
        process_manage_connections(Add(new_conn_2.into()), &mut state, &outbox).unwrap();

        tokio::time::sleep(Duration::from_millis(50)).await;
        assert!(handles.from_to_game_host.try_recv().is_err());

        // You can remove connections
        assert!(state.needs_sync_conns.contains(&new_conn_1));
        process_manage_connections(Remove(new_conn_1.into()), &mut state, &outbox).unwrap();
        assert!(!state.needs_sync_conns.contains(&new_conn_1));
    }

    #[tokio::test]
    async fn test_processing_timeouts_for_awaited_turn() {
        let (_inbox, outbox, mut state, mut handles) = setup_test_infra::<GuessTheNumber>();

        // When turn matches the one you're awaiting it sends timeout info out
        let turn_num: TurnNum = 0.into();
        state.awaiting_turn = Some(turn_num);
        process_timeout(turn_num, &mut state, &outbox).unwrap();

        assert_eq!(
            handles.from_to_connections.recv().await.unwrap(),
            ToConnections {
                to: state.in_sync_conns.into_iter().collect(),
                msg: SubmitActionError(Timeout { turn_num })
            }
        );

        assert_eq!(
            handles.from_to_game_host.recv().await.unwrap(),
            SubmitActionResponse {
                player: state.player,
                response: ActionResponse::Timeout
            }
        );
    }

    #[tokio::test]
    async fn test_processing_timeouts_when_turn_is_not_awaited() {
        let (_inbox, outbox, mut state, mut handles) = setup_test_infra::<GuessTheNumber>();

        // When turn doesn't match the one you're awaiting it does nothing
        let turn_num: TurnNum = 0.into();
        state.awaiting_turn = Some(1.into());
        process_timeout(turn_num, &mut state, &outbox).unwrap();

        tokio::time::sleep(Duration::from_millis(50)).await;
        assert!(handles.from_to_connections.try_recv().is_err());
        assert!(handles.from_to_game_host.try_recv().is_err());
    }

    fn setup_test_infra<T: Play>() -> (Inbox<T>, Outbox<T>, State, MailboxHandles<T>) {
        let (to_from_connections, from_connections) = unbounded_channel();
        let (to_from_game_host, from_game_host) = unbounded_channel();
        let (to_from_runtime, from_runtime) = unbounded_channel();

        let (to_connections, from_to_connections) = unbounded_channel();
        let (to_game_host, from_to_game_host) = unbounded_channel();

        let (timeout_tx, timeout_rx) = unbounded_channel::<TurnNum>();

        let inbox = Inbox {
            from_connections,
            from_game_host,
            from_runtime,
        };

        let outbox = Outbox {
            to_connections,
            to_game_host,
        };

        let handles = MailboxHandles {
            to_from_connections,
            to_from_game_host,
            to_from_runtime,
            from_to_connections,
            from_to_game_host,
            timeout_rx,
        };

        let state = State {
            awaiting_turn: None,
            in_sync_conns: [ConnectionId::new()].into_iter().collect(),
            needs_sync_conns: [ConnectionId::new()].into_iter().collect(),
            player: 0.into(),
            primary: None,
            timeout: Duration::from_millis(1000),
            timeout_tx,
        };

        (inbox, outbox, state, handles)
    }
}
