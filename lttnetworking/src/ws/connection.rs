use crate::connection::RawConnection;
use crate::messages::closed::Closed;
use async_trait::async_trait;
use bytes::Bytes;
use futures_util::{SinkExt, StreamExt};
use lttcore::encoder::Encoder;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;

impl<S, E> From<WebSocketStream<S>> for WSConnection<S, E>
where
    S: AsyncRead + AsyncWrite + Unpin + Send + Sync,
    E: Encoder,
{
    fn from(ws: WebSocketStream<S>) -> Self {
        Self {
            ws,
            _encoder: Default::default(),
        }
    }
}

pub struct WSConnection<S, E>
where
    S: AsyncRead + AsyncWrite + Unpin + Send + Sync,
    E: Encoder,
{
    ws: WebSocketStream<S>,
    _encoder: std::marker::PhantomData<E>,
}

#[async_trait]
impl<S, E> RawConnection<E> for WSConnection<S, E>
where
    S: AsyncRead + AsyncWrite + Unpin + Send + Sync,
    E: Encoder,
{
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
