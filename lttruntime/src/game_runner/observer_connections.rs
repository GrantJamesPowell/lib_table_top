use super::channels::{
    AddConnectionReceiver, BytesSender, ToGameHostMsgSender, ToObserverMsgReceiver,
};
use super::id::ConnectionId;
use crate::messages::{ToGameHostMsg::*, ToObserverMsg::*};
use lttcore::play::Play;
use serde::Serialize;
use smallvec::SmallVec;
use tokio::select;

pub struct Inbox<T: Play> {
    pub to_observer_msg_receiver: ToObserverMsgReceiver<T>,
    pub add_observer_connection_receiver: AddConnectionReceiver,
}

pub struct Outbox<T: Play> {
    pub to_game_host_msg_sender: ToGameHostMsgSender<T>,
}

#[derive(Debug)]
struct Conn {
    sender: BytesSender,
    id: ConnectionId,
    in_sync: bool,
}

#[derive(Debug, Default)]
struct State {
    conns: SmallVec<[Conn; 2]>,
}

impl State {
    fn send_to<T: Serialize>(&mut self, msg: &T, f: impl Fn(&mut Conn) -> bool) {
        BytesSender::send_to(
            self.conns
                .iter_mut()
                .filter_map(|conn| if f(conn) { Some(&conn.sender) } else { None }),
            msg,
        )
    }

    fn are_all_in_sync(&self) -> bool {
        self.conns.iter().all(|conn| conn.in_sync)
    }
}

