use crate::auth::Authenticate;
use crate::connection::{ConnectionIO, RawConnection, SubConnection, SubConnectionId};
use crate::messages::{
    closed::Closed,
    conn_ctrl::{ClientConnControlMsg as CCCMsg, ServerConnControlMsg as SCCMsg},
    hello::{ClientHello, ServerHello, ServerInfo},
};
use crate::SupportedGames;
use crate::User;
use bytes::Bytes;
use lttcore::{encoder::Encoder, Play};
use lttruntime::Runtime;
use std::collections::HashMap;
use tokio::select;
use tokio::sync::mpsc;

pub async fn server_connection<S: SupportedGames, E: Encoder>(
    authenticate: &impl Authenticate,
    server_info: &ServerInfo,
    mut conn: impl RawConnection<E>,
) -> Closed {
    let user = match authenticate_conn(authenticate, server_info, &mut conn).await {
        Ok(user) => user,
        Err(closed) => {
            conn.close().await;
            return closed;
        }
    };

    let (from_sub_connections_sender, mut from_sub_connections_receiver) =
        mpsc::unbounded_channel::<(SubConnectionId, Bytes)>();

    let mut sub_connections: HashMap<SubConnectionId, mpsc::UnboundedSender<Bytes>> =
        Default::default();

    loop {
        select! {
            biased;
            msg = conn.next::<CCCMsg<S>>() => {
                match msg {
                    Ok(CCCMsg::StartSubConn { id, game_type }) => {
                        if !sub_connections.contains_key(&id) {
                            let (sender, receiver) = mpsc::unbounded_channel();

                            let sub_conn: SubConnection<E> = SubConnection {
                                id,
                                receiver,
                                sender: Some(from_sub_connections_sender.clone()),
                                _encoder: Default::default(),
                            };

                            sub_connections.insert(id, sender);
                            tokio::spawn(game_type.run_server_sub_conn(sub_conn));

                            let msg: SCCMsg<S> = SCCMsg::SubConnStarted { id, game_type };

                            if let Err(closed) = conn.send(msg).await {
                                return closed
                            }
                        } else {
                            // Client provided an already in use sub connection id
                            let msg: SCCMsg<S> = SCCMsg::SubConnClosed { id, reason: Closed::ClientError(format!("{:?} already exists", id)) };

                            if let Err(closed) = conn.send(msg).await {
                                return closed
                            }
                        }
                    }
                    Ok(CCCMsg::SubConnMsg { id, bytes }) => {
                        match sub_connections.get(&id).map(|sender| sender.send(bytes)) {
                            Some(Ok(())) => continue,
                            Some(Err(_)) => {
                                // The server sub conn state machine died
                                let msg: SCCMsg<S> = SCCMsg::SubConnClosed { id, reason: Closed::ServerError };
                                sub_connections.remove(&id);

                                if let Err(closed) = conn.send(msg).await {
                                    return closed
                                }
                            }
                            None => {
                                // The client sent a message to a dead/non-existant server sub connection
                                let msg: SCCMsg<S>= SCCMsg::SubConnClosed { id, reason: Closed::ClientError(format!("{:?} doesn't exist", id)) };

                                if let Err(closed) = conn.send(msg).await {
                                    return closed
                                }
                            }
                        }
                    }
                    Err(closed) => {
                        // The Client closed the connection
                        return closed
                    }
                }
            }
            Some((id, bytes)) = from_sub_connections_receiver.recv() => {
                let msg: SCCMsg<S> = SCCMsg::SubConnMsg { id, bytes };
                if let Err(_closed) = conn.send(msg).await {
                    return Closed::Normal
                }
            }
        }
    }
}

pub async fn authenticate_conn<E: Encoder>(
    auth: &impl Authenticate,
    server_info: &ServerInfo,
    conn: &mut impl ConnectionIO<E>,
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

pub async fn run_server_sub_conn<T: Play, E: Encoder, C: ConnectionIO<E>>(conn: C) {}
