use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct TurnNum(u64);

impl From<TurnNum> for u64 {
    fn from(turn_num: TurnNum) -> u64 {
        turn_num.0
    }
}

impl TurnNum {
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}
