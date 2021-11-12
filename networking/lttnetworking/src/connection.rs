use crate::messages::Closed;
use async_trait::async_trait;
use lttcore::id::ConnectionId;
use serde::{de::DeserializeOwned, Serialize};

#[async_trait]
pub trait ConnectionIO {
    async fn next<T: Send + DeserializeOwned>(&mut self) -> Result<T, Closed>;
    async fn send<T: Send + Serialize>(&mut self, msg: T) -> Result<(), Closed>;
    async fn close(&mut self);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FromConnection<T: DeserializeOwned> {
    pub from: ConnectionId,
    pub msg: T,
}

// #[derive(Default, Debug, Clone, PartialEq, Eq)]
// pub struct Connections(SmallVec<[ConnectionId; 4]>);
//
// impl Connections {
//     pub fn new(connections: impl IntoIterator<Item = ConnectionId>) -> Self {
//         connections.into_iter().collect()
//     }
//
//     pub fn contains(&self, connection_id: ConnectionId) -> bool {
//         self.0.binary_search(&connection_id).is_ok()
//     }
//
//     pub fn len(&self) -> usize {
//         self.0.len()
//     }
//
//     pub fn is_empty(&self) -> bool {
//         self.0.is_empty()
//     }
// }
//
// impl From<ConnectionId> for Connections {
//     fn from(connection_id: ConnectionId) -> Self {
//         [connection_id].into_iter().collect()
//     }
// }
//
// impl FromIterator<ConnectionId> for Connections {
//     fn from_iter<I: IntoIterator<Item = ConnectionId>>(iter: I) -> Self {
//         let mut conns: SmallVec<[ConnectionId; 4]> = iter.into_iter().collect();
//         conns.sort_unstable();
//         Self(conns)
//     }
// }
//
// impl IntoIterator for Connections {
//     type Item = ConnectionId;
//     type IntoIter = impl Iterator<Item = ConnectionId>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         self.0.into_iter()
//     }
// }
