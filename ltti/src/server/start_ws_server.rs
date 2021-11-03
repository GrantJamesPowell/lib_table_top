use anyhow::Result;
use futures_util::StreamExt;
use lttnetworking::messages::hello::ClientHello;
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

pub async fn start_ws_server(addr: impl ToSocketAddrs) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }

    Ok(())
}

async fn accept_connection(stream: TcpStream) -> Result<()> {
    let mut stream = tokio_tungstenite::accept_async(stream).await?;

    if let Some(msg) = stream.next().await {
        let msg = msg?.into_data();

        match bincode::deserialize::<ClientHello>(&msg) {
            Ok(_client_hello) => todo!(),
            Err(_err) => todo!(),
        }
    }

    stream.close(None).await?;

    Ok(())
}
