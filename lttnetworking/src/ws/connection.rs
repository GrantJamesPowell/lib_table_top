use crate::connection::RawConnection;
use crate::messages::closed::Closed;
use async_trait::async_trait;
use bytes::Bytes;
use futures_util::{SinkExt, StreamExt};
use lttcore::encoder::Encoding;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;

pub struct WSConnection<S>
where
    S: AsyncRead + AsyncWrite + Unpin + Send + Sync,
{
    ws: WebSocketStream<S>,
    encoding: Encoding,
}

impl<S> From<WebSocketStream<S>> for WSConnection<S>
where
    S: AsyncRead + AsyncWrite + Unpin + Send + Sync,
{
    fn from(ws: WebSocketStream<S>) -> Self {
        Self {
            ws,
            encoding: Encoding::Bincode,
        }
    }
}

#[async_trait]
impl<S> RawConnection for WSConnection<S>
where
    S: AsyncRead + AsyncWrite + Unpin + Send + Sync,
{
    fn encoding(&self) -> Encoding {
        self.encoding
    }

    async fn close(&mut self) {
        let _ = self.ws.close(None).await;
    }

    async fn next_bytes(&mut self) -> Result<Bytes, Closed> {
        match self.ws.next().await {
            Some(Ok(msg)) => Ok(msg.into_data().into()),
            _ => Err(Closed::Hangup),
        }
    }

    async fn send_bytes(&mut self, bytes: Bytes) -> Result<(), Closed> {
        let bytes: Vec<u8> = bytes.as_ref().into();

        self.ws
            .send(Message::binary(bytes))
            .await
            .map_err(|_| Closed::Hangup)
    }
}
