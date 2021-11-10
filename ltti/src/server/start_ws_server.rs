use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

pub async fn start_ws_server(addr: impl ToSocketAddrs) -> anyhow::Result<()> {
    let listener = TcpListener::bind(addr).await?;

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }

    Ok(())
}

async fn accept_connection(_stream: TcpStream) -> anyhow::Result<()> {
    // let mut ws = tokio_tungstenite::accept_async(stream).await?;
    // let buf: Vec<u8> = Vec::new();

    // if let Some(Ok(msg)) = ws.next().await {
    //     let result = process_client_hello(authorize, &msg.into_data(), buf).await;

    //     match result {
    //         Ok((_user, output)) => {
    //             let msg = Message::binary(output);
    //             ws.send(msg).await?;
    //             todo!()
    //         }
    //         Err(output) => {
    //             let msg = Message::binary(output);
    //             ws.send(msg).await?;
    //             todo!()
    //         }
    //     }
    // }

    // ws.close(None).await?;

    Ok(())
}
