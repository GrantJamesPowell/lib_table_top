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
    timeout_tx: UnboundedSender<TurnNum>,
}

pub struct MailInbox<T: Play> {
    pub from_connections: UnboundedReceiver<FromConnection<FromPlayerMsg<T>>>,
    pub from_game_host: UnboundedReceiver<ToPlayerMsg<T>>,
    pub from_runtime: UnboundedReceiver<ManageConnections>,
}

pub struct MailOutbox<T: Play> {
    pub to_connections: UnboundedSender<ToConnections<ToPlayerMsg<T>>>,
    pub to_game_host: UnboundedSender<ToGameHostMsg<T>>,
}

pub async fn player_connections<T: Play>(
    player: Player,
    timeout: Duration,
    mut inbox: MailInbox<T>,
    outbox: MailOutbox<T>,
) -> anyhow::Result<()> {
    let (timeout_tx, mut timeout_rx) = unbounded_channel::<TurnNum>();

    let mut state = State {
        player,
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
                match msg {
                    Add(conns) => {
                        if state.needs_sync_conns.is_empty() {
                            outbox.to_game_host.send(RequestPlayerState { player: state.player })?;
                        }
                        state.needs_sync_conns.extend(conns);
                    },
                    Remove(remove) => {
                        state.in_sync_conns.retain(|id| !remove.contains(*id));
                        state.needs_sync_conns.retain(|id| !remove.contains(*id));
                    }
                }
            }

            // Messages hot off the wire from clients
            Some(FromConnection { from, msg }) = inbox.from_connections.recv() => {
                match msg {
                    RequestPrimary => {
                        outbox.to_connections.send(ToConnections {
                            to: from.into(),
                            msg: SetPrimaryStatus(true)
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
                                player,
                                response: ActionResponse::Response(action)
                            })?;

                            state.awaiting_turn = None;
                        }

                        if !is_connection_primary {
                            outbox.to_connections.send(ToConnections {
                                to: from.into(),
                                msg: SubmitActionError(NotPrimary)
                            })?;
                        }

                        if !is_correct_turn {
                            outbox.to_connections.send(ToConnections {
                                to: from.into(),
                                msg: SubmitActionError(InvalidTurn {
                                    attempted: turn,
                                    correct: state.awaiting_turn
                                })
                            })?;
                        }
                    }
                }
            }

            // Messages from the game host
            Some(msg) = inbox.from_game_host.recv() => {
                match msg {
                    SyncState(_) => {
                        let needs_sync = std::mem::take(&mut state.needs_sync_conns);

                        outbox.to_connections.send(ToConnections {
                            to: needs_sync.iter().copied().collect(),
                            msg
                        })?;

                        state.in_sync_conns.extend(needs_sync);
                    }
                    Update(ref player_update) => {
                        if player_update.is_player_input_needed_this_turn(player) {
                            let turn_num = player_update.turn_num();
                            state.awaiting_turn = Some(turn_num);
                            let sender = state.timeout_tx.clone();

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
                    SetPrimaryStatus(_) | SubmitActionError(_) => {
                        panic!("The game host generated a player message it shouldn't have")
                    }
                }
            }

            // Timeout for a turn
            // Note: Since we hold a sender for this channel it will never return `None`
            // so this `select!` block will never yield to an `else` clause
            Some(turn_num) = timeout_rx.recv() => {
                if state.awaiting_turn == Some(turn_num) {
                    state.awaiting_turn = None;

                    outbox.to_connections.send(ToConnections {
                        to: state.in_sync_conns.iter().copied().collect(),
                        msg: SubmitActionError(Timeout { turn_num })
                    })?;

                    outbox.to_game_host.send(SubmitActionResponse {
                        player,
                        response: ActionResponse::Timeout
                    })?;
                }
            }

            else => {
                break;
            }
        }
    }

    Ok(())
}
