use crate::messages::closed::Closed;
use async_trait::async_trait;
use bytes::Bytes;
use lttcore::encoder::Encoder;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::sync::mpsc;
use uuid::Uuid;

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SubConnectionId(Uuid);

#[async_trait]
pub trait RawConnection<E: Encoder> {
    async fn next_bytes(&mut self) -> Result<Bytes, Closed>;
    async fn send_bytes(&mut self, bytes: Bytes) -> Result<(), Closed>;
    async fn close(&mut self);
}

pub struct ConnectionIO<R: RawConnection<E>, E: Encoder> {
    raw: R,
    _phantom: std::marker::PhantomData<E>,
}

impl<R: RawConnection<E>, E: Encoder> From<R> for ConnectionIO<R, E> {
    fn from(raw: R) -> Self {
        Self {
            raw,
            _phantom: Default::default(),
        }
    }
}

pub struct SubConnection;

impl<R: RawConnection<E>, E: Encoder> ConnectionIO<R, E> {
    pub async fn next<T: Send + DeserializeOwned>(&mut self) -> Result<T, Closed> {
        let bytes = match self.raw.next_bytes().await {
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

    pub async fn send<T: Send + Serialize>(&mut self, msg: T) -> Result<(), Closed> {
        todo!()
    }

    pub async fn close(&mut self) {
        self.raw.close().await;
    }
}
