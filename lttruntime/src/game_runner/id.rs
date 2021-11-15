use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConnectionId(u64);

#[derive(Debug)]
pub(crate) struct ConnectionIdSource(AtomicU64);

impl Default for ConnectionIdSource {
    fn default() -> Self {
        ConnectionIdSource::new()
    }
}

impl ConnectionIdSource {
    pub fn new() -> Self {
        Self(AtomicU64::new(0))
    }

    pub fn next(&self) -> ConnectionId {
        ConnectionId(self.0.fetch_add(1, Ordering::Relaxed))
    }
}
