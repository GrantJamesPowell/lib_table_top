use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use lttnetworking::interface::ConnectionIO;
use lttnetworking::messages::Closed;
use serde::{de::DeserializeOwned, Serialize};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;

pub struct WebSocketServerConnection<S: AsyncRead + AsyncWrite + Unpin + Send> {
    ws: WebSocketStream<S>,
}

#[async_trait]
impl<S: AsyncRead + AsyncWrite + Unpin + Send> ConnectionIO for WebSocketServerConnection<S> {
    async fn close(&mut self) {
        let _ = self.ws.close(None).await;
    }

    async fn next<T: Send + DeserializeOwned>(&mut self) -> Result<T, Closed> {
        let msg = match self.ws.next().await {
            Some(Ok(msg)) => msg,
            _ => {
                self.close().await;
                return Err(Closed::Hangup);
            }
        };

        match bincode::deserialize::<T>(&msg.into_data()) {
            Ok(msg) => Ok(msg),
            Err(_) => {
                self.close().await;
                return Err(Closed::InvalidMsg);
            }
        }
    }

    async fn send<T: Send + Serialize>(&mut self, msg: T) -> Result<(), Closed> {
        let msg = bincode::serialize(&msg).unwrap();
        self.ws
            .send(Message::binary(msg))
            .await
            .map_err(|_| Closed::Hangup)
    }
}
