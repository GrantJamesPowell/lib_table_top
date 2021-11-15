use super::channels::{
    AddConnectionReceiver, BytesSender, FromPlayerMsgWithConnectionIdReceiver, ToGameHostMsgSender,
    ToPlayerMsgReceiver,
};
use super::id::ConnectionId;
use crate::messages::{
    FromPlayerMsg::{self, *},
    SubmitActionErrorKind::*,
    ToGameHostMsg::*,
    ToPlayerMsg::{self, *},
};
use lttcore::{encoder::Encoder, play::ActionResponse, Play, Player, TurnNum};
use serde::Serialize;
use smallvec::SmallVec;
use std::time::Duration;
use tokio::select;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};

#[derive(Debug)]
struct Conn {
    sender: BytesSender,
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
                conn.sender.send(bytes.clone()).is_ok()
            } else {
                !conn.sender.is_closed()
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
    pub from_player_msg_receiver: FromPlayerMsgWithConnectionIdReceiver<T>,
    pub to_player_msg_receiver: ToPlayerMsgReceiver<T>,
    pub add_player_connection_receiver: AddConnectionReceiver,
}

pub struct Outbox<T: Play> {
    pub to_game_host_msg_sender: ToGameHostMsgSender<T>,
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
            Some((id, sender)) = inbox.add_player_connection_receiver.recv() => {
                // Request state if we haven't already
                if state.are_all_in_sync() {
                    outbox.to_game_host_msg_sender.send(RequestPlayerState {
                        player: state.player,
                    })?;
                }
                state.conns.push(Conn { id, sender, primary: false, in_sync: false });
            }

            // Messages hot off the wire from clients
            Some(msg) = inbox.from_player_msg_receiver.recv() => {
                process_from_connection::<T, E>(msg, &mut state, &outbox)?;
            }

