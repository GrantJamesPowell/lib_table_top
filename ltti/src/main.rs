use futures_util::{future, pin_mut, Sink, SinkExt, Stream, StreamExt, TryStreamExt};
use std::io::Error;
use std::pin::Pin;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite as ws;
use ws::tungstenite::protocol::Message;

const ADDR: &'static str = "127.0.0.1:8080";

#[tokio::main]
async fn main() -> Result<(), Error> {
    let server = tokio::spawn(async {
        let listener = TcpListener::bind(ADDR).await.expect("foobar");
        println!("Listening on: {}", ADDR);
        while let Ok((stream, _)) = listener.accept().await {
            tokio::spawn(accept_connection(stream));
        }
    });

    for i in 1..10 {
        let join = tokio::spawn(async move {
            let (mut ws_stream, _) = ws::connect_async(format!("ws://{}/", ADDR))
                .await
                .expect("Failed to connect");
            println!("WebSocket handshake {:?} completed", i);

            let msg = Message::text("foo bar baz");
            ws_stream.send(msg).await;
            if let Some(msg) = ws_stream.next().await {
                println!(
                    "Received {:?}\n",
                    msg.map(|x| x.into_text().unwrap()).unwrap()
                );
            }
            ws_stream.close(None).await.expect("closed properly");
        });

        join.await?;
    }

    server.await?;

    Ok(())
}

async fn accept_connection(stream: TcpStream) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    println!("Peer address: {}", addr);

    let ws_stream = ws::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    println!("New WebSocket connection: {}", addr);

    let (write, read) = ws_stream.split();
    // We should not forward messages other than text or binary.
    read.try_filter(|msg| future::ready(msg.is_text() || msg.is_binary()))
        .forward(write)
        .await
        .expect("Failed to forward messages")
}
