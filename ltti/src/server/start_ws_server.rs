use async_trait::async_trait;
use futures_util::{Sink, SinkExt, Stream, StreamExt};
use lttnetworking::{
    hello::{process_client_hello, Auth},
    Token, User,
};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use tokio_tungstenite::tungstenite::Message;

struct DummyAuth;

#[async_trait]
impl Auth for DummyAuth {
    async fn authorize(_token: Token) -> Option<User> {
        Some(User {
            username: "Grant".into(),
            user_id: uuid::Uuid::nil(),
        })
    }
}

pub async fn start_ws_server(addr: impl ToSocketAddrs) -> anyhow::Result<()> {
    let listener = TcpListener::bind(addr).await?;

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection::<DummyAuth>(stream));
    }

    Ok(())
}

async fn accept_connection<Authorizer: Auth>(stream: TcpStream) -> anyhow::Result<()> {
    let mut ws = tokio_tungstenite::accept_async(stream).await?;
    let buf: Vec<u8> = Vec::new();

    if let Some(Ok(msg)) = ws.next().await {
        let result = process_client_hello::<Authorizer>(&msg.into_data(), buf).await;

        match result {
            Ok((_user, output)) => {
                let msg = Message::binary(output);
                ws.send(msg).await?;
                todo!()
            }
            Err(output) => {
                let msg = Message::binary(output);
                ws.send(msg).await?;
                todo!()
            }
        }
    }

    ws.close(None).await?;

    Ok(())
}
