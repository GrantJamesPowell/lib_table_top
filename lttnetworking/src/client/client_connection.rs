use crate::connection::{ConnectionIO, RawConnection, SubConnection, SubConnectionId};
use crate::messages::closed::Closed;
use crate::messages::conn_ctrl::{
    ClientConnControlMsg as CCCMsg, ServerConnControlMsg as SCCMsg, SubConnMode,
};
use crate::messages::hello::{ClientHello, ServerHello, ServerInfo};
use crate::{SupportedGames, Token, User};
use bytes::Bytes;
use lttcore::encoder::Encoder;
use std::collections::HashMap;
use tokio::select;
use tokio::sync::mpsc;

pub async fn client_connection<S: SupportedGames<E>, E: Encoder>(
    credentials: Token,
    mut jobs: impl Iterator<Item = (S, SubConnMode, Box<dyn FnOnce(SubConnection<E>)>)>,
    concurrency: u64,
    mut conn: impl RawConnection<E>,
) -> Result<Closed, Closed> {
    let (_user, server_info) = authenticate_conn(credentials, &mut conn).await?;
    let concurrency: usize = server_info
        .max_sub_connections
        .min(concurrency)
        .try_into()
        .unwrap();

    let (from_sub_connections_sender, mut from_sub_connections_receiver) =
        mpsc::unbounded_channel::<(SubConnectionId, Bytes)>();

    let mut pending_jobs: HashMap<
        SubConnectionId,
        (SubConnMode, Box<dyn FnOnce(SubConnection<E>)>),
    > = Default::default();
    let mut running_jobs: HashMap<SubConnectionId, mpsc::UnboundedSender<Bytes>> =
        Default::default();

    for (game_type, sub_conn_mode, fun) in jobs.take(concurrency) {
        let id = SubConnectionId::new();
        let msg: CCCMsg<S, E> = CCCMsg::StartSubConn {
            id,
            game_type,
            _encoder: Default::default(),
        };
        conn.send(msg).await?;
        pending_jobs.insert(id, (sub_conn_mode, fun));
    }

    loop {
        select! {
            biased;
            msg = conn.next::<SCCMsg<S, E>>() => {
                match msg? {
                    SCCMsg::SubConnStarted { id, .. } => {
                        let (sub_conn_mode, fun) = pending_jobs
                            .remove(&id)
                            .expect("server only sends us already pending jobs");

                        let (sender, receiver) = mpsc::unbounded_channel();

                        let mut sub_conn = SubConnection {
                            id,
                            receiver,
                            sender: Some(from_sub_connections_sender.clone()),
                            _encoder: Default::default()
                        };

                        sub_conn.send(sub_conn_mode).await?;
                        running_jobs.insert(id, sender);
                        fun(sub_conn);
                    }
                    SCCMsg::SubConnMsg { id, bytes, .. } => {
                        match running_jobs.get(&id).map(|sender| sender.send(bytes)) {
                            Some(Ok(())) => continue,
                            Some(Err(_)) => {
                                todo!("job failed")
                            },
                            None => {
                                panic!("we forgot to put conn id in the jobs map")
                            }

                        }
                    }
                    SCCMsg::SubConnClosed { id, .. } => {
                        pending_jobs.remove(&id);

                        if let Some((game_type, sub_conn_mode, fun)) = jobs.pop() {
                            let id = SubConnectionId::new();
                            let msg: CCCMsg<S, E> = CCCMsg::StartSubConn { id, game_type, _encoder: Default::default() };
                            conn.send(msg).await?;
                            pending_jobs.insert(id, (sub_conn_mode, fun));
                        }

                        if jobs.is_empty() && pending_jobs.is_empty() && running_jobs.is_empty() {
                            conn.close().await;
                            return Ok(Closed::Normal)
                        }

                    }
                }
            }
            Some((id, bytes)) = from_sub_connections_receiver.recv() => {
                let msg: CCCMsg<S, E> = CCCMsg::SubConnMsg { id, bytes, _encoder: Default::default() };
                conn.send(msg).await?;
            }
        }
    }
}

pub async fn authenticate_conn<E: Encoder>(
    credentials: Token,
    conn: &mut impl ConnectionIO<E>,
) -> Result<(User, ServerInfo), Closed> {
    let client_hello = ClientHello { credentials };
    conn.send(client_hello).await?;
    let ServerHello { user, server_info } = conn.next::<Result<ServerHello, Closed>>().await??;
    Ok((user, server_info))
}