            // Messages from the game host
            Some(msg) = inbox.to_player_msg_receiver.recv() => {
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

                    outbox.to_game_host_msg_sender.send(SubmitActionResponse {
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
                outbox.to_game_host_msg_sender.send(SubmitActionResponse {
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

#[cfg(test)]
mod tests {
    use super::super::channels::{
        AddConnectionSender, FromPlayerMsgWithConnectionIdSender, ToGameHostMsgReceiver,
        ToPlayerMsgSender,
    };
    use super::super::id::ConnectionIdSource;
    use super::*;
    use lttcore::encoder::json::JsonEncoder;
    use lttcore::examples::{
        guess_the_number::{Guess, Settings},
        GuessTheNumber,
    };
    use lttcore::{pov::PlayerUpdate, GamePlayer, GameProgression};
    use tokio::sync::mpsc::error::TryRecvError;
    use tokio::time::sleep;

    struct MailboxHandles<T: Play> {
        from_player_msg_sender: FromPlayerMsgWithConnectionIdSender<T>,
        to_player_msg_sender: ToPlayerMsgSender<T>,
        add_player_connection_sender: AddConnectionSender,
        to_game_host_msg_receiver: ToGameHostMsgReceiver<T>,
    }

    #[tokio::test]
    async fn test_player_connections() {
        let (inbox, outbox, mut mailbox_handles) = setup_test_infra::<GuessTheNumber>();
        let player: Player = 0.into();
        let (_game_progression, game_player, player_update) = setup_guess_the_number(player);
        let connection_id_source = ConnectionIdSource::new();
        let (connections, mut connection_streams): (Vec<_>, Vec<_>) = (0..=2)
            .map(|_| {
                let (sender, receiver) = unbounded_channel();
                ((connection_id_source.next(), sender), receiver)
            })
            .unzip();

        let _handle = tokio::spawn(player_connections::<GuessTheNumber, JsonEncoder>(
            player,
            Duration::from_millis(50),
            inbox,
            outbox,
        ));

        // On the first connections added, it sends a request for the game state
        mailbox_handles
            .add_player_connection_sender
            .send(connections[0].clone())
            .unwrap();
        assert_eq!(
            mailbox_handles.to_game_host_msg_receiver.recv().await,
            Some(RequestPlayerState { player })
        );

        // On the second, it does not re-request the game state
        mailbox_handles
            .add_player_connection_sender
            .send(connections[1].clone())
            .unwrap();
        sleep(Duration::from_millis(50)).await;
        assert_eq!(
            mailbox_handles.to_game_host_msg_receiver.try_recv(),
            Err(TryRecvError::Empty)
        );

        // If an update arrives it doesn't get sent out to connections awaiting the full sync
        mailbox_handles
            .to_player_msg_sender
            .send(player_update.clone().into())
            .unwrap();

        sleep(Duration::from_millis(50)).await;
        for stream in connection_streams.iter_mut().take(2) {
            assert!(stream.try_recv().is_err());
        }

        // If the state arrives it gets sent to awaiting connections
        mailbox_handles
            .to_player_msg_sender
            .send(game_player.clone().into())
            .unwrap();
        for stream in connection_streams.iter_mut().take(2) {
            let msg = stream.recv().await.unwrap();
            let decoded: ToPlayerMsg<GuessTheNumber> = JsonEncoder::deserialize(msg).unwrap();
            assert_eq!(decoded, SyncState(game_player.clone()));
        }

        // Add connection id 3 which doesn't have the state yet
        mailbox_handles
            .add_player_connection_sender
            .send(connections[2].clone())
            .unwrap();

        // Get an update (only 1 & 2 get it because 3 is waiting on the state)
        mailbox_handles
            .to_player_msg_sender
            .send(player_update.clone().into())
            .unwrap();

        sleep(Duration::from_millis(50)).await;
        assert!(connection_streams[2].try_recv().is_err());

        for stream in connection_streams.iter_mut().take(2) {
            let msg = stream.recv().await.unwrap();
            let decoded: ToPlayerMsg<GuessTheNumber> = JsonEncoder::deserialize(msg).unwrap();
            assert_eq!(decoded, Update(player_update.clone()))
        }

        // Once the state is sent, only connections waiting on it get it
        mailbox_handles
            .to_player_msg_sender
            .send(game_player.clone().into())
            .unwrap();

        sleep(Duration::from_millis(50)).await;
        for stream in connection_streams.iter_mut().take(2) {
            assert!(stream.try_recv().is_err());
        }

        let msg = connection_streams[2].recv().await.unwrap();
        let decoded: ToPlayerMsg<GuessTheNumber> = JsonEncoder::deserialize(msg).unwrap();
        assert_eq!(decoded, SyncState(game_player.clone()));
    }

    // #[tokio::test]
    // async fn test_managing_connections() {
    //     let (_inbox, outbox, mut state, mut handles) = setup_test_infra::<GuessTheNumber>();

    //     let new_conn_1 = ConnectionId::new();
    //     let new_conn_2 = ConnectionId::new();

    //     // It sends a request to game host if it's the first connection being added
    //     // since the last sync state
    //     state.needs_sync_conns.clear();
    //     process_manage_connections(Add(new_conn_1.into()), &mut state, &outbox).unwrap();

    //     assert_eq!(
    //         handles.to_game_host_msg_receiver.recv().await.unwrap(),
    //         RequestPlayerState {
    //             player: state.player
    //         }
    //     );

    //     // It doesn't do it the second time if it's before the game host has responded
    //     process_manage_connections(Add(new_conn_2.into()), &mut state, &outbox).unwrap();

    //     tokio::time::sleep(Duration::from_millis(50)).await;
    //     assert!(handles.to_game_host_msg_receiver.try_recv().is_err());

    //     // You can remove connections
    //     assert!(state.needs_sync_conns.contains(&new_conn_1));
    //     process_manage_connections(Remove(new_conn_1.into()), &mut state, &outbox).unwrap();
    //     assert!(!state.needs_sync_conns.contains(&new_conn_1));
    // }

    // #[tokio::test]
    // async fn test_processing_timeouts_for_awaited_turn() {
    //     let (_inbox, outbox, mut state, mut handles) = setup_test_infra::<GuessTheNumber>();

    //     // When turn matches the one you're awaiting it sends timeout info out
    //     let turn_num: TurnNum = 0.into();
    //     state.awaiting_turn = Some(turn_num);
    //     process_timeout(turn_num, &mut state, &outbox).unwrap();

    //     assert_eq!(
    //         handles.from_to_connections.recv().await.unwrap(),
    //         ToConnections {
    //             to: state.in_sync_conns.into_iter().collect(),
    //             msg: SubmitActionError(Timeout { turn_num })
    //         }
    //     );

    //     assert_eq!(
    //         handles.to_game_host_msg_receiver.recv().await.unwrap(),
    //         SubmitActionResponse {
    //             player: state.player,
    //             response: ActionResponse::Timeout
    //         }
    //     );
    // }

    fn setup_guess_the_number(
        player: Player,
    ) -> (
        GameProgression<GuessTheNumber>,
        GamePlayer<GuessTheNumber>,
        PlayerUpdate<'static, GuessTheNumber>,
    ) {
        let settings: Settings = (0..=10).try_into().unwrap();
        let mut game_progression = GameProgression::from_settings(settings);
        let guess: Guess = 4.into();
        let update = game_progression.submit_actions([(0.into(), ActionResponse::Response(guess))]);
        let player_update = update.player_update(player).into_owned();
        let game_player = game_progression.game_player(player);

        (game_progression, game_player, player_update)
    }

    fn setup_test_infra<T: Play>() -> (Inbox<T>, Outbox<T>, MailboxHandles<T>) {
        let (from_player_msg_sender, from_player_msg_receiver) = unbounded_channel();
        let (to_player_msg_sender, to_player_msg_receiver) = unbounded_channel();
        let (add_player_connection_sender, add_player_connection_receiver) = unbounded_channel();
        let (to_game_host_msg_sender, to_game_host_msg_receiver) = unbounded_channel();

        let inbox = Inbox {
            from_player_msg_receiver,
            to_player_msg_receiver,
            add_player_connection_receiver,
        };

        let outbox = Outbox {
            to_game_host_msg_sender,
        };

        let handles = MailboxHandles {
            from_player_msg_sender,
            to_player_msg_sender,
            add_player_connection_sender,
            to_game_host_msg_receiver,
        };

        (inbox, outbox, handles)
    }
}
