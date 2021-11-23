use super::connection::WSConnection;
use crate::auth::Authenticate;
use crate::messages::closed::Closed;
use crate::messages::hello::ServerInfo;
use crate::server::server_connection::run_server_connection;
use crate::SupportedGames;
use lttcore::encoder::Encoder;
use std::sync::Arc;
use tokio::net::TcpStream;

pub async fn accept_connection<Games, Enc, Auth>(
    authenticate: Auth,
    server_info: Arc<ServerInfo>,
    runtimes: Arc<Games::Runtimes>,
    stream: TcpStream,
) -> Result<Closed, Closed>
where
    Games: SupportedGames<Enc>,
    Enc: Encoder,
    Auth: Authenticate,
{
    let ws = tokio_tungstenite::accept_async(stream)
        .await
        .map_err(|_| Closed::Hangup)?;

    let ws: WSConnection<_> = ws.into();
    run_server_connection::<Games, Enc, _, _>(authenticate, &server_info, runtimes, ws).await
}
