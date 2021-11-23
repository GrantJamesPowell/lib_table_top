use crate::client::run_client_connection;
use crate::client::Job;
use crate::messages::closed::Closed;
use crate::ws::connection::WSConnection;
use crate::Token;

use url::Url;

pub async fn run_jobs<Jobs>(
    addr: Url,
    credentials: Token,
    max_concurrency: u8,
    jobs: Jobs,
) -> Result<Closed, Closed>
where
    Jobs: Iterator<Item = Box<dyn Job>>,
{
    let (ws, _) = tokio_tungstenite::connect_async(addr)
        .await
        .map_err(|_| Closed::Hangup)?;

    let conn: WSConnection<_> = ws.into();

    run_client_connection(credentials, max_concurrency, jobs, conn).await
}
