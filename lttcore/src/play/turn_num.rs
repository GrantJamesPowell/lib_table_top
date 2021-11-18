use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct TurnNum(u64);

impl From<u64> for TurnNum {
    fn from(n: u64) -> Self {
        Self(n)
    }
}

impl From<TurnNum> for u64 {
    fn from(turn_num: TurnNum) -> u64 {
        turn_num.0
    }
}

impl TurnNum {
    pub fn increment(&mut self) {
        self.0 = self
            .0
            .checked_add(1)
            .expect("turn number does not overflow");
    }
}
