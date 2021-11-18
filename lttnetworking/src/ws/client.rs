use crate::Token;
use crate::client::Job;
use crate::client::run_client_connection;
use crate::messages::closed::Closed;
use crate::ws::connection::WSConnection;
use lttcore::encoder::Encoder;
use url::Url;

pub async fn run_jobs<Enc>(
    addr: Url,
    credentials: Token,
    max_concurrency: u8,
    jobs: impl Iterator<Item = Box<dyn Job<Enc>>>,
) -> Result<Closed, Closed>
where
    Enc: Encoder,
{
    let (ws, _) = tokio_tungstenite::connect_async(addr)
        .await
        .map_err(|_| Closed::Hangup)?;

    let conn: WSConnection<_, Enc> = ws.into();

    run_client_connection(
        credentials,
        max_concurrency,
        jobs,
        conn
    ).await
}
