use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct TurnNum(u64);

impl From<u64> for TurnNum {
    fn from(n: u64) -> Self {
        Self(n)
    }
}

impl Into<u64> for TurnNum {
    fn into(self) -> u64 {
        self.0
    }
}

impl TurnNum {
    pub fn as_u64(&self) -> u64 {
        self.0
    }

    pub fn increment(&mut self) {
        self.0 += 1;
    }
}
