use serde::Serialize;
use smallvec::SmallVec;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ConnectionId(Uuid);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionMsg<T: Serialize> {
    pub to: Connections,
    pub msg: T,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManageConnections {
    Add(Connections),
    Remove(Connections),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Connections(SmallVec<[ConnectionId; 4]>);

impl Connections {
    pub fn contains(&self, connection_id: ConnectionId) -> bool {
        self.0.binary_search(&connection_id).is_ok()
    }
}

impl FromIterator<ConnectionId> for Connections {
    fn from_iter<I: IntoIterator<Item = ConnectionId>>(iter: I) -> Self {
        let mut conns: SmallVec<[ConnectionId; 4]> = iter.into_iter().collect();
        conns.sort_unstable();
        Self(conns)
    }
}

impl IntoIterator for Connections {
    type Item = ConnectionId;
    type IntoIter = impl Iterator<Item = ConnectionId>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
