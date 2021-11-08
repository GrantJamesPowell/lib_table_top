use crate::connection::{
    ConnectionId, FromConnection,
    ManageConnections::{self, *},
    ToConnections,
};
use crate::messages::{
    FromPlayerMsg::{self, *},
    ToGameHostMsg::{self, *},
    ToPlayerMsg::{self, *},
};
use lttcore::{Play, Player};
use smallvec::SmallVec;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Conn {
    id: ConnectionId,
    primary: bool,
    needs_state: bool,
}

pub enum Mail<T: Play> {
    ManageConnections(ManageConnections),
    ToPlayerMsg(ToPlayerMsg<T>),
    FromPlayerMsg(FromConnection<FromPlayerMsg<T>>),
}

use Mail::{FromPlayerMsg as FPM, ManageConnections as MC, ToPlayerMsg as TPM};

pub async fn player_connections<T: Play>(
    player: Player,
    mut mailbox: UnboundedReceiver<Mail<T>>,
    to_connections: UnboundedSender<ToConnections<ToPlayerMsg<T>>>,
    to_game_host: UnboundedSender<ToGameHostMsg<T>>,
) -> anyhow::Result<()> {
    let mut connections: SmallVec<[Conn; 4]> = Default::default();
    let mut state_requested = false;

    while let Some(mail) = mailbox.recv().await {
        match mail {
            FPM(FromConnection {
                from,
                msg: RequestPrimary,
            }) => {
                for conn in connections.iter_mut() {
                    if conn.id == from {
                        to_connections.send(ToConnections {
                            to: conn.id.into(),
                            msg: SetPrimaryStatus(true),
                        })?;

                        conn.primary = true;
                    } else {
                        if conn.primary {
                            conn.primary = false;

                            to_connections.send(ToConnections {
                                to: conn.id.into(),
                                msg: SetPrimaryStatus(false),
                            })?;
                        }
                    }
                }
            }
            MC(Add(new_conns)) => {
                connections.extend(new_conns.into_iter().map(|id| Conn {
                    id,
                    needs_state: true,
                    primary: false,
                }));

                if !state_requested {
                    to_game_host.send(RequestPlayerState { player })?;
                    state_requested = true;
                }
            }
            MC(Remove(remove_conns)) => connections.retain(|conn| !remove_conns.contains(conn.id)),
            _ => todo!(),
        }
    }

    Ok(())
}
