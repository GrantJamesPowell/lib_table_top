use crate::auth::Authenticate;
use crate::connection::{ConnectionIO, RawConnection};
use crate::id::SubConnectionId;
use crate::messages::{
    closed::Closed,
    conn_ctrl::{ClientConnControlMsg as CCCMsg, ServerConnControlMsg as SCCMsg},
    hello::{ClientHello, ServerHello, ServerInfo},
};
use crate::SupportedGames;
use crate::User;
use bytes::Bytes;
use lttcore::encoder::Encoder;
use lttruntime::Runtime;
use std::collections::HashMap;
use tokio::sync::mpsc::UnboundedSender;

pub async fn server_connection<S: SupportedGames, E: Encoder, R: RawConnection<E>>(
    authenticate: &impl Authenticate,
    server_info: &ServerInfo,
    conn: R,
) -> Closed {
    let mut conn: ConnectionIO<R, E> = conn.into();

    let user = match authenticate_conn(authenticate, server_info, &mut conn).await {
        Ok(user) => user,
        Err(closed) => {
            conn.close().await;
            return closed;
        }
    };

    let sub_connections: HashMap<SubConnectionId, UnboundedSender<Bytes>> = Default::default();

    while let Ok(msg) = conn.next::<CCCMsg<S>>().await {
        match msg {
            CCCMsg::StartSubConn {
                id: _,
                game_type: _,
            } => {
                todo!()
            }
            CCCMsg::SubConnMsg { id, bytes } => {
                match sub_connections.get(&id).map(|sender| sender.send(bytes)) {
                    Some(Ok(())) => continue,
                    Some(Err(_)) => {
                        todo!("When the sender fails")
                    }
                    None => {
                        todo!("client sent an invalid sub connection id")
                    }
                }
            }
        }
    }

    Closed::Normal
}

pub async fn authenticate_conn<R: RawConnection<E>, E: Encoder>(
    auth: &impl Authenticate,
    server_info: &ServerInfo,
    conn: &mut ConnectionIO<R, E>,
) -> Result<User, Closed> {
    let ClientHello { credentials } = conn.next().await?;

    match auth.authenticate(&credentials).await {
        Some(user) => {
            let hello: Result<ServerHello, Closed> = Ok(ServerHello {
                user: user.clone(),
                server_info: server_info.clone(),
            });
            conn.send(hello).await?;
            Ok(user)
        }
        None => {
            let err: Result<ServerHello, Closed> = Err(Closed::InvalidCredentials);
            conn.send(err).await?;
            Err(Closed::InvalidCredentials)
        }
    }
}
