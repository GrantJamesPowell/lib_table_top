use crate::connection::{ConnectionIO, RawConnection, SubConnId, SubConnection};
use crate::messages::closed::Closed;
use crate::messages::conn_ctrl::{
    ClientConnControlMsg as CCCMsg, ServerConnControlMsg as SCCMsg, SubConnMode,
};
use crate::messages::hello::{ClientHello, ServerHello, ServerInfo};
use crate::{Token, User};
use async_trait::async_trait;
use bytes::Bytes;
use lttcore::encoder::Encoder;
use std::collections::HashMap;
use tokio::select;
use tokio::sync::mpsc;

#[async_trait]
pub trait Job<E: Encoder> {
    async fn run(self, sub_conn: SubConnection<E>);
    fn game_type(&self) -> &'static str;
    fn sub_conn_mode(&self) -> SubConnMode;
}

struct State<E: Encoder> {
    pending: HashMap<SubConnId, Box<dyn Job<E>>>,
    running: HashMap<SubConnId, mpsc::UnboundedSender<Bytes>>,
}

pub async fn client_connection<E: Encoder>(
    credentials: Token,
    max_concurrency: u8,
    mut jobs: impl Iterator<Item = Box<dyn Job<E>>>,
    mut conn: impl RawConnection<E>,
) -> Result<Closed, Closed> {
    let (_user, server_info) = authenticate_conn(credentials, &mut conn).await?;
    let concurrency: usize = server_info.max_sub_connections.min(max_concurrency).into();
    let mut state: State<E> = State {
        pending: HashMap::new(),
        running: HashMap::new(),
    };

    let (from_sub_connections_sender, mut from_sub_connections_receiver) =
        mpsc::unbounded_channel::<(SubConnId, Bytes)>();

    for _ in 0..concurrency {
        if let Some(job) = jobs.next() {
            let id = SubConnId::new();
            conn.send(CCCMsg::StartSubConn {
                id,
                game_type: job.game_type().to_string(),
            })
            .await?;
            state.pending.insert(id, job);
        }
    }

    loop {
        select! {
            biased;
            msg = conn.next::<SCCMsg>() => {
                match msg? {
                    SCCMsg::SubConnStarted { id, .. } => {
                        let job = state.pending
                            .remove(&id)
                            .expect("server only sends us pending jobs");

                        let (sender, receiver) = mpsc::unbounded_channel();

                        let mut sub_conn: SubConnection<E> = SubConnection {
                            id,
                            receiver,
                            sender: Some(from_sub_connections_sender.clone()),
                            _encoder: Default::default()
                        };

                        sub_conn.send(job.sub_conn_mode()).await?;
                        state.running.insert(id, sender);
                        // tokio::spawn(async { fun(sub_conn) });
                    }
                    SCCMsg::SubConnMsg { id, bytes } => {
                        match state.running.get(&id).map(|sender| sender.send(bytes)) {
                            Some(Ok(())) => continue,
                            Some(Err(_)) => {
                                todo!("job failed")
                            },
                            None => {
                                panic!("we forgot to put conn id in the jobs map")
                            }

                        }
                    }
                    SCCMsg::SubConnClosed { id, reason: _ } => {
                        state.pending.remove(&id);
                        state.running.remove(&id);

                        if let Some(job) = jobs.next() {
                            let id = SubConnId::new();
                            conn.send(CCCMsg::StartSubConn { id, game_type: job.game_type().to_string() }).await?;
                            state.pending.insert(id, job);
                        } else if state.pending.is_empty() && state.running.is_empty() {
                            return Ok(Closed::Normal)
                        }
                    }
                }
            }
            Some((id, bytes)) = from_sub_connections_receiver.recv() => {
                conn.send(CCCMsg::SubConnMsg { id, bytes }).await?;
            }
        }
    }
}

pub async fn authenticate_conn<E: Encoder>(
    credentials: Token,
    conn: &mut impl ConnectionIO<E>,
) -> Result<(User, ServerInfo), Closed> {
    conn.send(ClientHello { credentials }).await?;
    let ServerHello { user, server_info } = conn.next::<Result<ServerHello, Closed>>().await??;
    Ok((user, server_info))
}
