use crate::connection::{ConnectionId, ConnectionMsg, Connections, ManageConnections};
use crate::server::messages::{GameHostMsg, ObserverMsg};
use lttcore::Play;
use smallvec::SmallVec;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Conn {
    id: ConnectionId,
    needs_state: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mail<T: Play> {
    OM(ObserverMsg<T>),
    MC(ManageConnections),
}

impl<T: Play> From<ObserverMsg<T>> for Mail<T> {
    fn from(observer_msg: ObserverMsg<T>) -> Self {
        Self::OM(observer_msg)
    }
}

impl<T: Play> From<ManageConnections> for Mail<T> {
    fn from(manage_connections: ManageConnections) -> Self {
        Self::MC(manage_connections)
    }
}

use GameHostMsg::*;
use Mail::*;
use ManageConnections::*;
use ObserverMsg::*;

pub async fn observer_connections<T: Play>(
    mut mailbox: UnboundedReceiver<Mail<T>>,
    to_client: UnboundedSender<ConnectionMsg<ObserverMsg<T>>>,
    to_game_host: UnboundedSender<GameHostMsg<T>>,
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

                to_client.send(ConnectionMsg { to, msg })?;

                for conn in connections.iter_mut() {
                    conn.needs_state = false;
                }
            }
            OM(msg @ Update(_)) => {
                println!("Here!");
                println!("Conns {:?}", connections);
                let to: Connections = connections
                    .iter()
                    .filter(|conn| !conn.needs_state)
                    .map(|conn| conn.id)
                    .collect();

                if !to.is_empty() {
                    to_client.send(ConnectionMsg { to, msg })?;
                }
            }
            MC(Add(new_conns)) => {
                connections.extend(new_conns.into_iter().map(|id| Conn {
                    id,
                    needs_state: true,
                }));

                if !state_requested {
                    to_game_host.send(RequestObserverState)?;
                    state_requested = true;
                }
            }
            MC(Remove(remove_conns)) => connections.retain(|conn| !remove_conns.contains(conn.id)),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use lttcore::examples::{
        guess_the_number::{PublicInfoUpdate, Settings},
        GuessTheNumber,
    };
    use lttcore::pov::ObserverUpdate;
    use lttcore::{GameProgression, PlayerSet};
    use std::borrow::Cow;
    use tokio::sync::mpsc::error::TryRecvError;
    use tokio::sync::mpsc::unbounded_channel;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_observer_connections_work() {
        let connection_id_1 = ConnectionId::new();
        let connection_id_2 = ConnectionId::new();
        let connection_id_3 = ConnectionId::new();

        let (to_mailbox, mailbox) = unbounded_channel::<Mail<GuessTheNumber>>();
        let (to_client, mut client) = unbounded_channel();
        let (to_game_host, mut game_host) = unbounded_channel();

        tokio::spawn(observer_connections(mailbox, to_client, to_game_host));

        // On the first addition, it sends a state request to the game host
        to_mailbox.send(Add(connection_id_1.into()).into()).unwrap();
        assert_eq!(game_host.recv().await, Some(RequestObserverState));

        // On the second, it does not
        to_mailbox.send(Add(connection_id_2.into()).into()).unwrap();
        sleep(Duration::from_millis(100)).await;
        assert_eq!(game_host.try_recv(), Err(TryRecvError::Empty));

        // If an update gets sent, it doesn't get sent out to connections awaiting state
        let observer_update: ObserverUpdate<'static, GuessTheNumber> = ObserverUpdate {
            turn_num: 3.into(),
            action_requests: PlayerSet::empty(),
            public_info_update: Cow::Owned(PublicInfoUpdate {
                secret_number: 1,
                guesses: Default::default(),
            }),
        };

        to_mailbox.send(OM(observer_update.clone().into())).unwrap();
        sleep(Duration::from_millis(100)).await;
        assert_eq!(client.try_recv(), Err(TryRecvError::Empty));

        // If a state arrives it gets sent to the awaiting connections
        let settings: Settings = Default::default();
        let game: GameProgression<GuessTheNumber> = GameProgression::from_settings(settings);
        let observer_msg: ObserverMsg<GuessTheNumber> = game.game_observer().into();
        to_mailbox.send(OM(observer_msg.clone())).unwrap();

        assert_eq!(
            client.recv().await,
            Some(ConnectionMsg {
                to: Connections::new([connection_id_1, connection_id_2]),
                msg: observer_msg
            })
        );

        // Updates get sent to clients who have already received the state

        // Add connection id 3 which doesn't have the state yet
        to_mailbox.send(Add(connection_id_3.into()).into()).unwrap();

        // Get an update (only 1 & 2 get it)
        to_mailbox.send(OM(observer_update.clone().into())).unwrap();

        assert_eq!(
            client.recv().await,
            Some(ConnectionMsg {
                to: Connections::new([connection_id_1, connection_id_2]),
                msg: observer_update.into()
            })
        );
    }
}