pub async fn observer_connections<T: Play>(
    mut inbox: Inbox<T>,
    outbox: Outbox<T>,
) -> anyhow::Result<()> {
    let mut state = State {
        conns: Default::default(),
    };

    loop {
        select! {
            Some((id, sender)) = inbox.add_observer_connection_receiver.recv() => {
                if state.are_all_in_sync() {
                    outbox.to_game_host_msg_sender.send(RequestObserverState)?;
                }
                state.conns.push(Conn {
                    id,
                    sender,
                    in_sync: false
                })
            }
            Some(msg) = inbox.to_observer_msg_receiver.recv() => {
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

#[cfg(test)]
mod tests {
    use super::super::id::ConnectionIdSource;
    use super::*;
    use crate::messages::ToObserverMsg;
    use lttcore::encoder::Encoding;
    use lttcore::examples::{
        guess_the_number::{Guess, Settings},
        GuessTheNumber,
    };
    use lttcore::{play::ActionResponse, pov::ObserverUpdate};
    use lttcore::{GameObserver, GameProgression};

    use super::super::channels::{
        bytes_channels, AddConnectionSender, ToGameHostMsgReceiver, ToObserverMsgSender,
    };
    use tokio::sync::mpsc::error::TryRecvError;
    use tokio::sync::mpsc::unbounded_channel;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_observer_connections() {
        let (inbox, outbox, mut mailbox_handles) = setup_test_infra::<GuessTheNumber>();
        let (_game_progression, game_observer, observer_update) = setup_guess_the_number();
        let connection_id_source = ConnectionIdSource::new();

        let (connections, mut connection_streams): (Vec<_>, Vec<_>) = (0..=2)
            .map(|_| {
                let (sender, receiver) = bytes_channels(Encoding::PrettyJson);
                ((connection_id_source.next(), sender), receiver)
            })
            .unzip();

        let _handle = tokio::spawn(observer_connections::<GuessTheNumber>(inbox, outbox));

        // On the first connections added, it sends a request for the game state
        mailbox_handles
            .add_observer_connection_sender
            .send(connections[0].clone())
            .unwrap();
        assert_eq!(
            mailbox_handles.to_game_host_msg_receiver.recv().await,
            Some(RequestObserverState)
        );

        // On the second, it does not re-request the game state
        mailbox_handles
            .add_observer_connection_sender
            .send(connections[1].clone())
            .unwrap();
        sleep(Duration::from_millis(50)).await;
        assert_eq!(
            mailbox_handles.to_game_host_msg_receiver.try_recv(),
            Err(TryRecvError::Empty)
        );

        // If an update arrives it doesn't get sent out to connections awaiting the full sync
        mailbox_handles
            .to_observer_msg_sender
            .send(observer_update.clone().into())
            .unwrap();

        sleep(Duration::from_millis(50)).await;
        for stream in connection_streams.iter_mut().take(2) {
            assert!(stream.try_next_bytes().is_err());
        }

        // If the state arrives it gets sent to awaiting connections
        mailbox_handles
            .to_observer_msg_sender
            .send(game_observer.clone().into())
            .unwrap();
        for stream in connection_streams.iter_mut().take(2) {
            let msg = stream.next_bytes().await.unwrap();
            let decoded: ToObserverMsg<GuessTheNumber> =
                Encoding::PrettyJson.deserialize(msg).unwrap();
            assert_eq!(decoded, SyncState(game_observer.clone()));
        }

        // Add connection id 3 which doesn't have the state yet
        mailbox_handles
            .add_observer_connection_sender
            .send(connections[2].clone())
            .unwrap();

        // Get an update (only 1 & 2 get it because 3 is waiting on the state)
        mailbox_handles
            .to_observer_msg_sender
            .send(observer_update.clone().into())
            .unwrap();

        sleep(Duration::from_millis(50)).await;
        assert!(connection_streams[2].try_next_bytes().is_err());

        for stream in connection_streams.iter_mut().take(2) {
            let msg = stream.next_bytes().await.unwrap();
            let decoded: ToObserverMsg<GuessTheNumber> =
                Encoding::PrettyJson.deserialize(msg).unwrap();
            assert_eq!(decoded, Update(observer_update.clone()))
        }

        // Once the state is sent, only connections waiting on it get it
        mailbox_handles
            .to_observer_msg_sender
            .send(game_observer.clone().into())
            .unwrap();

        sleep(Duration::from_millis(50)).await;
        for stream in connection_streams.iter_mut().take(2) {
            assert!(stream.try_next_bytes().is_err());
        }

        let msg = connection_streams[2].next_bytes().await.unwrap();
        let decoded: ToObserverMsg<GuessTheNumber> = Encoding::PrettyJson.deserialize(msg).unwrap();
        assert_eq!(decoded, SyncState(game_observer.clone()));
    }

    struct MailboxHandles<T: Play> {
        to_observer_msg_sender: ToObserverMsgSender<T>,
        add_observer_connection_sender: AddConnectionSender,
        to_game_host_msg_receiver: ToGameHostMsgReceiver<T>,
    }

    fn setup_guess_the_number() -> (
        GameProgression<GuessTheNumber>,
        GameObserver<GuessTheNumber>,
        ObserverUpdate<'static, GuessTheNumber>,
    ) {
        let settings: Settings = (0..=10).try_into().unwrap();
        let mut game_progression = GameProgression::from_settings(settings);
        let guess: Guess = 4.into();
        let update = game_progression.submit_actions([(0.into(), ActionResponse::Response(guess))]);
        let observer_update = update.observer_update().into_owned();
        let game_observer = game_progression.game_observer();

        (game_progression, game_observer, observer_update)
    }

    fn setup_test_infra<T: Play>() -> (Inbox<T>, Outbox<T>, MailboxHandles<T>) {
        let (to_observer_msg_sender, to_observer_msg_receiver) = unbounded_channel();
        let (add_observer_connection_sender, add_observer_connection_receiver) =
            unbounded_channel();
        let (to_game_host_msg_sender, to_game_host_msg_receiver) = unbounded_channel();

        let inbox = Inbox {
            to_observer_msg_receiver,
            add_observer_connection_receiver,
        };

        let outbox = Outbox {
            to_game_host_msg_sender,
        };

        let handles = MailboxHandles {
            to_observer_msg_sender,
            add_observer_connection_sender,
            to_game_host_msg_receiver,
        };

        (inbox, outbox, handles)
    }
}
