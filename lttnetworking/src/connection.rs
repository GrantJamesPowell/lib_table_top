use crate::messages::closed::Closed;
use async_trait::async_trait;
use bytes::Bytes;
use lttcore::encoder::Encoder;
use lttcore::uuid_id;
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::mpsc;

uuid_id!(SubConnId);

#[async_trait]
pub trait ConnectionIO<E: Encoder>: Send + Sync {
    async fn next<T: Send + DeserializeOwned>(&mut self) -> Result<T, Closed>;
    async fn send<T: Send + Serialize>(&mut self, msg: T) -> Result<(), Closed>;
    async fn close(&mut self);
}

#[async_trait]
pub trait RawConnection<E: Encoder>: Sync + Send {
    async fn next_bytes(&mut self) -> Result<Bytes, Closed>;
    async fn send_bytes(&mut self, bytes: Bytes) -> Result<(), Closed>;
    async fn close(&mut self);
}

pub struct SubConnection<E: Encoder> {
    pub id: SubConnId,
    pub receiver: mpsc::UnboundedReceiver<Bytes>,
    pub sender: Option<mpsc::UnboundedSender<(SubConnId, Bytes)>>,
    pub _encoder: std::marker::PhantomData<E>,
}

#[async_trait]
impl<E: Encoder> RawConnection<E> for SubConnection<E> {
    async fn next_bytes(&mut self) -> Result<Bytes, Closed> {
        self.receiver.recv().await.ok_or(Closed::Hangup)
    }

    async fn send_bytes(&mut self, bytes: Bytes) -> Result<(), Closed> {
        let sender = self.sender.as_ref().ok_or(Closed::Hangup)?;
        sender.send((self.id, bytes)).map_err(|_| Closed::Hangup)
    }

    async fn close(&mut self) {
        self.receiver.close();
        self.sender.take();
    }
}

#[async_trait]
impl<E: Encoder, R: RawConnection<E>> ConnectionIO<E> for R {
    async fn next<T: Send + DeserializeOwned>(&mut self) -> Result<T, Closed> {
        let bytes = match self.next_bytes().await {
            Ok(bytes) => bytes,
            _ => {
                self.close().await;
                return Err(Closed::Hangup);
            }
        };

        match E::deserialize::<T>(bytes) {
            Ok(msg) => Ok(msg),
            Err(_) => {
                self.close().await;
                return Err(Closed::InvalidMsg);
            }
        }
    }

    async fn send<T: Send + Serialize>(&mut self, msg: T) -> Result<(), Closed> {
        let serialized = E::serialize(&msg).map_err(|_| Closed::ServerError)?;
        self.send_bytes(serialized).await
    }

    async fn close(&mut self) {
        self.close().await;
    }
}
