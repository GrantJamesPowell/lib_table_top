use crate::connection::{
    ConnectionId, ConnectionMsg,
    ManageConnections::{self, *},
};
use crate::messages::{
    ToGameHostMsg::{self, *},
    ToPlayerMsg,
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
    FromPlayerMsg,
}

use Mail::{FromPlayerMsg as FC, ManageConnections as MC, ToPlayerMsg as PM};

pub async fn player_connections<T: Play>(
    player: Player,
    mut mailbox: UnboundedReceiver<Mail<T>>,
    _to_clients: UnboundedSender<ConnectionMsg<ToPlayerMsg<T>>>,
    to_game_host: UnboundedSender<ToGameHostMsg<T>>,
) -> anyhow::Result<()> {
    let mut connections: SmallVec<[Conn; 4]> = Default::default();
    let mut state_requested = false;

    while let Some(mail) = mailbox.recv().await {
        match mail {
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
