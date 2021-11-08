use crate::connection::{
    ConnectionId, Connections, FromConnection,
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

type Conns = SmallVec<[Conn; 4]>;

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
    let mut conns: Conns = Default::default();
    let mut state_requested = false;

    while let Some(mail) = mailbox.recv().await {
        match mail {
            FPM(FromConnection { from, msg }) => match msg {
                RequestPrimary => {
                    for msg in set_primary(from, &mut conns) {
                        to_connections.send(msg)?;
                    }
                }
                SubmitAction { action, turn } => {
                    println!("{:?}, {:?}", action, turn);
                    todo!()
                }
            },
            TPM(msg) => {
                if let SyncState(_) = msg {
                    state_requested = false;
                }

                if let Some(to_connections_msg) = forward(msg, &mut conns) {
                    to_connections.send(to_connections_msg)?;
                }
            }
            MC(Add(new_conns)) => {
                conns.extend(new_conns.into_iter().map(|id| Conn {
                    id,
                    needs_state: true,
                    primary: false,
                }));

                if !state_requested {
                    to_game_host.send(RequestPlayerState { player })?;
                }

                state_requested = true;
            }
            MC(Remove(remove_conns)) => conns.retain(|conn| !remove_conns.contains(conn.id)),
        }
    }

    Ok(())
}

fn forward<T: Play>(
    msg: ToPlayerMsg<T>,
    conns: &mut Conns,
) -> Option<ToConnections<ToPlayerMsg<T>>> {
    match msg {
        SyncState(_) => {
            let to: Connections = conns
                .iter()
                .filter(|conn| conn.needs_state)
                .map(|conn| conn.id)
                .collect();

            for conn in conns.iter_mut() {
                conn.needs_state = false;
            }

            if !to.is_empty() {
                Some(ToConnections { to, msg })
            } else {
                None
            }
        }
        Update(_) => {
            let to: Connections = conns
                .iter()
                .filter(|conn| !conn.needs_state)
                .map(|conn| conn.id)
                .collect();

            if !to.is_empty() {
                Some(ToConnections { to, msg })
            } else {
                None
            }
        }
        _ => panic!("Player connections should never receive anything but an update/state from the upstream")
    }
}

fn set_primary<T: Play>(
    new_primary: ConnectionId,
    conns: &mut Conns,
) -> impl Iterator<Item = ToConnections<ToPlayerMsg<T>>> + '_ {
    conns.iter_mut().filter_map(move |conn| {
        if conn.id == new_primary {
            conn.primary = true;

            Some(ToConnections {
                to: conn.id.into(),
                msg: SetPrimaryStatus(true),
            })
        } else {
            if conn.primary {
                conn.primary = false;

                Some(ToConnections {
                    to: conn.id.into(),
                    msg: SetPrimaryStatus(false),
                })
            } else {
                None
            }
        }
    })
}
