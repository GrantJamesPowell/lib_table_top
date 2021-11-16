use crate::auth::Authenticate;
use crate::connection::{ConnectionIO, RawConnection, SubConnId, SubConnection};
use crate::messages::{
    closed::Closed,
    conn_ctrl::{ClientConnControlMsg as CCCMsg, ServerConnControlMsg as SCCMsg},
    hello::{ClientHello, ServerHello, ServerInfo},
};
use crate::SupportedGames;
use crate::User;
use bytes::Bytes;
use lttcore::encoder::Encoder;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::select;
use tokio::sync::mpsc;

pub async fn run_server_connection<Games, Enc, Conn, Auth>(
    authenticate: Auth,
    server_info: &ServerInfo,
    runtimes: Arc<Games::Runtimes>,
    mut conn: Conn,
) -> Result<Closed, Closed>
where
    Games: SupportedGames<Enc>,
    Enc: Encoder,
    Conn: RawConnection<Enc>,
    Auth: Authenticate,
{
    let _user = authenticate_conn(&authenticate, server_info, &mut conn).await?;
    let (from_sub_connections_sender, mut from_sub_connections_receiver) =
        mpsc::unbounded_channel::<(SubConnId, Bytes)>();

    let mut sub_connections: HashMap<SubConnId, mpsc::UnboundedSender<Bytes>> = Default::default();

    loop {
        select! {
            biased;
            msg = conn.next::<CCCMsg>() => {
                match msg? {
                    CCCMsg::StartSubConn { id, game_type, .. } => {
                        if sub_connections.contains_key(&id) {
                            // Sub connection id was taken
                            let msg = SCCMsg::SubConnClosed { id, reason: Closed::ClientError(format!("subconnection {} already exists", id))};
                            conn.send(msg).await?;
                            continue
                        }

                        match Games::try_from_str(&game_type) {
                            Some(game_type) => {
                                let (sender, receiver) = mpsc::unbounded_channel();

                                let sub_conn: SubConnection<Enc> = SubConnection {
                                    id,
                                    receiver,
                                    sender: Some(from_sub_connections_sender.clone()),
                                    _encoder: Default::default(),
                                };

                                sub_connections.insert(id, sender);
                                tokio::spawn(game_type.run_server_sub_conn(sub_conn, Arc::clone(&runtimes)));
                                conn.send(SCCMsg::SubConnStarted { id }).await?;
                            }
                            None => {
                                // game_type wasn't a supported game
                                let reason = Closed::Unsupported(format!("game type '{}' is not supported", game_type));
                                conn.send(SCCMsg::SubConnClosed { id, reason }).await?;
                            }
                        }
                    }
                    CCCMsg::SubConnMsg { id, bytes, .. } => {
                        match sub_connections.get(&id).map(|sender| sender.send(bytes)) {
                            Some(Ok(())) => continue,
                            Some(Err(_)) => {
                                // The server sub conn state machine died
                                sub_connections.remove(&id);
                                conn.send(SCCMsg::SubConnClosed { id, reason: Closed::ServerError }).await?;
                            }
                            None => {
                                // The client sent a message to a dead/non-existant server sub connection
                                let err = format!("subconnection {} doesn't exist", id);
                                conn.send(SCCMsg::SubConnClosed { id, reason: Closed::ClientError(err) }).await?;
                            }
                        }
                    }
                }
            }
            Some((id, bytes)) = from_sub_connections_receiver.recv() => {
                conn.send(SCCMsg::SubConnMsg { id, bytes }).await?;
            }
        }
    }
}

pub async fn authenticate_conn<Enc: Encoder>(
    auth: &dyn Authenticate,
    server_info: &ServerInfo,
    conn: &mut impl ConnectionIO<Enc>,
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
            conn.close().await;
            Err(Closed::InvalidCredentials)
        }
    }
}
