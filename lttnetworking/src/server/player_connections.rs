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
use lttcore::{Play, Player, TurnNum};
use smallvec::SmallVec;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::select;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Conn {
    id: ConnectionId,
    primary: bool,
    needs_state: bool,
}

#[derive(Debug)]
struct State {
    player: Player,
    state_requested: bool,
    conns: Conns,
    awaiting_turn: Option<TurnNum>,
    timeout_tx: UnboundedSender<()>,
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
    let (timeout_tx, mut timeout_rx) = unbounded_channel::<()>();

    let mut state = State {
        player,
        timeout_tx,
        state_requested: false,
        conns: Default::default(),
        awaiting_turn: None,
    };

    loop {
        select! {
            msg = mailbox.recv() => {
                match msg {
                    Some(mail) => process_mail(mail, &to_connections, &to_game_host, &mut state)?,
                    None => break
                }
            }
            _timeout = timeout_rx.recv() => {
                todo!()
            }
        }
    }

    Ok(())
}

fn process_mail<T: Play>(
    msg: Mail<T>,
    to_connections: &UnboundedSender<ToConnections<ToPlayerMsg<T>>>,
    to_game_host: &UnboundedSender<ToGameHostMsg<T>>,
    state: &mut State
) -> anyhow::Result<()> {
    match msg {
        FPM(FromConnection { from, msg }) => match msg {
            RequestPrimary => {
                for msg in set_primary(from, state) {
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
                state.state_requested = false;
            }

            if let Some(to_connections_msg) = forward(msg, state) {
                to_connections.send(to_connections_msg)?;
            }
        }
        MC(Add(new_conns)) => {
            state.conns.extend(new_conns.into_iter().map(|id| Conn {
                id,
                needs_state: true,
                primary: false,
            }));

            if !state.state_requested {
                to_game_host.send(RequestPlayerState { player: state.player })?;
            }

            state.state_requested = true;
        }
        MC(Remove(remove_conns)) => state.conns.retain(|conn| !remove_conns.contains(conn.id)),
    }

    Ok(())
}

fn forward<T: Play>(
    msg: ToPlayerMsg<T>,
    state: &mut State,
) -> Option<ToConnections<ToPlayerMsg<T>>> {
    match msg {
        SyncState(_) => {
            let to: Connections = state
                .conns
                .iter()
                .filter(|conn| conn.needs_state)
                .map(|conn| conn.id)
                .collect();

            for conn in state.conns.iter_mut() {
                conn.needs_state = false;
            }

            if !to.is_empty() {
                Some(ToConnections { to, msg })
            } else {
                None
            }
        }
        Update(_) => {
            let to: Connections = state
                .conns
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
    state: &mut State,
) -> impl Iterator<Item = ToConnections<ToPlayerMsg<T>>> + '_ {
    state.conns.iter_mut().filter_map(move |conn| {
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
